use crate::models::{
    TestNetCreateParams, TestNetCreateRequest, TestNetInfo, TestNetMessage, TestNetState,
};

pub struct ShivaClient {}

/**
 * Client to the network manager for performing operations.
 * Wraps the manager communications and requires a `flume::sender` for
 * manager communications.
 */
impl ShivaClient {
    pub fn new() -> Self {
        return Self {};
    }

    /**
     * Creates a testnet based on the `TestNetCreateRequest`
     * provided
     */
    pub async fn create_testnets(
        &self,
        tnm_tx: flume::Sender<TestNetMessage>,
        id: String,
        create_request: TestNetCreateRequest,
    ) -> Result<(), String> {
        let res = tnm_tx
            .send_async(TestNetMessage::Create(TestNetCreateParams {
                uuid: id.clone(),
                node_count: create_request.node_count.clone(),
                polling_interval: create_request.polling_interval.clone(),
                epoch_length: create_request.epoch_length.clone(),
                custom_build_path: create_request.custom_build_path.clone(),
                lit_action_server_custom_build_path: create_request
                    .lit_action_server_custom_build_path
                    .clone(),
                existing_config_path: None,
                ecdsa_round_timeout: None,
                enable_rate_limiting: None,
                which: None,
            }))
            .await
            .map_err(|e| e.to_string())?;

        Ok(res)
    }

    pub async fn poll_testnet_status(
        &self,
        tnm_tx: flume::Sender<TestNetMessage>,
        id: String,
    ) -> Result<TestNetState, String> {
        let (p_tx, p_rx) = flume::unbounded::<TestNetState>();

        let _res = tnm_tx
            .send_async(TestNetMessage::Poke(id.to_string(), p_tx))
            .await
            .map_err(|e| e.to_string())?;

        let poll_status = p_rx.recv().map_err(|e| e.to_string())?;

        Ok(poll_status)
    }

    pub async fn delete_testnet(
        &self,
        tnm_tx: flume::Sender<TestNetMessage>,
        id: String,
    ) -> Result<bool, String> {
        let (p_tx, p_rx) = flume::unbounded::<bool>();

        let _res = tnm_tx
            .send_async(TestNetMessage::Delete(id.to_string(), p_tx))
            .await
            .map_err(|e| e.to_string())?;
        let del_status = p_rx.recv().map_err(|e| e.to_string())?;

        Ok(del_status)
    }

    /*
        Helper to get the state of a testnet used in each request handler to poll a testnet state
    */
    pub async fn get_tn_status(
        &self,
        id: String,
        tnm_tx: flume::Sender<TestNetMessage>,
    ) -> Result<Option<TestNetInfo>, String> {
        let (p_tx, p_rx) = flume::unbounded::<Option<TestNetInfo>>();

        let _res = tnm_tx
            .send_async(TestNetMessage::GetInfo(id.to_string(), p_tx))
            .await
            .map_err(|e| e.to_string())?;
        let info_reciever = p_rx.recv().map_err(|e| e.to_string())?;

        return Ok(info_reciever);
    }

    pub async fn get_testnet_ids(
        &self,
        tnm_tx: flume::Sender<TestNetMessage>,
    ) -> Result<Vec<String>, String> {
        let (p_tx, p_rx) = flume::unbounded::<Vec<String>>();

        let _res = tnm_tx
            .send_async(TestNetMessage::GetTestnets(p_tx))
            .await
            .map_err(|e| e.to_string())?;

        let testnet_ids = p_rx.recv().map_err(|e| e.to_string())?;

        Ok(testnet_ids)
    }

    pub async fn stop_random_node(
        &self,
        id: String,
        tnm_tx: flume::Sender<TestNetMessage>,
    ) -> Result<Option<bool>, String> {
        let (p_tx, p_rx) = flume::unbounded::<Option<bool>>();

        let _res = tnm_tx
            .send_async(TestNetMessage::StopRandom(id.to_string(), p_tx))
            .await
            .map_err(|e| e.to_string())?;

        let stop_res = p_rx.recv().map_err(|e| e.to_string())?;

        Ok(stop_res)
    }

    pub async fn stop_random_node_wait(
        &self,
        id: String,
        tnm_tx: flume::Sender<TestNetMessage>,
    ) -> Result<Option<bool>, String> {
        let (p_tx, p_rx) = flume::unbounded::<Option<bool>>();

        let _res = tnm_tx
            .send_async(TestNetMessage::StopRandomAndWait(id.to_string(), p_tx))
            .await
            .map_err(|e| e.to_string())?;

        let poll_status = p_rx.recv().map_err(|e| e.to_string())?;

        Ok(poll_status)
    }

    pub async fn transition_epoch_wait(
        &self,
        id: String,
        tnm_tx: flume::Sender<TestNetMessage>,
    ) -> Result<bool, String> {
        let (p_tx, p_rx) = flume::unbounded::<bool>();

        let _res = tnm_tx
            .send_async(TestNetMessage::TransitionEpochAndWait(id.to_string(), p_tx))
            .await
            .map_err(|e| e.to_string())?;

        let transition_status = p_rx.recv().map_err(|e| e.to_string())?;

        Ok(transition_status)
    }
}
