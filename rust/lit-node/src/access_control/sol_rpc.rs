use crate::auth::auth_material::JsonAuthSig;
use crate::auth::validators::solana::validate_solana_auth_sig;
use crate::error::{
    blockchain_err_code, conversion_err, parser_err, unexpected_err_code, validation_err_code,
    Result, EC,
};
use lit_core::error::Unexpected;
use mpl_token_metadata::state::Metadata;
use serde_json::{json, Value};
use solana_client::client_error::ClientErrorKind::RpcError;
use solana_client::{rpc_client::RpcClient, rpc_request::RpcRequest};
use solana_program::borsh::try_from_slice_unchecked;
use solana_sdk::pubkey::Pubkey;
use spl_associated_token_account::get_associated_token_address;
use std::collections::HashMap;
use std::result::Result as StdResult;
use std::str::FromStr;

use super::{substitute_special_params, validate_boolean_expression};
use crate::models::{
    AccessControlBooleanOperator, SolPdaInterface, SolRpcConditionItem, SolRpcConditionV2,
    SolRpcConditionV2Options,
};

const VALID_CHAIN_NAMES: [&str; 3] = ["solana", "solanaDevnet", "solanaTestnet"];

fn rpc_url<C>(chain: C) -> Result<String>
where
    C: AsRef<str>,
{
    if !VALID_CHAIN_NAMES.contains(&chain.as_ref()) {
        return Err(blockchain_err_code(
            format!("invalid chain for solana: {}", chain.as_ref()),
            EC::NodeBlockchainChainUnknown,
            None,
        ));
    }
    super::rpc_url(chain.as_ref())
}

pub fn get_sol_rpc_client<C>(chain: C) -> Result<RpcClient>
where
    C: AsRef<str>,
{
    let rpc_addr = rpc_url(chain.as_ref())?;
    let client = RpcClient::new(rpc_addr);

    Ok(client)
}

pub async fn check_access_control_conditions(
    conditions: &Vec<SolRpcConditionItem>,
    auth_sig: &JsonAuthSig,
    bls_root_pubkey: &String,
    current_action_ipfs_id: Option<&String>,
) -> Result<bool> {
    // No need to add Ok false message since we're displaying a generic message in web_client_endpoints
    match validate_solana_auth_sig(auth_sig) {
        Ok(sig_is_valid) => {
            debug!("Signature checked and the result is {:?}", sig_is_valid);
            match sig_is_valid {
                true => {
                    check_condition_group(
                        conditions,
                        auth_sig,
                        bls_root_pubkey,
                        current_action_ipfs_id,
                    )
                    .await
                }
                false => Ok(false),
            }
        }
        Err(err) => {
            warn!("Error checking auth sig: {:?}", err);
            Ok(false)
        }
    }
}

async fn check_condition_group(
    conditions: &Vec<SolRpcConditionItem>,
    auth_sig: &JsonAuthSig,
    bls_root_pubkey: &String,
    current_action_ipfs_id: Option<&String>,
) -> Result<bool> {
    if !validate_boolean_expression(conditions) {
        return Err(validation_err_code(
            "Invalid boolean Solana Access Control Conditions",
            EC::NodeInvalidBooleanConditionType,
            None,
        ));
    }

    let mut results: Vec<bool> = Vec::new();
    let mut operators: Vec<AccessControlBooleanOperator> = Vec::new();
    for condition_item in conditions {
        match condition_item {
            SolRpcConditionItem::Condition(condition) => {
                results.push(
                    check_condition(condition, auth_sig, bls_root_pubkey, current_action_ipfs_id)
                        .await?,
                );
            }
            SolRpcConditionItem::Operator(operator) => {
                operators.push(operator.operator.clone());
            }
            SolRpcConditionItem::Group(group) => {
                results.push(
                    Box::pin(check_condition_group(
                        group,
                        auth_sig,
                        bls_root_pubkey,
                        current_action_ipfs_id,
                    ))
                    .await?,
                );
            }
        }
    }

    // okay we now have a bunch of results and operators.  We need to apply the operators.
    // reverse the results so that we can pop off the first 2 results at a time
    results.reverse();

    // pull 2 results out, apply the operator, and push the result back on the results vector
    for operator in operators {
        let res1 = results
            .pop()
            .expect_or_err("Could not pop result off of results vector")?;
        let res2 = results
            .pop()
            .expect_or_err("Could not pop result off of results vector")?;

        match operator {
            AccessControlBooleanOperator::And => results.push(res1 && res2),
            AccessControlBooleanOperator::Or => results.push(res1 || res2),
        }
    }

    Ok(*results
        .last()
        .expect_or_err("Could not get last result from results vector")?)
}

