//! Constants used by the CEP85 contract.

pub const PREFIX_ACCESS_KEY_NAME: &str = "cep85_contract_package_access";
pub const PREFIX_CEP85: &str = "cep85";
pub const PREFIX_CONTRACT_NAME: &str = "cep85_contract_hash";
pub const PREFIX_CONTRACT_VERSION: &str = "cep85_contract_version";
pub const PREFIX_CONTRACT_PACKAGE_NAME: &str = "cep85_contract_package_hash";

pub const ENTRY_POINT_BALANCE_OF: &str = "balance_of";
pub const ENTRY_POINT_BALANCE_OF_BATCH: &str = "balance_of_batch";
pub const ENTRY_POINT_INIT: &str = "init";
pub const ENTRY_POINT_MINT: &str = "mint";
pub const ENTRY_POINT_BATCH_MINT: &str = "batch_mint";
pub const ENTRY_POINT_BURN: &str = "burn";
pub const ENTRY_POINT_BATCH_BURN: &str = "batch_burn";
pub const ENTRY_POINT_IS_APPROVED_FOR_ALL: &str = "is_approved_for_all";
pub const ENTRY_POINT_SAFE_BATCH_TRANSFER_FROM: &str = "safe_batch_transfer_from";
pub const ENTRY_POINT_SAFE_TRANSFER: &str = "safe_transfer_from";
pub const ENTRY_POINT_SET_APPROVAL_FOR_ALL: &str = "set_approval_for_all";
pub const ENTRY_POINT_SUPPLY_OF: &str = "supply_of";
pub const ENTRY_POINT_SUPPLY_OF_BATCH: &str = "supply_of_batch";
pub const ENTRY_POINT_TOTAL_SUPPLY_OF: &str = "total_supply_of";
pub const ENTRY_POINT_TOTAL_SUPPLY_OF_BATCH: &str = "total_supply_of_batch";
pub const ENTRY_POINT_SET_TOTAL_SUPPLY_OF: &str = "set_total_supply_of";
pub const ENTRY_POINT_SET_TOTAL_SUPPLY_OF_BATCH: &str = "set_total_supply_of_batch";
pub const ENTRY_POINT_URI: &str = "uri";
pub const ENTRY_POINT_SET_URI: &str = "set_uri";
pub const ENTRY_POINT_IS_NON_FUNGIBLE: &str = "is_non_fungible";
pub const ENTRY_POINT_TOTAL_FUNGIBLE_SUPPLY: &str = "total_fungible_supply";
pub const ENTRY_POINT_CHANGE_SECURITY: &str = "change_security";

pub const ARG_ACCOUNT: &str = "account";
pub const ARG_ACCOUNTS: &str = "accounts";
pub const ARG_AMOUNT: &str = "amount";
pub const ARG_AMOUNTS: &str = "amounts";
pub const ARG_APPROVED: &str = "approved";
pub const ARG_DATA: &str = "data";
pub const ARG_FROM: &str = "from";
pub const ARG_ID: &str = "id";
pub const ARG_IDS: &str = "ids";
pub const ARG_OPERATOR: &str = "operator";
pub const ARG_OWNER: &str = "owner";
pub const ARG_RECIPIENT: &str = "recipient";
pub const ARG_TO: &str = "to";
pub const ARG_TOTAL_SUPPLY: &str = "total_supply";
pub const ARG_TOTAL_SUPPLIES: &str = "total_supplies";
pub const ARG_URI: &str = "uri";
pub const ARG_NAME: &str = "name";
pub const ARG_EVENTS_MODE: &str = "events_mode";
pub const ARG_ENABLE_MINT_BURN: &str = "enable_mint_burn";

pub const BALANCES: &str = "balances";
pub const CONTRACT_HASH: &str = "contract_hash";
pub const ENABLE_MINT_BURN: &str = "enable_mint_burn";
pub const EVENTS_MODE: &str = "events_mode";

pub const SECURITY_BADGES: &str = "security_badges";
pub const ADMIN_LIST: &str = "admin_list";
pub const MINTER_LIST: &str = "minter_list";
pub const BURNER_LIST: &str = "burner_list";
pub const META_LIST: &str = "meta_list";
pub const NONE_LIST: &str = "none_list";

pub const NAME: &str = "name";
pub const OPERATORS: &str = "operators";
pub const PACKAGE_HASH: &str = "package_hash";
pub const SUPPLY: &str = "supply";
pub const TOTAL_SUPPLY: &str = "total_supply";
pub const TOKEN_CONTRACT: &str = "token_contract";
pub const TRANSFER_FILTER_CONTRACT: &str = "transfer_filter_contract";
pub const TRANSFER_FILTER_METHOD: &str = "transfer_filter_method";
pub const TOKEN_URI: &str = "token_uri";
pub const URI: &str = "uri";
