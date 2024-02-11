use cait_sith::{
    presign, signature_share,
    triples::{generate_triple, TriplePub, TripleShare},
    CSCurve, KeygenOutput, PresignArguments, PresignOutput,
};
use log::info;
use tracing::instrument;

use crate::error::{unexpected_err, Result};

use super::models::{EcdsaKeyShare, ProtocolTransactionParams};
use super::CsEcdsaState;

impl CsEcdsaState {
    // keygen is now handled by gennaro
    // pub async fn do_keygen_by_id<C>(

    // refresh is now handled by Gennaro
    // pub async fn do_refresh<C>(

    // reshare is now handled Gennaro
    // pub async fn do_reshare<C>(

    #[allow(clippy::type_complexity)]
    #[instrument(skip_all, fields(txn_prefix = txn_params.txn_prefix))]
    pub async fn do_triples<C>(
        &self,
        txn_params: ProtocolTransactionParams,
    ) -> Result<(u16, (TripleShare<C>, TriplePub<C>), Vec<String>)>
    where
        C: CSCurve,
    {
        let (txn_prefix, self_participant, participants, all_participants, node_config, threshold) =
            txn_params.into();

        let protocol_name = "triples";
        debug!(
            "Starting Beaver Triples {} with {:?} participants, self: {:?}, threshold: {:?}",
            txn_prefix, &participants, &self_participant, threshold
        );

        let protocol = generate_triple::<C>(&participants, self_participant, threshold)
            .map_err(|e| unexpected_err(e, Some("Unable to generate Beaver Triples".into())))?;
        let triple_result = self
            .execute_protocol(
                protocol_name,
                &participants,
                &self_participant,
                protocol,
                &node_config,
                &all_participants,
            )
            .await
            .map_err(|e| unexpected_err(e, Some("Unable to execute Beaver Triples".into())));

        debug!(
            "Triples for {} finished using {:?} participants.  Result: {}",
            txn_prefix,
            participants.len(),
            triple_result.is_ok()
        );

        match triple_result {
            Ok((index, (share, pub_share))) => Ok((index, (share, pub_share), node_config.peers)),
            Err(e) => Err(e),
        }
    }

    pub async fn do_presign<C>(
        &self,
        txn_params: ProtocolTransactionParams,
        keyshare: EcdsaKeyShare<C>,
        share0: TripleShare<C>,
        share1: TripleShare<C>,
        pub0: &TriplePub<C>,
        pub1: &TriplePub<C>,
    ) -> Result<(u16, PresignOutput<C>)>
    where
        C: CSCurve,
    {
        let protocol_name = "presign";
        let (txn_prefix, self_participant, participants, all_participants, node_config, threshold) =
            txn_params.into();

        info!(
            "Starting Presign {} with {:?} participants, self: {:?}, threshold: {:?}",
            txn_prefix, &participants, &self_participant, threshold
        );

        let keygen_out = KeygenOutput {
            public_key: keyshare.public_key,
            private_share: keyshare.private_share,
        };
        let presign_arguments = PresignArguments {
            triple0: (share0, pub0.clone()),
            triple1: (share1, pub1.clone()),
            keygen_out,
            threshold,
        };
        let protocol = presign::<C>(&participants, self_participant, presign_arguments)
            .map_err(|e| unexpected_err(e, Some("Unable to generate presignature".into())))?;
        self.execute_protocol(
            protocol_name,
            &participants,
            &self_participant,
            protocol,
            &node_config,
            &all_participants,
        )
        .await
        .map_err(|e| unexpected_err(e, Some("Unable to execute presignature".into())))
    }

    pub async fn do_signature_share<C: CSCurve>(
        &self,
        txn_params: ProtocolTransactionParams,
        presignature: PresignOutput<C>,
        msg_hash: C::Scalar,
    ) -> Result<C::Scalar> {
        let protocol_name = "signature_share";
        let (txn_prefix, self_participant, participants, all_participants, node_config, threshold) =
            txn_params.into();
        info!(
            "Starting signature share with {:?} participants, self: {:?}, threshold: {:?}",
            &participants, &self_participant, threshold
        );

        signature_share::<C>(participants, self_participant, presignature, msg_hash)
            .map_err(|e| unexpected_err(e, Some("Unable to generate signature share".into())))
    }
}