pub async fn check_condition(
    condition: &SolRpcConditionV2Options,
    auth_sig: &JsonAuthSig,
    bls_root_pubkey: &String,
    current_action_ipfs_id: Option<&String>,
) -> Result<bool> {
    let condition = SolRpcConditionV2 {
        method: condition.method.clone(),
        params: condition.params.clone(),
        pda_params: condition.pda_params.clone().unwrap_or_default(),
        pda_interface: condition.pda_interface.clone().unwrap_or(SolPdaInterface {
            offset: 0,
            fields: HashMap::new(),
        }),
        pda_key: condition.pda_key.clone().unwrap_or("".to_string()),
        chain: condition.chain.clone(),
        return_value_test: condition.return_value_test.clone(),
    };
    let mut substituted_params: Vec<serde_json::Value> = Vec::new();
    for param in condition.params.clone() {
        match param {
            serde_json::Value::String(str) => {
                let subbed_param = super::substitute_special_params(
                    &str,
                    auth_sig,
                    bls_root_pubkey,
                    current_action_ipfs_id,
                )
                .await
                .map_err(|e| validation_err_code(e, EC::NodeInvalidSIWESpecialParam, None))?;
                substituted_params.push(serde_json::Value::String(subbed_param));
            }
            _ => substituted_params.push(param.clone()),
        }
    }
    if condition.method.is_empty()
        && substituted_params.len() == 1
        && substituted_params[0].is_string()
    {
        // check wallet address
        return check_return_value_string(
            &condition,
            substituted_params[0]
                .as_str()
                .ok_or_else(|| {
                    conversion_err(
                        "could not convert sol param to string when checking for a specific wallet",
                        None,
                    )
                })?
                .to_string(),
            auth_sig,
            bls_root_pubkey,
            current_action_ipfs_id,
        )
        .await;
    }

    // calculate PDA if needed
    let mut params: serde_json::Value = substituted_params.clone().into();
    let mut pda_address = String::new();
    let mut rpc_method = condition.method.clone();
    if !condition.pda_params.is_empty() {
        let program_id = condition.pda_params[0].as_str();
        let program_id_pubkey = Pubkey::from_str(program_id).map_err(|e| {
            parser_err(
                e,
                Some("Could not get program id pubkey from program address string".into()),
            )
        })?;
        let pda_params = &condition.pda_params[1..];

        let seeds: Vec<Vec<u8>> = pda_params
            .iter()
            .filter_map(|p| {
                if p.contains("pubkey:") {
                    return match Pubkey::from_str(&p[7..]) {
                        Ok(pubkey) => Some(pubkey.to_bytes().to_vec()),
                        Err(e) => {
                            // TODO: This code replaces a very dangerous 'expect()' but should maybe
                            // be turned into a loop (cannot return errors in filter..
                            warn!("Could not get pubkey from pda_params: {:?}", e);
                            None
                        }
                    };
                }
                Some(p.as_bytes().to_vec())
            })
            .collect();
        let seeds: Vec<&[u8]> = seeds.iter().map(Vec::as_slice).collect();

        let (program_address, bitcode) = Pubkey::find_program_address(&seeds, &program_id_pubkey);
        pda_address = program_address.to_string(); // test string for Strata.  This should always work: //"9pNPwpAB4dAmSfzNQ8Dbj5e5Wfo6CdNTTCHVdKffcpcM".to_string();

        params = serde_json::Value::Array(vec![
            serde_json::Value::String(pda_address.clone()),
            json!({"encoding": "base64"}),
        ]);
        // we support things like getBalance(getPDA) so we need to
        // change the rpc method from getBalance(getPDA) to just getBalance.
        // note that we already substituted the params above to be the PDA address instead of the
        // original params
        if condition.method.contains('(') {
            let parts: Vec<&str> = condition.method.split('(').collect();
            rpc_method = parts[0].to_string();
        }

        // now we have to call getAccountInfo and parse the data into fields
        let sol_client = get_sol_rpc_client(&condition.chain)?;
        let rpc_request = RpcRequest::GetAccountInfo;
        let resp: StdResult<Value, solana_client::client_error::ClientError> =
            sol_client.send(rpc_request, params.clone());
        debug!("resp: {:?}", resp);
        let mut pda_fields: HashMap<String, Pubkey> = HashMap::new();
        match resp {
            Ok(parsed_response) => {
                // filter on pda_interface
                match parsed_response["value"].clone() {
                    serde_json::Value::Object(obj) => {
                        if let serde_json::Value::Array(data_array) = obj["data"].clone() {
                            let data_base64 = data_array[0].as_str().ok_or_else(|| {
                                conversion_err(
                                    "Could not unwrap data_array element 0 to base64 string",
                                    None,
                                )
                            })?;
                            let data = data_encoding::BASE64
                                .decode(data_base64.as_bytes())
                                .map_err(|e| {
                                    parser_err(
                                        e,
                                        Some("Could not decode base64 data from response".into()),
                                    )
                                })?;

                            let mut current_offset = condition.pda_interface.offset;
                            for (key, value) in condition.pda_interface.fields.clone() {
                                let field = data[current_offset..current_offset + value].to_vec();
                                current_offset += value;
                                let pubkey = Pubkey::new(&field);
                                pda_fields.insert(key.to_string(), pubkey);
                            }
                            debug!("pda_fields: {:?}", pda_fields);
                            //now, use the pda_field defined in returnValueTest: { key } to set
                            // the RPC params
                            if !pda_fields.contains_key(&condition.pda_key) {
                                return Err(unexpected_err_code(
                                    "Error making RPC Call - the key in returnValueTest does not exist in the PDA fields",
                                    EC::NodeRpcError,
                                    None,
                                ));
                            }

                            let key_to_check = pda_fields
                                .get(&condition.pda_key)
                                .expect_or_err(
                                    "Error making RPC Call - the value in the pda_field is None",
                                )
                                .map_err(|e| unexpected_err_code(e, EC::NodeRpcError, None))?;
                            params = serde_json::Value::Array(vec![serde_json::Value::String(
                                key_to_check.to_string(),
                            )]);
                        } else {
                            return Err(unexpected_err_code(
                                "Error making RPC Call - the data in the response is not an array",
                                EC::NodeRpcError,
                                None,
                            ));
                        }
                    }
                    _ => {
                        return Err(unexpected_err_code("Error making RPC Call getAccountInfo in getPDA - couldn't parse result into an object", 
                                                       EC::NodeRpcError, None));
                    }
                };
            }
            Err(err) => {
                return Err(unexpected_err_code(
                    err,
                    EC::NodeRpcError,
                    Some("Error making RPC Call".into()),
                ));
            }
        }
    }

    if condition.method == "balanceOfMetaplexCollection" {
        // check the balance of an nft collection for the user
        return check_balance_of_metaplex_collection(&condition, auth_sig);
    } else if condition.method == "balanceOfToken" {
        // check the balance of a token that the user should hold
        return check_balance_of_token(
            &condition,
            auth_sig,
            bls_root_pubkey,
            current_action_ipfs_id,
        )
        .await;
    } else if condition.method == "getPDA" {
        return check_return_value_string(
            &condition,
            pda_address,
            auth_sig,
            bls_root_pubkey,
            current_action_ipfs_id,
        )
        .await;
    }

    let sol_client = get_sol_rpc_client(&condition.chain)?;
    let rpc_request: RpcRequest;
    match rpc_method.as_str() {
        "getBlock" => rpc_request = RpcRequest::GetBlock,
        "getBalance" => rpc_request = RpcRequest::GetBalance,
        "getAccountInfo" => rpc_request = RpcRequest::GetAccountInfo,
        "getTokenAccountBalance" => rpc_request = RpcRequest::GetTokenAccountBalance,
        "getTokenAccountsByDelegate" => rpc_request = RpcRequest::GetTokenAccountsByDelegate,
        "getTokenAccountsByOwner" => rpc_request = RpcRequest::GetTokenAccountsByOwner,
        "getHealth" => rpc_request = RpcRequest::GetHealth,
        _ => {
            return Err(validation_err_code(
                format!("Unsupported Solana RPC method: {}", rpc_method),
                EC::NodeInvalidSolanaRpcMethod,
                None,
            ))
        }
    };

    debug!("rpc request: {:?}", rpc_request);
    // call substitute to get the address
    debug!("params: {:?}", params);

    let parsed_response: Value = sol_client.send(rpc_request, params).map_err(|e| {
        unexpected_err_code(e, EC::NodeRpcError, Some("Error making RPC Call".into()))
    })?;

    check_return_value(
        &condition,
        &parsed_response["value"],
        auth_sig,
        bls_root_pubkey,
        current_action_ipfs_id,
    )
    .await
}

