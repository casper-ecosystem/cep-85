//! Constants used by the CEP1155 contract.

pub const PREFIX_ACCESS_KEY_NAME: &str = "cep1155_contract_package_access";
pub const PREFIX_CEP1155: &str = "cep1155";
pub const PREFIX_CONTRACT_NAME: &str = "cep1155_contract_hash";
pub const PREFIX_CONTRACT_VERSION: &str = "cep1155_contract_version";
pub const PREFIX_CONTRACT_PACKAGE_NAME: &str = "cep1155_contract_package_hash";

pub const ENTRY_POINT_BALANCE_OF: &str = "balance_of";
pub const ENTRY_POINT_BALANCE_OF_BATCH: &str = "balance_of_batch";
pub const ENTRY_POINT_INIT: &str = "init";
pub const ENTRY_POINT_IS_APPROVED_FOR_ALL: &str = "is_approved_for_all";
pub const ENTRY_POINT_SAFE_BATCH_TRANSFER_FROM: &str = "safe_batch_transfer_from";
pub const ENTRY_POINT_SAFE_TRANSFER: &str = "safe_transfer_from";
pub const ENTRY_POINT_SET_APPROVAL_FOR_ALL: &str = "set_approval_for_all";

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

pub const BALANCES: &str = "balances";
pub const CONTRACT_HASH: &str = "contract_hash";
pub const ENABLE_MINT_BURN: &str = "enable_mint_burn";
pub const EVENTS_MODE: &str = "events_mode";
pub const IDENTIFIER_MODE: &str = "identifier_mode";
pub const NAME: &str = "name";
pub const NUMBER_OF_MINTED_TOKENS: &str = "number_of_minted_tokens";
pub const OPERATORS: &str = "operators";
pub const PACKAGE_HASH: &str = "package_hash";
pub const TOKEN_CONTRACT: &str = "token_contract";
pub const TOTAL_TOKEN_SUPPLY: &str = "total_token_supply";
pub const TRANSFER_FILTER_CONTRACT: &str = "transfer_filter_contract";
pub const TRANSFER_FILTER_METHOD: &str = "transfer_filter_method";
