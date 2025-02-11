//! Contains definition of the entry points.
use crate::constants::{
    ADMIN_LIST, ARG_ACCOUNT, ARG_ACCOUNTS, ARG_AMOUNT, ARG_AMOUNTS, ARG_APPROVED,
    ARG_CONTRACT_HASH, ARG_DATA, ARG_ENABLE_BURN, ARG_EVENTS_MODE, ARG_FROM, ARG_ID, ARG_IDS,
    ARG_OPERATOR, ARG_OWNER, ARG_PACKAGE_HASH, ARG_RECIPIENT, ARG_TO, ARG_TOTAL_SUPPLIES,
    ARG_TOTAL_SUPPLY, ARG_TRANSFER_FILTER_CONTRACT, ARG_TRANSFER_FILTER_METHOD, ARG_URI,
    BURNER_LIST, ENTRY_POINT_BALANCE_OF, ENTRY_POINT_BALANCE_OF_BATCH, ENTRY_POINT_BATCH_BURN,
    ENTRY_POINT_BATCH_MINT, ENTRY_POINT_BATCH_TRANSFER_FROM, ENTRY_POINT_BURN,
    ENTRY_POINT_CHANGE_SECURITY, ENTRY_POINT_INIT, ENTRY_POINT_IS_APPROVED_FOR_ALL,
    ENTRY_POINT_IS_NON_FUNGIBLE, ENTRY_POINT_MINT, ENTRY_POINT_SET_APPROVAL_FOR_ALL,
    ENTRY_POINT_SET_MODALITIES, ENTRY_POINT_SET_TOTAL_SUPPLY_OF,
    ENTRY_POINT_SET_TOTAL_SUPPLY_OF_BATCH, ENTRY_POINT_SET_URI, ENTRY_POINT_SUPPLY_OF,
    ENTRY_POINT_SUPPLY_OF_BATCH, ENTRY_POINT_TOTAL_FUNGIBLE_SUPPLY, ENTRY_POINT_TOTAL_SUPPLY_OF,
    ENTRY_POINT_TOTAL_SUPPLY_OF_BATCH, ENTRY_POINT_TRANSFER_FROM, ENTRY_POINT_UPGRADE,
    ENTRY_POINT_URI, META_LIST, MINTER_LIST, NONE_LIST,
};
use alloc::{boxed::Box, vec};
use casper_types::{
    bytesrepr::Bytes, CLType, CLTyped, EntryPoint, EntryPointAccess, EntryPointPayment,
    EntryPointType, EntryPoints, Parameter,
};