async fn check_return_value(
    condition: &SolRpcConditionV2,
    returned_value: &Value,
    auth_sig: &JsonAuthSig,
    bls_root_pubkey: &String,
    current_action_ipfs_id: Option<&String>,
) -> Result<bool> {
    debug!("check_return_value of {:?}", returned_value.to_string());
    let value_to_check;
    if condition.return_value_test.key.is_empty() {
        value_to_check = returned_value.clone();
    } else {
        match jsonpath_plus::find(&condition.return_value_test.key, returned_value) {
            Ok(filtered_vals) => {
                if filtered_vals.is_empty() {
                    warn!(
                        "Could not find key {:?} in JSON response",
                        condition.return_value_test.key
                    );
                    return Ok(false);
                } else {
                    value_to_check = filtered_vals[0].clone();
                }
            }
            Err(e) => {
                return Err(validation_err_code(
                    e,
                    EC::NodeInvalidACCReturnValueTest,
                    Some(format!(
                        "Could not find key {:?} in JSON response",
                        condition.return_value_test.key
                    )),
                ));
            }
        }
    }

    info!("value_to_check- {:?}", value_to_check);
    match value_to_check {
        Value::Number(val) => check_return_value_uint(
            condition,
            val.as_u64()
                .ok_or_else(|| parser_err("Could not parse value as u64", None))?,
        ),
        Value::String(val) => {
            check_return_value_string(
                condition,
                val.to_string(),
                auth_sig,
                bls_root_pubkey,
                current_action_ipfs_id,
            )
            .await
        }
        _ => {
            warn!("Error, unsupported value as response: {:?}", value_to_check);
            Ok(false)
        }
    }
}

