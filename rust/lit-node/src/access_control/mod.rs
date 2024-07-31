use lit_core::{config::LitConfig, error::Unexpected};
use siwe::Message;
use std::env;
use std::sync::Arc;
use web3::{
    contract::{Contract, Options},
    types::{Address, Bytes, CallRequest, U256},
};

use lit_blockchain::resolver::rpc;
use lit_blockchain::resolver::rpc::RpcHealthcheckPoller;

use crate::functions::action_client;
use crate::{auth::auth_material::JsonAuthSig, utils::encoding};
use crate::{
    auth::{auth_material::AuthSigItem, resources::LitResourceAbility},
    error::{
        blockchain_err, blockchain_err_code, config_err, conversion_err, conversion_err_code,
        parser_err_code, serializer_err_code, unexpected_err, unexpected_err_code, validation_err,
        validation_err_code, Result, EC,
    },
};
use crate::{
    models::{
        AccessControlBooleanOperator, AccessControlConditionItem, ControlConditionItem,
        JsonAccessControlCondition, JsonAccessControlConditionOperator, PoapEntry,
    },
    utils::web::EndpointVersion,
};

pub mod cosmos;
pub mod evm_contract;
pub mod sol_rpc;
pub mod unified;

pub fn get_solana_auth_sig(auth_sig_item: &AuthSigItem) -> Result<&JsonAuthSig> {
    let auth_sig: &JsonAuthSig;
    match auth_sig_item {
        AuthSigItem::Multiple(multiple_auth_sigs) => match &multiple_auth_sigs.solana {
            Some(solana_auth_sig) => {
                auth_sig = solana_auth_sig;
            }
            None => {
                return Err(validation_err_code(
                    "No Solana auth sig found",
                    EC::NodeInvalidSolanaAuthSig,
                    None,
                )
                .add_source_to_details());
            }
        },
        AuthSigItem::Single(single_auth_sig) => {
            auth_sig = single_auth_sig;
        }
    }
    Ok(auth_sig)
}

pub fn get_ethereum_auth_sig(auth_sig_item: &AuthSigItem) -> Result<&JsonAuthSig> {
    let auth_sig: &JsonAuthSig;
    match auth_sig_item {
        AuthSigItem::Multiple(multiple_auth_sigs) => match &multiple_auth_sigs.ethereum {
            Some(ethereum_auth_sig) => {
                auth_sig = ethereum_auth_sig;
            }
            None => {
                return Err(validation_err_code(
                    "No Ethereum auth sig found",
                    EC::NodeInvalidEthereumAuthSig,
                    None,
                )
                .add_source_to_details());
            }
        },
        AuthSigItem::Single(single_auth_sig) => {
            auth_sig = single_auth_sig;
        }
    }
    Ok(auth_sig)
}

pub fn get_web3<C>(chain: C) -> Result<web3::Web3<web3::transports::Http>>
where
    C: AsRef<str>,
{
    let url = rpc_url(chain.as_ref()).map_err(|e| {
        validation_err_code(e, EC::NodeWrongNetwork, Some("Chain not found".into()))
    })?;
    let transport = web3::transports::Http::new(&url)
        .map_err(|e| conversion_err_code(e, EC::NodeHTTPConversionError, None))?;

    Ok(web3::Web3::new(transport))
}

pub async fn rpc_call<C>(call_request: &CallRequest, chain: C) -> Result<Bytes>
where
    C: AsRef<str>,
{
    get_web3(chain.as_ref())
        .map_err(|e| {
            blockchain_err_code(e, EC::NodeRpcError, Some("Web3 Error".into())).add_msg_to_details()
        })?
        .eth()
        .call(call_request.to_owned(), None)
        .await
        .map_err(|e| {
            blockchain_err_code(
                e,
                EC::NodeBlockchainError,
                Some("Error making RPC Call".into()),
            )
        })
}

#[allow(clippy::too_many_arguments)]
pub(crate) async fn check_access_control_conditions(
    conditions: &Vec<AccessControlConditionItem>,
    auth_sig: &JsonAuthSig,
    requested_lit_resource_ability: &LitResourceAbility,
    chain: &Option<String>,
    cfg: Arc<LitConfig>,
    request_id: &String,
    bls_root_pubkey: &String,
    endpoint_version: &EndpointVersion,
    current_action_ipfs_id: Option<&String>,
) -> Result<bool> {
    let validate_res = auth_sig
        .validate_and_get_wallet_sig(
            requested_lit_resource_ability,
            chain,
            &cfg,
            bls_root_pubkey,
            endpoint_version,
        )
        .await
        .map_err(|e| {
            validation_err_code(e, EC::NodeInvalidAuthSig, Some("Invalid AuthSig".into()))
        });
    match validate_res {
        Ok(valid_auth_sig) => {
            let fixed_conditions = insert_ands_if_necessary(conditions);
            let conditions_met = check_condition_group(
                &fixed_conditions,
                &valid_auth_sig,
                cfg,
                request_id,
                bls_root_pubkey,
                endpoint_version,
                current_action_ipfs_id,
            )
            .await?;
            Ok(conditions_met)
        }
        Err(_) => Ok(false),
    }
}

// Before we added proper boolean operators to AccessControlConditions,
// we accepted an array, and all items in the array were ANDed together.
// This function inserts ANDs if there are multiple conditions in the array
// but no operators present
fn insert_ands_if_necessary(
    conditions: &Vec<AccessControlConditionItem>,
) -> Vec<AccessControlConditionItem> {
    // does this vec have more than 1 item?  abort if not.
    if conditions.len() < 2 {
        return conditions.clone();
    }

    // are there any operators?  if so, then abort and don't do this processing
    let operators_found = check_for_operators(conditions);
    if operators_found {
        return conditions.clone();
    }

    let mut conditions_with_ands: Vec<AccessControlConditionItem> = Vec::new();
    for (i, x) in conditions.iter().enumerate() {
        conditions_with_ands.push(x.clone());

        // insert an "and" unless this is the last item
        if i < conditions.len() - 1 {
            conditions_with_ands.push(AccessControlConditionItem::Operator(
                JsonAccessControlConditionOperator {
                    operator: AccessControlBooleanOperator::And,
                },
            ));
        }
    }
    conditions_with_ands
}

