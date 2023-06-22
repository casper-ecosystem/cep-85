//! Error handling on the Casper platform.
use casper_types::ApiError;

/// Errors that the contract can return.
///
/// When an `Error` is returned from a smart contract, it is converted to an [`ApiError::User`].
///
/// While the code consuming this contract needs to define further error variants, it can
/// return those via the [`Error::User`] variant or equivalently via the [`ApiError::User`]
/// variant.
#[repr(u16)]
#[derive(Clone, Copy)]
pub enum Cep1155Error {
    InvalidKey = 6000,
    /// An invalid event mode was specified.
    InvalidEventsMode = 60001,
    /// The event mode required was not specified.
    MissingEventsMode = 60002,
    /// Missing optional named arg
    Phantom = 60003,
    /// Failed to read the runtime arguments provided.
    FailedToGetArgBytes = 60004,
    /// This contract instance cannot be initialized again.
    ContractAlreadyInitialized = 6005,
    // Identifier Mode is not correct
    InvalidIdentifierMode = 60006,
    MissingIdentifierMode = 60007,
    UnexpectedKeyVariant = 60008,
    MissingStorageUref = 60009,
    InvalidStorageUref = 60010,
    MissingNumberOfMintedTokens = 60011,
    InvalidNumberOfMintedTokens = 60012,
    MissingTotalTokenSupply = 60013,
    InvalidTotalTokenSupply = 60014,
    TokenSupplyDepleted = 60015,
    /// Spender does not have enough balance.
    InsufficientBalance = 60016,
    /// Operation would cause an integer overflow.
    Overflow = 60017,
    /// Params len are not matching
    MismatchParamsLength = 60018,
    FailedToCreateDictionary = 60019,
    MissingOperator = 60020,
    InvalidOperator = 60021,
    MissingOwner = 60022,
    InvalidOwner = 60023,
    MissingAccount = 60024,
    InvalidAccount = 60025,
    MissingId = 60026,
    InvalidId = 60027,
    MissingAccounts = 60028,
    InvalidAccounts = 60029,
    MissingIds = 60030,
    InvalidIds = 60031,
    SelfOperatorApproveal = 60032,
    MissingFrom = 60033,
    InvalidFrom = 60034,
    MissingAmount = 60035,
    InvalidAmount = 60036,
    MissingAmounts = 60037,
    InvalidAmounts = 60038,
    NotApproved = 60039,
    MissingTo = 60040,
    InvalidTo = 60041,
    MissingData = 60042,
    InvalidData = 60043,
    SelfTransfer = 60044,
    FailToTransferBalance = 60045,
    FailToBatchTransferBalance = 60046,
    InvalidRecipient = 60047,
    MissingRecipient = 60048,
    MissingContractHash = 60049,
    MintBurnDisabled = 60050,
    InvalidEnableMBFlag = 60051,
    MissingEnableMBFlag = 60052,
    InvalidTransferFilterContract = 60053,
    MissingTransferFilterContract = 60054,
    MissingCollectionName = 60055,
    InvalidCollectionName = 60056,
    MissingPackageHash = 60057,
    InvalidContractHash = 60058,
    InvalidPackageHash = 60059,
    TransferFilterContractDenied = 60060,
}

impl From<Cep1155Error> for ApiError {
    fn from(error: Cep1155Error) -> Self {
        ApiError::User(error as u16)
    }
}
