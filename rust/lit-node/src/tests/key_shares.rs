// Valid key shares taken from a test run of our BLS implementation.
pub(crate) const TEST_BLS_KEY_SHARE: &str = "{
    \"hex_private_share\": \"14b887a1414cd47382b11d3478a0b4f6d7f9890e5c9be0c334cadf0e392a1087\",
    \"hex_public_key\": \"a5b236b172c1b2cfb5f0942b73b01bbd9af660771069f4d1b924f3280810016ebff10167fe80d06e2ced53558d2a8c2f\",
    \"curve_type\": 1,
    \"index\": 2,
    \"threshold\": 2,
    \"total_shares\": 3,
    \"txn_prefix\": \"test txn_prefix\"
}";

pub(crate) const TEST_BLS_KEY_SHARE_2: &str ="{
    \"hex_private_share\":\"1e3c67031281339dd2bb1c32f7e35d7b628382d3079c9314e7fa36cc3d5b00ab\",
    \"hex_public_key\":\"a4666eb29d2a0494f73f13280d441daceb566eb838d7aae4f87ba4ba232baf12d76f8a0b86309c73be68553aeed31ab6\",
    \"curve_type\": 1,
    \"index\":0,
    \"threshold\":2,
    \"total_shares\":3,
    \"txn_prefix\":\"EPOCH_DKG_1.BLS\"
}";

// A valid key share taken from a test run of our Cait-Sith implementation.
pub(crate) const TEST_ECDSA_KEY_SHARE: &str = "{
    \"hex_private_share\": \"62B2794C091C0F6D8871CC3F77506B07C6CDC872FE98063A5D83C0F9E8599B39\",
    \"hex_public_key\": \"03522EC015CA40C781EB60FA9CB98C555390AC0634C3B0366BF682B357E0857D3C\",
    \"curve_type\": 2,
    \"index\": 2,
    \"threshold\": 2,
    \"total_shares\": 3,
    \"txn_prefix\": \"test txn_prefix\"
}";

// Test BLS public key and corresponding private key shares with 2/3 threshold
pub(crate) const TEST_BLS_PUB_KEY: &str = "b5ab5cece2d13ecb8363128c8076ad3708d274ba01cc11331aedc6ed5d8b2a477a21f333312918039f6ec146dc97e42c";
pub(crate) const TEST_BLS_PRI_KEY_SHARE_1: &str =
    "18ebaba9f88cd9820ec0b63603539479ad18245da7831007cfaa92a1b390d41a";
pub(crate) const TEST_BLS_PRI_KEY_SHARE_2: &str =
    "016d66abcb7d96b06351f92091dd906f5ebe849f2b428023285eb8c10699a66c";
#[allow(dead_code)]
pub(crate) const TEST_BLS_PRI_KEY_SHARE_3: &str =
    "5ddcc900c80bd126eb1d14132a09646a642288e3af004c3d8112dedf59a278bf";
// Blinders are also scalars, independent from the other keys
pub(crate) const TEST_BLS_BLINDER: &str =
    "52dc3bed022962b5584a760f4b24b44161dc72642965656d60de64965c71a51e";

// Test ECDSA public key and corresponding private key shares with 2/3 threshold
pub(crate) const TEST_ECDSA_PUB_KEY: &str =
    "0215b81f6b17e4a3f09ed9b618f0f5ae96f31770d64a2c3d7522d58a837f6b50f7";
pub(crate) const TEST_ECDSA_PRI_KEY_SHARE_1: &str =
    "259d4c57d02cc2b688ab821c6f6d02f8fc8f37cbde54a2fe15dff89bb0396fc2";
pub(crate) const TEST_ECDSA_PRI_KEY_SHARE_2: &str =
    "9bb3eea41e12043f1092476f8f59a05948cedd961d28b113333006c003979620";
#[allow(dead_code)]
pub(crate) const TEST_ECDSA_PRI_KEY_SHARE_3: &str =
    "11ca90f06bf745c798790cc2af463dbada5fa679acb41eec90adb65786bf7b3d";
// Blinders are also scalars, independent from the other keys
pub(crate) const TEST_ECDSA_BLINDER: &str =
    "0B4A40A99C64439F8E8742FD29582D3ED5C18689147293B7332CF6E2C5F35F16";

pub(crate) fn hex_to_bls_dec_key_share(hex: &str, share_index: u8) -> Vec<u8> {
    let mut bytes = hex::decode(hex).unwrap();
    let mut share = vec![share_index];
    bytes.reverse();
    share.extend(bytes);
    share
}

pub(crate) fn hex_to_ecdsa_dec_key_share(hex: &str, share_index: u8) -> Vec<u8> {
    let bytes = hex::decode(hex).unwrap();
    let mut share = vec![share_index];
    share.extend(bytes);
    share
}
