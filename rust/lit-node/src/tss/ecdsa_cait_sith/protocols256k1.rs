use super::{models::ProtocolTransactionParams, CsEcdsaState};
use crate::tss::common::key_share::KeyShare;
use crate::tss::common::key_share_helper::KeyHelper;
use crate::tss::common::traits::dkg::BasicDkg;
use crate::tss::common::traits::key_persistence::KeyPersistence;
use crate::{
    error::{unexpected_err, Result},
    peers::peer_state::models::{SimplePeer, SimplePeerExt},
    tasks::beaver_manager::models::BeaverTriplePair,
    tasks::beaver_manager::models::{BeaverMessage, TripleRequest},
    tss::common::storage::read_key_share_from_disk,
    tss::hd_key_ecdsa::HdKeyDeriver,
};
use cait_sith::{
    protocol::Participant,
    triples::{TriplePub, TripleShare},
    KeygenOutput, PresignOutput,
};
use digest::{Digest, FixedOutput};
use elliptic_curve::{
    group::prime::PrimeCurveAffine, ops::Reduce, Curve, CurveArithmetic, ScalarPrimitive,
};
use futures::future::join_all;
use k256::{
    ecdsa::hazmat::DigestPrimitive, elliptic_curve::sec1::ToEncodedPoint, FieldBytes, Scalar,
    Secp256k1,
};

use lit_core::utils::binary::bytes_to_hex;
use serde::{Deserialize, Serialize};
use tracing::instrument;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CsSigshare {
    pub share: Scalar,
    pub public_key: k256::AffinePoint,
    pub presignature_big_r: k256::AffinePoint,
    pub msg_hash: Scalar,
}

const ID_SIGN_CTX: &[u8] = b"LIT_HD_KEY_ID_K256_XMD:SHA-256_SSWU_RO_NUL_";

impl CsEcdsaState {
    // keygen is now using Gennaro DKG - see that implementation in gennaro_dkg.rs
    // pub async fn keygen_by_id(&self, txn_prefix: &str, peers: &Vec<String>) -> Result<String>

    // refresh now uses gennaro
    // pub async fn refresh_for_key(&self, pubkey: &str, dkg_id: &str) -> Result<bool> {

    // reshare now uses gennaro
    // pub async fn reshare_for_key(

    #[instrument(skip_all)]
    pub async fn get_hd_key_threshold(
        &self,
        hd_root_key: &str, // any one will do!
        all_peers: &Vec<SimplePeer>,
        epoch: u64,
    ) -> Result<u16> {
        let addr = &self.state.peer_state.addr;
        let share_index = all_peers.share_index(addr)?;
        debug!(
            "Share index of {} is {}, peers: {:?} ",
            addr,
            share_index,
            all_peers.len()
        );
        debug!(
            "Actual current epoch peers : {:?}",
            self.state.peer_state.peers().await?.len()
        );
        let staker_address = &self.state.peer_state.hex_staker_address();
        let keyshare = read_key_share_from_disk::<KeyShare>(
            hd_root_key,
            share_index,
            epoch,
            self.curve_type(),
            staker_address,
        )
        .await
        .map_err(|e| unexpected_err(e, Some("Could not read key share from disk".into())))?;
        Ok(keyshare.threshold)
    }

    #[instrument(skip_all, fields(txn_prefix = txn_prefix))]
    #[allow(clippy::too_many_arguments)]
    pub async fn get_triple_pair(
        &self,
        message_bytes: &[u8],
        public_key: Vec<u8>,
        request_id: Vec<u8>,
        txn_prefix: &str,
        threshold: u16,
        peer_subset: Vec<SimplePeer>,
        enable_triple_pregen: bool,
    ) -> Result<Option<BeaverTriplePair>> {
        let ps = &mut self.state.peer_state.as_ref();

        let req = TripleRequest {
            message_bytes: message_bytes.to_vec(),
            public_key: public_key.to_vec(),
            request_id: request_id.clone(),
            txn_prefix: txn_prefix.to_string(),
            peers: peer_subset,
            threshold,
        };

        let request_hash = crate::tasks::beaver_manager::listener::request_hash(req.clone());
        trace!(
            "Sending txn {} as request hash {} to beavermanager",
            txn_prefix,
            request_hash
        );

        let (tx, rx) = flume::bounded(1);
        let msg = BeaverMessage::RequestTriple(req, tx);
        ps.bm_tx.send_async(msg).await.map_err(|e| {
            unexpected_err(e, Some("Could not send request to beavermanager".into()))
        })?;
        debug!("Sent request to beavermanager for txn {}.", txn_prefix);

        let triple_pair = rx.recv_async().await.map_err(|e| {
            unexpected_err(e, Some("Could not receive response from beavermanager when requesting triple - Not enough triples.".into()))
        })?;
        debug!("Got beaver triple shares for {}.", txn_prefix);

        triple_pair
    }

