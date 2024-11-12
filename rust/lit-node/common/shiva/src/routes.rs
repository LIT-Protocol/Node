use rand::{distributions::Alphanumeric, Rng};
use rocket::response::status;
use rocket::serde::json::{serde_json::json, Json, Value};
use rocket::{get, http::Status, post, State};

use crate::client::ShivaClient;

use super::models::{TestNetCreateRequest, TestNetInfo, TestNetMessage, TestNetResponse};

#[post("/test/create/testnet", format = "json", data = "<create_request>")]
pub(crate) async fn create_testnet(
    _quit_tx: &State<tokio::sync::broadcast::Sender<bool>>,
    tnm_tx: &State<flume::Sender<TestNetMessage>>,
    client: &State<ShivaClient>,
    create_request: Json<TestNetCreateRequest>,
) -> status::Custom<Value> {
    let testnet_ids = client.get_testnet_ids(tnm_tx.inner().clone()).await;
    let testnet_ids = testnet_ids.unwrap();
    if testnet_ids.len() > 1 {
        return status::Custom(
            Status::BadRequest,
            json!(TestNetResponse::<()> {
                testnet_id: "".to_string(),
                command: "CREATE_TESTNET".to_string(),
                was_canceled: false,
                body: Some(()),
                last_state_observed: None,
                messages: None,
                errors: Some(vec![
                    "Currently only a single testnet may be managed at a time".to_string()
                ]),
            }),
        );
    }

    let session_id: String = rand::thread_rng()
        .sample_iter(Alphanumeric)
        .take(36)
        .map(char::from)
        .collect();

    if create_request.custom_build_path.is_some()
        && create_request.lit_action_server_custom_build_path.is_none()
    {
        return status::Custom(
            Status::BadRequest,
            json!(TestNetResponse::<()> {
                testnet_id: session_id,
                command: "CREATE_TESTNET".to_string(),
                was_canceled: false,
                body: Some(()),
                last_state_observed: None,
                messages: None,
                errors: Some(vec![
                    "Must provide lit action and lit node binaries for running custom builds"
                        .to_string()
                ]),
            }),
        );
    } else if create_request.custom_build_path.is_none()
        && create_request.lit_action_server_custom_build_path.is_some()
    {
        return status::Custom(
            Status::BadRequest,
            json!(TestNetResponse::<()> {
                testnet_id: session_id,
                command: "CREATE_TESTNET".to_string(),
                was_canceled: false,
                body: Some(()),
                last_state_observed: None,
                messages: None,
                errors: Some(vec![
                    "Must provide lit action and lit node binaries for running custom builds"
                        .to_string()
                ]),
            }),
        );
    }
    let _res = client
        .create_testnets(
            tnm_tx.inner().clone(),
            session_id.to_string(),
            create_request.0,
        )
        .await;
    status::Custom(
        Status::Ok,
        json!(TestNetResponse::<()> {
            testnet_id: session_id,
            command: "CREATE_TESTNET".to_string(),
            was_canceled: false,
            body: Some(()),
            last_state_observed: Some("BUSY".to_string()),
            messages: None,
            errors: None,
        }),
    )
}

#[get("/test/delete/testnet/<id>")]
pub(crate) async fn delete_testnet(
    _quit_tx: &State<tokio::sync::broadcast::Sender<bool>>,
    client: &State<ShivaClient>,
    tnm_tx: &State<flume::Sender<TestNetMessage>>,
    id: &str,
) -> status::Custom<Value> {
    let del_status = client
        .delete_testnet(tnm_tx.inner().clone(), id.to_string())
        .await;

    match del_status {
        Ok(status) => {
            return status::Custom(
                Status::Ok,
                json!(TestNetResponse::<bool> {
                    testnet_id: id.to_string(),
                    command: "SHUTDOWN".to_string(),
                    was_canceled: false,
                    body: Some(status),
                    last_state_observed: Some("TERM".to_string()),
                    messages: None,
                    errors: None,
                }),
            )
        }
        Err(e) => {
            return status::Custom(
                Status::InternalServerError,
                json!(TestNetResponse::<()> {
                    testnet_id: id.to_string(),
                    command: "SHUTDOWN".to_string(),
                    was_canceled: false,
                    body: Some(()),
                    last_state_observed: Some("BUSY".to_string()),
                    messages: None,
                    errors: Some(vec![e.to_string()]),
                }),
            );
        }
    }
}