pub fn validate_boolean_expression<T>(conditions: &Vec<ControlConditionItem<T>>) -> bool {
    #[derive(PartialEq)]
    enum State {
        Start,
        Operator,
        Condition,
    }

    let mut current_state = State::Start;
    for condition_item in conditions {
        match current_state {
            State::Start | State::Operator => match condition_item {
                ControlConditionItem::Operator(_) => return false,
                ControlConditionItem::Group(group) => {
                    if !validate_boolean_expression(group) {
                        return false;
                    }
                    current_state = State::Condition;
                }
                _ => current_state = State::Condition,
            },
            State::Condition => match condition_item {
                ControlConditionItem::Operator(_) => current_state = State::Operator,
                _ => return false,
            },
        }
    }

    current_state == State::Condition
}

async fn check_condition_group(
    conditions: &Vec<AccessControlConditionItem>,
    auth_sig: &JsonAuthSig,
    cfg: Arc<LitConfig>,
    request_id: &String,
    bls_root_pubkey: &String,
    endpoint_version: &EndpointVersion,
    current_action_ipfs_id: Option<&String>,
) -> Result<bool> {
    if !validate_boolean_expression(conditions) {
        return Err(validation_err_code(
            "Invalid boolean Access Control Conditions",
            EC::NodeInvalidBooleanConditionType,
            None,
        ));
    }

    let mut results: Vec<bool> = Vec::new();
    let mut operators: Vec<AccessControlBooleanOperator> = Vec::new();
    for condition_item in conditions {
        match condition_item {
            AccessControlConditionItem::Condition(condition) => {
                results.push(
                    check_condition(
                        condition,
                        auth_sig,
                        cfg.clone(),
                        request_id,
                        bls_root_pubkey,
                        endpoint_version,
                        current_action_ipfs_id,
                    )
                    .await?,
                );
            }
            AccessControlConditionItem::Operator(operator) => {
                operators.push(operator.operator.clone());
            }
            AccessControlConditionItem::Group(group) => {
                results.push(
                    Box::pin(check_condition_group(
                        group,
                        auth_sig,
                        cfg.clone(),
                        request_id,
                        bls_root_pubkey,
                        endpoint_version,
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

    // check if the number of operators is the same as the size of results
    if operators.len() != results.len() - 1 {
        return Err(validation_err_code(
            "Invalid number of operators",
            EC::NodeAccessControlConditionsCheckFailed,
            None,
        ));
    }

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

// recursively search condition groups for operators
fn check_for_operators(conditions: &Vec<AccessControlConditionItem>) -> bool {
    for condition in conditions {
        match condition {
            AccessControlConditionItem::Operator(_) => {
                return true;
            }
            AccessControlConditionItem::Group(group) => {
                if check_for_operators(group) {
                    return true;
                }
            }
            _ => {}
        }
    }
    false
}

pub fn rpc_url<C>(chain: C) -> Result<String>
where
    C: AsRef<str>,
{
    let url = rpc::ENDPOINT_MANAGER.rpc_url(chain.as_ref()).map_err(|e| {
        blockchain_err_code(
            e,
            EC::NodeBlockchainChainUnknown,
            Some(format!("config not found for chain: {}", chain.as_ref())),
        )
    })?;
    Ok(url)
}

async fn check_condition(
    condition: &JsonAccessControlCondition,
    auth_sig: &JsonAuthSig,
    cfg: Arc<LitConfig>,
    request_id: &str,
    bls_root_pubkey: &String,
    endpoint_version: &EndpointVersion,
    current_action_ipfs_id: Option<&String>,
) -> Result<bool> {
    let web3 = get_web3(condition.chain.as_str()).map_err(|e| {
        blockchain_err_code(e, EC::NodeRpcError, Some("Web3 Error".into())).add_msg_to_details()
    })?;

    if !condition.standard_contract_type.is_empty() {
        if condition.standard_contract_type == "POAP" {
            check_condition_via_poap(condition, auth_sig).await
        } else if condition.standard_contract_type == "timestamp" {
            return check_condition_via_timestamp(condition).await;
        } else if condition.standard_contract_type == "SIWE" {
            return check_condition_via_siwe(condition, auth_sig).await;
        } else if condition.standard_contract_type == "LitAction" {
            #[cfg(feature = "lit-actions")]
            return check_condition_via_lit_action(
                condition,
                auth_sig,
                cfg,
                request_id,
                bls_root_pubkey,
                endpoint_version,
                current_action_ipfs_id,
            )
            .await;
            #[cfg(not(feature = "lit-actions"))]
            return Err(unexpected_err_code(
                "Lit Actions are not enabled",
                EC::NodeLitActionsNotEnabled,
                None,
            ));
        } else {
            return check_condition_via_contract_call(
                &web3,
                condition,
                auth_sig,
                bls_root_pubkey,
                current_action_ipfs_id,
            )
            .await;
        }
    } else if condition.method.is_empty() {
        return check_condition_via_signature(
            &web3,
            condition,
            auth_sig,
            bls_root_pubkey,
            current_action_ipfs_id,
        )
        .await;
    } else {
        return check_condition_via_rpc_method(
            &web3,
            condition,
            auth_sig,
            bls_root_pubkey,
            current_action_ipfs_id,
        )
        .await;
    }
}

async fn check_condition_via_signature(
    web3: &web3::Web3<web3::transports::Http>,
    condition: &JsonAccessControlCondition,
    auth_sig: &JsonAuthSig,
    bls_root_pubkey: &String,
    current_action_ipfs_id: Option<&String>,
) -> Result<bool> {
    let subbed_param = substitute_special_params(
        &condition.parameters[0],
        auth_sig,
        bls_root_pubkey,
        current_action_ipfs_id,
    )
    .await?;
    if &condition.parameters[0] == ":userAddress" {
        let account_address =
            Address::from_slice(&encoding::hex_to_bytes(&subbed_param).map_err(|e| {
                conversion_err_code(e, EC::NodeConditionAddressConversionError, None)
            })?);
        check_return_value_addr(
            condition,
            account_address,
            auth_sig,
            bls_root_pubkey,
            current_action_ipfs_id,
        )
        .await
    } else {
        check_return_value_str(condition, subbed_param)
    }
}

async fn check_condition_via_timestamp(condition: &JsonAccessControlCondition) -> Result<bool> {
    let web3 = get_web3(condition.chain.as_str()).map_err(|e| {
        blockchain_err_code(e, EC::NodeRpcError, Some("Web3 Error".into())).add_msg_to_details()
    })?;

    let latest_block = web3
        .eth()
        .block_number()
        .await
        .map_err(|e| blockchain_err_code(e, EC::NodeBlockchainError, None))?;

    // let block_timestamp =
    let possible_block = web3
        .eth()
        .block(web3::types::BlockId::from(latest_block))
        .await
        .map_err(|e| {
            blockchain_err_code(e, EC::NodeRpcError, Some("Error making RPC Call".into()))
        })?;

    let block_timestamp = possible_block
        .ok_or_else(|| {
            blockchain_err_code(
                "Could not get block when trying to get block timestamp",
                EC::NodeRpcError,
                None,
            )
        })?
        .timestamp;

    check_return_value_int(condition, block_timestamp).map_err(|e| validation_err(e, None))
}

async fn get_poaps_for_user(
    condition: &JsonAccessControlCondition,
    auth_sig: &JsonAuthSig,
) -> Result<Vec<PoapEntry>> {
    // use POAP API to get all POAPs the user owns
    let poap_api_key = env::var("POAP_API_KEY")
        .map_err(|e| config_err(e, Some("POAP_API_KEY env var not found".into())))?;
    let url = format!(
        "{}{}",
        "https://api.poap.tech/actions/scan/".to_owned(),
        &auth_sig.address
    );
    let client = reqwest::Client::builder()
        .build()
        .map_err(|e| unexpected_err(e, None))?;
    let result = client
        .get(url)
        .header("X-API-Key", poap_api_key)
        .send()
        .await
        .map_err(|e| {
            blockchain_err_code(e, EC::NodeRpcError, Some("Error making RPC Call".into()))
        })?;

    let poaps = result
        .json::<Vec<PoapEntry>>()
        .await
        .map_err(|e| serializer_err_code(e, EC::NodePOAPJSONError, None))?;

    Ok(poaps)
}

async fn check_condition_via_poap(
    condition: &JsonAccessControlCondition,
    auth_sig: &JsonAuthSig,
) -> Result<bool> {
    if condition.method == "tokenURI" {
        let poaps = get_poaps_for_user(condition, auth_sig).await?;
        for poap in &poaps {
            let poap_event_contains_value = condition.return_value_test.comparator == "contains"
                && poap.event.name.contains(&condition.return_value_test.value);
            let poap_event_equals_value = condition.return_value_test.comparator == "="
                && poap.event.name == condition.return_value_test.value;
            if poap_event_contains_value || poap_event_equals_value {
                return Ok(true);
            }
        }
        Ok(false)
    } else if condition.method == "eventId" {
        let poaps = get_poaps_for_user(condition, auth_sig).await?;
        for poap in &poaps {
            if condition.return_value_test.comparator == "="
                && poap.event.id.to_string() == condition.return_value_test.value
            {
                return Ok(true);
            }
        }
        return Ok(false);
    } else {
        warn!("Unsupported method for contract ABI: {}", condition.method);
        return Ok(false);
    }
}

async fn check_condition_via_rpc_method(
    web3: &web3::Web3<web3::transports::Http>,
    condition: &JsonAccessControlCondition,
    auth_sig: &JsonAuthSig,
    bls_root_pubkey: &String,
    current_action_ipfs_id: Option<&String>,
) -> Result<bool> {
    // run a rpc call
    if condition.method == "eth_getBalance" {
        let address_to_check = substitute_special_params(
            &condition.parameters[0],
            auth_sig,
            bls_root_pubkey,
            current_action_ipfs_id,
        )
        .await
        .map_err(|e| validation_err_code(e, EC::NodeInvalidSIWESpecialParam, None))?;
        let balance = web3
            .eth()
            .balance(
                Address::from_slice(&encoding::hex_to_bytes(&address_to_check).map_err(|e| {
                    conversion_err_code(e, EC::NodeConditionAddressConversionError, None)
                })?),
                None,
            )
            .await
            .map_err(|e| {
                blockchain_err_code(e, EC::NodeRpcError, Some("Error making RPC Call".into()))
            })?;

        check_return_value_int(condition, balance)
    } else {
        warn!("Error - unsupported rpc method: {}", condition.method);
        Ok(false)
    }
}

async fn check_condition_via_siwe(
    condition: &JsonAccessControlCondition,
    auth_sig: &JsonAuthSig,
) -> Result<bool> {
    // parse the sig
    let message: Message = auth_sig.signed_message.parse().map_err(|e| {
        parser_err_code(
            e,
            EC::NodeSIWEMessageError,
            Some("Error parsing SIWE message".into()),
        )
    })?;

    if condition.parameters[0] == ":domain" {
        let domain = message.domain;

        check_return_value_str(condition, domain.to_string())
    } else if condition.parameters[0] == ":resources" {
        let resources = message
            .resources
            .iter()
            .map(|r| r.to_string())
            .collect::<Vec<String>>();

        check_return_value_array(condition, resources, auth_sig)
    } else {
        warn!("Error - unsupported siwe method");
        Ok(false)
    }
}

#[cfg(feature = "lit-actions")]
async fn check_condition_via_lit_action(
    condition: &JsonAccessControlCondition,
    auth_sig: &JsonAuthSig,
    cfg: Arc<LitConfig>,
    request_id: &str,
    bls_root_pubkey: &String,
    endpoint_version: &EndpointVersion,
    current_action_ipfs_id: Option<&String>,
) -> Result<bool> {
    use crate::error::{memory_limit_err_code, timeout_err_code};
    use crate::models::DenoExecutionEnv;
    use crate::utils::web::get_auth_context;
    use crate::utils::web::get_ipfs_file;
    use std::error::Error;

    let ipfs_id = condition.contract_address.replace("ipfs://", "");

    // pull down the code from ipfs
    let code_from_ipfs = get_ipfs_file(&ipfs_id.to_string(), &cfg)
        .await
        .map_err(|e| unexpected_err(e, Some("Error getting ipfs file".into())))?;

    // substitute in the special params
    let mut subbed_params = Vec::with_capacity(condition.parameters.len());
    for param in &condition.parameters {
        let subbed_param =
            substitute_special_params(param, auth_sig, bls_root_pubkey, current_action_ipfs_id)
                .await?;
        subbed_params.push(subbed_param);
    }

    let method_to_run = condition.method.clone();
    let params = format!("\"{}\"", subbed_params.clone().join("\",\""));

    let code_to_run = format!(
        "{}\nconst litAsyncWrapper = async () => {{const actionTestResponse = await {}({}); Lit.Actions.setResponse({{response: actionTestResponse.toString()}});}}\n litAsyncWrapper();",
        code_from_ipfs, method_to_run, params
    );

    debug!("Running code: {}", code_to_run);

    let auth_context = get_auth_context(
        Some(auth_sig.clone()),
        None,
        None,
        None,
        false,
        cfg.clone(),
        None,
        bls_root_pubkey,
        endpoint_version,
    )
    .await
    .map_err(|e| unexpected_err(e, Some("Error getting auth context".into())))?;

    let deno_execution_env = DenoExecutionEnv {
        tss_state: None,
        auth_context,
        cfg,
    };

    let execution_result = action_client::ClientBuilder::default()
        .js_env(deno_execution_env)
        .auth_sig(Some(auth_sig.clone()))
        .request_id(request_id.to_string())
        .endpoint_version(*endpoint_version)
        .build()
        .map_err(|e| {
            unexpected_err_code(
                e,
                EC::NodeJsExecutionError,
                Some("Error building action client".into()),
            )
        })
        .and_then(|mut client| {
            client.execute_js_blocking(action_client::ExecutionOptions {
                code: code_to_run,
                action_ipfs_id: Some(ipfs_id),
                ..Default::default()
            })
        });

    let execution_state = match execution_result {
        Ok(state) => state,
        Err(err) => {
            match err.kind() {
                lit_api_core::error::Kind::Timeout => {
                    return Err(timeout_err_code(err, EC::NodeJsTimeoutError, None));
                }
                lit_api_core::error::Kind::MemoryLimit => {
                    return Err(memory_limit_err_code(err, EC::NodeJsMemoryLimitError, None));
                }
                _ => {}
            }
            if let Some(source_err) = err.source() {
                return Err(unexpected_err_code(
                    source_err.to_string(),
                    EC::NodeJsExecutionError,
                    Some("Error executing JS".into()),
                ));
            }
            return Err(unexpected_err_code(
                err,
                EC::NodeJsExecutionError,
                Some("Error executing JS".into()),
            ));
        }
    };

    check_return_value_str(condition, execution_state.response)
}

// TODO: This needs refactoring. It's way too long for a single method.
async fn check_condition_via_contract_call(
    web3: &web3::Web3<web3::transports::Http>,
    condition: &JsonAccessControlCondition,
    auth_sig: &JsonAuthSig,
    bls_root_pubkey: &String,
    current_action_ipfs_id: Option<&String>,
) -> Result<bool> {
    let abi: Vec<u8>;
    if condition.standard_contract_type == "ERC1155" {
        abi = include_bytes!("../../abis/ERC1155.json").to_vec();
    } else if condition.standard_contract_type == "ERC721"
        || condition.standard_contract_type == "ERC721MetadataName"
    {
        abi = include_bytes!("../../abis/ERC721.json").to_vec();
    } else if condition.standard_contract_type == "ERC20" {
        abi = include_bytes!("../../abis/ERC20.json").to_vec();
    } else if condition.standard_contract_type == "MolochDAOv2.1" {
        abi = include_bytes!("../../abis/MolochDAOv2_1.json").to_vec();
    } else if condition.standard_contract_type == "CASK" {
        abi = include_bytes!("../../abis/CASK.json").to_vec();
    } else if condition.standard_contract_type == "Creaton" {
        abi = include_bytes!("../../abis/Creaton.json").to_vec();
    } else if condition.standard_contract_type == "ProofOfHumanity" {
        abi = include_bytes!("../../abis/ProofOfHumanity.json").to_vec();
    } else if condition.standard_contract_type == "PKPPermissions" {
        abi = include_bytes!("../../../lit-core/lit-blockchain/abis/PKPPermissions.json").to_vec();
    } else {
        warn!("Error: unsupported standard contract type");
        return Ok(false);
    }

    let contract_address = web3::types::Address::from_slice(
        &encoding::hex_to_bytes(&condition.contract_address)
            .map_err(|e| conversion_err_code(e, EC::NodeConditionAddressConversionError, None))?,
    );
    let contract = Contract::from_json(web3.eth(), contract_address, &abi)
        .map_err(|e| blockchain_err(e, Some("failed to init contract".into())))?;

    debug!("Attempting to query {} chain", condition.chain);
    if condition.standard_contract_type == "PKPPermissions" {
        if condition.method == "isPermittedAction" || condition.method == "isPermittedAddress" {
            let raw_token_id = condition.parameters[0].clone();
            let token_id;

            if let Some(stripped) = raw_token_id.strip_prefix("0x") {
                match U256::from_str_radix(stripped, 16) {
                    Ok(_token_id) => token_id = _token_id,
                    Err(e) => {
                        return Err(parser_err_code(
                            e,
                            EC::NodeConditionTokenIdParsingError,
                            Some("Count not parse token id as hex".into()),
                        )
                        .add_msg_to_details())
                    }
                }
            } else {
                match U256::from_dec_str(&condition.parameters[1]) {
                    Ok(_token_id) => token_id = _token_id,
                    Err(e) => {
                        return Err(parser_err_code(
                            e,
                            EC::NodeConditionTokenIdParsingError,
                            Some("Count not parse token id as int".into()),
                        )
                        .add_msg_to_details())
                    }
                }
            }

            let resp;

            if condition.method == "isPermittedAction" {
                let ipfs_id = condition.parameters[1].clone();
                let ipfs_id_bytes = encoding::hex_to_bytes(&ipfs_id)
                    .map_err(|e| validation_err_code(e, EC::NodeInvalidIPFSID, None))?;
                let params = (token_id, ipfs_id_bytes);

                debug!("isPermittedAction params: {:?}", params);
                resp = contract
                    .query(&condition.method, params, None, Options::default(), None)
                    .await;
                debug!("Got response from isPermittedAddress: {:?}", resp);
            } else {
                // isPermittedAddress
                let address_to_check = Address::from_slice(
                    &encoding::hex_to_bytes(auth_sig.user_address(bls_root_pubkey).await?)
                        .map_err(|e| {
                            conversion_err_code(e, EC::NodeAuthSigAddressConversionError, None)
                        })?,
                );
                let params = (token_id, address_to_check);

                debug!("isPermittedAddress params: {:?}", params);
                resp = contract
                    .query(&condition.method, params, None, Options::default(), None)
                    .await;
                debug!("Got response from isPermittedAddress: {:?}", resp);
            }

            match resp {
                Ok(is_permitted) => check_return_value_bool(condition, is_permitted),
                Err(err) => Err(unexpected_err_code(
                    err,
                    EC::NodeRpcError,
                    Some("Error making RPC Call".into()),
                )),
            }
        } else {
            warn!("Unsupported method for contract ABI");
            Ok(false)
        }
    } else if condition.standard_contract_type == "ERC1155" {
        if condition.method == "balanceOf" {
            let subbed_param = substitute_special_params(
                &condition.parameters[0],
                auth_sig,
                bls_root_pubkey,
                current_action_ipfs_id,
            )
            .await?;
            let account_address =
                Address::from_slice(&encoding::hex_to_bytes(&subbed_param).map_err(|e| {
                    conversion_err_code(e, EC::NodeSIWESpecialParamAddressConversionError, None)
                })?);
            let raw_token_id = condition.parameters[1].clone();
            let token_id = if let Some(stripped) = raw_token_id.strip_prefix("0x") {
                match U256::from_str_radix(stripped, 16) {
                    Ok(token_id) => token_id,
                    Err(e) => {
                        return Err(parser_err_code(
                            e,
                            EC::NodeConditionTokenIdParsingError,
                            Some("Count not parse token id as hex".into()),
                        )
                        .add_msg_to_details())
                    }
                }
            } else {
                match U256::from_dec_str(&condition.parameters[1]) {
                    Ok(token_id) => token_id,
                    Err(e) => {
                        return Err(parser_err_code(
                            e,
                            EC::NodeConditionTokenIdParsingError,
                            Some("Count not parse token id as hex".into()),
                        )
                        .add_msg_to_details())
                    }
                }
            };
            let params = (account_address, token_id);
            debug!("balanceOf params: {:?}", params);
            let balance = contract
                .query(&condition.method, params, None, Options::default(), None)
                .await
                .map_err(|e| {
                    blockchain_err_code(e, EC::NodeRpcError, Some("Error making RPC Call".into()))
                })?;

            return check_return_value_int(condition, balance);
        } else if condition.method == "balanceOfBatch" {
            let mut account_addresses: Vec<Address> = Vec::new();
            for a in condition.parameters[0].split(',') {
                let subbed_param = substitute_special_params(
                    &a.to_string(),
                    auth_sig,
                    bls_root_pubkey,
                    current_action_ipfs_id,
                )
                .await?;
                account_addresses.push(Address::from_slice(
                    &encoding::hex_to_bytes(&subbed_param).map_err(|e| {
                        conversion_err_code(e, EC::NodeSIWESpecialParamAddressConversionError, None)
                    })?,
                ));
            }
            let mut token_ids: Vec<U256> = Vec::new();
            for t in condition.parameters[1].split(',') {
                token_ids.push(
                    U256::from_dec_str(t).map_err(|e| {
                        parser_err_code(e, EC::NodeConditionTokenIdParsingError, None)
                    })?,
                );
            }
            if token_ids.len() != account_addresses.len() {
                warn!("Parameter mismatch: the number of token ids does not match the number of account addresses");
                return Ok(false);
            }

            let params = (account_addresses, token_ids);
            debug!("balanceOfBatch params: {:?}", params);
            let balances: Vec<U256> = contract
                .query(&condition.method, params, None, Options::default(), None)
                .await
                .map_err(|e| {
                    blockchain_err_code(e, EC::NodeRpcError, Some("Error making RPC Call".into()))
                })?;

            // if any of the balances meet the condition, return true
            for b in balances {
                match check_return_value_int(condition, b) {
                    Ok(result) => {
                        if result {
                            return Ok(true);
                        }
                    }
                    Err(e) => {
                        error!("Error checking return value: {:?}", e);
                    }
                }
            }

            return Ok(false);
        } else {
            warn!("Unsupported method for contract ABI");
            return Ok(false);
        }
    } else if condition.standard_contract_type == "ERC721" {
        if condition.method == "ownerOf" {
            let token_id;
            match U256::from_dec_str(&condition.parameters[0]) {
                Ok(_token_id) => token_id = _token_id,
                Err(e) => {
                    return Err(parser_err_code(
                        e,
                        EC::NodeConditionTokenIdParsingError,
                        Some("Could not parse token id".into()),
                    ))
                }
            }
            let params = (token_id,);
            debug!("ownerOf params: {:?}", params);

            let owner = contract
                .query(&condition.method, params, None, Options::default(), None)
                .await
                .map_err(|e| {
                    blockchain_err_code(e, EC::NodeRpcError, Some("Error making RPC Call".into()))
                })?;

            return check_return_value_addr(
                condition,
                owner,
                auth_sig,
                bls_root_pubkey,
                current_action_ipfs_id,
            )
            .await;
        } else if condition.method == "balanceOf" {
            let subbed_param = substitute_special_params(
                &condition.parameters[0],
                auth_sig,
                bls_root_pubkey,
                current_action_ipfs_id,
            )
            .await?;
            let account_address =
                Address::from_slice(&encoding::hex_to_bytes(&subbed_param).map_err(|e| {
                    conversion_err_code(e, EC::NodeSIWESpecialParamAddressConversionError, None)
                })?);
            let params = (account_address,);
            debug!("balanceOf params: {:?}", params);
            let balance = contract
                .query(&condition.method, params, None, Options::default(), None)
                .await
                .map_err(|e| {
                    blockchain_err_code(e, EC::NodeRpcError, Some("Error making RPC Call".into()))
                })?;

            return check_return_value_int(condition, balance);
        } else {
            warn!("Unsupported method for contract ABI");
            return Ok(false);
        }
    } else if condition.standard_contract_type == "ERC20" {
        if condition.method == "balanceOf" {
            let subbed_param = substitute_special_params(
                &condition.parameters[0],
                auth_sig,
                bls_root_pubkey,
                current_action_ipfs_id,
            )
            .await?;
            let account_address =
                Address::from_slice(&encoding::hex_to_bytes(&subbed_param).map_err(|e| {
                    conversion_err_code(e, EC::NodeSIWESpecialParamAddressConversionError, None)
                })?);
            let params = (account_address,);
            debug!("balanceOf params: {:?}", params);
            let balance = contract
                .query(&condition.method, params, None, Options::default(), None)
                .await
                .map_err(|e| {
                    blockchain_err_code(e, EC::NodeRpcError, Some("Error making RPC Call".into()))
                })?;

            return check_return_value_int(condition, balance);
        } else {
            warn!("Unsupported method for contract ABI");
            return Ok(false);
        }
    } else if condition.standard_contract_type == "MolochDAOv2.1" {
        if condition.method == "members" {
            // println!("abi: {:?}", abi);
            let subbed_param = substitute_special_params(
                &condition.parameters[0],
                auth_sig,
                bls_root_pubkey,
                current_action_ipfs_id,
            )
            .await?;
            let account_address =
                Address::from_slice(&encoding::hex_to_bytes(&subbed_param).map_err(|e| {
                    conversion_err_code(e, EC::NodeSIWESpecialParamAddressConversionError, None)
                })?);
            let params = (account_address,);
            debug!("members params: {:?}", params);
            let member: (Address, U256, U256, bool, U256, U256) = contract
                .query(&condition.method, params, None, Options::default(), None)
                .await
                .map_err(|e| blockchain_err_code(e, EC::NodeRpcError, Some("rpc_error".into())))?;

            debug!("member is {:?}", member);
            /* Items in member, in order:
            0  delegateKey   address :  0x0000000000000000000000000000000000000000
            1  shares   uint256 :  0
            2  loot   uint256 :  0
            3  exists   bool :  false
            4  highestIndexYesVote   uint256 :  0
            5  jailed   uint256 :  0
            */
            // check that the member exists and is not jailed
            return Ok(member.3 && member.5 == U256([0, 0, 0, 0]));
        } else {
            warn!("Unsupported method for contract ABI");
            return Ok(false);
        }
    } else if condition.standard_contract_type == "Creaton" {
        if condition.method == "subscribers" {
            let subbed_param = substitute_special_params(
                &condition.parameters[0],
                auth_sig,
                bls_root_pubkey,
                current_action_ipfs_id,
            )
            .await?;
            let account_address =
                Address::from_slice(&encoding::hex_to_bytes(&subbed_param).map_err(|e| {
                    conversion_err_code(e, EC::NodeSIWESpecialParamAddressConversionError, None)
                })?);
            let params = (account_address,);
            debug!("subscribers params: {:?}", params);
            let status: u8 = contract
                .query(&condition.method, params, None, Options::default(), None)
                .await
                .map_err(|e| {
                    blockchain_err_code(e, EC::NodeRpcError, Some("Error making RPC Callr".into()))
                })?;

            debug!("status is {:?}", status);
            // check that the member exists and is not jailed
            return Ok(status == 1);
        } else {
            warn!("Unsupported method for contract ABI");
            return Ok(false);
        }
    } else if condition.standard_contract_type == "ProofOfHumanity" {
        if condition.method == "isRegistered" {
            let subbed_param = substitute_special_params(
                &condition.parameters[0],
                auth_sig,
                bls_root_pubkey,
                current_action_ipfs_id,
            )
            .await?;
            let account_address =
                Address::from_slice(&encoding::hex_to_bytes(&subbed_param).map_err(|e| {
                    conversion_err_code(e, EC::NodeSIWESpecialParamAddressConversionError, None)
                })?);
            let params = (account_address,);
            debug!("isRegistered params: {:?}", params);
            let status: bool = contract
                .query(&condition.method, params, None, Options::default(), None)
                .await
                .map_err(|e| blockchain_err_code(e, EC::NodeRpcError, Some("rpc_error".into())))?;

            debug!("status is {:?}", status);
            // check that the member exists and is not jailed
            return Ok(status);
        } else {
            warn!("Unsupported method for contract ABI");
            return Ok(false);
        }
    } else if condition.standard_contract_type == "CASK" {
        if condition.method == "getActiveSubscriptionCount" {
            let subbed_param = substitute_special_params(
                &condition.parameters[0],
                auth_sig,
                bls_root_pubkey,
                current_action_ipfs_id,
            )
            .await
            .map_err(|e| validation_err_code(e, EC::NodeInvalidSIWESpecialParam, None))?;
            let consumer_address =
                Address::from_slice(&encoding::hex_to_bytes(&subbed_param).map_err(|e| {
                    conversion_err_code(e, EC::NodeSIWESpecialParamAddressConversionError, None)
                })?);

            let subbed_param = substitute_special_params(
                &condition.parameters[1],
                auth_sig,
                bls_root_pubkey,
                current_action_ipfs_id,
            )
            .await
            .map_err(|e| validation_err_code(e, EC::NodeInvalidSIWESpecialParam, None))?;
            let provider_address =
                Address::from_slice(&encoding::hex_to_bytes(&subbed_param).map_err(|e| {
                    conversion_err_code(e, EC::NodeSIWESpecialParamAddressConversionError, None)
                })?);
            let account_id: u32 = condition.parameters[2].parse().map_err(|e| {
                conversion_err_code(
                    e,
                    EC::NodeInvalidSIWESpecialParam,
                    Some("Could not convert account_id from string to u32".into()),
                )
                .add_msg_to_details()
            })?;

            let params = (consumer_address, provider_address, account_id);
            debug!("getActiveSubscriptionCount params: {:?}", params);
            let sub_count: U256 = contract
                .query(&condition.method, params, None, Options::default(), None)
                .await
                .map_err(|e| {
                    blockchain_err_code(e, EC::NodeRpcError, Some("Error making RPC Call".into()))
                })?;
            debug!("sub_count is {:?}", sub_count);
            return check_return_value_int(condition, sub_count);
        } else {
            warn!("Unsupported method for contract ABI");
            return Ok(false);
        }
    } else {
        warn!("Error - unsupported access control condition method on contract.");
        return Ok(false);
    }
}

#[allow(clippy::bool_comparison)]
fn check_return_value_bool(
    condition: &JsonAccessControlCondition,
    returned_value: bool,
) -> Result<bool> {
    debug!("Checking return value {:?}", returned_value);

    let valid_return_value = condition
        .return_value_test
        .value
        .eq_ignore_ascii_case("true");

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

fn check_return_value_int(
    condition: &JsonAccessControlCondition,
    returned_value: U256,
) -> Result<bool> {
    debug!("Checking return value {:?}", returned_value);

    let valid_return_value = match U256::from_dec_str(&condition.return_value_test.value) {
        Ok(valid_return_value) => valid_return_value,
        Err(e) => {
            return Err(parser_err_code(
                e,
                EC::NodeInvalidACCReturnValueTest,
                Some("failed to parse valid return value from access control condition".into()),
            ));
        }
    };

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
    condition: &JsonAccessControlCondition,
    returned_value: Address,
    auth_sig: &JsonAuthSig,
    bls_root_pubkey: &String,
    current_action_ipfs_id: Option<&String>,
) -> Result<bool> {
    debug!("Checking return value {:?}", returned_value);
    let subbed_param = substitute_special_params(
        &condition.return_value_test.value,
        auth_sig,
        bls_root_pubkey,
        current_action_ipfs_id,
    )
    .await
    .map_err(|e| validation_err_code(e, EC::NodeInvalidSIWESpecialParam, None))?;
    let valid_return_value = Address::from_slice(
        &encoding::hex_to_bytes(subbed_param).map_err(|e| conversion_err(e, None))?,
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
    } else {
        warn!("Error - unsupported return value test comparator");
        return Ok(false);
    }
}

fn check_return_value_str(
    condition: &JsonAccessControlCondition,
    returned_value: String,
) -> Result<bool> {
    debug!("Checking string return value {:?}", returned_value);
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
    } else if condition.return_value_test.comparator == "!contains" {
        return Ok(!returned_value.contains(&valid_return_value));
    } else {
        warn!("Error - unsupported return value test comparator");
        return Ok(false);
    }
}

fn check_return_value_array(
    condition: &JsonAccessControlCondition,
    returned_values: Vec<String>,
    auth_sig: &JsonAuthSig,
) -> Result<bool> {
    debug!("Checking array return values {:?}", returned_values);
    let valid_return_value = condition.return_value_test.value.clone();

    trace!(
        "Testing: Is {:?} {:?} {:?}",
        returned_values,
        condition.return_value_test.comparator,
        valid_return_value
    );

    if condition.return_value_test.comparator == "contains" {
        Ok(returned_values.contains(&valid_return_value))
    } else if condition.return_value_test.comparator == "!contains" {
        return Ok(!returned_values.contains(&valid_return_value));
    } else {
        warn!("Error - unsupported return value test comparator");
        return Ok(false);
    }
}

pub async fn substitute_special_params(
    param: &String,
    auth_sig: &JsonAuthSig,
    bls_root_pubkey: &String,
    current_action_ipfs_id: Option<&String>,
) -> Result<String> {
    if param == ":userAddress" {
        return auth_sig.user_address(bls_root_pubkey).await;
    } else if param == ":currentActionIpfsId" {
        return Ok(current_action_ipfs_id.unwrap_or(&"".to_string()).clone());
    } else if param.starts_with(":litParam") {
        // find the param inside the SIWE
        let message: Message = auth_sig.signed_message.parse().map_err(|e| {
            parser_err_code(
                e,
                EC::NodeSIWEMessageError,
                Some("Error parsing SIWE message".into()),
            )
        })?;

        // loop over the SIWE resources and search for the correct one
        for resource in message.resources.iter() {
            let resource_as_string = resource.to_string();
            let resource_parts = resource_as_string.split(':').collect::<Vec<&str>>();
            if resource_parts.len() < 3 {
                return Err(validation_err_code("Error replacing special param. Resource name is invalid and does not have enough parts.", EC::NodeInvalidSIWEResource, None)
                    .add_source_to_details());
            }
            let resource_name = format!("{}:{}", resource_parts[0], resource_parts[1]);
            debug!("resource_name: {:?}", resource_name);
            debug!("param: {:?}", param);
            if resource_name == param[1..] {
                let resource_name_with_colon = format!("{}:", resource_name);

                let param_to_sub = resource_as_string
                    .strip_prefix(resource_name_with_colon.as_str())
                    .expect_or_err(
                        "Error replacing special param. The param to substitute is invalid.",
                    )
                    .map_err(|e| {
                        validation_err_code(e, EC::NodeInvalidSIWESpecialParam, None)
                            .add_source_to_details()
                    })?;
                debug!("param_to_sub before base64url decoding: {:?}", param_to_sub);
                // base64url decode the param
                let param_to_sub_decoded =
                    data_encoding::BASE64URL_NOPAD.decode(param_to_sub.as_bytes())
                        .map_err(|e| validation_err_code(e, EC::NodeInvalidSIWESpecialParam, Some("Error decoding special param with base64url. The param to substitute is invalid.".into())).add_source_to_details())?;
                let param_to_sub_decoded = String::from_utf8(param_to_sub_decoded)
                    .map_err(|e| validation_err_code(e, EC::NodeInvalidSIWESpecialParam, Some("Error decoding special param with utf8. The param to substitute is invalid.".into())).add_source_to_details())?;
                println!(
                    "param_to_sub after base64url decoding: {:?}",
                    param_to_sub_decoded
                );
                return Ok(param_to_sub_decoded);
            }
        }
    }
    Ok(param.clone())
}

#[cfg(test)]
mod tests {
    use crate::{
        access_control::validate_boolean_expression,
        models::{
            AccessControlBooleanOperator, AccessControlConditionItem, JsonAccessControlCondition,
            JsonAccessControlConditionOperator, JsonReturnValueTest,
        },
    };

    #[test]
    fn test_validate_boolean_expression() {
        let condition_a = AccessControlConditionItem::Condition(JsonAccessControlCondition {
            contract_address: "".to_string(),
            chain: "ethereum".to_string(),
            standard_contract_type: "".to_string(),
            method: "eth_getBalance".to_string(),
            parameters: vec![":userAddress".to_string(), "latest".to_string()],
            return_value_test: JsonReturnValueTest {
                comparator: ">=".to_string(),
                value: "0".to_string(),
            },
        });
        let condition_b = AccessControlConditionItem::Condition(JsonAccessControlCondition {
            contract_address: "0xc0ad7861fe8848002a3d9530999dd29f6b6cae75".to_string(),
            chain: "ethereum".to_string(),
            standard_contract_type: "ERC20".to_string(),
            method: "balanceOf".to_string(),
            parameters: vec![":userAddress".to_string()],
            return_value_test: JsonReturnValueTest {
                comparator: ">".to_string(),
                value: "10".to_string(),
            },
        });
        let operator_a = AccessControlConditionItem::Operator(JsonAccessControlConditionOperator {
            operator: AccessControlBooleanOperator::And,
        });
        let operator_b = AccessControlConditionItem::Operator(JsonAccessControlConditionOperator {
            operator: AccessControlBooleanOperator::Or,
        });
        let group_a = AccessControlConditionItem::Group(vec![
            condition_a.clone(),
            operator_a.clone(),
            condition_b.clone(),
        ]);
        let group_b = AccessControlConditionItem::Group(vec![
            condition_a.clone(),
            operator_a.clone(),
            condition_b.clone(),
            operator_b.clone(),
        ]);

        assert!(validate_boolean_expression(&vec![
            condition_a.clone(),
            operator_a.clone(),
            condition_b.clone()
        ]));
        assert!(!validate_boolean_expression(&vec![
            condition_a.clone(),
            operator_a.clone(),
            condition_b.clone(),
            operator_b.clone()
        ]));
        assert!(!validate_boolean_expression(&vec![
            condition_a.clone(),
            operator_a.clone(),
            operator_b.clone(),
            condition_b.clone()
        ]));
        assert!(!validate_boolean_expression(&vec![operator_a.clone(),]));
        assert!(!validate_boolean_expression(&vec![
            condition_a.clone(),
            condition_b.clone()
        ]));
        assert!(!validate_boolean_expression(&vec![
            condition_a.clone(),
            operator_a.clone()
        ]));
        assert!(validate_boolean_expression(&vec![
            condition_a.clone(),
            operator_a.clone(),
            group_a.clone()
        ]));
        assert!(validate_boolean_expression(&vec![
            group_a.clone(),
            operator_b.clone(),
            condition_a.clone()
        ]));
        assert!(validate_boolean_expression(&vec![
            group_a.clone(),
            operator_a.clone(),
            group_a.clone()
        ]));
        assert!(validate_boolean_expression(&vec![group_a.clone()]));
        assert!(!validate_boolean_expression(&vec![
            group_a.clone(),
            operator_a.clone(),
            operator_b.clone(),
            group_a.clone()
        ]));
        assert!(!validate_boolean_expression(&vec![
            group_a.clone(),
            operator_a.clone(),
            group_b.clone()
        ]));
        assert!(!validate_boolean_expression(&vec![
            group_a.clone(),
            operator_a.clone()
        ]));
        assert!(!validate_boolean_expression(&vec![
            group_b.clone(),
            operator_a.clone(),
            condition_a.clone()
        ]));
        assert!(validate_boolean_expression(&vec![
            condition_a,
            operator_a,
            group_a,
            operator_b,
            condition_b
        ]));
    }
}
