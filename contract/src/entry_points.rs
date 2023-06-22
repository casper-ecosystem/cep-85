//! Contains definition of the entry points.
use alloc::{boxed::Box, vec};

use casper_types::{CLType, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Parameter};

use crate::constants::{
    ARG_ACCOUNT, ARG_ACCOUNTS, ARG_AMOUNT, ARG_AMOUNTS, ARG_APPROVED, ARG_FROM, ARG_ID, ARG_IDS,
    ARG_OPERATOR, ARG_TO, CONTRACT_HASH, ENTRY_POINT_BALANCE_OF, ENTRY_POINT_BALANCE_OF_BATCH,
    ENTRY_POINT_INIT, ENTRY_POINT_IS_APPROVED_FOR_ALL, ENTRY_POINT_SAFE_BATCH_TRANSFER_FROM,
    ENTRY_POINT_SAFE_TRANSFER, ENTRY_POINT_SET_APPROVAL_FOR_ALL, ENTRY_POINT_SUPPLY_OF,
    PACKAGE_HASH, TRANSFER_FILTER_CONTRACT,
};

/// Returns the `init` entry point.
pub(crate) fn init() -> EntryPoint {
    EntryPoint::new(
        ENTRY_POINT_INIT,
        vec![
            Parameter::new(PACKAGE_HASH, CLType::Key),
            Parameter::new(CONTRACT_HASH, CLType::U256),
            Parameter::new(
                TRANSFER_FILTER_CONTRACT,
                CLType::Option(Box::new(CLType::Key)),
            ),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the `balance_of` entry point.
pub fn balance_of() -> EntryPoint {
    EntryPoint::new(
        ENTRY_POINT_BALANCE_OF,
        vec![
            Parameter::new(ARG_ACCOUNT, CLType::Key),
            Parameter::new(ARG_ID, CLType::U256),
        ],
        CLType::U256,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the `balance_of_batch` entry point.
pub fn balance_of_batch() -> EntryPoint {
    EntryPoint::new(
        ENTRY_POINT_BALANCE_OF_BATCH,
        vec![
            Parameter::new(ARG_ACCOUNTS, CLType::List(Box::new(CLType::Key))),
            Parameter::new(ARG_IDS, CLType::List(Box::new(CLType::U256))),
        ],
        CLType::List(Box::new(CLType::U256)),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the `set_approval_for_all` entry point.
pub fn set_approval_for_all() -> EntryPoint {
    EntryPoint::new(
        ENTRY_POINT_SET_APPROVAL_FOR_ALL,
        vec![
            Parameter::new(ARG_OPERATOR, CLType::Key),
            Parameter::new(ARG_APPROVED, CLType::Bool),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the `is_approved_for_all` entry point.
pub fn is_approved_for_all() -> EntryPoint {
    EntryPoint::new(
        ENTRY_POINT_IS_APPROVED_FOR_ALL,
        vec![
            Parameter::new(ARG_ACCOUNT, CLType::Key),
            Parameter::new(ARG_OPERATOR, CLType::Key),
        ],
        CLType::Bool,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the `safe_transfer_from` entry point.
pub fn safe_transfer_from() -> EntryPoint {
    EntryPoint::new(
        ENTRY_POINT_SAFE_TRANSFER,
        vec![
            Parameter::new(ARG_FROM, CLType::Key),
            Parameter::new(ARG_TO, CLType::Key),
            Parameter::new(ARG_ID, CLType::U256),
            Parameter::new(ARG_AMOUNT, CLType::U256),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}
/// Returns the `safe_batch_transfer_from` entry point.
pub fn safe_batch_transfer_from() -> EntryPoint {
    EntryPoint::new(
        ENTRY_POINT_SAFE_BATCH_TRANSFER_FROM,
        vec![
            Parameter::new(ARG_FROM, CLType::Key),
            Parameter::new(ARG_TO, CLType::Key),
            Parameter::new(ARG_IDS, CLType::List(Box::new(CLType::U256))),
            Parameter::new(ARG_AMOUNTS, CLType::List(Box::new(CLType::U256))),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the `total_supply` entry point.
pub fn supply_of() -> EntryPoint {
    EntryPoint::new(
        ENTRY_POINT_SUPPLY_OF,
        vec![Parameter::new(ARG_ID, CLType::U256)],
        CLType::U256,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

// TODO ?
// /// Returns the `set_total_supply` entry point.
// pub fn set_total_supply() -> EntryPoint {
//     EntryPoint::new(
//         ENTRY_POINT_SET_TOTAL_SUPPLY,
//         vec![
//             Parameter::new(ARG_ID, CLType::U256),
//             Parameter::new(ARG_TOTAL_SUPPLY, CLType::U256),
//         ],
//         CLType::Unit,
//         EntryPointAccess::Public,
//         EntryPointType::Contract,
//     )
// }

/// Returns the default set of CEP1155 token entry points.
pub fn generate_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(init());
    entry_points.add_entry_point(balance_of());
    entry_points.add_entry_point(balance_of_batch());
    entry_points.add_entry_point(set_approval_for_all());
    entry_points.add_entry_point(is_approved_for_all());
    entry_points.add_entry_point(safe_transfer_from());
    entry_points.add_entry_point(safe_batch_transfer_from());
    entry_points.add_entry_point(supply_of());
    entry_points
}
