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
    InvalidEnableBurnFlag = 22,
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
    MissingEnableMBFlag = 48,
    MissingEventsMode = 49,
    MissingFrom = 50,
    MissingId = 51,
    MissingIds = 52,
    MissingOperator = 53,
    MissingOwner = 54,
    MissingPackageHash = 55,
    MissingRecipient = 56,
    MissingStorageUref = 57,
    MissingTo = 58,
    MissingTotalSupply = 59,
    MissingTotalSupplies = 60,
    MissingTransferFilterContract = 61,
    MissingTransferFilterMethod = 62,
    MissingUri = 63,
    MismatchParamsLength = 64,
    NotApproved = 65,
    Overflow = 66,
    OverflowBatchBurn = 67,
    OverflowBatchMint = 68,
    OverflowBurn = 69,
    OverflowMint = 70,
    Phantom = 71,
    SelfOperatorApproval = 72,
    SelfTransfer = 73,
    TokenSupplyDepleted = 74,
    TransferFilterContractDenied = 75,
    UnexpectedKeyVariant = 76,
    InvalidUpgradeFlag = 77,
}

impl From<Cep85Error> for ApiError {
    fn from(error: Cep85Error) -> Self {
        ApiError::User(error as u16)
    }
}
