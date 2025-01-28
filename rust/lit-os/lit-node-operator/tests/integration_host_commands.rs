use ethers::types::Address;
use lit_api_core::logging::{FileLoggingPlugin, TracingPlugin};
use lit_blockchain::contracts::host_commands::HostCommands;
use lit_blockchain::resolver::contract::ContractResolver;
use lit_cli_os::config::LitCliOsConfig;
use lit_cli_os::guest::instance::GuestInstanceItem;
use lit_core::config::{LitConfig, LitConfigBuilder, ENV_LIT_CONFIG_FILE};
use lit_core::utils::binary::bytes_to_hex;
use lit_logging::config::LitLoggingConfig;
use lit_logging::plugin::log_service::LogServicePlugin;
use lit_node::config::LitNodeConfig;
use lit_os_core::guest::env::build::GuestBuildEnv;
use lit_os_core::guest::env::instance::GuestInstanceEnv;
use std::io::BufRead;
use std::time::Duration;
use std::{env, path::PathBuf, sync::Arc};
use test_common::testnet::Testnet;
use tokio::time::sleep;

// Import the main module to test
use lit_node_operator::host_commands_listener::HostCommandsListener;

#[tokio::test]
async fn test_host_commands_listener() -> Result<(), Box<dyn std::error::Error>> {
    // deploy the contracts
    let num_nodes = 3;
    let mut testnet = Testnet::builder().num_staked_and_joined_validators(num_nodes).build().await;

    let testnet_contracts =
        Testnet::setup_contracts(&mut testnet, None).await.expect("Failed to setup contracts");
    let subnet_id = bytes_to_hex(&testnet_contracts.contract_addresses().staking);

    // get current path
    let current_path = env::current_dir().expect("Failed to get current path");
    // we are likely in lit-assets/rust/lit-os/<something> and we want to be in lit-assets/rust/lit-node/config
    // find the config directory by moving up the directory tree until we find lit-node
    let mut rust_dir = current_path.clone();
    while !rust_dir.join("lit-node").join("config").exists() {
        rust_dir = rust_dir.parent().expect("Failed to get parent directory").to_path_buf();
    }
    let lit_node_dir = rust_dir.join("lit-node");
    // println!("lit_node_dir: {:?}", lit_node_dir);
    env::set_current_dir(&lit_node_dir)?;

    // set to ../../blockchain/contracts
    let lit_blockchain_dir =
        lit_node_dir.parent().unwrap().parent().unwrap().join("blockchain").join("contracts");

    // set the lit config file env var path.  this will be picked up by the lit-logging config system and loaded
    env::set_var(
        ENV_LIT_CONFIG_FILE,
        lit_blockchain_dir.join("node_configs").join("lit_config0.toml").to_str().unwrap(),
    );

    // make sure we output the logs we are checking for
    env::set_var("RUST_LOG", "lit_node_operator=trace");

    // copy the config file to the cloud-init/config.toml path
    let cloud_init_dir = lit_blockchain_dir.join("node_configs").join("cloud-init");
    if !cloud_init_dir.exists() {
        std::fs::create_dir_all(&cloud_init_dir)?;
    }
    let cloud_init_path = cloud_init_dir.join("config.toml");
    // copy the config file to the cloud-init/config.toml path
    std::fs::copy(
        lit_blockchain_dir.join("node_configs").join("lit_config0.toml"),
        cloud_init_path,
    )?;
    // println!("ENV_LIT_CONFIG_FILE: {:?}", env::var(ENV_LIT_CONFIG_FILE));

    // Load config
    let cfg = <LitConfig as LitNodeConfig>::try_new()?;

    let logging_rt = tokio::runtime::Runtime::new().expect("failed to create Logging Runtime");
    let cloned_cfg = cfg.clone();
    logging_rt.spawn_blocking(move || {
        lit_logging::builder(&cloned_cfg, "lit-node-operator")
            .plugin(FileLoggingPlugin::new())
            .plugin(TracingPlugin::new())
            .plugin(LogServicePlugin::new())
            .init()
            .expect("failed to init logging");
    });

    // Get the instance item
    let mut instance_item = GuestInstanceItem {
        instance_env: GuestInstanceEnv::default(),
        build_env: GuestBuildEnv::default(),
        release_env: None,
        path: lit_blockchain_dir.join("node_configs"),
    };
    instance_item.instance_env.subnet_id = Some(subnet_id);
    instance_item.instance_env.instance_id = Some("test".to_string());
    instance_item.build_env.build_release = Some("dev".to_string());

    // Set up the contract resolver
    let subnet =
        instance_item.instance_env.subnet_id.clone().expect("node-type nodes must have a subnet");
    let env = instance_item.build_env.env().expect("node-type nodes must have an env");

    let resolver = ContractResolver::new(subnet.clone(), env, None);
    let host_commands_contract = resolver.host_commands_contract(&cfg).await?;

    // Create the listener
    let listener = HostCommandsListener::new(host_commands_contract.clone(), instance_item)?;

    // Spawn the listener in a separate task
    let listener_handle = tokio::spawn(async move {
        listener.listen_for_events().await.expect("Listener failed");
    });

    // Give the listener some time to start up
    sleep(Duration::from_secs(2)).await;

    println!(
        "Calling contract at address {:?} to emit a restart event",
        host_commands_contract.address()
    );

    // call the contract to emit a restart event
    let deployer_signing_provider = testnet.deploy_account.signing_provider.clone();

    let host_commands_contract_with_signer =
        HostCommands::new(host_commands_contract.address(), Arc::new(deployer_signing_provider));

    // get epoch time 5 mins from now
    let five_mins_from_now =
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs()
            as i64
            + 300; // 5 minutes = 300 seconds
    host_commands_contract_with_signer
        .restart(
            format!("0x{}", subnet).parse::<Address>().unwrap(),
            five_mins_from_now.into(),
            false,
        )
        .send()
        .await?;

    println!("Restart event emitted");

    // Wait some time for events to be processed
    sleep(Duration::from_secs(5)).await;

    // check the log file for the restart event log
    let log_file_path =
        lit_node_dir.join("test_logs").join(cfg.staker_address().unwrap().to_ascii_lowercase());
    // println!("Log file path: {:?}", log_file_path);
    let log_file = std::fs::File::open(log_file_path).expect("Failed to open log file");
    let log_reader = std::io::BufReader::new(log_file);
    let log_lines =
        log_reader.lines().collect::<Result<Vec<_>, _>>().expect("Failed to read log file");
    // println!("Log lines: {:?}", log_lines);

    // looking for these logs:
    // New event in event listener: RestartFilter(RestartFilter { stake_address: 0xf5059a5d33d5853360d16c683c16e67980206f36, expiration_time: 1733197158, force: false })
    // Restart event

    assert!(log_lines
        .iter()
        .any(|line| line.contains("New event in event listener: RestartFilter")));
    assert!(log_lines.contains(&"Restart event".to_string()));

    // Clean up
    listener_handle.abort();
    logging_rt.shutdown_background();

    Ok(())
}
