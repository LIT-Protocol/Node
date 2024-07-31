use blsful::inner_types::{Bls12381G1, G1Projective};
use k256::Secp256k1;
use verifiable_share_encryption::VerifiableEncryptionDecryptor as VED;

use crate::tss::common::backup::VerifiableBackup;
use crate::tss::common::curve_type::CurveType;
use crate::tss::common::key_share::KeyShare;
use crate::tss::common::key_share_helper::KeyHelper;
use crate::tss::common::traits::key_persistence::KeyPersistence;

pub trait RestorableKeyShare: VED {
    type LocalKeyShareType: serde::Serialize + std::marker::Sync;
    fn construct_local_key_share(
        public_key: &Self::Point,
        private_key: Self::Scalar,
        encrypted_key_share: &VerifiableBackup<Self>,
    ) -> Self::LocalKeyShareType;
    fn curve_type() -> CurveType;
}

impl RestorableKeyShare for Bls12381G1 {
    type LocalKeyShareType = KeyShare;
    fn construct_local_key_share(
        public_key: &Self::Point,
        private_share: Self::Scalar,
        encrypted_key_share: &VerifiableBackup<Self>,
    ) -> Self::LocalKeyShareType {
        // Generate the key share wrapper to be kept locally
        let key_helper = KeyHelper::<G1Projective>::default();

        KeyShare {
            hex_private_share: key_helper.secret_to_hex(&private_share),
            hex_public_key: key_helper.pk_to_hex(public_key),
            curve_type: CurveType::BLS.into(),
            index: encrypted_key_share.share_index,
            threshold: encrypted_key_share.threshold,
            total_shares: encrypted_key_share.total_shares,
            txn_prefix: encrypted_key_share.txn_prefix.clone(),
        }
    }
    fn curve_type() -> CurveType {
        CurveType::BLS
    }
}

impl RestorableKeyShare for Secp256k1 {
    type LocalKeyShareType = KeyShare;
    fn construct_local_key_share(
        public_key: &Self::Point,
        private_share: Self::Scalar,
        encrypted_key_share: &VerifiableBackup<Self>,
    ) -> Self::LocalKeyShareType {
        let key_helper = KeyHelper::<k256::ProjectivePoint>::default();

        // Generate the key share wrapper to be kept locally
        KeyShare {
            hex_private_share: key_helper.secret_to_hex(&private_share),
            hex_public_key: key_helper.pk_to_hex(public_key),
            curve_type: CurveType::K256.into(),
            index: encrypted_key_share.share_index,
            threshold: encrypted_key_share.threshold,
            total_shares: encrypted_key_share.total_shares,
            txn_prefix: encrypted_key_share.txn_prefix.clone(),
        }
    }
    fn curve_type() -> CurveType {
        CurveType::K256
    }
}
