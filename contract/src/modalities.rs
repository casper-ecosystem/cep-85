use crate::error::Cep1155Error;
use core::convert::TryFrom;

#[repr(u8)]
#[derive(PartialEq, Eq)]
#[allow(clippy::upper_case_acronyms)]
pub enum EventsMode {
    NoEvents = 0,
    CES = 1,
}

impl TryFrom<u8> for EventsMode {
    type Error = Cep1155Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(EventsMode::NoEvents),
            1 => Ok(EventsMode::CES),
            _ => Err(Cep1155Error::InvalidEventsMode),
        }
    }
}

#[repr(u8)]
#[non_exhaustive]
#[derive(PartialEq, Eq)]
pub enum TransferFilterContractResult {
    DenyTransfer = 0,
    ProceedTransfer,
}

impl From<u8> for TransferFilterContractResult {
    fn from(value: u8) -> Self {
        match value {
            0 => TransferFilterContractResult::DenyTransfer,
            _ => TransferFilterContractResult::ProceedTransfer,
        }
    }
}