pub fn init() -> EntryPoint {
    EntryPoint::new(
        ENTRY_POINT_INIT,
        vec![
            Parameter::new(ARG_CONTRACT_HASH, CLType::Key),
            Parameter::new(ARG_PACKAGE_HASH, CLType::Key),
            Parameter::new(ARG_TRANSFER_FILTER_CONTRACT, CLType::Key),
            Parameter::new(ARG_TRANSFER_FILTER_METHOD, CLType::String),
            Parameter::new(ADMIN_LIST, CLType::List(Box::new(CLType::Key))),
            Parameter::new(MINTER_LIST, CLType::List(Box::new(CLType::Key))),
            Parameter::new(BURNER_LIST, CLType::List(Box::new(CLType::Key))),
            Parameter::new(META_LIST, CLType::List(Box::new(CLType::Key))),
            Parameter::new(NONE_LIST, CLType::List(Box::new(CLType::Key))),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Called,
        EntryPointPayment::Caller,
    )
}

pub fn upgrade() -> EntryPoint {
    EntryPoint::new(
        ENTRY_POINT_UPGRADE,
        vec![Parameter::new(ARG_PACKAGE_HASH, CLType::Key)],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Called,
        EntryPointPayment::Caller,
    )
}

pub fn mint() -> EntryPoint {
    EntryPoint::new(
        ENTRY_POINT_MINT,
        vec![
            Parameter::new(ARG_RECIPIENT, CLType::Key),
            Parameter::new(ARG_ID, CLType::U256),
            Parameter::new(ARG_AMOUNT, CLType::U256),
            Parameter::new(ARG_URI, CLType::String),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Called,
        EntryPointPayment::Caller,
    )
}

pub fn batch_mint() -> EntryPoint {
    EntryPoint::new(
        ENTRY_POINT_BATCH_MINT,
        vec![
            Parameter::new(ARG_RECIPIENT, CLType::Key),
            Parameter::new(ARG_IDS, CLType::List(Box::new(CLType::U256))),
            Parameter::new(ARG_AMOUNTS, CLType::List(Box::new(CLType::U256))),
            Parameter::new(ARG_URI, CLType::String),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Called,
        EntryPointPayment::Caller,
    )
}

pub fn burn() -> EntryPoint {
    EntryPoint::new(
        ENTRY_POINT_BURN,
        vec![
            Parameter::new(ARG_OWNER, CLType::Key),
            Parameter::new(ARG_ID, CLType::U256),
            Parameter::new(ARG_AMOUNT, CLType::U256),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Called,
        EntryPointPayment::Caller,
    )
}

pub fn batch_burn() -> EntryPoint {
    EntryPoint::new(
        ENTRY_POINT_BATCH_BURN,
        vec![
            Parameter::new(ARG_OWNER, CLType::Key),
            Parameter::new(ARG_IDS, CLType::List(Box::new(CLType::U256))),
            Parameter::new(ARG_AMOUNTS, CLType::List(Box::new(CLType::U256))),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Called,
        EntryPointPayment::Caller,
    )
}

pub fn balance_of() -> EntryPoint {
    EntryPoint::new(
        ENTRY_POINT_BALANCE_OF,
        vec![
            Parameter::new(ARG_ACCOUNT, CLType::Key),
            Parameter::new(ARG_ID, CLType::U256),
        ],
        CLType::Option(Box::new(CLType::U256)),
        EntryPointAccess::Public,
        EntryPointType::Called,
        EntryPointPayment::Caller,
    )
}

pub fn balance_of_batch() -> EntryPoint {
    EntryPoint::new(
        ENTRY_POINT_BALANCE_OF_BATCH,
        vec![
            Parameter::new(ARG_ACCOUNTS, CLType::List(Box::new(CLType::Key))),
            Parameter::new(ARG_IDS, CLType::List(Box::new(CLType::U256))),
        ],
        CLType::List(Box::new(CLType::Option(Box::new(CLType::U256)))),
        EntryPointAccess::Public,
        EntryPointType::Called,
        EntryPointPayment::Caller,
    )
}

pub fn set_approval_for_all() -> EntryPoint {
    EntryPoint::new(
        ENTRY_POINT_SET_APPROVAL_FOR_ALL,
        vec![
            Parameter::new(ARG_OPERATOR, CLType::Key),
            Parameter::new(ARG_APPROVED, CLType::Bool),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Called,
        EntryPointPayment::Caller,
    )
}

pub fn is_approved_for_all() -> EntryPoint {
    EntryPoint::new(
        ENTRY_POINT_IS_APPROVED_FOR_ALL,
        vec![
            Parameter::new(ARG_ACCOUNT, CLType::Key),
            Parameter::new(ARG_OPERATOR, CLType::Key),
        ],
        CLType::Bool,
        EntryPointAccess::Public,
        EntryPointType::Called,
        EntryPointPayment::Caller,
    )
}

pub fn transfer_from() -> EntryPoint {
    EntryPoint::new(
        ENTRY_POINT_TRANSFER_FROM,
        vec![
            Parameter::new(ARG_FROM, CLType::Key),
            Parameter::new(ARG_TO, CLType::Key),
            Parameter::new(ARG_ID, CLType::U256),
            Parameter::new(ARG_AMOUNT, CLType::U256),
            Parameter::new(ARG_DATA, Bytes::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Called,
        EntryPointPayment::Caller,
    )
}

pub fn batch_transfer_from() -> EntryPoint {
    EntryPoint::new(
        ENTRY_POINT_BATCH_TRANSFER_FROM,
        vec![
            Parameter::new(ARG_FROM, CLType::Key),
            Parameter::new(ARG_TO, CLType::Key),
            Parameter::new(ARG_IDS, CLType::List(Box::new(CLType::U256))),
            Parameter::new(ARG_AMOUNTS, CLType::List(Box::new(CLType::U256))),
            Parameter::new(ARG_DATA, Bytes::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Called,
        EntryPointPayment::Caller,
    )
}

pub fn supply_of() -> EntryPoint {
    EntryPoint::new(
        ENTRY_POINT_SUPPLY_OF,
        vec![Parameter::new(ARG_ID, CLType::U256)],
        CLType::Option(Box::new(CLType::U256)),
        EntryPointAccess::Public,
        EntryPointType::Called,
        EntryPointPayment::Caller,
    )
}

pub fn supply_of_batch() -> EntryPoint {
    EntryPoint::new(
        ENTRY_POINT_SUPPLY_OF_BATCH,
        vec![Parameter::new(
            ARG_IDS,
            CLType::List(Box::new(CLType::U256)),
        )],
        CLType::List(Box::new(CLType::Option(Box::new(CLType::U256)))),
        EntryPointAccess::Public,
        EntryPointType::Called,
        EntryPointPayment::Caller,
    )
}

pub fn total_supply_of() -> EntryPoint {
    EntryPoint::new(
        ENTRY_POINT_TOTAL_SUPPLY_OF,
        vec![Parameter::new(ARG_ID, CLType::U256)],
        CLType::Option(Box::new(CLType::U256)),
        EntryPointAccess::Public,
        EntryPointType::Called,
        EntryPointPayment::Caller,
    )
}

pub fn total_supply_of_batch() -> EntryPoint {
    EntryPoint::new(
        ENTRY_POINT_TOTAL_SUPPLY_OF_BATCH,
        vec![Parameter::new(
            ARG_IDS,
            CLType::List(Box::new(CLType::U256)),
        )],
        CLType::List(Box::new(CLType::Option(Box::new(CLType::U256)))),
        EntryPointAccess::Public,
        EntryPointType::Called,
        EntryPointPayment::Caller,
    )
}

pub fn set_total_supply_of() -> EntryPoint {
    EntryPoint::new(
        ENTRY_POINT_SET_TOTAL_SUPPLY_OF,
        vec![
            Parameter::new(ARG_ID, CLType::U256),
            Parameter::new(ARG_TOTAL_SUPPLY, CLType::U256),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Called,
        EntryPointPayment::Caller,
    )
}

pub fn set_total_supply_of_batch() -> EntryPoint {
    EntryPoint::new(
        ENTRY_POINT_SET_TOTAL_SUPPLY_OF_BATCH,
        vec![
            Parameter::new(ARG_IDS, CLType::List(Box::new(CLType::U256))),
            Parameter::new(ARG_TOTAL_SUPPLIES, CLType::List(Box::new(CLType::U256))),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Called,
        EntryPointPayment::Caller,
    )
}

pub fn uri() -> EntryPoint {
    EntryPoint::new(
        ENTRY_POINT_URI,
        vec![Parameter::new(ARG_ID, CLType::U256)],
        CLType::Option(Box::new(CLType::String)),
        EntryPointAccess::Public,
        EntryPointType::Called,
        EntryPointPayment::Caller,
    )
}

pub fn set_uri() -> EntryPoint {
    EntryPoint::new(
        ENTRY_POINT_SET_URI,
        vec![
            Parameter::new(ARG_ID, CLType::U256),
            Parameter::new(ARG_URI, CLType::String),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Called,
        EntryPointPayment::Caller,
    )
}

pub fn is_non_fungible() -> EntryPoint {
    EntryPoint::new(
        ENTRY_POINT_IS_NON_FUNGIBLE,
        vec![Parameter::new(ARG_ID, CLType::U256)],
        CLType::Option(Box::new(CLType::Bool)),
        EntryPointAccess::Public,
        EntryPointType::Called,
        EntryPointPayment::Caller,
    )
}

pub fn total_fungible_supply() -> EntryPoint {
    EntryPoint::new(
        ENTRY_POINT_TOTAL_FUNGIBLE_SUPPLY,
        vec![Parameter::new(ARG_ID, CLType::U256)],
        CLType::Option(Box::new(CLType::U256)),
        EntryPointAccess::Public,
        EntryPointType::Called,
        EntryPointPayment::Caller,
    )
}

pub fn change_security() -> EntryPoint {
    EntryPoint::new(
        ENTRY_POINT_CHANGE_SECURITY,
        vec![
            Parameter::new(ADMIN_LIST, CLType::List(Box::new(CLType::Key))),
            Parameter::new(MINTER_LIST, CLType::List(Box::new(CLType::Key))),
            Parameter::new(BURNER_LIST, CLType::List(Box::new(CLType::Key))),
            Parameter::new(META_LIST, CLType::List(Box::new(CLType::Key))),
            Parameter::new(NONE_LIST, CLType::List(Box::new(CLType::Key))),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Called,
        EntryPointPayment::Caller,
    )
}

pub fn set_modalities() -> EntryPoint {
    EntryPoint::new(
        ENTRY_POINT_SET_MODALITIES,
        vec![
            Parameter::new(ARG_ENABLE_BURN, CLType::Bool),
            Parameter::new(ARG_EVENTS_MODE, CLType::U8),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Called,
        EntryPointPayment::Caller,
    )
}

/// Returns the default set of CEP85 token entry points.
pub fn generate_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(init());
    entry_points.add_entry_point(upgrade());
    entry_points.add_entry_point(balance_of());
    entry_points.add_entry_point(balance_of_batch());
    entry_points.add_entry_point(mint());
    entry_points.add_entry_point(batch_mint());
    entry_points.add_entry_point(burn());
    entry_points.add_entry_point(batch_burn());
    entry_points.add_entry_point(set_approval_for_all());
    entry_points.add_entry_point(is_approved_for_all());
    entry_points.add_entry_point(transfer_from());
    entry_points.add_entry_point(batch_transfer_from());
    entry_points.add_entry_point(supply_of());
    entry_points.add_entry_point(supply_of_batch());
    entry_points.add_entry_point(total_supply_of());
    entry_points.add_entry_point(total_supply_of_batch());
    entry_points.add_entry_point(set_total_supply_of());
    entry_points.add_entry_point(set_total_supply_of_batch());
    entry_points.add_entry_point(uri());
    entry_points.add_entry_point(set_uri());
    entry_points.add_entry_point(is_non_fungible());
    entry_points.add_entry_point(total_fungible_supply());
    entry_points.add_entry_point(change_security());
    entry_points.add_entry_point(set_modalities());
    entry_points
}
