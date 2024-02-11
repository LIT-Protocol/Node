use chrono::{Duration, Utc};
use std::str::FromStr;

use crate::{
    error::{parser_err_code, validation_err_code, Result, EC},
    siwe_db::utils::{check_expiration_validity, convert_siwe_timestamp_to_utc_datetime},
};

pub fn validate_siwe(siwe_message: &siwe::Message) -> Result<()> {
    let issued_at = siwe_message.issued_at.clone();
    let now = siwe::TimeStamp::from_str(&Utc::now().to_rfc3339()).map_err(|e| {
        parser_err_code(
            e,
            EC::NodeSIWEMessageError,
            Some("Could not parse current time".into()),
        )
        .add_source_to_details()
    })?;
    let grace_period_seconds = 60;
    let now_add_grace_period = siwe::TimeStamp::from_str(
        &(Utc::now() + Duration::seconds(grace_period_seconds)).to_rfc3339(),
    )
    .map_err(|e| {
        parser_err_code(
            e,
            EC::NodeSIWEMessageError,
            Some("Could not parse current time".into()),
        )
        .add_source_to_details()
    })?;

    // Validate that issued_at is not in the past (within the grace period if so)
    if issued_at.as_ref() > now_add_grace_period.as_ref() {
        return Err(validation_err_code(
            format!(
                "Session key issued_at {} is in the future beyond the grace period of {} seconds (now is {})",
                issued_at, grace_period_seconds, now
            ),
            EC::NodeSIWEMessageError,
            None,
        )
        .add_source_to_details());
    }

    let now_subtract_grace_period = siwe::TimeStamp::from_str(
        &(Utc::now() - Duration::seconds(grace_period_seconds)).to_rfc3339(),
    )
    .map_err(|e| {
        parser_err_code(
            e,
            EC::NodeSIWEMessageError,
            Some("Could not parse current time".into()),
        )
        .add_source_to_details()
    })?;

    // Validate expiration if it is provided.
    if let Some(expiration) = siwe_message.expiration_time.clone() {
        // Validate that expires_at is in the future of issue_at irrespective of the user's time drift
        if expiration.as_ref() < issued_at.as_ref() {
            return Err(validation_err_code(
                format!(
                    "Session key expiration {} is in behind issue_at which is {}",
                    expiration, issued_at
                ),
                EC::NodeExpWrongOrTooLarge,
                None,
            )
            .add_source_to_details());
        }

        // Validate that expires_at is in the future (within the grace period if so)
        if expiration.as_ref() < now_subtract_grace_period.as_ref() {
            return Err(validation_err_code(
                format!(
                    "Session key expiration {} is in the past beyond the grace period of {} seconds (now is {})",
                    expiration, grace_period_seconds, now
                ),
                EC::NodeSIWEMessageError,
                None,
            )
            .add_source_to_details());
        }

        let issued_at_datetime = convert_siwe_timestamp_to_utc_datetime(issued_at)?;
        let expiration_datetime = convert_siwe_timestamp_to_utc_datetime(expiration)?;

        check_expiration_validity(issued_at_datetime, expiration_datetime)?;
    } else {
        return Err(parser_err_code(
            "Session key expiration time is not set",
            EC::NodeSIWEMessageError,
            None,
        )
        .add_source_to_details());
    }

    Ok(())
}
