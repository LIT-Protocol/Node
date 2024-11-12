use crate::{
    error::{conversion_err_code, unexpected_err, unexpected_err_code, Result, EC},
    models::EthBlock,
};
use ethers::providers::Middleware;
use rusqlite::{params, Connection};
use tokio::sync::mpsc;

use crate::siwe_db::rpc::fetch_block_from_hash;

const INDEXER_URL: &str = "http://block-indexer.litgateway.com/get_all_valid_blocks";

fn db_conn(port: u16) -> Result<Connection> {
    let in_container = std::env::var("IN_CONTAINER").unwrap_or("0".to_string()) == "1";
    // if running in a container with a volume mount used where this file will be created.
    // the nodes will not be able to access their respective database files due to a lock being held
    // which will not be released. By initalizing the database in a directory within the container
    // allows the nodes to read and write to the database over this connection
    if in_container {
        Connection::open(format!("/var/tmp/node_{}.db", port))
            .map_err(|e| unexpected_err_code(e, EC::NodeSystemFault, None))
    } else {
        Connection::open(format!("node_{}.db", port))
            .map_err(|e| unexpected_err_code(e, EC::NodeSystemFault, None))
    }
}

pub fn db_initial_setup(port: u16) -> Result<()> {
    let conn = db_conn(port)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS
            blockhash_timestamp(
                blockhash TEXT PRIMARY KEY,
                timestamp TEXT NOT NULL,
                block_number INTEGER NOT NULL
            )",
        [],
    )
    .map_err(|e| unexpected_err_code(e, EC::NodeSystemFault, None))?;

    Ok(())
}

fn db_batch_write(conn: &mut Connection, block_records: &Vec<EthBlock>) -> Result<()> {
    let tx = conn
        .transaction()
        .map_err(|e| unexpected_err(e, Some("Error init transaction statement".to_string())))?;

    {
        let mut stmt = tx
            .prepare(
                "INSERT OR REPLACE INTO blockhash_timestamp(
            blockhash,
            timestamp,
            block_number
        )
        VALUES (?, ?, ?)",
            )
            .map_err(|e| {
                unexpected_err(e, Some("Error preparing transaction statement".to_string()))
            })?;

        for record in block_records {
            stmt.execute(params![
                record.blockhash,
                record.timestamp,
                record.block_number
            ])
            .map_err(|e| {
                unexpected_err(e, Some("Error executing transaction statement".to_string()))
            })?;
        }
    }

    tx.commit().map_err(|e| {
        unexpected_err(
            e,
            Some("Error committing transaction statement".to_string()),
        )
    })
}

pub async fn init_fill_db(port: u16, quit_rx: mpsc::Receiver<bool>) -> Result<()> {
    let blocks: Vec<EthBlock> = reqwest::get(INDEXER_URL)
        .await
        .map_err(|e| {
            unexpected_err(
                e,
                Some("Unable to get Eth blocks info from the indexer".into()),
            )
        })?
        .json::<Vec<EthBlock>>()
        .await
        .map_err(|e| {
            conversion_err_code(
                e,
                EC::NodeSerializationError,
                Some("Unable to deserialize indexer response into JSON".into()),
            )
        })?;

    let mut conn = db_conn(port)?;
    let _ = db_batch_write(&mut conn, &blocks);

    Ok(())
}

async fn fetch_and_store_block_info<M: Middleware + 'static>(
    block_hash: &str,
    port: u16,
    rpc_provider: M,
) -> Result<EthBlock> {
    let conn = db_conn(port)?;

    let block = fetch_block_from_hash(block_hash, rpc_provider).await?;

    conn.execute(
        "INSERT OR REPLACE INTO blockhash_timestamp(
            blockhash,
            timestamp,
            block_number
        ) VALUES (?1, ?2, ?3)",
        params![block.blockhash, &block.timestamp, block.block_number],
    )
    .map_err(|e| unexpected_err_code(e, EC::NodeSystemFault, None))?;

    Ok(block)
}