#[get("/test/poll/testnet/<id>")]
pub(crate) async fn poll_testnet(
    _quit_tx: &State<tokio::sync::broadcast::Sender<bool>>,
    client: &State<ShivaClient>,
    tnm_tx: &State<flume::Sender<TestNetMessage>>,
    id: &str,
) -> status::Custom<Value> {
    let poll_status = client
        .poll_testnet_status(tnm_tx.inner().clone(), id.to_string())
        .await;

    match poll_status {
        Ok(status) => {
            return status::Custom(
                Status::Ok,
                json!(TestNetResponse::<String> {
                    testnet_id: id.to_string(),
                    command: "POKE".to_string(),
                    was_canceled: false,
                    body: Some(format!("{:?}", status)),
                    last_state_observed: Some(format!("{:?}", status)),
                    messages: None,
                    errors: None,
                }),
            )
        }
        Err(e) => {
            return status::Custom(
                Status::InternalServerError,
                json!(TestNetResponse::<String> {
                    testnet_id: id.to_string(),
                    command: "POKE".to_string(),
                    was_canceled: false,
                    body: Some("UNKNOWN".to_string()),
                    last_state_observed: Some("UNKNOWN".to_string()),
                    messages: None,
                    errors: Some(vec![e.to_string()]),
                }),
            );
        }
    }
}

#[get("/test/get/info/testnet/<id>")]
pub(crate) async fn get_info_testnet(
    _quit_tx: &State<tokio::sync::broadcast::Sender<bool>>,
    client: &State<ShivaClient>,
    tnm_tx: &State<flume::Sender<TestNetMessage>>,
    id: &str,
) -> status::Custom<Value> {
    let info_status = client
        .get_tn_status(id.to_string(), tnm_tx.inner().clone())
        .await;
    match info_status {
        Ok(info) => {
            if let Some(testnet_info) = info {
                let status = client
                    .get_tn_status(id.to_string(), tnm_tx.inner().clone())
                    .await;
                return status::Custom(
                    Status::Ok,
                    json!(TestNetResponse::<TestNetInfo> {
                        testnet_id: id.to_string(),
                        command: "GET_INFO".to_string(),
                        was_canceled: false,
                        body: Some(testnet_info),
                        last_state_observed: Some(format!("{:?}", status.unwrap())),
                        messages: None,
                        errors: None,
                    }),
                );
            } else {
                return status::Custom(
                    Status::InternalServerError,
                    json!(TestNetResponse::<TestNetInfo> {
                        testnet_id: id.to_string(),
                        command: "GET_INFO".to_string(),
                        was_canceled: false,
                        body: None,
                        last_state_observed: Some("UNKNOWN".to_string()),
                        messages: None,
                        errors: None,
                    }),
                );
            }
        }
        Err(e) => {
            return status::Custom(
                Status::InternalServerError,
                json!(TestNetResponse::<()> {
                    testnet_id: id.to_string(),
                    command: "GET_INFO".to_string(),
                    was_canceled: false,
                    body: Some(()),
                    last_state_observed: Some(format!("{:?}", "UNKNOWN")),
                    messages: None,
                    errors: Some(vec![e.to_string()]),
                }),
            );
        }
    }
}

#[get("/test/get/testnets")]
pub(crate) async fn get_testnets(
    _quit_tx: &State<tokio::sync::broadcast::Sender<bool>>,
    client: &State<ShivaClient>,
    tnm_tx: &State<flume::Sender<TestNetMessage>>,
) -> status::Custom<Value> {
    let testnets = client.get_testnet_ids(tnm_tx.inner().clone()).await;
    match testnets {
        Ok(ids) => return status::Custom(Status::Ok, json!(ids.clone())),
        Err(e) => {
            return status::Custom(
                Status::InternalServerError,
                json!(TestNetResponse::<()> {
                    testnet_id: "".to_string(),
                    command: "GET_TESTNETS".to_string(),
                    was_canceled: false,
                    body: Some(()),
                    last_state_observed: Some(format!("{:?}", "UNKNOWN")),
                    messages: None,
                    errors: Some(vec![e.to_string()]),
                }),
            );
        }
    }
}

