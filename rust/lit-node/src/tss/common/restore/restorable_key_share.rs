use blsful::inner_types::{Bls12381G1, G1Projective};
use k256::Secp256k1;
use verifiable_share_encryption::VerifiableEncryptionDecryptor as VED;

use crate::tss::blsful::models::BlsKeyShare;
use crate::tss::common::backup::VerifiableBackup;
use crate::tss::common::traits::keyresolver::KeyResolver;
use crate::tss::ecdsa_cait_sith::models::EcdsaKeyShare;

pub trait RestorableKeyShare: VED {
    type LocalKeyShareType: serde::Serialize + std::marker::Sync + KeyResolver;
    fn construct_local_key_share(
        public_key: &Self::Point,
        private_key: Self::Scalar,
        encrypted_key_share: &VerifiableBackup<Self>,
    ) -> Self::LocalKeyShareType;
}

impl RestorableKeyShare for Bls12381G1 {
    type LocalKeyShareType = BlsKeyShare<G1Projective>;
    fn construct_local_key_share(
        public_key: &Self::Point,
        private_share: Self::Scalar,
        encrypted_key_share: &VerifiableBackup<Self>,
    ) -> Self::LocalKeyShareType {
        // Generate the key share wrapper to be kept locally
        BlsKeyShare::<G1Projective> {
            private_share,
            public_key: *public_key,
            index: encrypted_key_share.share_index,
            threshold: encrypted_key_share.threshold,
            total_shares: encrypted_key_share.total_shares,
            txn_prefix: encrypted_key_share.txn_prefix.clone(),
        }
    }
}

impl RestorableKeyShare for Secp256k1 {
    type LocalKeyShareType = EcdsaKeyShare<Secp256k1>;
    fn construct_local_key_share(
        public_key: &Self::Point,
        private_share: Self::Scalar,
        encrypted_key_share: &VerifiableBackup<Self>,
    ) -> Self::LocalKeyShareType {
        // Generate the key share wrapper to be kept locally
        EcdsaKeyShare {
            private_share,
            public_key: public_key.into(),
            index: encrypted_key_share.share_index,
            threshold: encrypted_key_share.threshold,
            total_shares: encrypted_key_share.total_shares,
            txn_prefix: encrypted_key_share.txn_prefix.clone(),
        }
    }
}