pub fn last_n_blocks(port: u16, n: usize) -> Result<Vec<EthBlock>> {
    let conn = db_conn(port)?;
    let mut stmt = conn.prepare(
        "SELECT blockhash, timestamp, block_number FROM blockhash_timestamp ORDER BY block_number DESC LIMIT ?",
    )
    .map_err(|e| unexpected_err_code(e, EC::NodeSystemFault, None))?;
    let blockhashes = stmt
        .query_map([&n], |row| {
            Ok(EthBlock {
                blockhash: row.get(0)?,
                timestamp: row.get(1)?,
                block_number: row.get(2)?,
            })
        })
        .map_err(|e| unexpected_err_code(e, EC::NodeSystemFault, None))?;

    let collected_blockhashes = blockhashes
        .collect::<std::result::Result<Vec<EthBlock>, rusqlite::Error>>()
        .map_err(|e| unexpected_err_code(e, EC::NodeSystemFault, None))?;
    Ok(collected_blockhashes)
}

pub async fn retrieve_and_store_blockhash<M: Middleware + 'static>(
    block_hash: String,
    port: u16,
    rpc_provider: M,
) -> Result<EthBlock> {
    let conn = db_conn(port)?;

    let query_res = conn.query_row(
        "SELECT blockhash, timestamp, block_number FROM blockhash_timestamp WHERE blockhash=?",
        [&block_hash],
        |row| {
            Ok(EthBlock {
                blockhash: row.get(0)?,
                timestamp: row.get(1)?,
                block_number: row.get(2)?,
            })
        },
    );

    match query_res {
        Ok(block) => Ok(block),
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            fetch_and_store_block_info(&block_hash, port, rpc_provider).await
        }
        Err(e) => Err(unexpected_err_code(e, EC::NodeSystemFault, None)),
    }
}

#[cfg(test)]
mod siwe_db_tests {
    use chrono::{Duration, Utc};
    use ethers::{
        providers::Provider,
        types::{Block, H256, U256, U64},
    };
    use rusqlite::{params, Connection};
    use std::process::Command;
    use tokio::sync::mpsc;

    use crate::{
        siwe_db::{
            db::{
                db_batch_write, db_conn, db_initial_setup, fetch_and_store_block_info,
                init_fill_db, retrieve_and_store_blockhash, EthBlock,
            },
            utils::MAX_TIMESTAMP_VALIDITY_DAYS,
        },
        utils::encoding,
    };

    #[test]
    fn test_db_initial_setup() {
        // Pre clean-up
        remove_db_files(0);

        let port = 0;
        assert!(db_initial_setup(port).is_ok());

        let conn = db_conn(port).unwrap();

        // Check if the table exists
        let count: i64 = conn.query_row(
            "SELECT count(*) FROM sqlite_master WHERE type='table' AND name='blockhash_timestamp'",
            params![],
            |row| row.get(0),
        ).unwrap();
        assert_eq!(count, 1);

        // Check the columns
        let mut stmt = conn
            .prepare("PRAGMA table_info(blockhash_timestamp)")
            .unwrap();
        let rows = stmt
            .query_map(params![], |row| {
                Ok((
                    row.get::<_, String>(1).unwrap(),
                    row.get::<_, String>(2).unwrap(),
                ))
            })
            .unwrap();

        let mut columns = Vec::new();
        for row in rows {
            columns.push(row.unwrap());
        }

        // Check that all expected columns and their types exist
        assert!(columns.contains(&("blockhash".to_string(), "TEXT".to_string())));
        assert!(columns.contains(&("timestamp".to_string(), "TEXT".to_string())));
        assert!(columns.contains(&("block_number".to_string(), "INTEGER".to_string())));

        // Post clean-up
        remove_db_files(0);
    }

