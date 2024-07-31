use super::{substitute_special_params, validate_boolean_expression};
use crate::auth::auth_material::{JsonAuthSig, MultipleAuthSigs};
use crate::auth::validators::cosmos::validate_cosmos_auth_sig;
use crate::error::{
    blockchain_err_code, serializer_err_code, unexpected_err_code, validation_err_code, Result, EC,
};
use crate::models::{
    AccessControlBooleanOperator, CosmosBlock, CosmosCondition, CosmosConditionItem,
};
use chrono::{DateTime, Utc};
use lit_core::error::Unexpected;
use tracing::warn;

const VALID_CHAIN_NAMES: [&str; 6] = [
    "cosmos",
    "kyve",
    "cheqdMainnet",
    "cheqdTestnet",
    "juno",
    "evmos",
];

fn rpc_url<C>(chain: C) -> Result<String>
where
    C: AsRef<str>,
{
    if !VALID_CHAIN_NAMES.contains(&chain.as_ref()) {
        return Err(blockchain_err_code(
            format!("invalid chain for cosmos: {}", chain.as_ref()),
            EC::NodeBlockchainChainUnknown,
            None,
        ));
    }
    super::rpc_url(chain.as_ref())
}

pub async fn check_access_control_conditions(
    conditions: &Vec<CosmosConditionItem>,
    auth_sigs: &MultipleAuthSigs,
    bls_root_pubkey: &String,
    current_action_ipfs_id: Option<&String>,
) -> Result<bool> {
    // need to loop over all conditions and check auth sig for all of them
    check_condition_group(
        conditions,
        auth_sigs,
        bls_root_pubkey,
        current_action_ipfs_id,
    )
    .await
}

