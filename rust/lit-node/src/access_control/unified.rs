use super::{cosmos, evm_contract, sol_rpc, validate_boolean_expression};
use crate::auth::auth_material::{AuthSigItem, MultipleAuthSigs};
use crate::auth::resources::LitResourceAbility;
use crate::error::{validation_err, validation_err_code, Result, EC};
use crate::models::{
    AccessControlBooleanOperator, UnifiedAccessControlCondition, UnifiedAccessControlConditionItem,
    UnifiedConditionCheckResult,
};
use crate::utils::web::EndpointVersion;
use lit_core::config::LitConfig;
use lit_core::error::Unexpected;
use moka::future::Cache;
use std::borrow::BorrowMut;
use std::sync::Arc;

#[allow(clippy::too_many_arguments)]
pub(crate) async fn check_access_control_conditions(
    conditions: &Vec<UnifiedAccessControlConditionItem>,
    auth_sig_item: &AuthSigItem,
    chain: Option<String>,
    requested_lit_resource_ability: &LitResourceAbility,
    cfg: Arc<LitConfig>,
    request_id: &String,
    bls_root_pubkey: &String,
    endpoint_version: &EndpointVersion,
    current_action_ipfs_id: Option<&String>,
    ipfs_cache: Cache<String, Arc<String>>,
) -> Result<UnifiedConditionCheckResult> {
    // massage auth_sig into the MultipleAuthSigs struct before doing the check
    let auth_sigs = {
        match auth_sig_item {
            AuthSigItem::Multiple(multiple_auth_sigs) => {
                // Validate and extract all the provided auth sigs.
                let mut mult_auth_sigs = multiple_auth_sigs.to_owned();
                mult_auth_sigs
                    .borrow_mut()
                    .validate_all_and_extract_wallet_sigs(
                        requested_lit_resource_ability,
                        &cfg,
                        bls_root_pubkey,
                        endpoint_version,
                    )
                    .await?;
                mult_auth_sigs.to_owned()
            }
            AuthSigItem::Single(single_auth_sig) => {
                let valid_auth_sig = single_auth_sig
                    .validate_and_get_wallet_sig(
                        requested_lit_resource_ability,
                        &chain,
                        &cfg,
                        bls_root_pubkey,
                        endpoint_version,
                    )
                    .await?;
                MultipleAuthSigs::populate_by_chain(&chain, single_auth_sig)
            }
        }
    };

    let res = check_condition_group(
        conditions,
        &auth_sigs,
        cfg,
        request_id,
        bls_root_pubkey,
        endpoint_version,
        current_action_ipfs_id,
        ipfs_cache,
    )
    .await?;
    Ok(res)
}

#[allow(clippy::too_many_arguments)]
async fn check_condition_group(
    conditions: &Vec<UnifiedAccessControlConditionItem>,
    auth_sigs: &MultipleAuthSigs,
    cfg: Arc<LitConfig>,
    request_id: &String,
    bls_root_pubkey: &String,
    endpoint_version: &EndpointVersion,
    current_action_ipfs_id: Option<&String>,
    ipfs_cache: Cache<String, Arc<String>>,
) -> Result<UnifiedConditionCheckResult> {
    if !validate_boolean_expression(conditions) {
        return Err(validation_err_code(
            "Invalid boolean Unified Access Control Conditions",
            EC::NodeInvalidBooleanConditionType,
            None,
        ));
    }

    let mut results: Vec<UnifiedConditionCheckResult> = Vec::new();
    let mut operators: Vec<AccessControlBooleanOperator> = Vec::new();
    for condition_item in conditions {
        match condition_item {
            UnifiedAccessControlConditionItem::Condition(condition) => {
                results.push(
                    check_condition(
                        condition,
                        auth_sigs,
                        cfg.clone(),
                        request_id,
                        bls_root_pubkey,
                        endpoint_version,
                        current_action_ipfs_id,
                        ipfs_cache.clone(),
                    )
                    .await?,
                );
            }
            UnifiedAccessControlConditionItem::Operator(operator) => {
                operators.push(operator.operator.clone());
            }
            UnifiedAccessControlConditionItem::Group(group) => {
                results.push(
                    Box::pin(check_condition_group(
                        group,
                        auth_sigs,
                        cfg.clone(),
                        request_id,
                        bls_root_pubkey,
                        endpoint_version,
                        current_action_ipfs_id,
                        ipfs_cache.clone(),
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
            // just pick one randomly, since both wallet sigs were successful
            AccessControlBooleanOperator::And if res1.result && res2.result => results.push(res1),
            AccessControlBooleanOperator::Or if res1.result => results.push(res1),
            AccessControlBooleanOperator::Or if res2.result => results.push(res2),
            _ => results.push(UnifiedConditionCheckResult {
                result: false,
                successful_auth_sig: res1.successful_auth_sig, // irrelevant since success is false
            }),
        }
    }

    Ok(results
        .last()
        .expect_or_err("Could not get last result from results vector")?
        .to_owned())
}

#[allow(clippy::too_many_arguments)]
async fn check_condition(
    condition: &UnifiedAccessControlCondition,
    auth_sigs: &MultipleAuthSigs,
    cfg: Arc<LitConfig>,
    request_id: &str,
    bls_root_pubkey: &String,
    endpoint_version: &EndpointVersion,
    current_action_ipfs_id: Option<&String>,
    ipfs_cache: Cache<String, Arc<String>>,
) -> Result<UnifiedConditionCheckResult> {
    // do the check depending on the condition type
    let auth_sig;
    let condition_check = match condition {
        UnifiedAccessControlCondition::JsonAccessControlCondition(access_control_condition) => {
            auth_sig = auth_sigs.ethereum.clone().ok_or_else(|| {
                validation_err(
                    "ETH auth sig is missing while checking a JsonAccessControlCondition",
                    None,
                )
            })?;

            super::check_condition(
                access_control_condition,
                &auth_sig,
                cfg,
                request_id,
                bls_root_pubkey,
                endpoint_version,
                current_action_ipfs_id,
                ipfs_cache,
            )
            .await?
        }
        UnifiedAccessControlCondition::EVMContractCondition(evm_contract_condition) => {
            auth_sig = auth_sigs.ethereum.clone().ok_or_else(|| {
                validation_err(
                    "ETH auth sig is missing while checking a JsonAccessControlCondition",
                    None,
                )
            })?;

            evm_contract::check_condition(
                evm_contract_condition,
                &auth_sig,
                bls_root_pubkey,
                current_action_ipfs_id,
            )
            .await?
        }
        UnifiedAccessControlCondition::SolRpcCondition(sol_rpc_condition_version) => {
            auth_sig = auth_sigs.solana.clone().ok_or_else(|| {
                validation_err(
                    "SOL auth sig is missing while checking a SolRpcCondition",
                    None,
                )
            })?;

            sol_rpc::check_condition(
                sol_rpc_condition_version,
                &auth_sig,
                bls_root_pubkey,
                current_action_ipfs_id,
            )
            .await?
        }
        UnifiedAccessControlCondition::CosmosCondition(cosmos_condition) => {
            auth_sig = cosmos::get_auth_sig_for_chain_string(auth_sigs, &cosmos_condition.chain)?;

            cosmos::check_condition(
                cosmos_condition,
                auth_sigs,
                bls_root_pubkey,
                current_action_ipfs_id,
            )
            .await?
        }
    };

    Ok(UnifiedConditionCheckResult {
        result: condition_check,
        successful_auth_sig: auth_sig.clone(),
    })
}
