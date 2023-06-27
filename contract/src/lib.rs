#![no_std]

extern crate alloc;

pub mod balances;
pub mod constants;
pub mod entry_points;
pub mod error;
pub mod modalities;
pub mod operators;
pub mod supply;
pub mod security;
pub mod uri;

// A feature to allow the contract to be used
// as a library and a binary.
#[cfg(feature = "contract-support")]
pub mod events;
#[cfg(feature = "contract-support")]
pub mod utils;
