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
#[derive(Clone, Copy, Debug)]
pub enum Cep85Error {
    BurnDisabled = 1,
    ContractAlreadyInitialized = 2,
    ExceededMaxTotalSupply = 3,
    FailedToBatchTransferBalance = 4,
    FailedToCreateArg = 5,
    FailedToCreateDictionary = 6,
    FailedToGetArgBytes = 7,
    FailToTransferBalance = 8,
    InsufficientBalance = 9,
    InsufficientRights = 10,
    InvalidAccount = 11,
    InvalidAccounts = 12,
    InvalidAdminList = 13,
    InvalidAmount = 14,
    InvalidAmounts = 15,
    InvalidBurnTarget = 16,
    InvalidBurnerList = 17,
    InvalidCollectionName = 18,
    InvalidContractHash = 19,
    InvalidData = 20,
    InvalidEnableBurnFlag = 21,
    InvalidEventsMode = 22,
    InvalidFrom = 23,
    InvalidId = 24,
    InvalidIds = 25,
    InvalidKey = 26,
    InvalidMetaList = 27,
    InvalidMinterList = 28,
    InvalidNoneList = 29,
    InvalidOperator = 30,
    InvalidOwner = 31,
    InvalidPackageHash = 32,
    InvalidRecipient = 33,
    InvalidStorageUref = 34,
    InvalidTo = 35,
    InvalidTotalSupply = 36,
    InvalidTotalSupplies = 37,
    InvalidTransferFilterContract = 38,
    InvalidTransferFilterMethod = 39,
    InvalidUri = 40,
    MissingAccount = 41,
    MissingAccounts = 42,
    MissingAmount = 43,
    MissingAmounts = 44,
    MissingCollectionName = 45,
    MissingContractHash = 46,
    MissingEnableMBFlag = 47,
    MissingEventsMode = 48,
    MissingFrom = 49,
    MissingId = 50,
    MissingIds = 51,
    MissingOperator = 52,
    MissingOwner = 53,
    MissingPackageHash = 54,
    MissingRecipient = 55,
    MissingStorageUref = 56,
    MissingTo = 57,
    MissingTotalSupply = 58,
    MissingTotalSupplies = 59,
    MissingTransferFilterContract = 60,
    MissingTransferFilterMethod = 61,
    MissingUri = 62,
    MismatchParamsLength = 63,
    NotApproved = 64,
    Overflow = 65,
    OverflowBatchBurn = 66,
    OverflowBatchMint = 67,
    OverflowBurn = 68,
    OverflowMint = 69,
    Phantom = 70,
    SelfOperatorApproval = 71,
    SelfTransfer = 72,
    TokenSupplyDepleted = 73,
    TransferFilterContractDenied = 74,
    UnexpectedKeyVariant = 75,
    InvalidUpgradeFlag = 76,
    MissingKey = 77,
    InvalidKeyName = 78,
    InvalidValue = 79,
    MissingValue = 80,
    NonSuppliedTokenId = 81,
}

impl From<Cep85Error> for ApiError {
    fn from(error: Cep85Error) -> Self {
        ApiError::User(error as u16)
    }
}
