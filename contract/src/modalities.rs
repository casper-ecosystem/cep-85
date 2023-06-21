use core::convert::TryFrom;

use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use casper_types::{
    bytesrepr::{self, FromBytes, ToBytes, U256_SERIALIZED_LENGTH, U8_SERIALIZED_LENGTH},
    CLTyped, U256,
};

use crate::error::Cep1155Error;

#[repr(u8)]
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum TokenIdentifierMode {
    Ordinal = 0,
    // TODO
    // Hash = 1,
}

impl TryFrom<u8> for TokenIdentifierMode {
    type Error = Cep1155Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(TokenIdentifierMode::Ordinal),
            // 1 => Ok(TokenIdentifierMode::Hash),
            _ => Err(Cep1155Error::InvalidIdentifierMode),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Copy)]
pub enum TokenIdentifier {
    Index(U256),
    // Hash(String),
}

impl TokenIdentifier {
    pub fn new_index(index: U256) -> Self {
        TokenIdentifier::Index(index)
    }

    // pub fn new_hash(hash: String) -> Self {
    //     TokenIdentifier::Hash(hash)
    // }

    pub fn get_index(&self) -> Option<U256> {
        if let Self::Index(index) = self {
            return Some(*index);
        }
        None
    }

    // pub fn get_hash(self) -> Option<String> {
    //     if let Self::Hash(hash) = self {
    //         return Some(hash);
    //     }
    //     None
    // }

    pub fn get_dictionary_item_key(&self) -> String {
        match self {
            TokenIdentifier::Index(token_index) => token_index.to_string(),
            // TokenIdentifier::Hash(hash) => hash.clone(),
        }
    }
}

impl ToBytes for TokenIdentifier {
    fn to_bytes(&self) -> Result<Vec<u8>, bytesrepr::Error> {
        let mut bytes = Vec::new();
        match self {
            TokenIdentifier::Index(index) => {
                bytes.push(TokenIdentifierMode::Ordinal as u8);
                bytes.append(&mut index.to_bytes()?);
            } /* TokenIdentifier::Hash(string) => {
               *     bytes.push(TokenIdentifierMode::Hash as u8);
               *     bytes.append(&mut string.to_bytes()?);
               * } */
        };
        Ok(bytes)
    }

    fn serialized_length(&self) -> usize {
        U8_SERIALIZED_LENGTH
            + match self {
                TokenIdentifier::Index(_) => U256_SERIALIZED_LENGTH,
                //TokenIdentifier::Hash(string) => string.serialized_length(),
            }
    }
}

impl FromBytes for TokenIdentifier {
    fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), bytesrepr::Error> {
        let (identifier_mode, bytes) = u8::from_bytes(bytes)?;
        let identifier_mode = TokenIdentifierMode::try_from(identifier_mode)
            .map_err(|_| bytesrepr::Error::Formatting)?;
        match identifier_mode {
            TokenIdentifierMode::Ordinal => {
                let (index, bytes) = U256::from_bytes(bytes)?;
                Ok((TokenIdentifier::Index(index), bytes))
            } /* TokenIdentifierMode::Hash => {
               *     let (string, bytes) = String::from_bytes(bytes)?;
               *     Ok((TokenIdentifier::Hash(string), bytes))
               * } */
        }
    }
}

impl ToString for TokenIdentifier {
    fn to_string(&self) -> String {
        match self {
            TokenIdentifier::Index(index) => index.to_string(),
            //  TokenIdentifier::Hash(hash) => hash.to_string(),
        }
    }
}

impl CLTyped for TokenIdentifier {
    fn cl_type() -> casper_types::CLType {
        casper_types::CLType::String
    }
}

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