fn check_return_value_uint(condition: &SolRpcConditionV2, returned_value: u64) -> Result<bool> {
    debug!("Checking return value {:?}", returned_value);

    let valid_return_value: u64 = serde_json::from_str(&condition.return_value_test.value)
        .map_err(|e| {
            validation_err_code(
                e,
                EC::NodeInvalidACCReturnValueTest,
                Some("failed to parse valid return value from access control condition".into()),
            )
        })?;

    trace!(
        "Testing: Is {:?} {:?} {:?}",
        returned_value,
        condition.return_value_test.comparator,
        valid_return_value
    );

    if condition.return_value_test.comparator == ">" {
        Ok(returned_value > valid_return_value)
    } else if condition.return_value_test.comparator == "<" {
        return Ok(returned_value < valid_return_value);
    } else if condition.return_value_test.comparator == ">=" {
        return Ok(returned_value >= valid_return_value);
    } else if condition.return_value_test.comparator == "<=" {
        return Ok(returned_value <= valid_return_value);
    } else if condition.return_value_test.comparator == "=" {
        return Ok(returned_value == valid_return_value);
    } else if condition.return_value_test.comparator == "!=" {
        return Ok(returned_value != valid_return_value);
    } else {
        warn!("Error - unsupported return value test comparator");
        return Ok(false);
    }
}

