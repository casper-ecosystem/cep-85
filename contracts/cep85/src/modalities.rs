use alloc::vec;
use casper_types::{
    bytesrepr::{FromBytes, ToBytes, U8_SERIALIZED_LENGTH},
    CLType, CLTyped,
};

use crate::error::Cep85Error;

#[repr(u8)]
#[derive(PartialEq, Eq)]
#[allow(clippy::upper_case_acronyms)]
pub enum EventsMode {
    NoEvents = 0,
    CES = 1,
}

impl TryFrom<u8> for EventsMode {
    type Error = Cep85Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(EventsMode::NoEvents),
            1 => Ok(EventsMode::CES),
            _ => Err(Cep85Error::InvalidEventsMode),
        }
    }
}

#[repr(u8)]
#[non_exhaustive]
#[derive(PartialEq, Eq, Clone, Copy, Default, Debug)]
pub enum TransferFilterContractResult {
    #[default]
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

impl CLTyped for TransferFilterContractResult {
    fn cl_type() -> casper_types::CLType {
        CLType::U8
    }
}

impl FromBytes for TransferFilterContractResult {
    fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), casper_types::bytesrepr::Error> {
        match bytes.split_first() {
            None => Err(casper_types::bytesrepr::Error::EarlyEndOfStream),
            Some((byte, rem)) => match TransferFilterContractResult::try_from(*byte) {
                Ok(kind) => Ok((kind, rem)),
                Err(_) => Err(casper_types::bytesrepr::Error::EarlyEndOfStream),
            },
        }
    }
}
impl ToBytes for TransferFilterContractResult {
    fn to_bytes(&self) -> Result<alloc::vec::Vec<u8>, casper_types::bytesrepr::Error> {
        Ok(vec![*self as u8])
    }

    fn serialized_length(&self) -> usize {
        U8_SERIALIZED_LENGTH
    }
}