#[get("/test/action/stop/random/<id>")]
pub(crate) async fn stop_random_node_testnet(
    _quit_tx: &State<tokio::sync::broadcast::Sender<bool>>,
    client: &State<ShivaClient>,
    tnm_tx: &State<flume::Sender<TestNetMessage>>,
    id: &str,
) -> status::Custom<Value> {
    let stop_status = client
        .stop_random_node(id.to_string(), tnm_tx.inner().clone())
        .await;
    match stop_status {
        Ok(status) => {
            let current_state = client
                .get_tn_status(id.to_string(), tnm_tx.inner().clone())
                .await;
            return status::Custom(
                Status::Ok,
                json!(TestNetResponse::<bool> {
                    testnet_id: id.to_string(),
                    command: "STOP_RANDOM".to_string(),
                    was_canceled: false,
                    body: Some(status.unwrap()),
                    last_state_observed: Some(format!("{:?}", current_state.unwrap())),
                    messages: None,
                    errors: None,
                }),
            );
        }
        Err(e) => {
            return status::Custom(
                Status::InternalServerError,
                json!(TestNetResponse::<()> {
                    testnet_id: "".to_string(),
                    command: "STOP_RANDOM".to_string(),
                    was_canceled: false,
                    body: Some(()),
                    last_state_observed: Some(format!("{:?}", "UNKNOWN")),
                    messages: None,
                    errors: Some(vec![e.to_string()]),
                }),
            );
        }
    }
}

#[get("/test/action/stop/random/wait/<id>")]
pub(crate) async fn stop_random_node_and_wait_testnet(
    _quit_tx: &State<tokio::sync::broadcast::Sender<bool>>,
    client: &State<ShivaClient>,
    tnm_tx: &State<flume::Sender<TestNetMessage>>,
    id: &str,
) -> status::Custom<Value> {
    let wait_status = client
        .stop_random_node_wait(id.to_string(), tnm_tx.inner().clone())
        .await;
    match wait_status {
        Ok(status) => {
            let state = client
                .get_tn_status(id.to_string(), tnm_tx.inner().clone())
                .await;
            return status::Custom(
                Status::Ok,
                json!(TestNetResponse::<bool> {
                    testnet_id: id.to_string(),
                    command: "STOP_RANDOOM_AND_WAIT".to_string(),
                    was_canceled: false,
                    body: Some(status.unwrap()),
                    last_state_observed: Some(format!("{:?}", state.unwrap())),
                    messages: None,
                    errors: None,
                }),
            );
        }
        Err(e) => {
            return status::Custom(
                Status::InternalServerError,
                json!(TestNetResponse::<()> {
                    testnet_id: "".to_string(),
                    command: "STOP_RANDOOM_AND_WAIT".to_string(),
                    was_canceled: false,
                    body: Some(()),
                    last_state_observed: Some(format!("{:?}", "UNKNOWN")),
                    messages: None,
                    errors: Some(vec![e.to_string()]),
                }),
            );
        }
    }
}

#[get("/test/action/transition/epoch/wait/<id>")]
pub(crate) async fn transition_epoch_and_wait(
    _quit_tx: &State<tokio::sync::broadcast::Sender<bool>>,
    client: &State<ShivaClient>,
    tnm_tx: &State<flume::Sender<TestNetMessage>>,
    id: &str,
) -> status::Custom<Value> {
    let transition_status = client
        .transition_epoch_wait(id.to_string(), tnm_tx.inner().clone())
        .await;
    match transition_status {
        Ok(status) => {
            let current_state = client
                .get_tn_status(id.to_string(), tnm_tx.inner().clone())
                .await;
            return status::Custom(
                Status::Ok,
                json!(TestNetResponse::<bool> {
                    testnet_id: id.to_string(),
                    command: "TRANSITION_EPOCH_AND_WAIT".to_string(),
                    was_canceled: false,
                    body: Some(status),
                    last_state_observed: Some(format!("{:?}", current_state.unwrap())),
                    messages: None,
                    errors: None,
                }),
            );
        }
        Err(e) => {
            return status::Custom(
                Status::InternalServerError,
                json!(TestNetResponse::<()> {
                    testnet_id: "".to_string(),
                    command: "TRANSITION_EPOCH_AND_WAIT".to_string(),
                    was_canceled: false,
                    body: Some(()),
                    last_state_observed: Some(format!("{:?}", "UNKNOWN")),
                    messages: None,
                    errors: Some(vec![e.to_string()]),
                }),
            );
        }
    }
}