async fn check_return_value_string(
    condition: &SolRpcConditionV2,
    returned_value: String,
    auth_sig: &JsonAuthSig,
    bls_root_pubkey: &String,
    current_action_ipfs_id: Option<&String>,
) -> Result<bool> {
    debug!("Checking return value {:?}", returned_value);
    let valid_return_value = substitute_special_params(
        &condition.return_value_test.value,
        auth_sig,
        bls_root_pubkey,
        current_action_ipfs_id,
    )
    .await
    .map_err(|e| validation_err_code(e, EC::NodeInvalidSIWESpecialParam, None))?;

    trace!(
        "Testing: Is {:?} {:?} {:?}",
        returned_value,
        condition.return_value_test.comparator,
        valid_return_value
    );

    if condition.return_value_test.comparator == ">" {
        Ok(returned_value > valid_return_value)
    } else if condition.return_value_test.comparator == "<" {
        return Ok(returned_value < valid_return_value);
    } else if condition.return_value_test.comparator == ">=" {
        return Ok(returned_value >= valid_return_value);
    } else if condition.return_value_test.comparator == "<=" {
        return Ok(returned_value <= valid_return_value);
    } else if condition.return_value_test.comparator == "=" {
        return Ok(returned_value == valid_return_value);
    } else if condition.return_value_test.comparator == "!=" {
        return Ok(returned_value != valid_return_value);
    } else {
        warn!("Error - unsupported return value test comparator");
        return Ok(false);
    }
}