async fn check_condition_group(
    conditions: &Vec<CosmosConditionItem>,
    auth_sigs: &MultipleAuthSigs,
    bls_root_pubkey: &String,
    current_action_ipfs_id: Option<&String>,
) -> Result<bool> {
    if !validate_boolean_expression(conditions) {
        return Err(validation_err_code(
            "Invalid boolean Cosmos Access Control Conditions",
            EC::NodeInvalidBooleanConditionType,
            None,
        ));
    }

    let mut results: Vec<bool> = Vec::new();
    let mut operators: Vec<AccessControlBooleanOperator> = Vec::new();
    for condition_item in conditions {
        match condition_item {
            CosmosConditionItem::Condition(condition) => {
                results.push(
                    check_condition(
                        condition,
                        auth_sigs,
                        bls_root_pubkey,
                        current_action_ipfs_id,
                    )
                    .await?,
                );
            }
            CosmosConditionItem::Operator(operator) => {
                operators.push(operator.operator.clone());
            }
            CosmosConditionItem::Group(group) => {
                results.push(
                    Box::pin(check_condition_group(
                        group,
                        auth_sigs,
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

pub fn get_auth_sig_for_chain_string(
    auth_sigs: &MultipleAuthSigs,
    chain: &String,
) -> Result<JsonAuthSig> {
    if chain == "cosmos" {
        match auth_sigs.cosmos.as_ref() {
            Some(x) => Ok(x.clone()),
            None => Err(validation_err_code(
                "cosmos auth sig not found but chain is cosmos",
                EC::NodeInvalidCosmosAuthSig,
                None,
            )),
        }
    } else if chain == "kyve" {
        match auth_sigs.kyve.as_ref() {
            Some(x) => Ok(x.clone()),
            None => Err(validation_err_code(
                "kyve auth sig not found but chain is kyve",
                EC::NodeInvalidKyveAuthSig,
                None,
            )),
        }
    } else if chain == "cheqd" || chain == "cheqdMainnet" || chain == "cheqdTestnet" {
        match auth_sigs.cheqd.as_ref() {
            Some(x) => Ok(x.clone()),
            None => Err(validation_err_code(
                "cheqd auth sig not found but chain is cheqd",
                EC::NodeInvalidCheqdAuthSig,
                None,
            )),
        }
    } else if chain == "juno" {
        match auth_sigs.juno.as_ref() {
            Some(x) => Ok(x.clone()),
            None => Err(validation_err_code(
                "juno auth sig not found but chain is juno",
                EC::NodeInvalidJunoAuthSig,
                None,
            )),
        }
    } else {
        let signature_not_found_message =
            format!("signature for cosmos-sdk chain {} not found", chain);
        Err(validation_err_code(
            signature_not_found_message,
            EC::NodeInvalidCosmosSDKSignature,
            None,
        ))
    }
}

pub async fn check_condition(
    condition: &CosmosCondition,
    auth_sigs: &MultipleAuthSigs,
    bls_root_pubkey: &String,
    current_action_ipfs_id: Option<&String>,
) -> Result<bool> {
    // delegate to timelock, if applicable
    if condition.method.is_some()
        && condition
            .method
            .as_ref()
            .expect_or_err("could not get method from condition")?
            .as_str()
            == "timelock"
    {
        return check_condition_timelock(condition, auth_sigs).await;
    }

    let auth_sig = get_auth_sig_for_chain_string(auth_sigs, &condition.chain)?;

    match validate_cosmos_auth_sig(&auth_sig, &condition.chain.clone()) {
        Ok(sig_is_valid) => {
            debug!("Signature checked and the result is {:?}", sig_is_valid);
            if !sig_is_valid {
                return Ok(false);
            }
        }
        Err(err) => {
            warn!("Error checking auth sig: {:?}", err);
            return Ok(false);
        }
    }

    if condition.path == ":userAddress" {
        // just check the address against the auth sig address
        return check_return_value_string(
            condition,
            auth_sig.address.clone(),
            &auth_sig,
            bls_root_pubkey,
            current_action_ipfs_id,
        )
        .await;
    }

    let substituted_path = substitute_special_params_in_url(&condition.path, &auth_sig);

    // hit the URL, check the value
    let base_url = rpc_url(&condition.chain)?;
    let url = format!("{}{}", base_url, substituted_path);
    debug!("hitting cosmos url: {}", url);
    let resp = reqwest::get(url).await;

    match resp {
        Ok(result) => {
            let body = result
                .text()
                .await
                .map_err(|e| unexpected_err_code(e, EC::NodeCosmosResponseBodyError, None))?;
            debug!("cosmos response body: {}", body);

            let result = serde_json::from_str::<serde_json::Value>(&body)
                .map_err(|e| serializer_err_code(e, EC::NodeCosmosJSONError, None))?;

            check_return_value(
                condition,
                &result,
                &auth_sig,
                bls_root_pubkey,
                current_action_ipfs_id,
            )
            .await
        }
        Err(err) => Err(unexpected_err_code(
            err,
            EC::NodeRpcError,
            Some("Error making RPC Call".into()),
        )),
    }
}

pub async fn check_condition_timelock(
    condition: &CosmosCondition,
    auth_sigs: &MultipleAuthSigs,
) -> Result<bool> {
    // validate cosmos condition - case: method is timelock
    if condition.method.is_some()
        && condition
            .method
            .as_ref()
            .expect_or_err("could not get method from condition")?
            .as_str()
            != "timelock"
    {
        return Err(validation_err_code(
            "method is not timelock",
            EC::NodeCosmosInvalidCondition,
            None,
        ));
    }

    // validate cosmos condition - case: parameters are not empty
    if condition.parameters.is_none()
        || condition
            .parameters
            .as_ref()
            .expect_or_err("could not get parameters from condition")?
            .is_empty()
    {
        return Err(validation_err_code(
            "parameters are empty",
            EC::NodeCosmosInvalidCondition,
            None,
        ));
    }

    // validate cosmos condition - case: parameters are max 1
    if condition
        .parameters
        .as_ref()
        .expect_or_err("could not get parameters from condition")?
        .len()
        > 1
    {
        return Err(validation_err_code(
            "parameters are more than 1 item",
            EC::NodeCosmosInvalidCondition,
            None,
        ));
    }

    let auth_sig = get_auth_sig_for_chain_string(auth_sigs, &condition.chain)?;

    match validate_cosmos_auth_sig(&auth_sig, &condition.chain.clone()) {
        Ok(sig_is_valid) => {
            debug!("Signature checked and the result is {:?}", sig_is_valid);
            if !sig_is_valid {
                return Ok(false);
            }
        }
        Err(err) => {
            warn!("Error checking auth sig: {:?}", err);
            return Ok(false);
        }
    }

    let substituted_path = substitute_special_params_in_url(&condition.path, &auth_sig);

    // define base url
    let base_url = rpc_url(&condition.chain)?;

    // define block height url
    let block_height_url = get_block_height_url(
        &base_url,
        condition
            .parameters
            .as_ref()
            .expect_or_err("could not get parameters from condition")?,
    );

    // get timestamp from block height
    let timestamp = get_timestamp_from_block_height(&block_height_url).await?;

    // hit the URL, check the value
    let url = format!("{}{}", base_url, substituted_path);
    debug!("hitting cosmos url: {}", url);
    let resp = reqwest::get(url).await;

    match resp {
        Ok(result) => {
            let body = result
                .text()
                .await
                .map_err(|e| unexpected_err_code(e, EC::NodeCosmosResponseBodyError, None))?;
            debug!("cosmos response body: {}", body);

            let result = serde_json::from_str::<serde_json::Value>(&body)
                .map_err(|e| serializer_err_code(e, EC::NodeCosmosJSONError, None))?;

            check_return_value_timelock(condition, &(timestamp as u64), &result, &auth_sig).await
        }
        Err(err) => Err(unexpected_err_code(
            err,
            EC::NodeRpcError,
            Some("Error making RPC Call".into()),
        )),
    }
}

async fn check_return_value(
    condition: &CosmosCondition,
    returned_value: &serde_json::Value,
    auth_sig: &JsonAuthSig,
    bls_root_pubkey: &String,
    current_action_ipfs_id: Option<&String>,
) -> Result<bool> {
    debug!("check_return_value of {:?}", returned_value.to_string());
    let mut value_to_check;

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

                    // need to check the type here.  because if this is a string, we can concatenate for the "contains" operator.  i suppose we shouldn't do that if it's a number.
                    if filtered_vals.len() > 1
                        && condition.return_value_test.comparator == "contains"
                    {
                        if let serde_json::Value::String(_) = value_to_check {
                            // it's a string.  concate all items
                            let mut concatenated_string = String::new();
                            for item in filtered_vals {
                                concatenated_string.push_str(
                                    item.as_str()
                                        .expect_or_err("could not get string from item")?,
                                );
                                concatenated_string.push(' ');
                            }
                            value_to_check = serde_json::Value::String(concatenated_string);
                        }
                    }
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
                )
                .add_source_to_details());
            }
        }
    }

    match condition.return_value_test.value.parse::<i64>() {
        Ok(_) => {
            // Numerical comparison
            match value_to_check {
                serde_json::Value::String(val) => match val.parse::<u64>() {
                    Ok(val_u64) => check_return_value_uint(condition, val_u64),
                    Err(e) => Err(validation_err_code(
                        "Could not parse value as u64",
                        EC::NodeInvalidACCReturnValueTest,
                        None,
                    )),
                },
                _ => {
                    warn!("unsupported value as response: {:?}", value_to_check);
                    Ok(false)
                }
            }
        }
        Err(_) => {
            // String comparison
            check_return_value_string(
                condition,
                value_to_check.to_string(),
                auth_sig,
                bls_root_pubkey,
                current_action_ipfs_id,
            )
            .await
        }
    }
}

async fn check_return_value_timelock(
    condition: &CosmosCondition,
    timestamp: &u64,
    returned_value: &serde_json::Value,
    auth_sig: &JsonAuthSig,
) -> Result<bool> {
    debug!("check_return_value of {:?}", returned_value.to_string());
    let mut value_to_check;

    if condition.return_value_test.key.is_empty() {
        warn!("key is empty");
        return Ok(false);
    }

    match jsonpath_plus::find(&condition.return_value_test.key, returned_value) {
        Ok(filtered_vals) => {
            if filtered_vals.is_empty() {
                warn!(
                    "Could not find key {:?} in JSON response",
                    condition.return_value_test.key
                );
                Ok(false)
            } else {
                match condition.return_value_test.value.parse::<u64>() {
                    Ok(time_interval) => {
                        // check against all path values
                        for item in filtered_vals {
                            value_to_check = item.clone();

                            // calculate timestamp from value_to_check
                            let timestamp_to_check = DateTime::parse_from_rfc3339(
                                value_to_check.as_str().unwrap_or_default(),
                            )
                            .unwrap_or_default()
                            .timestamp()
                                as u64;

                            // calculate difference between timestamp and value_to_check in seconds
                            let diff = timestamp - timestamp_to_check;

                            // check if diff is within time_interval
                            if condition.return_value_test.comparator == "<" {
                                if diff < time_interval {
                                    return Ok(true);
                                }
                                continue;
                            } else if condition.return_value_test.comparator == "<=" {
                                if diff <= time_interval {
                                    return Ok(true);
                                }
                                continue;
                            } else {
                                return Err(validation_err_code(
                                    "Invalid comparator",
                                    EC::NodeInvalidACCReturnValueTest,
                                    None,
                                ));
                            };
                        }

                        // no match found
                        Ok(false)
                    }
                    Err(e) => Err(validation_err_code(
                        e,
                        EC::NodeInvalidACCReturnValueTest,
                        None,
                    )),
                }
            }
        }
        Err(e) => Err(validation_err_code(
            e,
            EC::NodeInvalidACCReturnValueTest,
            Some(format!(
                "Could not find key {:?} in JSON response",
                condition.return_value_test.key
            )),
        )
        .add_source_to_details()),
    }
}

fn check_return_value_uint(condition: &CosmosCondition, returned_value: u64) -> Result<bool> {
    debug!("Checking return value {:?}", returned_value);

    let valid_return_value: u64 = serde_json::from_str(&condition.return_value_test.value)
        .map_err(|e| {
            validation_err_code(
                e,
                EC::NodeInvalidACCReturnValueTest,
                Some("failed to parse valid return value from access control condition".into()),
            )
            .add_msg_to_details()
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
        Ok(returned_value < valid_return_value)
    } else if condition.return_value_test.comparator == ">=" {
        Ok(returned_value >= valid_return_value)
    } else if condition.return_value_test.comparator == "<=" {
        Ok(returned_value <= valid_return_value)
    } else if condition.return_value_test.comparator == "=" {
        Ok(returned_value == valid_return_value)
    } else if condition.return_value_test.comparator == "!=" {
        Ok(returned_value != valid_return_value)
    } else {
        warn!("Error - unsupported return value test comparator");
        Ok(false)
    }
}

async fn check_return_value_string(
    condition: &CosmosCondition,
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
        Ok(returned_value < valid_return_value)
    } else if condition.return_value_test.comparator == ">=" {
        Ok(returned_value >= valid_return_value)
    } else if condition.return_value_test.comparator == "<=" {
        Ok(returned_value <= valid_return_value)
    } else if condition.return_value_test.comparator == "=" {
        Ok(returned_value == valid_return_value)
    } else if condition.return_value_test.comparator == "!=" {
        Ok(returned_value != valid_return_value)
    } else if condition.return_value_test.comparator == "contains" {
        Ok(returned_value.contains(&valid_return_value))
    } else {
        warn!("Error - unsupported return value test comparator");
        Ok(false)
    }
}

pub fn substitute_special_params_in_url(url: &str, auth_sig: &JsonAuthSig) -> String {
    url.replace(":userAddress", &auth_sig.address)
}

pub fn get_block_height_url(url: &str, parameters: &[String]) -> String {
    let mut url = url.to_string();

    if let Some(block_height) = parameters.first() {
        match block_height.as_str() {
            "latest" => {
                url = format!("{}{}", url, "/cosmos/base/tendermint/v1beta1/blocks/latest");
            }
            _ => {
                url = format!(
                    "{}{}{}",
                    url, "/cosmos/base/tendermint/v1beta1/blocks/", block_height
                );
            }
        }
    } else {
        url = format!("{}{}", url, "/cosmos/base/tendermint/v1beta1/blocks/latest");
    }
    url
}

pub async fn get_timestamp_from_block_height(url: &str) -> Result<i64> {
    let client = reqwest::Client::new();
    let response = client.get(url).send().await.map_err(|e| {
        unexpected_err_code(
            e,
            EC::NodeCosmosBlockHeightRequestError,
            Some("failed to get block height from access control condition".into()),
        )
    })?;

    let response_body = response
        .text()
        .await
        .map_err(|e| unexpected_err_code(e, EC::NodeCosmosResponseBodyError, None))?;

    debug!("cosmos block height response body: {:?}", response_body);

    let block: CosmosBlock = serde_json::from_str(&response_body).map_err(|e| {
        validation_err_code(
            e,
            EC::NodeCosmosBlockHeightParseError,
            Some("failed to parse block from access control condition".into()),
        )
        .add_msg_to_details()
    })?;

    let timestamp = block.block.header.time;

    debug!("cosmos block height timestamp: {:?}", timestamp);

    match DateTime::parse_from_rfc3339(&timestamp)
        .map(|dt| dt.with_timezone(&Utc))
        .map_err(|e| {
            validation_err_code(
                e,
                EC::NodeCosmosBlockHeightParseError,
                Some("failed to parse block from access control condition".into()),
            )
            .add_msg_to_details()
        }) {
        Ok(timestamp) => Ok(timestamp.timestamp()),
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use super::check_condition;
    use crate::{
        auth::auth_material::{JsonAuthSig, MultipleAuthSigs},
        constants::CHAIN_COSMOS,
        models::{CosmosCondition, JsonReturnValueTestV2},
    };

    fn get_auth_sig() -> MultipleAuthSigs {
        MultipleAuthSigs {
            ethereum: None,
            solana: None,
            cosmos: Some(JsonAuthSig::new(
                "ekIkVTCIVet/AhZULgMZ4TmZxf/8zht8pHFptbs4OqweydV3P0axBgd/CpVqIlsniNLMtAvluhqpoUq36IxSCw==".to_string(),
                "cosmos.signArbitrary".to_string(),
                "9f4311c72762213b0900a30d6d93c7d2a561c987e8b1d3286a05ab261be214e7".to_string(),
                "cosmos1n8yrje7g3mkrmj4x2m5u67sdpwcwgtxwzv4vwv".to_string(),
                None,
            )),
            kyve: None,
            cheqd: None,
            juno: None
        }
    }

    // fn get_auth_sig_cheqd() -> MultipleAuthSigs {
    //     MultipleAuthSigs {
    //         ethereum: None,
    //         solana: None,
    //         cosmos: None,
    //         kyve: None,
    //         cheqd: Some(JsonAuthSig::new(
    //             "MSXl+OVsWVrbKswjvuQJDvBDjiTI9s3bufl7mqe5Z7MSEVlIPw8ufPmT8NeaiPtJ1bRCsjiVArhgkb9Tv57Jsg==".to_string(),
    //             "cosmos.signArbitrary".to_string(),
    //             "59776380f2f8b17483781f64d9cc8c7194fef6c5e1dfe97baff1dafd9a98d5db".to_string(),
    //             "cheqd1rnr5jrt4exl0samwj0yegv99jeskl0hsxmcz96".to_string(),
    //             None,
    //         )),
    //         juno: None
    //     }
    // }

    // #[tokio::test]
    // async fn test_check_condition_timelock() {
    //     let timelock_condition = CosmosCondition {
    //         path: "/cosmos/tx/v1beta1/txs?events=transfer.recipient='cheqd1xl5wccz667lk06ahama26pdqvrkz5aws6m0ztp'&events=transfer.amount='147603000000000ncheq'&order_by=2&pagination.limit=1".to_string(),
    //         chain: "cheqdMainnet".to_string(),
    //         method: Some("timelock".to_string()),
    //         parameters: Some(vec!["latest".to_string()]),
    //         return_value_test: JsonReturnValueTestV2 {
    //             key: "$.tx_responses.*.timestamp".to_string(),
    //             comparator: "<=".to_string(),
    //             value: "3153600000".to_string(), // 100 years
    //         },
    //     };

    //     let check_timelock_condition =
    //         check_condition_timelock(&timelock_condition, &get_auth_sig_cheqd()).await;

    //     assert_eq!(check_timelock_condition.is_ok(), true);
    //     assert_eq!(check_timelock_condition.unwrap(), true);
    // }

    // #[tokio::test]
    // async fn test_check_condition_timelock_negative() {
    //     let timelock_condition = CosmosCondition {
    //         path: "/cosmos/tx/v1beta1/txs?events=transfer.recipient='cheqd1xl5wccz667lk06ahama26pdqvrkz5aws6m0ztp'&events=transfer.amount='147603000000000ncheq'&order_by=2&pagination.limit=1".to_string(),
    //         chain: "cheqdMainnet".to_string(),
    //         method: Some("timelock".to_string()),
    //         parameters: Some(vec!["latest".to_string()]),
    //         return_value_test: JsonReturnValueTestV2 {
    //             key: "$.tx_responses.*.timestamp".to_string(),
    //             comparator: "<=".to_string(),
    //             value: "10".to_string(), // 10 seconds
    //         },
    //     };

    //     let check_timelock_condition =
    //         check_condition_timelock(&timelock_condition, &get_auth_sig_cheqd()).await;

    //     assert_eq!(check_timelock_condition.is_ok(), true);
    //     assert_eq!(check_timelock_condition.unwrap(), false);
    // }

    #[tokio::test]
    async fn test_check_condition_string() {
        let address_condition = CosmosCondition {
            path: ":userAddress".to_string(),
            chain: CHAIN_COSMOS.to_string(),
            method: None,
            parameters: None,
            return_value_test: JsonReturnValueTestV2 {
                key: "".to_string(),
                comparator: "=".to_string(),
                value: "cosmos1n8yrje7g3mkrmj4x2m5u67sdpwcwgtxwzv4vwv".to_string(),
            },
        };

        let check_balance_condition =
            check_condition(&address_condition, &get_auth_sig(), &"".to_string(), None).await;
        assert!(check_balance_condition.is_ok());
        assert!(check_balance_condition.unwrap());
    }

    // FIXME: Fix so that this test doesn't rely on actually making an outbound network call.
    #[ignore]
    #[tokio::test]
    async fn test_check_condition_numerical() {
        let balance_condition = CosmosCondition {
            path: "/cosmos/bank/v1beta1/balances/cosmos1n8yrje7g3mkrmj4x2m5u67sdpwcwgtxwzv4vwv"
                .to_string(),
            chain: CHAIN_COSMOS.to_string(),
            method: None,
            parameters: None,
            return_value_test: JsonReturnValueTestV2 {
                key: "$.balances[0].amount".to_string(),
                comparator: ">=".to_string(),
                value: "0".to_string(),
            },
        };

        let check_balance_condition =
            check_condition(&balance_condition, &get_auth_sig(), &"".to_string(), None).await;
        assert!(check_balance_condition.is_ok());
        assert!(check_balance_condition.unwrap());
    }
}
