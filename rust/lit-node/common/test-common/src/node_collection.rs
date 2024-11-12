use super::testnet::actions::Actions;

use anyhow::Result;
use futures::future::join_all;
use lit_node::models::JsonSDKHandshakeResponse;
use reqwest::Response;
use std::collections::HashSet;
use tracing::info;

use tracing::error;

use reqwest;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum NodeStakingStatus {
    PreviouslyStaked,
    StakedAndJoined,
    FailedToStake,
    Unstaked,
}

// Config that will be used for launching node.
// May either be passed as environment variables, or loaded into a config file.
#[derive(Debug)]
pub struct NodeConfig {
    pub lit_domain_name: String,
    pub rocket_port: String,
    pub staker_address: String,
    pub enable_proxied_http_client: Option<bool>,
}

pub async fn hit_endpoints_with_json_body(
    actions: &Actions,
    cmd: String,
    json_body: String,
) -> Vec<String> {
    let portnames = get_current_validator_portnames(actions).await;

    let mut responses: Vec<String> = Vec::new();
    let request_id = &uuid::Uuid::new_v4().to_string();
    for port in portnames {
        let client = reqwest::Client::new();
        let resp_string = client
            .post(format!("http://127.0.0.1:{}/{}", port, cmd))
            .header("Content-Type", "application/json")
            .header("X-Request-Id", request_id)
            .body(json_body.clone())
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        responses.push(resp_string)
    }
    responses
}

pub async fn hit_endpoints_with_json_body_join_all(
    actions: &Actions,
    cmd: String,
    json_body: String,
) -> Result<Vec<String>> {
    let portnames = get_current_validator_portnames(actions).await;

    let mut v = Vec::new();
    let request_id = &uuid::Uuid::new_v4().to_string();
    for port in portnames {
        let client = reqwest::Client::new();
        let resp = client
            .post(format!("http://127.0.0.1:{}/{}", port, cmd))
            .header("Content-Type", "application/json")
            .header("X-Request-Id", request_id)
            .body(json_body.clone())
            .send();
        v.push(resp);
    }

    info!("Starting sign/execute for {}", json_body.clone());
    let results = join_all(v).await;
    info!("Finished sign/execute for {}", json_body.clone());

    let mut responses: Vec<String> = Vec::new();
    for result in results {
        match result {
            Ok(resp) => {
                responses.push(resp.text().await.unwrap());
            }
            Err(e) => {
                error!("Error hitting an endpoint: {:?}", e);
                // return Err(anyhow::anyhow!(
                //     "Error in hit_endpoints_with_json_body_join_all: {:?}",
                //     e
                // ));
            }
        }
    }

    info!("responses: {:?}", responses);

    Ok(responses)
}

pub async fn hit_ports_with_json_body_join_all(
    portnames: Vec<String>,
    cmd: String,
    json_body: String,
) -> Result<Vec<String>> {
    let mut v = Vec::new();
    let request_id = &uuid::Uuid::new_v4().to_string();

    for port in portnames {
        info!(
            "Sending to : {} \n\n{}",
            format!("http://127.0.0.1:{}/{}", port, cmd),
            json_body.clone()
        );
        let client = reqwest::Client::new();
        let resp = client
            .post(format!("http://127.0.0.1:{}/{}", port, cmd))
            .header("Content-Type", "application/json")
            .header("X-Request-Id", request_id)
            .body(json_body.clone())
            .send();
        v.push(resp);
    }

    info!("Starting sign for {}", json_body.clone());
    let results = join_all(v).await;
    info!("Finished sign for {}", json_body.clone());

    let mut responses: Vec<String> = Vec::new();
    for result in results {
        match result {
            Ok(resp) => {
                responses.push(resp.text().await.unwrap());
            }
            Err(e) => {
                error!("Error hitting an endpoint: {:?}", e);
                // return Err(anyhow::anyhow!(
                //     "Error in hit_endpoints_with_json_body_join_all: {:?}",
                //     e
                // ));
            }
        }
    }

    info!("responses: {:?}", responses);

    Ok(responses)
}

pub async fn get_network_pubkey(actions: &Actions) -> String {
    let results = hit_endpoints_with_json_body(
        actions,
        "/web/handshake".to_string(),
        r#"{"clientPublicKey":"blah"}"#.to_string(),
    )
    .await;

    // Parse response
    let responses = results
        .iter()
        .map(|result| serde_json::from_str(result).unwrap())
        .collect::<Vec<JsonSDKHandshakeResponse>>();

    responses[0].network_public_key.clone()
}