    #[test]
    fn test_db_batch_write() {
        // Pre clean-up
        remove_db_files(1);

        let port = 1;
        assert!(db_initial_setup(port).is_ok());

        let mut conn = db_conn(port).unwrap();

        let (
            block_hash1,
            block_hash2,
            block_timestamp,
            block_timestamp_expired,
            block_number,
            mut records,
        ) = add_blocks_to_db(&mut conn);

        retrieve_db_and_assert(&conn, &block_hash1, &block_timestamp, block_number);
        retrieve_db_and_assert(&conn, &block_hash2, &block_timestamp_expired, block_number);

        // Check replacing block with the same block hash as it's the primary key
        let new_block_timestamp = Utc::now().timestamp().to_string();
        let block3 = EthBlock {
            blockhash: block_hash1.clone(),
            timestamp: new_block_timestamp.clone(),
            block_number,
        };
        records.push(block3.clone());
        assert!(db_batch_write(&mut conn, &records).is_ok());

        retrieve_db_and_assert(&conn, &block_hash1, &new_block_timestamp, block_number);

        // Post clean-up
        remove_db_files(1);
    }

    #[tokio::test]
    async fn test_fetch_and_store_block_info() {
        // Pre clean-up
        remove_db_files(2);

        // Set up DB
        let port = 2;
        assert!(db_initial_setup(port).is_ok());

        let mut conn = db_conn(port).unwrap();
        let block_hash = "0x5ca951404c1c7e7b75cf02604f61faf7be7ccfdc3b72c1e7203cdeddff41e1f4";

        let (block_hash1, block_hash2, block_timestamp, _, block_number, _) =
            add_blocks_to_db(&mut conn);

        let mut stmt = conn.prepare("SELECT blockhash, timestamp, block_number FROM blockhash_timestamp WHERE blockhash = ?").unwrap();
        let rows = stmt
            .query_map([block_hash], |row| {
                let blockhash: String = row.get(0).unwrap();
                let timestamp: String = row.get(1).unwrap();
                let block_number: usize = row.get(2).unwrap();
                Ok((blockhash, timestamp, block_number))
            })
            .unwrap();

        // Zero rows since blockhash not in the DB
        let rows_vec = rows.collect::<Vec<_>>();
        assert_eq!(rows_vec.len(), 0);

        // Set up Mock Provider
        let (provider, mock) = Provider::mocked();
        let now = Utc::now().timestamp();
        let block_number: usize = 14402566;

        let rpc_block: Block<H256> = Block {
            hash: Some(H256::from(
                encoding::bytes_to_zero_padded_32(encoding::hex_to_bytes(block_hash).unwrap())
                    .unwrap(),
            )),
            timestamp: U256::from(now),
            number: Some(U64::from(block_number)),
            ..Default::default()
        };

        // NOTE: Requests are retrieved back-to-front irrespective of invalid hashes.
        assert!(mock.push(rpc_block).is_ok());

        let block = fetch_and_store_block_info(block_hash, port, provider)
            .await
            .unwrap();

        // Returned hash & timestamp is stored in the DB
        assert_eq!(&block.blockhash, &block_hash);
        assert_eq!(&block.timestamp, &now.to_string());

        // Now the new blockhash is fetched & stored in our DB
        retrieve_db_and_assert(&conn, block_hash, &now.to_string(), block_number);

        // Post clean-up
        remove_db_files(2);
    }

    #[tokio::test]
    async fn test_retrieve_and_store_blockhash() {
        // Pre clean-up
        remove_db_files(3);

        // Set up DB
        let port = 3;
        assert!(db_initial_setup(port).is_ok());

        let mut conn = db_conn(port).unwrap();
        let block_hash = "0x5ca951404c1c7e7b75cf02604f61faf7be7ccfdc3b72c1e7203cdeddff41e1f4";

        let (block_hash1, block_hash2, block_timestamp, _, block_number, _) =
            add_blocks_to_db(&mut conn);

        // Set up Mock Provider
        let (provider, mock) = Provider::mocked();
        let now = Utc::now().timestamp();
        let block_number: usize = 14402566;

        let rpc_block: Block<H256> = Block {
            hash: Some(H256::from(
                encoding::bytes_to_zero_padded_32(encoding::hex_to_bytes(block_hash).unwrap())
                    .unwrap(),
            )),
            timestamp: U256::from(now),
            number: Some(U64::from(block_number)),
            ..Default::default()
        };

        // NOTE: Requests are retrieved back-to-front irrespective of invalid hashes.
        assert!(mock.push(rpc_block).is_ok());

        // 1. block_hash1 already exists in the DB
        // 2. block_timestamp is valid
        let block = retrieve_and_store_blockhash(block_hash1.clone(), port, provider.clone())
            .await
            .unwrap();

        assert_eq!(&block.blockhash, &block_hash1);
        assert_eq!(&block.timestamp, &block_timestamp);

        // 1. block_hash doesn't exists in the DB, it is fetched from the Mocked Provider
        // 2. block_timestamp is valid
        let block = retrieve_and_store_blockhash(block_hash.to_string(), port, provider.clone())
            .await
            .unwrap();

        assert_eq!(&block.blockhash, &block_hash);
        assert_eq!(&block.timestamp, &now.to_string());

        // Post clean-up
        remove_db_files(3);
    }