    #[instrument(skip_all, fields(txn_prefix = txn_params.txn_prefix))]
    #[allow(clippy::type_complexity)]
    pub async fn triples(
        &self,
        txn_params: &ProtocolTransactionParams,
    ) -> Result<(
        u16,
        (TripleShare<k256::Secp256k1>, TriplePub<k256::Secp256k1>),
        Vec<Participant>,
    )> {
        self.do_triples::<k256::Secp256k1>(txn_params).await
    }

    #[instrument(skip_all, fields(txn_prefix = txn_params.txn_prefix))]
    pub async fn presign(
        &self,
        txn_params: &ProtocolTransactionParams,
        keygen_out: &KeygenOutput<k256::Secp256k1>,
        share0: &TripleShare<k256::Secp256k1>,
        share1: &TripleShare<k256::Secp256k1>,
        pub0: &TriplePub<k256::Secp256k1>,
        pub1: &TriplePub<k256::Secp256k1>,
    ) -> Result<(u16, PresignOutput<k256::Secp256k1>)> {
        self.do_presign::<k256::Secp256k1>(txn_params, keygen_out, share0, share1, pub0, pub1)
            .await
    }

    #[instrument(skip_all, fields(txn_prefix = txn_params.txn_prefix))]
    #[allow(clippy::too_many_arguments)]
    pub async fn generate_hd_key_signature_share_from_tweak(
        &self,
        txn_params: &ProtocolTransactionParams,
        message: &[u8],
        tweak: &[u8],
        hd_root_keys: Vec<String>,
        triple_pair: &BeaverTriplePair,
        epoch: u64,
        peers: Vec<String>,
    ) -> Result<CsSigshare> {
        debug!(
            "generate_hd_key_signature_share_from_tweak() where tweak is: {:}",
            bytes_to_hex(tweak)
        );
        let deriver =
            HdKeyDeriver::<Secp256k1>::new_from_scalar_bytes(tweak).expect("HdKeyDeriver::new");
        self.generate_hd_key_signature_share(
            txn_params,
            message,
            deriver,
            hd_root_keys,
            triple_pair,
            epoch,
        )
        .await
    }

    #[instrument(skip_all, fields(txn_prefix = txn_params.txn_prefix))]
    pub async fn generate_hd_key_signature_share_from_key_id(
        &self,
        txn_params: &ProtocolTransactionParams,
        message: &[u8],
        key_id: &[u8],
        hd_root_keys: Vec<String>,
        triple_pair: &BeaverTriplePair,
        epoch_number: u64,
    ) -> Result<CsSigshare> {
        let deriver =
            HdKeyDeriver::<Secp256k1>::new(key_id, ID_SIGN_CTX).expect("HdKeyDeriver::new");
        self.generate_hd_key_signature_share(
            txn_params,
            message,
            deriver,
            hd_root_keys,
            triple_pair,
            epoch_number,
        )
        .await
    }

