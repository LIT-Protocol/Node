use test_common::{
    assertions::NetworkIntegrityChecker,
    init_test_config,
    testnet::{contracts::StakingContractConfig, NodeAccount, Testnet, WhichTestnet},
    validator::ValidatorCollection,
};

use ethers::types::U256;
use lit_node::config::CFG_KEY_ECDSA_ROUND_TIMEOUT_MS_DEFAULT;
use network_state::{get_next_random_network_state, NetworkState};
use tracing::info;

fn setup() {
    init_test_config();
}

#[tokio::test]
async fn test_many_epochs() {
    setup();

    info!("TEST: test_many_epochs");

    const NUM_EPOCHS: usize = 15;
    const MAX_VALIDATORS: usize = 12;
    const MIN_VALIDATORS: usize = 5;
    const INITIAL_VALIDATORS: usize = 6;
    const EPOCH_LENGTH: u64 = 3000;

    // Generate test plan
    let mut test_plan: Vec<NetworkState> = Vec::new();
    // Seed the first network state
    test_plan.push(NetworkState::new(2, INITIAL_VALIDATORS, 0, 0));
    for idx in 1..NUM_EPOCHS {
        let prev_validator_count = test_plan[idx - 1].get_validator_count();
        let network_state = get_next_random_network_state(
            MIN_VALIDATORS,
            MAX_VALIDATORS,
            prev_validator_count,
            test_plan[idx - 1].get_epoch_number() + 1,
        );
        test_plan.push(network_state);
    }

    // Print the test plan
    info!("Test plan:");
    for network_state in &test_plan {
        info!("{}", network_state);
    }

    // Setup the network
    let mut testnet = Testnet::builder()
        .which_testnet(WhichTestnet::Anvil)
        .num_staked_and_joined_validators(INITIAL_VALIDATORS)
        .num_staked_only_validators((MAX_VALIDATORS * 2) - INITIAL_VALIDATORS)
        .build()
        .await;

    let testnet_contracts = Testnet::setup_contracts(
        &mut testnet,
        Some(
            StakingContractConfig::builder()
                .epoch_length(U256::from(EPOCH_LENGTH))
                .max_triple_count(U256::from(0))
                .min_triple_count(U256::from(0))
                .build(),
        ),
    )
    .await
    .expect("Failed to setup contracts");

    let actions = testnet.actions(testnet_contracts.contracts());

    let mut validator_collection = ValidatorCollection::builder()
        .num_staked_nodes(MAX_VALIDATORS * 2) // this is doubled since the entire set can request to leave and a new one requests to join
        // explicitly indicate that the indices between INITIAL_VALIDATORS and (MAX_VALIDATORS * 2) are asleep as a vec
        .asleep_initially_override(Some((INITIAL_VALIDATORS..(MAX_VALIDATORS * 2)).collect()))
        .build(&testnet, &actions)
        .await
        .expect("Failed to build validator collection");

    let network_checker = NetworkIntegrityChecker::new(&actions).await;

    info!("Network is set up, time to start the test plan");
    for i in 1..test_plan.len() {
        let next_network_state = &test_plan[i];
        info!(
            "Transitioning from [{}] to [{}]",
            test_plan[i - 1],
            next_network_state
        );

        // Request to leave
        info!("Validators are requesting to leave");
        let node_accounts_requested_to_leave: Vec<NodeAccount> = {
            let validators = validator_collection
                .random_validators_request_to_leave(next_network_state.get_validators_dealt_out())
                .await
                .expect("Failed to request to leave");
            validators.iter().map(|v| v.account().clone()).collect()
        };

        // Request to join
        info!("Validators are requesting to join");
        validator_collection
            .random_validators_request_to_join(next_network_state.get_validators_dealt_in())
            .await
            .expect("Failed to request to join");

        // Fast forward time so nodes can lock, DKG and advance
        info!("Fast forwarding time");
        actions
            .increase_blockchain_timestamp(EPOCH_LENGTH as usize)
            .await;

        // Wait until network has advanced to the next epoch
        info!(
            "Waiting for epoch to advance to {}",
            next_network_state.get_epoch_number()
        );
        actions
            .wait_for_epoch(U256::from(next_network_state.get_epoch_number()))
            .await;

        // Assert the new network state is as expected
        info!("Checking network state");
        assert_eq!(
            actions.get_current_validator_count().await as usize,
            next_network_state.get_validator_count()
        );
        network_checker.check(&validator_collection).await;

        // Wait until the round timeout has passed so that ongoing TSS operations can finish before shutting down nodes.
        info!("Waiting for some time before shutting down nodes");
        tokio::time::sleep(tokio::time::Duration::from_millis(
            CFG_KEY_ECDSA_ROUND_TIMEOUT_MS_DEFAULT as u64,
        ))
        .await;

        // Shutdown the nodes that have requested to leave
        info!("Shutting down nodes that have requested to leave");
        validator_collection
            .stop_nodes_with_accounts(node_accounts_requested_to_leave)
            .expect("Failed to stop nodes");
    }
}

