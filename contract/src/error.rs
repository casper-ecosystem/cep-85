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
    /// An invalid event mode was specified.
    InvalidEventsMode = 60006,
    /// The event mode required was not specified.
    MissingEventsMode = 60007,
    /// An unknown error occurred.
    Phantom = 60008,
    /// Failed to read the runtime arguments provided.
    FailedToGetArgBytes = 60009,
    /// This contract instance cannot be initialized again.
    AlreadyInitialized = 60015,
}

impl From<Cep1155Error> for ApiError {
    fn from(error: Cep1155Error) -> Self {
        ApiError::User(error as u16)
    }
}
