#![no_std]

extern crate alloc;

// A feature to allow the contract to be used
// as a library and a binary.

pub mod utils;

pub mod constants;
pub mod entry_points;
pub mod error;
pub mod events;
pub mod modalities;