pub async fn handshake_returns_keys(actions: &Actions) -> bool {
    let results = hit_endpoints_with_json_body(
        actions,
        "/web/handshake".to_string(),
        r#"{"clientPublicKey":"blah"}"#.to_string(),
    )
    .await;

    // Parse response
    let responses = results
        .iter()
        .map(|result| serde_json::from_str(result).expect("Unable to parse response"))
        .collect::<Vec<JsonSDKHandshakeResponse>>();

    for response in &responses {
        // Ensure no errors
        if (response.subnet_public_key.contains("ERR"))
            || (response.network_public_key.contains("ERR"))
            || (response.network_public_key_set.contains("ERR"))
        {
            info!("Handshake response contains error: {:?}", response);
            return false;
        }

        // Ensure the network public key is the correct length
        if (response.subnet_public_key.len() != 96)
            || (response.network_public_key.len() != 96)
            || (response.network_public_key_set.len() != 96)
        {
            info!(
                "Handshake response contains incorrect length public key: {:?}",
                response
            );
            return false;
        }
    }

    info!("Handshake response contains correct keys");
    true
}

pub async fn get_node_versions(actions: &Actions) -> Vec<String> {
    let results = hit_endpoints_with_json_body_raw(
        actions,
        "/web/handshake".to_string(),
        r#"{"clientPublicKey":"blah"}"#.to_string(),
    )
    .await;

    // Parse response headers
    results
        .iter()
        .map(|response| {
            response
                .headers()
                .get("X-Lit-Node-Version")
                .unwrap()
                .to_str()
                .unwrap()
                .to_string()
        })
        .collect::<Vec<String>>()
}

pub async fn hit_endpoints_with_json_body_raw(
    actions: &Actions,
    cmd: String,
    json_body: String,
) -> Vec<Response> {
    let portnames = get_current_validator_portnames(actions).await;
    let mut responses: Vec<Response> = Vec::new();
    let request_id = &uuid::Uuid::new_v4().to_string();
    for port in portnames {
        let client = reqwest::Client::new();
        let resp = client
            .post(format!("http://127.0.0.1:{}/{}", port, cmd))
            .header("Content-Type", "application/json")
            .header("X-Request-Id", request_id)
            .body(json_body.clone())
            .send()
            .await
            .unwrap();
        responses.push(resp)
    }
    responses
}

/// This function is used to hit endpoints with different json bodies per port.
pub async fn hit_endpoints_with_json_body_per_port(
    actions: &Actions,
    cmd: String,
    json_body_vec: Vec<String>,
) -> Vec<String> {
    let portnames = get_current_validator_portnames(actions).await;

    // If the number of json bodies is not equal to the number of ports, then panic.
    assert_eq!(json_body_vec.len(), portnames.len());

    let request_id = uuid::Uuid::new_v4().to_string();
    let client = reqwest::Client::new();
    let futures = portnames.iter().enumerate().map(|(idx, port)| {
        let json_body = json_body_vec.get(idx).unwrap().clone();
        let request_id = request_id.clone();
        let cmd = cmd.clone();
        let client_clone = client.clone();
        async move {
            client_clone
                .post(format!("http://127.0.0.1:{}/{}", port, cmd))
                .header("Content-Type", "application/json")
                .header("X-Request-Id", &request_id)
                .body(json_body)
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap()
        }
    });
    let responses: Vec<String> = futures::future::join_all(futures).await;
    responses
}

pub fn choose_random_indices(array_size: usize, num_random_indices: usize) -> HashSet<usize> {
    let mut indices = HashSet::new();
    for _ in 0..num_random_indices {
        let mut idx = rand::random::<usize>() % array_size;
        while indices.contains(&idx) {
            idx = rand::random::<usize>() % array_size;
        }
        indices.insert(idx);
    }
    indices
}

pub async fn get_current_validator_portnames(actions: &Actions) -> Vec<String> {
    // Fetch the portnames from the chain state
    let validators = actions.get_current_validator_structs().await;
    validators
        .iter()
        .map(|validator| validator.port.to_string().clone())
        .collect::<Vec<String>>()
}