    #[instrument(skip_all, fields(txn_prefix = txn_params.txn_prefix))]
    pub async fn generate_hd_key_signature_share(
        &self,
        txn_params: &ProtocolTransactionParams,
        message: &[u8],
        deriver: HdKeyDeriver<Secp256k1>,
        hd_root_keys: Vec<String>,
        triple_pair: &BeaverTriplePair,
        epoch: u64,
    ) -> Result<CsSigshare> {
        let mut hd_key_shares: Vec<KeygenOutput<Secp256k1>> = Vec::new();

        let share_index = match txn_params.peers.share_index(&self.state.addr) {
            Ok(i) => i,
            Err(e) => {
                return Err(unexpected_err(
                    e,
                    Some("Could not get share self index from peers".into()),
                ))
            }
        };

        let mut threshold = 0; // override and pull this from the root keys.
        let mut total_shares = 0;

        info!(
            "For peer {}, using share index {} and epoch {}.",
            self.state.addr, share_index, epoch
        );
        let key_helper = KeyHelper::<k256::ProjectivePoint>::default();
        let staker_address = &self.state.peer_state.hex_staker_address();

        for hd_root_key in hd_root_keys {
            let keyshare = read_key_share_from_disk::<KeyShare>(
                &hd_root_key,
                share_index,
                epoch,
                self.curve_type(),
                staker_address,
            )
            .await
            .map_err(|e| {
                unexpected_err(
                    e,
                    Some(format!(
                        "Could not read key share (index/epoch) {}/{} from disk",
                        share_index, epoch,
                    )),
                )
            })?;

            let sk = keyshare.secret::<k256::Scalar>()?;
            let pk = keyshare.public_key::<k256::ProjectivePoint>(self.curve_type())?;

            let keygen_output = KeygenOutput::<Secp256k1> {
                public_key: pk.to_affine(),
                private_share: sk,
            };

            if threshold == 0 {
                threshold = keyshare.threshold
            }
            if total_shares == 0 {
                total_shares = keyshare.total_shares
            };

            hd_key_shares.push(keygen_output);
        }

        info!("HD Key Shares length: {:}", hd_key_shares.len());

        let hd_pub_keys = hd_key_shares
            .iter()
            .map(|x| x.public_key.to_curve())
            .collect::<Vec<_>>();

        let derived_pubkey = deriver.compute_public_key(&hd_pub_keys);
        info!(
            "Computed derived pubkey: {:?}",
            bytes_to_hex(derived_pubkey.to_affine().to_encoded_point(false))
        );

        let hd_secret_share = hd_key_shares
            .iter()
            .map(|x| x.private_share)
            .collect::<Vec<_>>();

        let private_share = deriver
            .compute_secret_key(&hd_secret_share)
            .map_err(|e| unexpected_err(e, Some("Could not compute secret key share".into())))?;

        let keygen_output = KeygenOutput::<Secp256k1> {
            public_key: derived_pubkey.to_affine(),
            private_share,
        };

        trace!("Computed secret share.");

        self.generate_signature_share(txn_params, &keygen_output, message, triple_pair)
            .await
    }

    #[instrument(skip_all, fields(txn_prefix = txn_prefix))]
    pub async fn generate_signature_share_from_pubkey(
        &self,
        txn_prefix: &str,
        public_key: &str,
        message: &[u8],
        shares: &BeaverTriplePair,
    ) -> Result<CsSigshare> {
        let txn_params =
            ProtocolTransactionParams::from_config(self.protocol_params(txn_prefix).await?.into());
        let share_index = self.state.peer_state.node_index_in_current_epoch().await? as u16;
        let epoch = self.state.peer_state.epoch().await;
        let staker_address = &self.state.peer_state.hex_staker_address();

        let keyshare = read_key_share_from_disk::<KeyShare>(
            public_key,
            share_index,
            epoch,
            self.curve_type(),
            staker_address,
        )
        .await
        .map_err(|e| unexpected_err(e, Some("Could not read key share from disk".into())))?;

        let key_helper = KeyHelper::<k256::ProjectivePoint>::default();

        let sk = key_helper.secret_from_hex(&keyshare.hex_private_share)?;
        let pk = key_helper.pk_from_hex(&keyshare.hex_public_key)?;
        let keygen_output = KeygenOutput::<Secp256k1> {
            public_key: pk.into(),
            private_share: sk,
        };

        self.generate_signature_share(&txn_params, &keygen_output, message, shares)
            .await
    }

    #[instrument(skip_all, fields(txn_prefix = txn_params.txn_prefix))]
    pub async fn generate_signature_share(
        &self,
        txn_params: &ProtocolTransactionParams,
        keygen_out: &KeygenOutput<k256::Secp256k1>,
        message: &[u8],
        triple_pair: &BeaverTriplePair,
    ) -> Result<CsSigshare> {
        trace!("generate_signature_share()");
        let start = std::time::Instant::now();

        let mut txn_params = txn_params.clone();
        txn_params.update_triple_info(triple_pair);

        info!("txn_params: {:?}  ", txn_params);

        let dkg_presign = &format!("PRS_{}", txn_params.txn_prefix);

        let (id, presignature) = self
            .presign(
                &txn_params,
                keygen_out,
                &triple_pair.share0,
                &triple_pair.share1,
                &triple_pair.pub0,
                &triple_pair.pub1,
            )
            .await
            .map_err(|e| unexpected_err(e, Some("Could not process presign".into())))?;

        let presignature_big_r = presignature.big_r;

        if message.len() != 32 {
            return Err(unexpected_err(
                "Message length to be signed is not 32 bytes.  Please hash it before sending it to the node to sign.  You can use SHA256 or Keccak256 for example".to_string(),
                None,
            ));
        }

        // this being commented out means that we do raw signing - we do not do a hash before signing and we assume the message has already been hashed
        // let msg_hash = self.scalar_hash(message);
        let scalar_primitive = ScalarPrimitive::<Secp256k1>::from_slice(message).map_err(|e| {
            unexpected_err(
                e,
                Some("Could not convert message to sign into ScalarPrimitive".into()),
            )
        })?;
        let msg_hash = Scalar::from(scalar_primitive);
        let sig_share = self
            .do_signature_share(&txn_params, presignature, msg_hash)
            .await?;

        info!("Sig share {:?}", &sig_share);

        let signature_share = CsSigshare {
            share: sig_share,
            public_key: keygen_out.public_key,
            presignature_big_r,
            msg_hash,
        };

        Ok(signature_share)
    }