fn check_balance_of_metaplex_collection(
    condition: &SolRpcConditionV2,
    auth_sig: &JsonAuthSig,
) -> Result<bool> {
    // 0. get the metaplex collection address
    // 1. find all tokens that the user holds
    // 2. filter out tokens that are not nfts
    // 3. get metadata for all tokens
    // 4. check for the presence of a matching verified metaplex collection

    // 0. get the metaplex collection address
    let collection_address: Pubkey;
    if let serde_json::Value::String(collection_address_str) = condition.params[0].clone() {
        match Pubkey::from_str(&collection_address_str) {
            Ok(result_address) => {
                collection_address = result_address;
            }
            Err(e) => {
                return Err(validation_err_code(
                    e,
                    EC::NodeInvalidMetaplexCollectionAddress,
                    Some("Invalid Metaplex collection address".into()),
                ));
            }
        }
    } else {
        return Err(validation_err_code(
            "Error, collection address is not a string",
            EC::NodeInvalidMetaplexCollectionAddress,
            None,
        ));
    }

    let token_program_address = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
    let sol_client = get_sol_rpc_client(&condition.chain)?;
    let user_tokens_params = serde_json::json!([auth_sig.address, { "programId": token_program_address }, { "encoding": "jsonParsed"}]);
    let user_tokens_resp: StdResult<Value, solana_client::client_error::ClientError> =
        sol_client.send(RpcRequest::GetTokenAccountsByOwner, user_tokens_params);
    match user_tokens_resp {
        Ok(resp) => {
            let all_tokens = resp["value"]
                .as_array()
                .ok_or_else(|| conversion_err("User tokens not an array", None))?;
            // 2. filter out tokens that are not NFTs
            let nfts: Vec<Value> = all_tokens
                .iter()
                .filter(|token| {
                    let amount = token["account"]["data"]["parsed"]["info"]["tokenAmount"]
                        ["uiAmount"]
                        .as_f64();
                    let decimals = token["account"]["data"]["parsed"]["info"]["tokenAmount"]
                        ["decimals"]
                        .as_u64();
                    match (amount, decimals) {
                        (Some(amount), Some(decimals)) => decimals == 0 && amount >= 1.0,
                        _ => false,
                    }
                })
                .cloned()
                .collect();
            debug!("Got {} NFTs for the user", nfts.len());
            // 3. get metadata for all tokens
            let mut verified_token_count: u64 = 0;
            for nft in nfts {
                let token_address = nft["account"]["data"]["parsed"]["info"]["mint"]
                    .as_str()
                    .expect_or_err("Could not get token address from nft")?
                    .to_string();
                let metadata_result = get_metaplex_metadata(condition, token_address.clone());
                if let Ok(metadata) = metadata_result {
                    if let Some(collection) = metadata.collection {
                        if collection.verified && collection.key == collection_address {
                            verified_token_count += 1;
                        }
                    }
                } else {
                    debug!("Could not get metadata for {} - skipping", token_address);
                    // return Err(anyhow!("rpc_error"));
                }
            }
            debug!(
                "Got {} verified tokens for the collection",
                verified_token_count
            );
            // now actually check that the verified token count matches the condition
            check_return_value_uint(condition, verified_token_count)
        }
        Err(err) => Err(unexpected_err_code(
            err,
            EC::NodeRpcError,
            Some("Error getting user tokens".into()),
        )),
    }

    // not required with the above match for now...
    //return Ok(false);

    // // 0. pull down the collection account and check that it is verified
    // // 1. find all NFTs that belong to a given collection by making a getProgramAccounts call
    // // 2. check if the user holds one of them
    // let sol_client = get_sol_rpc_client(&condition.chain)?;

    // let collection_resp: Result<serde_json::Value, solana_client::client_error::ClientError>;
    // let collection_address: String;
    // if let serde_json::Value::String(collection_address_str) = condition.params[0].clone() {
    //     collection_address = collection_address_str;
    //     let collection_params = json!([collection_address, {"encoding": "base64"}]);
    //     println!("collection_params: {:?}", collection_params);
    //     collection_resp = sol_client.send(RpcRequest::GetAccountInfo, collection_params);
    // } else {
    //     println!("Error, collection address is not a string");
    //     return Err(anyhow!("rpc_error"));
    // }

    // // the following match statement will return Ok(false) if the collection is not verified
    // // therefore, any code that runs after this match statement can assume that the
    // // collection is verified
    // match collection_resp {
    //     Ok(parsed_response) => {
    //         println!("parsed_response: {:?}", parsed_response);
    //         let result = parsed_response["value"].clone();
    //         let data = result["data"].clone();
    //         match data {
    //             serde_json::Value::Array(arr) => match &arr[0] {
    //                 serde_json::Value::String(val) => {
    //                     println!("before decoding base64: {:?}", val);
    //                     let decoded =
    //                         base64::decode(val).expect("could not decode base64 solana data");
    //                     let collection_result: Result<Collection, std::io::Error> =
    //                         try_from_slice_unchecked(&decoded);
    //                     match collection_result {
    //                         Ok(collection) => {
    //                             println!("collection: {:?}", collection);
    //                             if collection.verified != true {
    //                                 println!("Collection isn't verified.  Aborting");
    //                                 return Ok(false);
    //                             }
    //                         }
    //                         Err(err) => {
    //                             println!("Error parsing collection: {:?}", err);
    //                             return Err(anyhow!("rpc_error"));
    //                         }
    //                     }
    //                 }
    //                 _ => {
    //                     println!(
    //                         "trying to get metaplex metadata but it's not a string: {:?}",
    //                         arr
    //                     );
    //                     return Err(anyhow!("rpc_error"));
    //                 }
    //             },
    //             _ => {
    //                 println!(
    //                     "trying to get metaplex metadata but it's not an array: {:?}",
    //                     data
    //                 );
    //                 return Err(anyhow!("rpc_error"));
    //             }
    //         }
    //     }
    //     Err(err) => {
    //         println!("Error making RPC Call: {:?}", err);
    //         return Err(anyhow!("rpc_error"));
    //     }
    // }

    // // now 1. find all NFTs that belong to a given collection by making a getProgramAccounts call
    // let program_accounts_params = json!([collection_address, {"encoding": "jsonParsed"}]);
    // let program_accounts_resp: Result<serde_json::Value, solana_client::client_error::ClientError> =
    //     sol_client.send(RpcRequest::GetProgramAccounts, program_accounts_params);
    // match program_accounts_resp {
    //     Ok(program_accounts) => {
    //         println!("program accounts: {:?}", program_accounts);
    //     }
    //     Err(err) => {
    //         println!("Error making RPC Call: {:?}", err);
    //         return Err(anyhow!("rpc_error"));
    //     }
    // }

    // return Ok(false);
}

