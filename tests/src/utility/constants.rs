pub const CEP85_CONTRACT_WASM: &str = "cep85.wasm";
pub const CEP85_TEST_CONTRACT_WASM: &str = "cep85_test_contract.wasm";
use casper_types::{account::AccountHash, PublicKey, SecretKey};
use once_cell::sync::Lazy;

pub const CEP85_TEST_TOKEN_CONTRACT_NAME: &str = "cep85_contract_hash_casper_test";

pub const TOKEN_NAME: &str = "casper_test";
pub const TOKEN_URI: &str = "https://token-cdn-domain/{id}.json";

static ACCOUNT_USER_1_SECRET_KEY: Lazy<SecretKey> =
    Lazy::new(|| SecretKey::secp256k1_from_bytes([1u8; 32]).unwrap());
static ACCOUNT_USER_1_PUBLIC_KEY: Lazy<PublicKey> =
    Lazy::new(|| PublicKey::from(&*ACCOUNT_USER_1_SECRET_KEY));
pub static ACCOUNT_USER_1: Lazy<AccountHash> =
    Lazy::new(|| ACCOUNT_USER_1_PUBLIC_KEY.to_account_hash());
