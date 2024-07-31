use crate::error::{parser_err, validation_err_code, Result, EC};
use chrono::{DateTime, Duration, Utc};
use lit_api_core::error::Unexpected;
use rocket::time::OffsetDateTime;
use siwe::TimeStamp;
use std::str::FromStr;

pub const MAX_TIMESTAMP_VALIDITY_DAYS: i64 = 30;

pub fn make_timestamp_siwe_compatible(block_timestamp: &str) -> Result<TimeStamp> {
    let timestamp = i64::from_str(block_timestamp).map_err(|e| {
        parser_err(
            e,
            Some("Error converting block_timestamp to i64".to_owned()),
        )
    })?;
    let res = OffsetDateTime::from_unix_timestamp(timestamp).map_err(|e| {
        parser_err(
            e,
            Some("Error converting timestamp int to OffsetDateTime".to_owned()),
        )
    })?;

    Ok(TimeStamp::from(res))
}

pub fn check_block_timestamp_validity(block_timestamp: &str) -> Result<()> {
    let timestamp = i64::from_str(block_timestamp).map_err(|e| {
        parser_err(
            e,
            Some("Error converting block_timestamp to i64".to_owned()),
        )
    })?;
    let timestamp_date = chrono::DateTime::from_timestamp(timestamp, 0)
        .expect_or_err("invalid timestamp")
        .map_err(|e| parser_err(e, None))?;

    let now = Utc::now();
    let oldest_valid_timestamp = now - Duration::days(MAX_TIMESTAMP_VALIDITY_DAYS);

    if timestamp_date < oldest_valid_timestamp {
        return Err(validation_err_code(
            format!(
                "Blocktime {} is beyond the max expiry timestamp of {}",
                timestamp_date, oldest_valid_timestamp
            ),
            EC::NodeSIWEMessageError,
            None,
        ));
    }

    Ok(())
}

pub fn check_expiration_validity(
    issued_at: DateTime<Utc>,
    expiration: DateTime<Utc>,
) -> Result<()> {
    let max_expiry_time = issued_at + Duration::days(MAX_TIMESTAMP_VALIDITY_DAYS);
    if expiration > max_expiry_time {
        return Err(validation_err_code(
            format!(
                "Session key expiration {} is beyond the max expiry timestamp of {} (issued_at is {})",
                expiration, max_expiry_time, issued_at
            ),
            EC::NodeSIWEMessageError,
            None,
        )
        .add_source_to_details());
    }

    Ok(())
}

pub fn convert_siwe_timestamp_to_utc_datetime(timestamp: TimeStamp) -> Result<DateTime<Utc>> {
    let timestamp_datetime =
        chrono::DateTime::from_timestamp(timestamp.as_ref().unix_timestamp(), 0);

    timestamp_datetime
        .expect_or_err("invalid timestamp")
        .map_err(|e| parser_err(e, None))
}

#[cfg(test)]
mod siwe_timestamp_validity_tests {
    use chrono::{Duration, Utc};

    use crate::siwe_db::utils::{
        check_block_timestamp_validity, check_expiration_validity, make_timestamp_siwe_compatible,
        MAX_TIMESTAMP_VALIDITY_DAYS,
    };

    #[test]
    fn test_block_timestamp_validity() {
        let now = Utc::now().timestamp();
        let valid_timestamp = now.to_string();
        let invalid_timestamp = (now - 2592000).to_string();

        assert!(check_block_timestamp_validity(&valid_timestamp).is_ok());
        assert!(check_block_timestamp_validity(&invalid_timestamp).is_err());
    }

    #[test]
    fn test_expiration_validity() {
        let issued_at = Utc::now();
        let valid_datetime = issued_at + Duration::days(MAX_TIMESTAMP_VALIDITY_DAYS);
        let invalid_datetime = issued_at + Duration::days(MAX_TIMESTAMP_VALIDITY_DAYS + 1);

        assert!(check_expiration_validity(issued_at, issued_at).is_ok());
        assert!(check_expiration_validity(issued_at, valid_datetime).is_ok());
        assert!(check_expiration_validity(issued_at, invalid_datetime).is_err());
    }

    #[test]
    fn test_block_timestamp_siwe_compatibility() {
        let valid_block_timestamp = Utc::now().timestamp();
        let invalid_block_timestamp = "random string";

        assert!(make_timestamp_siwe_compatible(&valid_block_timestamp.to_string()).is_ok());
        assert!(make_timestamp_siwe_compatible(invalid_block_timestamp).is_err());
    }
}