async fn check_balance_of_token(
    condition: &SolRpcConditionV2,
    auth_sig: &JsonAuthSig,
    bls_root_pubkey: &String,
    current_action_ipfs_id: Option<&String>,
) -> Result<bool> {
    let token_account_address = get_token_account_address(
        auth_sig,
        condition.params[0].as_str().ok_or_else(|| {
            conversion_err("Could not convert condition.params[0] to string", None)
        })?,
    )?;
    let sol_client = get_sol_rpc_client(&condition.chain)?;

    let params: serde_json::Value = json!([token_account_address.to_string()]);
    debug!("params: {:?}", params);

    let resp: StdResult<Value, solana_client::client_error::ClientError> =
        sol_client.send(RpcRequest::GetTokenAccountBalance, params);

    match resp {
        Ok(parsed_response) => {
            debug!("parsed_response: {:?}", parsed_response);
            let result = parsed_response["value"].clone();
            check_return_value(
                condition,
                &result,
                auth_sig,
                bls_root_pubkey,
                current_action_ipfs_id,
            )
            .await
        }
        Err(err) => {
            if let RpcError(solana_client::rpc_request::RpcError::RpcResponseError {
                code,
                message,
                data,
            }) = &err.kind
            {
                if *code == -32602 && message == "Invalid param: could not find account" {
                    // the balance is zero, we couldn't find the acct.  return false.
                    return Ok(false);
                }
            }

            Err(validation_err_code(
                err,
                EC::NodeRpcError,
                Some("Error making RPC Call".into()),
            ))
        }
    }
}

fn get_token_account_address(auth_sig: &JsonAuthSig, token_address: &str) -> Result<Pubkey> {
    let token_pubkey = Pubkey::from_str(token_address).map_err(|e| {
        parser_err(
            e,
            Some("Could not get token_pubkey from token_address".into()),
        )
    })?;
    let user_wallet_pubkey = Pubkey::from_str(auth_sig.address.as_str()).map_err(|e| {
        parser_err(
            e,
            Some("Could not get user_wallet_pubkey from auth_sig.address".into()),
        )
    })?;

    Ok(get_associated_token_address(
        &user_wallet_pubkey,
        &token_pubkey,
    ))
}

// this fn is not currently used, but i put all the work in to figure out how to pull down metadata so
// i am preserving that work in this function for future use
fn get_metaplex_metadata(condition: &SolRpcConditionV2, token_address: String) -> Result<Metadata> {
    // example using a known token...
    // let token_pubkey = Pubkey::from_str("GkLTynMPrX62ezmdfcXaTHhpmYubypVTei2sdwq4KBg")
    //     .expect("Could not get token_pubkey from token_address");
    let token_pubkey = Pubkey::from_str(token_address.as_str()).map_err(|e| {
        parser_err(
            e,
            Some("Could not get token_pubkey from token_address".into()),
        )
    })?;
    let metaplex_metadata_address = "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s";
    let metaplex_metadata_pubkey = Pubkey::from_str(metaplex_metadata_address).map_err(|e| {
        parser_err(
            e,
            Some("Could not get metaplex_metadata_pubkey from metaplex_metadata_address".into()),
        )
    })?;
    let seeds = [
        "metadata".as_bytes(),
        &metaplex_metadata_pubkey.to_bytes(),
        &token_pubkey.to_bytes(),
    ];
    let (program_address, bitcode) =
        Pubkey::find_program_address(&seeds, &metaplex_metadata_pubkey);

    let account_into_params = json!([program_address.to_string(), {"encoding": "base64"}]);

    let sol_client = get_sol_rpc_client(&condition.chain)?;

    let parsed_response: Value = sol_client
        .send(RpcRequest::GetAccountInfo, account_into_params)
        .map_err(|e| {
            unexpected_err_code(e, EC::NodeRpcError, Some("Error making RPC Call".into()))
        })?;

    let result = parsed_response["value"].clone();
    let data = result["data"].clone();

    match data {
        Value::Array(arr) => match &arr[0] {
            Value::String(val) => {
                let decoded = data_encoding::BASE64.decode(val.as_bytes()).map_err(|e| {
                    parser_err(e, Some("could not decode base64 solana data".into()))
                })?;
                let metadata: Metadata = try_from_slice_unchecked(&decoded).map_err(|e| {
                    parser_err(e, Some("Could not parse metadata for solana token".into()))
                })?;

                debug!("metadata: {:#?}", metadata);
                return Ok(metadata);
            }
            _ => {
                debug!(
                    "trying to get metaplex metadata but it's not a string: {:?}",
                    arr
                )
            }
        },
        _ => {
            debug!(
                "trying to get metaplex metadata but it's not an array: {:?}",
                data
            )
        }
    }

    Err(unexpected_err_code(
        "Could not retrieve metadata for solana nft",
        EC::NodeSolanaNFTMetadataError,
        None,
    ))
}

