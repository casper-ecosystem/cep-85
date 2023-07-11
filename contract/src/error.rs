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
pub enum Cep85Error {
    BurnDisabled = 1,
    ContractAlreadyInitialized = 2,
    ExceededMaxTotalSupply = 3,
    FailedToBatchTransferBalance = 4,
    FailedToCreateArg = 5,
    FailedToCreateDictionary = 6,
    FailedToGetArgBytes = 7,
    FailToBatchTransferBalance = 8,
    FailToTransferBalance = 9,
    InsufficientBalance = 10,
    InsufficientRights = 11,
    InvalidAccount = 12,
    InvalidAccounts = 13,
    InvalidAdminList = 14,
    InvalidAmount = 15,
    InvalidAmounts = 16,
    InvalidBurnTarget = 17,
    InvalidBurnerList = 18,
    InvalidCollectionName = 19,
    InvalidContractHash = 20,
    InvalidData = 21,
    InvalidEnableMBFlag = 22,
    InvalidEventsMode = 23,
    InvalidFrom = 24,
    InvalidId = 25,
    InvalidIds = 26,
    InvalidKey = 27,
    InvalidMetaList = 28,
    InvalidMinterList = 29,
    InvalidNoneList = 30,
    InvalidOperator = 31,
    InvalidOwner = 32,
    InvalidPackageHash = 33,
    InvalidRecipient = 34,
    InvalidStorageUref = 35,
    InvalidTo = 36,
    InvalidTotalSupply = 37,
    InvalidTotalSupplies = 38,
    InvalidTransferFilterContract = 39,
    InvalidTransferFilterMethod = 40,
    InvalidUri = 41,
    MissingAccount = 42,
    MissingAccounts = 43,
    MissingAmount = 44,
    MissingAmounts = 45,
    MissingCollectionName = 46,
    MissingContractHash = 47,
    MissingData = 48,
    MissingEnableMBFlag = 49,
    MissingEventsMode = 50,
    MissingFrom = 51,
    MissingId = 52,
    MissingIds = 53,
    MissingOperator = 54,
    MissingOwner = 55,
    MissingPackageHash = 56,
    MissingRecipient = 57,
    MissingStorageUref = 58,
    MissingTo = 59,
    MissingTotalSupply = 60,
    MissingTotalSupplies = 61,
    MissingTransferFilterContract = 62,
    MissingTransferFilterMethod = 63,
    MissingUri = 64,
    MismatchParamsLength = 65,
    NotApproved = 66,
    Overflow = 67,
    OverflowBatchBurn = 68,
    OverflowBatchMint = 69,
    OverflowBurn = 70,
    OverflowMint = 71,
    Phantom = 72,
    SelfOperatorApproval = 73,
    SelfTransfer = 74,
    TokenSupplyDepleted = 75,
    TransferFilterContractDenied = 76,
    UnexpectedKeyVariant = 77,
}

impl From<Cep85Error> for ApiError {
    fn from(error: Cep85Error) -> Self {
        ApiError::User(error as u16)
    }
}
