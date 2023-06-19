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
}

impl From<Cep1155Error> for ApiError {
    fn from(error: Cep1155Error) -> Self {
        ApiError::User(error as u16)
    }
}