#[cfg(test)]
mod tests {
    use super::check_condition;
    use crate::{
        auth::auth_material::JsonAuthSig,
        constants::CHAIN_SOLANA,
        models::{JsonReturnValueTestV2, SolPdaInterface, SolRpcConditionV2Options},
    };
    use rocket::serde::json::Value::String;
    use std::collections::HashMap;

    fn get_auth_sig() -> JsonAuthSig {
        JsonAuthSig::new(
            "0b1b994b550d5f9b833d5b8b17f933890a22ff0dde1f6fd08be600f2de313a75492aa74e5bcae165cf0abcd071382fe6b4506b4df23b6acebba000d49fda4204".to_string(),
            "solana.signMessage".to_string(),
            "I am creating an account to use Lit Protocol at 2023-05-24T08:03:26.485Z".to_string(),
            "5Th1tyAQGFaZ2c6gBVW4MPe6L5xhvpnBc9pjMzAkEPPz".to_string(),
            None,
        )
    }

    #[tokio::test]
    async fn test_check_condition_string() {
        let address_condition = SolRpcConditionV2Options {
            method: "".to_string(),
            params: vec![String(":userAddress".to_string())],
            pda_params: Some(vec![]),
            pda_interface: Some(SolPdaInterface {
                offset: 0,
                fields: HashMap::new(),
            }),
            pda_key: Some("".to_string()),
            chain: CHAIN_SOLANA.to_string(),
            return_value_test: JsonReturnValueTestV2 {
                key: "".to_string(),
                comparator: "=".to_string(),
                value: "5Th1tyAQGFaZ2c6gBVW4MPe6L5xhvpnBc9pjMzAkEPPz".to_string(),
            },
        };
        let check_balance_condition =
            check_condition(&address_condition, &get_auth_sig(), &"".to_string(), None).await;
        assert!(check_balance_condition.is_ok());
        assert!(check_balance_condition.unwrap());
    }

    // Commented for now since the `get_sol_rpc_client()` panics if run in a tokio async context.
    // Can be uncommented when we refactor & extract out the Solana Client initiation in a Global Config.
    // #[tokio::test]
    // async fn test_check_condition_numerical() {
    //     let balance_condition = SolRpcConditionV2Options {
    //         method: "getBalance".to_string(),
    //         params: vec!(String(":userAddress".to_string())),
    //         pda_params: Some(vec!()),
    //         pda_interface: Some(SolPdaInterface { offset: 0, fields: HashMap::new() }),
    //         pda_key: Some("".to_string()),
    //         chain: CHAIN_SOLANA.to_string(),
    //         return_value_test: JsonReturnValueTestV2 {
    //             key: "".to_string(),
    //             comparator: ">=".to_string(),
    //             value: "0".to_string()
    //         }
    //     };
    //     let check_balance_condition = check_condition(&balance_condition, &get_auth_sig()).await;
    //     assert_eq!(check_balance_condition.is_ok(), true);
    //     assert_eq!(check_balance_condition.unwrap(), true);
    // }
}