mod network_state {
    use lit_node::utils::consensus::get_threshold_count;
    use rand::Rng;
    use std::fmt;

    #[derive(Debug, Clone)]
    pub struct NetworkState {
        epoch_number: usize,
        validator_count: usize,
        validators_dealt_in: usize,
        validators_dealt_out: usize,
    }

    impl NetworkState {
        pub fn new(
            epoch_number: usize,
            validator_count: usize,
            validators_dealt_in: usize,
            validators_dealt_out: usize,
        ) -> Self {
            Self {
                epoch_number,
                validator_count,
                validators_dealt_in,
                validators_dealt_out,
            }
        }

        pub fn get_epoch_number(&self) -> usize {
            self.epoch_number
        }

        pub fn get_validator_count(&self) -> usize {
            self.validator_count
        }

        pub fn get_validators_dealt_in(&self) -> usize {
            self.validators_dealt_in
        }

        pub fn get_validators_dealt_out(&self) -> usize {
            self.validators_dealt_out
        }

        pub fn get_threshold(&self) -> usize {
            get_threshold_count(self.validator_count)
        }
    }

    impl fmt::Display for NetworkState {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "Epoch: {}, Validators: {} (+{}/-{}), Threshold: {}",
                self.epoch_number,
                self.validator_count,
                self.validators_dealt_in,
                self.validators_dealt_out,
                self.get_threshold()
            )
        }
    }

    /// Given the previous validator count, randomize the next valid network state. The priority is to
    /// find (interesting) dealt_in / dealt_out combinations before checking whether it is a valid change.
    pub fn get_next_random_network_state(
        minimum_validators: usize,
        maximum_validators: usize,
        prev_validator_count: usize,
        epoch_number: usize,
    ) -> NetworkState {
        let rng = &mut test_common::rand::thread_rng();

        let mut valid_dealt_in_and_out;
        loop {
            // We want to make sure that we're not dealing out such that < threshold of current validators remain in the new epoch!
            let validators_dealt_out = rng
                .gen_range(0..=(prev_validator_count - get_threshold_count(prev_validator_count)));
            let validators_dealt_in = rng
                .gen_range(0..(maximum_validators - prev_validator_count + validators_dealt_out));

            // The change is valid if the new validator count is within the bounds, inclusive.
            let new_validator_count =
                prev_validator_count + validators_dealt_in - validators_dealt_out;
            valid_dealt_in_and_out = new_validator_count >= minimum_validators
                && new_validator_count <= maximum_validators;
            if valid_dealt_in_and_out {
                return NetworkState::new(
                    epoch_number,
                    new_validator_count,
                    validators_dealt_in,
                    validators_dealt_out,
                );
            }
        }
    }
}
