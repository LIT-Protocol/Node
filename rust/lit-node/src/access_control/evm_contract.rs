use crate::auth::auth_material::JsonAuthSig;
use crate::auth::resources::LitResourceAbility;
use crate::error::{conversion_err_code, validation_err, validation_err_code, Result, EC};
use async_recursion::async_recursion;
use ethabi::ethereum_types::{H160, U256};
use ethabi::token::{LenientTokenizer, Tokenizer};
use ethabi::Token;
use lit_core::config::LitConfig;
use lit_core::error::Unexpected;
use web3::types::{Bytes, CallRequest};

use super::{rpc_call, substitute_special_params, validate_boolean_expression};
use crate::models::{AccessControlBooleanOperator, EVMContractCondition, EVMContractConditionItem};
use crate::utils::encoding;

pub(crate) async fn check_access_control_conditions(
    conditions: &Vec<EVMContractConditionItem>,
    auth_sig: &JsonAuthSig,
    requested_lit_resource_ability: &LitResourceAbility,
    chain: &Option<String>,
    cfg: &LitConfig,
) -> Result<bool> {
    let validate_res = auth_sig
        .validate_and_get_wallet_sig(requested_lit_resource_ability, chain, cfg)
        .await
        .map_err(|e| {
            validation_err_code(e, EC::NodeInvalidAuthSig, Some("Invalid AuthSig".into()))
        });
    match validate_res {
        Ok(valid_auth_sig) => {
            let conditions_met = check_condition_group(conditions, &valid_auth_sig).await?;
            Ok(conditions_met)
        }
        Err(err) => {
            warn!("Error checking auth sig: {:?}", err);
            Ok(false)
        }
    }
}