    #[instrument(skip_all)]
    pub fn scalar_hash(&self, msg: &[u8]) -> <Secp256k1 as CurveArithmetic>::Scalar {
        let digest = <Secp256k1 as DigestPrimitive>::Digest::new_with_prefix(msg);
        let m_bytes: FieldBytes = digest.finalize_fixed();
        <Scalar as Reduce<<Secp256k1 as Curve>::Uint>>::reduce_bytes(&m_bytes)
    }

    #[instrument(skip_all, fields(txn_prefix = txn_prefix))]
    pub async fn generate_triple_pair(
        &self,
        peers: Option<Vec<SimplePeer>>,
        txn_prefix: String,
    ) -> Result<BeaverTriplePair> {
        info!("Generating triples for {} ...", txn_prefix);

        let mut v = Vec::new();
        let start = std::time::Instant::now();
        for i in 0..2 {
            let txn_prefix_triples = format!("TRP{}.{}", i, txn_prefix);
            let protocol_params = match self
                .protocol_params_with_options(&txn_prefix_triples, peers.clone(), None)
                .await
            {
                Ok(p) => p,
                Err(e) => {
                    error!(
                        "Could not get protocol params for {:?} - {:?}",
                        txn_prefix_triples, e
                    );
                    return Err(unexpected_err(
                        e,
                        Some("Could not get protocol params".into()),
                    ));
                }
            };

            let txn_params = ProtocolTransactionParams::from_config(protocol_params.into());
            debug!("Protocol params: {:?}", &txn_params);

            let s = self.clone();
            let t = tokio::spawn(async move { s.triples(&txn_params).await });

            v.push(t);
        }

        info!("Starting spawned BT threads for {} ...", txn_prefix);
        let mut result = join_all(v).await;

        let result0 = result.pop();
        let result1 = result.pop();

        let err_msg = format!(
            "Could not process triples for {}  in {:?}",
            txn_prefix,
            start.elapsed()
        );

        let (p0, (share0, pub0), peers0) = match result0 {
            Some(Ok(r)) => r.map_err(|e| unexpected_err(e, Some(err_msg.clone())))?,
            Some(Err(e)) => return Err(unexpected_err(e, Some(err_msg.clone()))),
            None => return Err(unexpected_err(err_msg.clone(), None)),
        };

        let (p0, (share1, pub1), peers1) = match result1 {
            Some(Ok(r)) => r.map_err(|e| unexpected_err(e, Some(err_msg.clone())))?,
            Some(Err(e)) => return Err(unexpected_err(e, Some(err_msg.clone()))),
            None => return Err(unexpected_err(err_msg.clone(), None)),
        };

        if peers0 != peers1 {
            error!(
                "Peer collections changed during generation of matching triples. {:?}, {:?}",
                peers0, peers1
            );
            return Err(unexpected_err(
                "Peer collections changed during generation of matching triples.".to_string(),
                None,
            ));
        }

        if pub1.threshold != pub0.threshold {
            error!(
                "Thresholds do not match during generation of matching triples pairs. {:?}, {:?}",
                pub0.threshold, pub1.threshold
            );
            return Err(unexpected_err(
                "Thresholds do not match during generation of matching triples pairs.".to_string(),
                None,
            ));
        }

        let peers = match peers {
            Some(p) => p,
            None => self.state.peer_state.peers().await?,
        };

        info!(
            "Generated triples for {} in {:?}",
            txn_prefix,
            start.elapsed()
        );

        let peer = peers.peer_at_address(&self.state.addr)?;
        let staker_hash = peer.key_hash;

        BeaverTriplePair::new(p0, share0, pub0, share1, pub1, staker_hash, peers)
    }
}