    #[tokio::test]
    async fn test_init_fill_db() {
        // Pre clean-up
        remove_db_files(4);

        // Set up DB
        let port = 4;
        assert!(db_initial_setup(port).is_ok());

        let conn = db_conn(port).unwrap();
        let (quit_tx, quit_rx) = mpsc::channel::<bool>(1);

        assert!(init_fill_db(port, quit_rx).await.is_ok());

        let num_rows: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM blockhash_timestamp",
                params![],
                |row| (row.get::<_, i64>(0)),
            )
            .unwrap();

        assert!(num_rows > 200_000);

        // Post clean-up
        remove_db_files(4);
    }

    fn add_blocks_to_db(
        conn: &mut Connection,
    ) -> (String, String, String, String, usize, Vec<EthBlock>) {
        let block_hash1 =
            "0x70dd3646979bc3d49af8ad6320d2b03149a72863f8e08f254e54fa8954f59143".to_string();
        let block_hash2 =
            "0xdc0818cf78f21a8e70579cb46a43643f78291264dda342ae31049421c82d21ae".to_string();
        let block_timestamp = Utc::now().timestamp().to_string();
        let block_timestamp_expired = (Utc::now() - Duration::days(MAX_TIMESTAMP_VALIDITY_DAYS))
            .timestamp()
            .to_string();
        let block_number: usize = 14402566;
        let block1 = EthBlock {
            blockhash: block_hash1.clone(),
            timestamp: block_timestamp.clone(),
            block_number,
        };
        let block2 = EthBlock {
            blockhash: block_hash2.clone(),
            timestamp: block_timestamp_expired.clone(),
            block_number,
        };

        let mut records: Vec<EthBlock> = Vec::new();
        records.push(block1.clone());
        records.push(block2.clone());
        assert!(db_batch_write(conn, &records).is_ok());

        (
            block_hash1,
            block_hash2,
            block_timestamp,
            block_timestamp_expired,
            block_number,
            records,
        )
    }

    fn retrieve_db_and_assert(
        conn: &Connection,
        block_hash: &str,
        expected_timestamp: &str,
        expected_number: usize,
    ) {
        let mut stmt = conn.prepare("SELECT blockhash, timestamp, block_number FROM blockhash_timestamp WHERE blockhash = ?").unwrap();
        let rows = stmt
            .query_map([block_hash], |row| {
                let blockhash: String = row.get(0).unwrap();
                let timestamp: String = row.get(1).unwrap();
                let block_number: usize = row.get(2).unwrap();
                Ok((blockhash, timestamp, block_number))
            })
            .unwrap();

        // Only 1 valid entry despite retry as we replace based on the blockhash (primary key)
        let rows_vec = rows.collect::<Vec<_>>();
        assert_eq!(rows_vec.len(), 1);

        for row in rows_vec.iter() {
            let (hash, timestamp, number) = row.as_ref().unwrap();
            assert_eq!(block_hash, hash);
            assert_eq!(expected_timestamp, timestamp);
            assert_eq!(&expected_number, number);
        }
    }

    // NOTE: We're using different ports for different tests to ensuring that deleting/updating a conn doesn't effect other tests
    fn remove_db_files(port: u16) {
        let _db_cleanup = Command::new("rm")
            .arg(format!("node_{}.db", port))
            .status()
            .expect("Failed to remove test db");
    }
}
