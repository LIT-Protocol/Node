use super::models::ProtocolTransactionParams;
use super::CsEcdsaState;
use crate::error::{unexpected_err, Result};
use cait_sith::{
    presign,
    protocol::Participant,
    signature_share,
    triples::{generate_triple, TriplePub, TripleShare},
    CSCurve, KeygenOutput, PresignArguments, PresignOutput,
};
use tracing::instrument;

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
        txn_params: &ProtocolTransactionParams,
    ) -> Result<(u16, (TripleShare<C>, TriplePub<C>), Vec<Participant>)>
    where
        C: CSCurve,
    {
        let (txn_prefix, self_participant, participants, peers, threshold) =
            txn_params.clone().into();

        let protocol_name = "triples";
        debug!(
            "Starting Beaver Triples {} with {:?} participants, self: {:?}, threshold: {:?}",
            txn_prefix, &participants, &self_participant, threshold
        );

        let protocol = generate_triple::<C>(&participants, self_participant, threshold as usize)
            .map_err(|e| unexpected_err(e, Some("Unable to generate Beaver Triples".into())))?;

        let triple_result = self
            .execute_protocol(protocol_name, txn_params, protocol)
            .await
            .map_err(|e| unexpected_err(e, Some("Unable to execute Beaver Triples".into())));

        match triple_result {
            Ok((index, (share, pub_share))) => Ok((index, (share, pub_share), participants)),
            Err(e) => Err(e),
        }
    }

    pub async fn do_presign<C>(
        &self,
        txn_params: &ProtocolTransactionParams,
        keygen_out: &KeygenOutput<C>,
        share0: &TripleShare<C>,
        share1: &TripleShare<C>,
        pub0: &TriplePub<C>,
        pub1: &TriplePub<C>,
    ) -> Result<(u16, PresignOutput<C>)>
    where
        C: CSCurve,
    {
        let protocol_name = "presign";
        let (txn_prefix, self_participant, participants, peers, threshold) =
            txn_params.clone().into();

        let presign_arguments = PresignArguments {
            triple0: (share0.clone(), pub0.clone()),
            triple1: (share1.clone(), pub1.clone()),
            keygen_out: keygen_out.clone(),
            threshold: threshold as usize,
        };

        let (bt_id, bt_participants) = match (txn_params.bt_id, txn_params.bt_participants.clone())
        {
            (Some(bt_id), Some(bt_participants)) => (bt_id, bt_participants),
            _ => {
                return Err(unexpected_err(
                    "Missing Triple-specific Participants",
                    Some("Unable to execute presignature".into()),
                ))
            }
        };

        let protocol = presign::<C>(
            &participants,
            self_participant,
            &bt_participants,
            bt_id,
            presign_arguments,
        )
        .map_err(|e| unexpected_err(e, Some("Unable to generate presignature".into())))?;
        self.execute_protocol(protocol_name, txn_params, protocol)
            .await
            .map_err(|e| unexpected_err(e, Some("Unable to execute presignature".into())))
    }

    pub async fn do_signature_share<C: CSCurve>(
        &self,
        txn_params: &ProtocolTransactionParams,
        presignature: PresignOutput<C>,
        msg_hash: C::Scalar,
    ) -> Result<C::Scalar> {
        let protocol_name = "signature_share";
        let (txn_prefix, self_participant, participants, peers, threshold) =
            txn_params.clone().into();

        signature_share::<C>(participants, self_participant, presignature, msg_hash)
            .map_err(|e| unexpected_err(e, Some("Unable to generate signature share".into())))
    }
}
