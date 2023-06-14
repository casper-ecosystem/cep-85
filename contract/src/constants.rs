//! Constants used by the CEP-1155 contract.

pub const PREFIX_ACCESS_KEY_NAME: &str = "cep1155_contract_package_access";
pub const PREFIX_CEP1155: &str = "cep1155";
pub const PREFIX_CONTRACT_NAME: &str = "cep1155_contract_hash";
pub const PREFIX_CONTRACT_VERSION: &str = "cep1155_contract_version";
pub const PREFIX_CONTRACT_PACKAGE_NAME: &str = "cep1155_contract_package";

/// Name of named-key for `name`.
pub const NAME: &str = "name";
/// Name of named-key for `symbol`
pub const SYMBOL: &str = "symbol";
/// Name of named-key for `decimals`
pub const DECIMALS: &str = "decimals";
/// Name of dictionary-key for `balances`
pub const BALANCES: &str = "balances";
/// Name of dictionary-key for `allowances`
pub const ALLOWANCES: &str = "allowances";
/// Name of named-key for `total_supply`
pub const TOTAL_SUPPLY: &str = "total_supply";
/// Name of named-key for `package_hash`
pub const PACKAGE_HASH: &str = "package_hash";

/// Name of `init` entry point.
pub const ENTRY_POINT_INIT: &str = "init";

pub const EVENTS_MODE: &str = "events_mode";
