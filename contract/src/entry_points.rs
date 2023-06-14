//! Contains definition of the entry points.
use alloc::{string::String, vec::Vec};

use casper_types::{CLType, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints};

use crate::constants::ENTRY_POINT_INIT;

/// Returns the `init` entry point.
fn init() -> EntryPoint {
    EntryPoint::new(
        String::from(ENTRY_POINT_INIT),
        Vec::new(),
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the default set of CEP-1155 token entry points.
pub fn generate_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(init());
    entry_points
}