#[async_recursion]
async fn check_condition_group(
    conditions: &Vec<EVMContractConditionItem>,
    auth_sig: &JsonAuthSig,
) -> Result<bool> {
    if !validate_boolean_expression(conditions) {
        return Err(validation_err_code(
            "Invalid boolean EVM Access Control Conditions",
            EC::NodeInvalidBooleanConditionType,
            None,
        ));
    }

    let mut results: Vec<bool> = Vec::new();
    let mut operators: Vec<AccessControlBooleanOperator> = Vec::new();
    for condition_item in conditions {
        match condition_item {
            EVMContractConditionItem::Condition(condition) => {
                results.push(check_condition(condition, auth_sig).await?);
            }
            EVMContractConditionItem::Operator(operator) => {
                operators.push(operator.operator.clone());
            }
            EVMContractConditionItem::Group(group) => {
                results.push(check_condition_group(group, auth_sig).await?);
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

    return Ok(*results
        .last()
        .expect_or_err("Could not get last result from results vector")?);
}

pub async fn check_condition(
    condition: &EVMContractCondition,
    auth_sig: &JsonAuthSig,
) -> Result<bool> {
    // bind the parameters and create the call request
    // let func: ethabi::Function = serde_json::from_str(&condition.function_abi)?;
    let func = condition.function_abi.clone();

    let mut param_tokens: Vec<Token> = Vec::new();
    let param_types: Vec<ethabi::ParamType> = func.inputs.iter().map(|i| i.kind.clone()).collect();
    if param_types.len() != condition.function_params.len() {
        return Err(validation_err_code(
            "Number of parameters in function does not match number of parameters in condition",
            EC::NodeMismatchParameters,
            None,
        )
        .add_source_to_details());
    }

    for (i, param_type) in param_types.iter().enumerate() {
        let mut substituted_param =
            substitute_special_params(&condition.function_params[i], auth_sig)
                .await
                .map_err(|e| validation_err(e, None))?;

        match param_type {
            ethabi::ParamType::Address
            | ethabi::ParamType::Bytes
            | ethabi::ParamType::FixedBytes(1)
            | ethabi::ParamType::FixedBytes(2)
            | ethabi::ParamType::FixedBytes(3)
            | ethabi::ParamType::FixedBytes(4)
            | ethabi::ParamType::FixedBytes(5)
            | ethabi::ParamType::FixedBytes(6)
            | ethabi::ParamType::FixedBytes(7)
            | ethabi::ParamType::FixedBytes(8)
            | ethabi::ParamType::FixedBytes(9)
            | ethabi::ParamType::FixedBytes(10)
            | ethabi::ParamType::FixedBytes(11)
            | ethabi::ParamType::FixedBytes(12)
            | ethabi::ParamType::FixedBytes(13)
            | ethabi::ParamType::FixedBytes(14)
            | ethabi::ParamType::FixedBytes(15)
            | ethabi::ParamType::FixedBytes(16)
            | ethabi::ParamType::FixedBytes(17)
            | ethabi::ParamType::FixedBytes(18)
            | ethabi::ParamType::FixedBytes(19)
            | ethabi::ParamType::FixedBytes(20)
            | ethabi::ParamType::FixedBytes(21)
            | ethabi::ParamType::FixedBytes(22)
            | ethabi::ParamType::FixedBytes(23)
            | ethabi::ParamType::FixedBytes(24)
            | ethabi::ParamType::FixedBytes(25)
            | ethabi::ParamType::FixedBytes(26)
            | ethabi::ParamType::FixedBytes(27)
            | ethabi::ParamType::FixedBytes(28)
            | ethabi::ParamType::FixedBytes(29)
            | ethabi::ParamType::FixedBytes(30)
            | ethabi::ParamType::FixedBytes(31)
            | ethabi::ParamType::FixedBytes(32) => {
                substituted_param = substituted_param.replace("0x", "")
            }
            _ => {}
        }
        match LenientTokenizer::tokenize(param_type, &substituted_param) {
            Ok(token) => {
                param_tokens.push(token);
            }
            Err(err) => {
                return Err(validation_err_code(
                    err,
                    EC::NodeConditionTokenizingError,
                    Some(format!(
                        "Error tokenizing param: {:?} with substituted param: {:?}",
                        param_type, substituted_param
                    )),
                ));
            }
        }
    }

    let func_params = func
        .encode_input(&param_tokens)
        .map_err(|e| validation_err_code(e, EC::NodeTokenEncodingDecodingError, None))?;
    let call_request = CallRequest {
        from: None,
        to: Some(web3::types::H160::from_slice(
            &encoding::hex_to_bytes(&condition.contract_address).map_err(|e| {
                conversion_err_code(e, EC::NodeConditionAddressConversionError, None)
            })?,
        )),
        data: Some(Bytes::from(func_params)),
        gas: None,
        gas_price: None,
        value: None,
        transaction_type: None,
        access_list: None,
        max_fee_per_gas: None,
        max_priority_fee_per_gas: None,
    };
    let call_result = rpc_call(&call_request, &condition.chain).await?;

    let parsed_result: Vec<Token> = func
        .decode_output(&call_result.0)
        .map_err(|e| validation_err_code(e, EC::NodeTokenEncodingDecodingError, None))?;
    debug!("parsed contract cal result: {:?}", parsed_result);

    let returned_value = if condition.return_value_test.key.is_empty() {
        // use the first return value
        // get the type
        parsed_result[0].clone()
    } else {
        // check against the key specified
        let key_index = func
            .outputs
            .iter()
            .position(|x| x.name == condition.return_value_test.key)
            .expect_or_err("Could not find key in function outputs")?;
        debug!(
            "Checking for value at index {:?} of parsed_result",
            key_index
        );
        parsed_result[key_index].clone()
    };

    match returned_value {
        Token::Bool(t) => check_return_value_bool(condition, t),
        Token::String(t) => check_return_value_string(condition, t),
        Token::Address(t) => {
            let t: [u8; 20] = t.into();
            check_return_value_addr(condition, t.into(), auth_sig).await
        }
        Token::Bytes(bytes) | Token::FixedBytes(bytes) => Err(validation_err_code(
            "Bytes and FixedBytes not supported",
            EC::NodeInvalidConditionTokenType,
            None,
        )
        .add_source_to_details()),
        Token::Uint(t) => {
            let t: [u8; 32] = t.into();
            check_return_value_uint(condition, t.into())
        }
        Token::Int(t) => {
            // FIXME: it's currently impossible to check signed integers because
            // of this github issue https://github.com/rust-ethereum/ethabi/issues/249
            // until this is fixed, we have to return an error.  When it's fixed, uncomment the below, and it should "just work"

            // return check_return_value_int(&condition, t);
            Err(validation_err_code("Signed integers are not supported yet.",
                                           EC::NodeInvalidConditionTokenType, None)
                .add_detail("Signed integers are not supported yet. See this github issue for why: https://github.com/rust-ethereum/ethabi/issues/249"))
        }
        Token::Array(arr) | Token::FixedArray(arr) => Err(validation_err_code(
            "Array and FixedArray not supported",
            EC::NodeInvalidConditionTokenType,
            None,
        )
        .add_source_to_details()),
        Token::Tuple(s) => {
            Err(validation_err_code(
                "Tuple not supported",
                EC::NodeInvalidConditionTokenType,
                None,
            )
            .add_source_to_details())
            // let s = s.iter().map(|ref t| format!("{}", t)).collect::<Vec<String>>().join(",");

            // write!(f, "({})", s)
        }
    }
}

#[allow(clippy::bool_comparison)]
fn check_return_value_bool(condition: &EVMContractCondition, returned_value: bool) -> Result<bool> {
    debug!("Checking return value {:?}", returned_value);

    let valid_return_value = condition.return_value_test.value.parse().map_err(|e| {
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

fn check_return_value_string(
    condition: &EVMContractCondition,
    returned_value: String,
) -> Result<bool> {
    debug!("Checking return value {:?}", returned_value);

    let valid_return_value = condition.return_value_test.value.clone();

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
    } else if condition.return_value_test.comparator == "contains" {
        return Ok(returned_value.contains(&valid_return_value));
    } else {
        warn!("Error - unsupported return value test comparator");
        return Ok(false);
    }
}

// fn check_return_value_int(
//     condition: &models::EVMContractCondition,
//     returned_value: i128,
// ) -> Result<bool> {
//     println!("Checking return value {:?}", returned_value);

//     let valid_return_value: i128 = condition
//         .return_value_test
//         .value
//         .parse()
//         .expect("failed to parse valid return value from access control condition");

//     println!(
//         "Testing: Is {:?} {:?} {:?}",
//         returned_value, condition.return_value_test.comparator, valid_return_value
//     );

//     if condition.return_value_test.comparator == ">" {
//         return Ok(returned_value > valid_return_value);
//     } else if condition.return_value_test.comparator == "<" {
//         return Ok(returned_value < valid_return_value);
//     } else if condition.return_value_test.comparator == ">=" {
//         return Ok(returned_value >= valid_return_value);
//     } else if condition.return_value_test.comparator == "<=" {
//         return Ok(returned_value <= valid_return_value);
//     } else if condition.return_value_test.comparator == "=" {
//         return Ok(returned_value == valid_return_value);
//     } else if condition.return_value_test.comparator == "!=" {
//         return Ok(returned_value != valid_return_value);
//     } else {
//         println!("Error - unsupported return value test comparator");
//         return Ok(false);
//     }
// }

fn check_return_value_uint(condition: &EVMContractCondition, returned_value: U256) -> Result<bool> {
    debug!("Checking return value {:?}", returned_value);

    let valid_return_value =
        U256::from_dec_str(&condition.return_value_test.value).map_err(|e| {
            validation_err_code(
                e,
                EC::NodeInvalidACCReturnValueTest,
                Some("Failed to parse valid return value from access control condition".into()),
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

async fn check_return_value_addr(
    condition: &EVMContractCondition,
    returned_value: H160, // this is an eth address
    auth_sig: &JsonAuthSig,
) -> Result<bool> {
    debug!("Checking return value {:?}", returned_value);
    let subbed_param = substitute_special_params(&condition.return_value_test.value, auth_sig)
        .await
        .map_err(|e| validation_err_code(e, EC::NodeInvalidSIWESpecialParam, None))?;

    let valid_return_value = H160::from_slice(
        &encoding::hex_to_bytes(subbed_param)
            .map_err(|e| conversion_err_code(e, EC::NodeConditionAddressConversionError, None))?,
    );
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
