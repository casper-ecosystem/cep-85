#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;

use core::convert::TryInto;

use alloc::{
    borrow::ToOwned,
    collections::BTreeMap,
    format,
    string::{String, ToString},
    vec,
    vec::Vec,
};
use casper_contract::{
    contract_api::{
        runtime::{self, call_contract, get_key, manage_message_topic, put_key, revert},
        storage,
    },
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    addressable_entity::NamedKeys, bytesrepr::Bytes, contract_messages::MessageTopicOperation,
    runtime_args, AddressableEntityHash, CLValue, EntityAddr, Key, RuntimeArgs, U256,
};
use cep85::{
    balances::{batch_transfer_balance, read_balance_from, transfer_balance, write_balance_to},
    constants::{
        ADMIN_LIST, ARG_ACCOUNT, ARG_ACCOUNTS, ARG_AMOUNT, ARG_AMOUNTS, ARG_APPROVED,
        ARG_CONTRACT_HASH, ARG_DATA, ARG_ENABLE_BURN, ARG_EVENTS_MODE, ARG_FROM, ARG_ID, ARG_IDS,
        ARG_NAME, ARG_OPERATOR, ARG_OWNER, ARG_PACKAGE_HASH, ARG_RECIPIENT,
        ARG_SESSION_NAMED_KEY_NAME, ARG_TO, ARG_TOTAL_SUPPLIES, ARG_TOTAL_SUPPLY,
        ARG_TRANSFER_FILTER_CONTRACT, ARG_TRANSFER_FILTER_METHOD, ARG_UPGRADE_FLAG, ARG_URI,
        BURNER_LIST, DEFAULT_DICT_ITEM_KEY_NAME, DICT_BALANCES, DICT_OPERATORS,
        DICT_SECURITY_BADGES, DICT_SUPPLY, DICT_TOKEN_URI, DICT_TOTAL_SUPPLY, ENTRY_POINT_INIT,
        ENTRY_POINT_UPGRADE, EVENTS, META_LIST, MINTER_LIST, NONE_LIST, PREFIX_ACCESS_KEY_NAME,
        PREFIX_CONTRACT_NAME, PREFIX_CONTRACT_PACKAGE_NAME, PREFIX_CONTRACT_VERSION,
    },
    entry_points::generate_entry_points,
    error::Cep85Error,
    events::{
        init_events, record_event_dictionary, ApprovalForAll, Burn, BurnBatch,
        ChangeEnableBurnMode, ChangeEventsMode, ChangeSecurity, Event, Mint, MintBatch,
        SetModalities, SetTotalSupply, Transfer, TransferBatch, Upgrade, Uri, UriBatch,
    },
    modalities::{EventsMode, TransferFilterContractResult},
    operators::{read_operator, write_operator},
    security::{change_sec_badge, sec_check, SecurityBadge},
    supply::{read_supply_of, read_total_supply_of, write_supply_of, write_total_supply_of},
    uri::{read_uri_of, write_uri_of},
    utils::{
        get_named_arg_with_user_errors, get_optional_named_arg_with_user_errors,
        get_stored_value_with_user_errors, get_transfer_filter_contract,
        get_transfer_filter_method, get_verified_caller,
        make_dictionary_item_key as utils_make_dictionary_item_key,
    },
};

/// Initiates the contracts states. Only used by the installer call,
/// later calls will cause it to revert.
#[no_mangle]
pub extern "C" fn init() {
    if get_key(ARG_PACKAGE_HASH).is_some() {
        revert(Cep85Error::ContractAlreadyInitialized);
    }

    put_key(
        ARG_PACKAGE_HASH,
        get_named_arg_with_user_errors::<Key>(
            ARG_PACKAGE_HASH,
            Cep85Error::MissingPackageHash,
            Cep85Error::InvalidPackageHash,
        )
        .unwrap_or_revert(),
    );

    put_key(
        ARG_CONTRACT_HASH,
        get_named_arg_with_user_errors::<Key>(
            ARG_CONTRACT_HASH,
            Cep85Error::MissingContractHash,
            Cep85Error::InvalidContractHash,
        )
        .unwrap_or_revert(),
    );

    let transfer_filter_contract_key: Option<Key> =
        get_optional_named_arg_with_user_errors::<Option<Key>>(
            ARG_TRANSFER_FILTER_CONTRACT,
            Cep85Error::InvalidTransferFilterContract,
        )
        .unwrap_or_default();

    let transfer_filter_contract: Option<AddressableEntityHash> =
        transfer_filter_contract_key.map(|transfer_filter_contract_key| {
            transfer_filter_contract_key
                .into_entity_hash()
                .unwrap_or_revert()
        });

    runtime::put_key(
        ARG_TRANSFER_FILTER_CONTRACT,
        storage::new_uref(transfer_filter_contract).into(),
    );

    let transfer_filter_method: Option<String> =
        get_optional_named_arg_with_user_errors::<Option<String>>(
            ARG_TRANSFER_FILTER_METHOD,
            Cep85Error::InvalidTransferFilterMethod,
        )
        .unwrap_or_default();

    runtime::put_key(
        ARG_TRANSFER_FILTER_METHOD,
        storage::new_uref(transfer_filter_method).into(),
    );

    storage::new_dictionary(DICT_BALANCES)
        .unwrap_or_revert_with(Cep85Error::FailedToCreateDictionary);
    storage::new_dictionary(DICT_OPERATORS)
        .unwrap_or_revert_with(Cep85Error::FailedToCreateDictionary);
    storage::new_dictionary(DICT_SUPPLY)
        .unwrap_or_revert_with(Cep85Error::FailedToCreateDictionary);
    storage::new_dictionary(DICT_TOTAL_SUPPLY)
        .unwrap_or_revert_with(Cep85Error::FailedToCreateDictionary);
    storage::new_dictionary(DICT_TOKEN_URI)
        .unwrap_or_revert_with(Cep85Error::FailedToCreateDictionary);

    init_events();

    storage::new_dictionary(DICT_SECURITY_BADGES)
        .unwrap_or_revert_with(Cep85Error::FailedToCreateDictionary);

    let mut badge_map: BTreeMap<Key, SecurityBadge> = BTreeMap::new();

    let admin_list: Option<Vec<Key>> =
        get_optional_named_arg_with_user_errors(ADMIN_LIST, Cep85Error::InvalidAdminList);
    let minter_list: Option<Vec<Key>> =
        get_optional_named_arg_with_user_errors(MINTER_LIST, Cep85Error::InvalidMinterList);
    let burner_list: Option<Vec<Key>> =
        get_optional_named_arg_with_user_errors(BURNER_LIST, Cep85Error::InvalidBurnerList);
    let meta_list: Option<Vec<Key>> =
        get_optional_named_arg_with_user_errors(META_LIST, Cep85Error::InvalidMetaList);
    let none_list: Option<Vec<Key>> =
        get_optional_named_arg_with_user_errors(NONE_LIST, Cep85Error::InvalidNoneList);

    if let Some(minter_list) = minter_list {
        for account_key in minter_list {
            badge_map.insert(account_key, SecurityBadge::Minter);
        }
    }
    if let Some(burner_list) = burner_list {
        for account_key in burner_list {
            badge_map.insert(account_key, SecurityBadge::Burner);
        }
    }
    if let Some(meta_list) = meta_list {
        for account_key in meta_list {
            badge_map.insert(account_key, SecurityBadge::Meta);
        }
    }

    if admin_list.is_none()
        || admin_list
            .as_ref()
            .unwrap_or_revert_with(Cep85Error::InvalidAdminList)
            .is_empty()
    {
        badge_map.insert(get_verified_caller().0, SecurityBadge::Admin);
    } else if let Some(admin_list) = admin_list {
        for account_key in admin_list {
            badge_map.insert(account_key, SecurityBadge::Admin);
        }
    }
    if let Some(none_list) = none_list {
        for account_key in none_list {
            badge_map.insert(account_key, SecurityBadge::None);
        }
    }

    change_sec_badge(&badge_map);
}

#[no_mangle]
pub extern "C" fn balance_of() {
    let account: Key = get_named_arg_with_user_errors(
        ARG_ACCOUNT,
        Cep85Error::MissingAccount,
        Cep85Error::InvalidAccount,
    )
    .unwrap_or_revert();
    let id: U256 =
        get_named_arg_with_user_errors(ARG_ID, Cep85Error::MissingId, Cep85Error::InvalidId)
            .unwrap_or_revert();
    if read_total_supply_of(&id).unwrap_or_default().is_zero() {
        runtime::ret(CLValue::from_t::<Option<U256>>(None).unwrap_or_revert());
    }
    let balance: U256 = read_balance_from(&account, &id);
    runtime::ret(CLValue::from_t(Some(balance)).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn balance_of_batch() {
    let accounts: Vec<Key> = get_named_arg_with_user_errors(
        ARG_ACCOUNTS,
        Cep85Error::MissingAccounts,
        Cep85Error::InvalidAccounts,
    )
    .unwrap_or_revert();
    let ids: Vec<U256> =
        get_named_arg_with_user_errors(ARG_IDS, Cep85Error::MissingIds, Cep85Error::InvalidIds)
            .unwrap_or_revert();

    if accounts.len() != ids.len() {
        runtime::revert(Cep85Error::MismatchParamsLength);
    }

    let mut batch_balances = Vec::new();
    for (&account, &id) in accounts.iter().zip(ids.iter()) {
        if read_total_supply_of(&id).unwrap_or_default().is_zero() {
            batch_balances.push(None);
        } else {
            batch_balances.push(Some(read_balance_from(&account, &id)));
        }
    }

    runtime::ret(CLValue::from_t(batch_balances).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn is_approved_for_all() {
    let owner: Key = get_named_arg_with_user_errors(
        ARG_ACCOUNT,
        Cep85Error::MissingAccount,
        Cep85Error::InvalidAccount,
    )
    .unwrap_or_revert();

    let operator: Key = get_named_arg_with_user_errors(
        ARG_OPERATOR,
        Cep85Error::MissingOperator,
        Cep85Error::InvalidOperator,
    )
    .unwrap_or_revert();

    let is_approved_for_all: bool = read_operator(&owner, &operator);

    runtime::ret(CLValue::from_t(is_approved_for_all).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn make_dictionary_item_key() {
    let owner: Key =
        get_named_arg_with_user_errors(ARG_OWNER, Cep85Error::MissingKey, Cep85Error::InvalidKey)
            .unwrap_or_revert();

    let id: Option<U256> =
        get_optional_named_arg_with_user_errors(ARG_ID, Cep85Error::InvalidValue);

    let dictionary_item_key: String = match id {
        Some(id) => utils_make_dictionary_item_key(&owner, &id),
        None => {
            let operator: Option<Key> =
                get_optional_named_arg_with_user_errors(ARG_OPERATOR, Cep85Error::InvalidOperator);
            match operator {
                Some(operator) => utils_make_dictionary_item_key(&owner, &operator),
                None => revert(Cep85Error::InvalidOperator),
            }
        }
    };

    let session_named_key_name: Option<String> = get_optional_named_arg_with_user_errors(
        ARG_SESSION_NAMED_KEY_NAME,
        Cep85Error::InvalidValue,
    );
    let session_named_key_name: &str = session_named_key_name
        .as_deref()
        .unwrap_or(DEFAULT_DICT_ITEM_KEY_NAME);

    put_key(
        session_named_key_name,
        storage::new_uref(dictionary_item_key).into(),
    );
}

#[no_mangle]
pub extern "C" fn set_approval_for_all() {
    let operator: Key = get_named_arg_with_user_errors(
        ARG_OPERATOR,
        Cep85Error::MissingOperator,
        Cep85Error::InvalidOperator,
    )
    .unwrap_or_revert();

    let (caller, caller_package) = get_verified_caller();

    // If caller tries to approve itself as operator that's probably a mistake and we revert.
    let is_self_approval: bool = match caller_package {
        Some(caller_package) => operator == caller_package || operator == caller,
        None => operator == caller,
    };

    if is_self_approval {
        runtime::revert(Cep85Error::SelfOperatorApproval);
    }

    let approved: bool = get_named_arg_with_user_errors(
        ARG_APPROVED,
        Cep85Error::MissingOperator,
        Cep85Error::InvalidOperator,
    )
    .unwrap_or_revert();

    write_operator(&caller, &operator, approved);
    record_event_dictionary(Event::ApprovalForAll(ApprovalForAll {
        owner: caller,
        operator,
        approved,
    }));
}

/// Transfer a specified amount of tokens from the `sender` to the `recipient`.
///
/// This function should only be called by an approved operator or by the sender themselves.
#[no_mangle]
pub extern "C" fn transfer_from() {
    let from: Key =
        get_named_arg_with_user_errors(ARG_FROM, Cep85Error::MissingFrom, Cep85Error::InvalidFrom)
            .unwrap_or_revert();

    let (caller, caller_package) = get_verified_caller();

    // Check if the caller is the spender or an operator
    let is_approved: bool = match caller_package {
        Some(caller_package) => {
            from == caller
                || from == caller_package
                || read_operator(&from, &caller_package)
                || read_operator(&from, &caller)
        }
        None => from == caller || read_operator(&from, &caller),
    };

    if !is_approved {
        runtime::revert(Cep85Error::NotApproved);
    }

    let to: Key =
        get_named_arg_with_user_errors(ARG_TO, Cep85Error::MissingTo, Cep85Error::InvalidTo)
            .unwrap_or_revert();

    let id: U256 =
        get_named_arg_with_user_errors(ARG_ID, Cep85Error::MissingId, Cep85Error::InvalidId)
            .unwrap_or_revert();

    if read_total_supply_of(&id).unwrap_or_default().is_zero() {
        revert(Cep85Error::NonSuppliedTokenId);
    }

    let amount: U256 = get_named_arg_with_user_errors(
        ARG_AMOUNT,
        Cep85Error::MissingAmount,
        Cep85Error::InvalidAmount,
    )
    .unwrap_or_revert();

    let data: Option<Bytes> =
        get_optional_named_arg_with_user_errors(ARG_DATA, Cep85Error::InvalidData);

    before_token_transfer(&caller, &from, &to, &[id], &[amount], data.clone());

    transfer_balance(&from, &to, &id, &amount);
    record_event_dictionary(Event::Transfer(Transfer {
        operator: caller,
        from,
        to,
        id,
        value: amount,
        data,
    }));
}

/// Batch transfer specified amounts of multiple tokens from the `sender` to the `recipient`.
///
/// This function should only be called by an approved operator or by the sender themselves.
#[no_mangle]
pub extern "C" fn batch_transfer_from() {
    let ids: Vec<U256> =
        get_named_arg_with_user_errors(ARG_IDS, Cep85Error::MissingIds, Cep85Error::InvalidIds)
            .unwrap_or_revert();

    let amounts: Vec<U256> = get_named_arg_with_user_errors(
        ARG_AMOUNTS,
        Cep85Error::MissingAmounts,
        Cep85Error::InvalidAmounts,
    )
    .unwrap_or_revert();

    if ids.len() != amounts.len() {
        runtime::revert(Cep85Error::MismatchParamsLength);
    }

    let from: Key =
        get_named_arg_with_user_errors(ARG_FROM, Cep85Error::MissingFrom, Cep85Error::InvalidFrom)
            .unwrap_or_revert();

    let (caller, caller_package) = get_verified_caller();

    // Check if the caller is the spender or an operator
    let is_approved: bool = match caller_package {
        Some(caller_package) => {
            from == caller
                || from == caller_package
                || read_operator(&from, &caller_package)
                || read_operator(&from, &caller)
        }
        None => from == caller || read_operator(&from, &caller),
    };

    if !is_approved {
        runtime::revert(Cep85Error::NotApproved);
    }

    for id in ids.iter() {
        if read_total_supply_of(id).unwrap_or_default().is_zero() {
            revert(Cep85Error::NonSuppliedTokenId);
        }
    }

    let to: Key =
        get_named_arg_with_user_errors(ARG_TO, Cep85Error::MissingTo, Cep85Error::InvalidTo)
            .unwrap_or_revert();

    let data: Option<Bytes> =
        get_optional_named_arg_with_user_errors(ARG_DATA, Cep85Error::InvalidData);

    before_token_transfer(&caller, &from, &to, &ids, &amounts, data.clone());

    batch_transfer_balance(&from, &to, &ids, &amounts);

    record_event_dictionary(Event::TransferBatch(TransferBatch {
        operator: caller,
        from,
        to,
        ids,
        values: amounts,
        data,
    }));
}

#[no_mangle]
pub extern "C" fn mint() {
    sec_check(vec![SecurityBadge::Admin, SecurityBadge::Minter]);

    let recipient: Key = get_named_arg_with_user_errors(
        ARG_RECIPIENT,
        Cep85Error::MissingRecipient,
        Cep85Error::InvalidRecipient,
    )
    .unwrap_or_revert();

    let id: U256 =
        get_named_arg_with_user_errors(ARG_ID, Cep85Error::MissingId, Cep85Error::InvalidId)
            .unwrap_or_revert();

    let amount: U256 = get_named_arg_with_user_errors(
        ARG_AMOUNT,
        Cep85Error::MissingAmount,
        Cep85Error::InvalidAmount,
    )
    .unwrap_or_revert();

    let recipient_balance = read_balance_from(&recipient, &id);
    let new_recipient_balance = recipient_balance.checked_add(amount).unwrap_or_default();

    let supply = read_supply_of(&id);
    let new_supply = supply
        .checked_add(amount)
        .unwrap_or_revert_with(Cep85Error::OverflowMint);
    let total_max_supply = read_total_supply_of(&id).unwrap_or_default();

    if total_max_supply != U256::zero() {
        if new_supply > total_max_supply {
            revert(Cep85Error::ExceededMaxTotalSupply);
        }
    } else {
        write_total_supply_of(&id, &new_supply);
    }

    write_supply_of(&id, &new_supply);
    write_balance_to(&recipient, &id, &new_recipient_balance);

    let uri: String = get_optional_named_arg_with_user_errors(ARG_URI, Cep85Error::InvalidUri)
        .unwrap_or_default();

    record_event_dictionary(Event::Mint(Mint {
        id,
        recipient,
        amount,
    }));

    if !uri.is_empty() {
        write_uri_of(&id, &uri);
        record_event_dictionary(Event::Uri(Uri {
            id: Some(id),
            value: uri,
        }));
    };
}

/// Batch mint specified amounts of multiple tokens to one `recipient`.
#[no_mangle]
pub extern "C" fn batch_mint() {
    sec_check(vec![SecurityBadge::Admin, SecurityBadge::Minter]);

    let recipient: Key = get_named_arg_with_user_errors(
        ARG_RECIPIENT,
        Cep85Error::MissingRecipient,
        Cep85Error::InvalidRecipient,
    )
    .unwrap_or_revert();

    let ids: Vec<U256> =
        get_named_arg_with_user_errors(ARG_IDS, Cep85Error::MissingIds, Cep85Error::InvalidIds)
            .unwrap_or_revert();

    let amounts: Vec<U256> = get_named_arg_with_user_errors(
        ARG_AMOUNTS,
        Cep85Error::MissingAmounts,
        Cep85Error::InvalidAmounts,
    )
    .unwrap_or_revert();

    if ids.len() != amounts.len() {
        revert(Cep85Error::MismatchParamsLength);
    }

    let uri: String = get_optional_named_arg_with_user_errors(ARG_URI, Cep85Error::InvalidUri)
        .unwrap_or_default();

    for (i, &id) in ids.iter().enumerate() {
        let amount = amounts[i];

        let recipient_balance = read_balance_from(&recipient, &id);
        let new_recipient_balance = recipient_balance.checked_add(amount).unwrap_or_default();

        let supply = read_supply_of(&id);
        let total_max_supply = read_total_supply_of(&id).unwrap_or_default();
        let new_supply = supply
            .checked_add(amount)
            .unwrap_or_revert_with(Cep85Error::OverflowBatchMint);

        if total_max_supply != U256::zero() {
            if new_supply > total_max_supply {
                revert(Cep85Error::ExceededMaxTotalSupply);
            }
        } else {
            write_total_supply_of(&id, &new_supply);
        }

        write_supply_of(&id, &new_supply);
        write_balance_to(&recipient, &id, &new_recipient_balance);
        if !uri.is_empty() {
            write_uri_of(&id, &uri);
        }
    }

    record_event_dictionary(Event::MintBatch(MintBatch {
        ids: ids.clone(),
        recipient,
        amounts,
    }));

    if !uri.is_empty() {
        record_event_dictionary(Event::UriBatch(UriBatch { value: uri, ids }));
    }
}

#[no_mangle]
pub extern "C" fn burn() {
    if !get_stored_value_with_user_errors::<bool>(
        ARG_ENABLE_BURN,
        Cep85Error::MissingEnableMBFlag,
        Cep85Error::InvalidEnableBurnFlag,
    ) {
        revert(Cep85Error::BurnDisabled);
    };

    sec_check(vec![SecurityBadge::Admin, SecurityBadge::Burner]);

    let owner: Key = get_named_arg_with_user_errors(
        ARG_OWNER,
        Cep85Error::MissingOwner,
        Cep85Error::InvalidOwner,
    )
    .unwrap_or_revert();

    let (caller, caller_package) = get_verified_caller();

    // Check if the caller is the owner or operator
    let is_approved: bool = match caller_package {
        Some(caller_package) => {
            owner == caller_package
                || owner == caller
                || read_operator(&owner, &caller_package)
                || read_operator(&owner, &caller)
        }
        None => owner == caller || read_operator(&owner, &caller),
    };

    if !is_approved {
        revert(Cep85Error::InvalidBurnTarget);
    }

    let id: U256 =
        get_named_arg_with_user_errors(ARG_ID, Cep85Error::MissingId, Cep85Error::InvalidId)
            .unwrap_or_revert();

    if read_total_supply_of(&id).unwrap_or_default().is_zero() {
        revert(Cep85Error::NonSuppliedTokenId);
    }

    let amount: U256 = get_named_arg_with_user_errors(
        ARG_AMOUNT,
        Cep85Error::MissingAmount,
        Cep85Error::InvalidAmount,
    )
    .unwrap_or_revert();

    let owner_balance = read_balance_from(&owner, &id);
    let new_owner_balance = owner_balance
        .checked_sub(amount)
        .unwrap_or_revert_with(Cep85Error::OverflowBurn);

    let new_supply = {
        let supply = read_supply_of(&id);
        supply
            .checked_sub(amount)
            .unwrap_or_revert_with(Cep85Error::OverflowBurn)
    };

    write_supply_of(&id, &new_supply);
    write_balance_to(&owner, &id, &new_owner_balance);
    record_event_dictionary(Event::Burn(Burn { id, owner, amount }));
}

#[no_mangle]
pub extern "C" fn batch_burn() {
    if !get_stored_value_with_user_errors::<bool>(
        ARG_ENABLE_BURN,
        Cep85Error::MissingEnableMBFlag,
        Cep85Error::InvalidEnableBurnFlag,
    ) {
        revert(Cep85Error::BurnDisabled);
    };

    sec_check(vec![SecurityBadge::Admin, SecurityBadge::Burner]);

    let owner: Key = get_named_arg_with_user_errors(
        ARG_OWNER,
        Cep85Error::MissingOwner,
        Cep85Error::InvalidOwner,
    )
    .unwrap_or_revert();

    let (caller, caller_package) = get_verified_caller();

    // Check if the caller is the owner or operator
    let is_approved: bool = match caller_package {
        Some(caller_package) => {
            owner == caller_package
                || owner == caller
                || read_operator(&owner, &caller_package)
                || read_operator(&owner, &caller)
        }
        None => owner == caller || read_operator(&owner, &caller),
    };

    if !is_approved {
        revert(Cep85Error::InvalidBurnTarget);
    }

    let ids: Vec<U256> =
        get_named_arg_with_user_errors(ARG_IDS, Cep85Error::MissingIds, Cep85Error::InvalidIds)
            .unwrap_or_revert();

    let amounts: Vec<U256> = get_named_arg_with_user_errors(
        ARG_AMOUNTS,
        Cep85Error::MissingAmounts,
        Cep85Error::InvalidAmounts,
    )
    .unwrap_or_revert();

    if ids.len() != amounts.len() {
        revert(Cep85Error::MismatchParamsLength);
    }

    for (i, &id) in ids.iter().enumerate() {
        if read_total_supply_of(&id).unwrap_or_default().is_zero() {
            revert(Cep85Error::NonSuppliedTokenId);
        }
        let amount = amounts[i];
        let owner_balance = read_balance_from(&owner, &id);
        let new_owner_balance = owner_balance
            .checked_sub(amount)
            .unwrap_or_revert_with(Cep85Error::OverflowBatchBurn);

        let new_supply = {
            let supply = read_supply_of(&id);
            supply
                .checked_sub(amount)
                .unwrap_or_revert_with(Cep85Error::OverflowBatchBurn)
        };

        write_supply_of(&id, &new_supply);
        write_balance_to(&owner, &id, &new_owner_balance);
    }
    record_event_dictionary(Event::BurnBatch(BurnBatch {
        ids,
        owner,
        amounts,
    }));
}

#[no_mangle]
pub extern "C" fn supply_of() {
    let id: U256 =
        get_named_arg_with_user_errors(ARG_ID, Cep85Error::MissingId, Cep85Error::InvalidId)
            .unwrap_or_revert();
    let total_supply = read_total_supply_of(&id).unwrap_or_default();
    if total_supply.is_zero() {
        runtime::ret(CLValue::from_t::<Option<U256>>(None).unwrap_or_revert());
    }
    let supply: U256 = read_supply_of(&id);
    runtime::ret(CLValue::from_t(Some(supply)).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn total_supply_of() {
    let id: U256 =
        get_named_arg_with_user_errors(ARG_ID, Cep85Error::MissingId, Cep85Error::InvalidId)
            .unwrap_or_revert();
    let total_supply = read_total_supply_of(&id).unwrap_or_default();
    if total_supply.is_zero() {
        runtime::ret(CLValue::from_t::<Option<U256>>(None).unwrap_or_revert());
    }
    runtime::ret(CLValue::from_t(Some(total_supply)).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn set_total_supply_of() {
    sec_check(vec![SecurityBadge::Admin]);

    let id: U256 =
        get_named_arg_with_user_errors(ARG_ID, Cep85Error::MissingId, Cep85Error::InvalidId)
            .unwrap_or_revert();

    let total_supply: U256 = get_named_arg_with_user_errors(
        ARG_TOTAL_SUPPLY,
        Cep85Error::MissingTotalSupply,
        Cep85Error::InvalidTotalSupply,
    )
    .unwrap_or_revert();

    let current_supply: U256 = read_supply_of(&id);

    if total_supply < current_supply {
        runtime::revert(Cep85Error::InvalidTotalSupply);
    }

    write_total_supply_of(&id, &total_supply);
    record_event_dictionary(Event::SetTotalSupply(SetTotalSupply { id, total_supply }));
}

#[no_mangle]
pub extern "C" fn supply_of_batch() {
    let ids: Vec<U256> =
        get_named_arg_with_user_errors(ARG_IDS, Cep85Error::MissingIds, Cep85Error::InvalidIds)
            .unwrap_or_revert();

    let mut batch_supplies = Vec::new();

    for id in ids {
        if read_total_supply_of(&id).unwrap_or_default().is_zero() {
            batch_supplies.push(None);
        } else {
            batch_supplies.push(Some(read_supply_of(&id)));
        }
    }

    runtime::ret(CLValue::from_t(batch_supplies).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn total_supply_of_batch() {
    let ids: Vec<U256> =
        get_named_arg_with_user_errors(ARG_IDS, Cep85Error::MissingIds, Cep85Error::InvalidIds)
            .unwrap_or_revert();

    let mut batch_total_supplies = Vec::new();

    for id in ids {
        let total_supply: U256 = read_total_supply_of(&id).unwrap_or_default();
        if total_supply.is_zero() {
            batch_total_supplies.push(None);
        } else {
            batch_total_supplies.push(Some(total_supply));
        }
    }

    runtime::ret(CLValue::from_t(batch_total_supplies).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn set_total_supply_of_batch() {
    sec_check(vec![SecurityBadge::Admin]);

    let ids: Vec<U256> =
        get_named_arg_with_user_errors(ARG_IDS, Cep85Error::MissingIds, Cep85Error::InvalidIds)
            .unwrap_or_revert();

    let total_supplies: Vec<U256> = get_named_arg_with_user_errors(
        ARG_TOTAL_SUPPLIES,
        Cep85Error::MissingTotalSupplies,
        Cep85Error::InvalidTotalSupplies,
    )
    .unwrap_or_revert();

    if ids.len() != total_supplies.len() {
        runtime::revert(Cep85Error::MismatchParamsLength);
    }

    for (id, total_supply) in ids.into_iter().zip(total_supplies.into_iter()) {
        let current_supply: U256 = read_supply_of(&id);

        if total_supply < current_supply {
            runtime::revert(Cep85Error::InvalidTotalSupply);
        }

        write_total_supply_of(&id, &total_supply);
        record_event_dictionary(Event::SetTotalSupply(SetTotalSupply { id, total_supply }));
    }
}

#[no_mangle]
pub extern "C" fn uri() {
    let id: Option<U256> = get_optional_named_arg_with_user_errors(ARG_ID, Cep85Error::InvalidId);
    if let Some(id) = id {
        let total_supply = read_total_supply_of(&id).unwrap_or_default();
        if total_supply.is_zero() {
            runtime::ret(CLValue::from_t::<Option<String>>(None).unwrap_or_revert());
        }
    }
    let uri: String = read_uri_of(id);
    runtime::ret(CLValue::from_t(Some(uri)).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn set_uri() {
    sec_check(vec![SecurityBadge::Admin, SecurityBadge::Meta]);

    let id: Option<U256> = get_optional_named_arg_with_user_errors(ARG_ID, Cep85Error::InvalidId);

    let uri: String =
        get_named_arg_with_user_errors(ARG_URI, Cep85Error::MissingUri, Cep85Error::InvalidUri)
            .unwrap_or_revert();

    match id {
        None => {
            // Do not save empty string as global uri
            if uri.is_empty() {
                revert(Cep85Error::MissingUri);
            }
            put_key(ARG_URI, storage::new_uref(uri.clone()).into());
            record_event_dictionary(Event::Uri(Uri {
                id: None,
                value: uri,
            }));
        }
        Some(id) => {
            let total_supply = read_total_supply_of(&id).unwrap_or_default();
            if total_supply.is_zero() {
                revert(Cep85Error::NonSuppliedTokenId)
            }
            // Empty string will "delete" dictionary row query for next read_uri_of()
            write_uri_of(&id, &uri);
            record_event_dictionary(Event::Uri(Uri {
                id: Some(id),
                value: uri,
            }));
        }
    };
}

#[no_mangle]
pub extern "C" fn is_non_fungible() {
    let id: U256 =
        get_named_arg_with_user_errors(ARG_ID, Cep85Error::MissingId, Cep85Error::InvalidId)
            .unwrap_or_revert();
    let total_supply = read_total_supply_of(&id).unwrap_or_default();
    if total_supply.is_zero() {
        runtime::ret(CLValue::from_t::<Option<bool>>(None).unwrap_or_revert());
    }
    let is_non_fungible: bool = U256::from(1_u32) == total_supply;
    runtime::ret(CLValue::from_t(Some(is_non_fungible)).unwrap_or_revert());
}

/// Calculates the difference between the total supply and the circulating supply of a token.
/// If the token is a non-fungible token (NFT), or if total supply has been reached returns 0.
#[no_mangle]
pub extern "C" fn total_fungible_supply() {
    let id: U256 =
        get_named_arg_with_user_errors(ARG_ID, Cep85Error::MissingId, Cep85Error::InvalidId)
            .unwrap_or_revert();
    let total_supply = read_total_supply_of(&id).unwrap_or_default();
    if total_supply.is_zero() {
        runtime::ret(CLValue::from_t::<Option<U256>>(None).unwrap_or_revert());
    }

    let current_supply = read_supply_of(&id);

    let total_fungible_supply = if total_supply >= current_supply {
        total_supply
            .checked_sub(current_supply)
            .unwrap_or(U256::zero())
    } else {
        U256::zero()
    };
    runtime::ret(CLValue::from_t(Some(total_fungible_supply)).unwrap_or_revert());
}

/// Admin EntryPoint to manipulate the security access granted to users.
/// One user can only possess one access group badge.
/// Change strength: None > Admin > Minter
/// Change strength meaning by example: If user is added to both Minter and Admin they will be an
/// Admin, also if a user is added to Admin and None then they will be removed from having rights.
/// Beware: do not remove the last Admin because that will lock out all admin functionality.
#[no_mangle]
pub extern "C" fn change_security() {
    sec_check(vec![SecurityBadge::Admin]);

    let admin_list: Option<Vec<Key>> =
        get_optional_named_arg_with_user_errors(ADMIN_LIST, Cep85Error::InvalidAdminList);
    let minter_list: Option<Vec<Key>> =
        get_optional_named_arg_with_user_errors(MINTER_LIST, Cep85Error::InvalidMinterList);
    let meta_list: Option<Vec<Key>> =
        get_optional_named_arg_with_user_errors(META_LIST, Cep85Error::InvalidMetaList);
    let none_list: Option<Vec<Key>> =
        get_optional_named_arg_with_user_errors(NONE_LIST, Cep85Error::InvalidNoneList);

    let mut badge_map: BTreeMap<Key, SecurityBadge> = BTreeMap::new();
    if get_stored_value_with_user_errors::<bool>(
        ARG_ENABLE_BURN,
        Cep85Error::MissingEnableMBFlag,
        Cep85Error::InvalidEnableBurnFlag,
    ) {
        let burner_list: Option<Vec<Key>> =
            get_optional_named_arg_with_user_errors(BURNER_LIST, Cep85Error::InvalidBurnerList);
        if let Some(burner_list) = burner_list {
            for account_key in burner_list {
                badge_map.insert(account_key, SecurityBadge::Burner);
            }
        }
    };

    if let Some(minter_list) = minter_list {
        for account_key in minter_list {
            badge_map.insert(account_key, SecurityBadge::Minter);
        }
    }

    if let Some(meta_list) = meta_list {
        for account_key in meta_list {
            badge_map.insert(account_key, SecurityBadge::Meta);
        }
    }
    if let Some(admin_list) = admin_list {
        for account_key in admin_list {
            badge_map.insert(account_key, SecurityBadge::Admin);
        }
    }
    if let Some(none_list) = none_list {
        for account_key in none_list {
            badge_map.insert(account_key, SecurityBadge::None);
        }
    }

    let (caller, _) = get_verified_caller();
    badge_map.remove(&caller);

    change_sec_badge(&badge_map);
    record_event_dictionary(Event::ChangeSecurity(ChangeSecurity {
        admin: caller,
        sec_change_map: badge_map,
    }));
}

// set_modalities allows the user to set any variable or any combination of variables
// simultaneously.
#[no_mangle]
pub extern "C" fn set_modalities() {
    // Only the installing account can change the mutable variables.
    sec_check(vec![SecurityBadge::Admin]);

    if let Some(enable_burn) = get_optional_named_arg_with_user_errors::<bool>(
        ARG_ENABLE_BURN,
        Cep85Error::InvalidEventsMode,
    ) {
        runtime::put_key(ARG_ENABLE_BURN, storage::new_uref(enable_burn).into());
        record_event_dictionary(Event::SetModalities(SetModalities {}));
        record_event_dictionary(Event::ChangeEnableBurnMode(ChangeEnableBurnMode {
            enable_burn,
        }));
    }

    if let Some(events_mode) = get_optional_named_arg_with_user_errors::<u8>(
        ARG_EVENTS_MODE,
        Cep85Error::InvalidEventsMode,
    ) {
        runtime::put_key(ARG_EVENTS_MODE, storage::new_uref(events_mode).into());
        match EventsMode::try_from(events_mode).unwrap_or_revert_with(Cep85Error::InvalidEventsMode)
        {
            EventsMode::NoEvents => {}
            EventsMode::CES => init_events(),
            EventsMode::Native => {
                let _ = manage_message_topic(EVENTS, MessageTopicOperation::Add);
            }
            EventsMode::NativeNCES => {
                init_events();
                let _ = manage_message_topic(EVENTS, MessageTopicOperation::Add);
            }
        };
        record_event_dictionary(Event::SetModalities(SetModalities {}));
        record_event_dictionary(Event::ChangeEventsMode(ChangeEventsMode { events_mode }));
    }
}

#[no_mangle]
pub extern "C" fn upgrade() {
    put_key(
        ARG_CONTRACT_HASH,
        get_named_arg_with_user_errors::<Key>(
            ARG_CONTRACT_HASH,
            Cep85Error::MissingContractHash,
            Cep85Error::InvalidContractHash,
        )
        .unwrap_or_revert(),
    );
    record_event_dictionary(Event::Upgrade(Upgrade {}));
}

fn install_contract() {
    let name: String = get_named_arg_with_user_errors(
        ARG_NAME,
        Cep85Error::MissingCollectionName,
        Cep85Error::InvalidCollectionName,
    )
    .unwrap_or_revert();

    let uri: String = get_named_arg_with_user_errors(
        ARG_URI,
        Cep85Error::MissingUri,
        Cep85Error::InvalidCollectionName,
    )
    .unwrap_or_revert();

    let events_mode: u8 =
        get_optional_named_arg_with_user_errors(ARG_EVENTS_MODE, Cep85Error::InvalidEventsMode)
            .unwrap_or_default();

    let enable_burn: bool =
        get_optional_named_arg_with_user_errors(ARG_ENABLE_BURN, Cep85Error::InvalidEnableBurnFlag)
            .unwrap_or_default();

    let transfer_filter_contract_key: Option<Key> = get_optional_named_arg_with_user_errors(
        ARG_TRANSFER_FILTER_CONTRACT,
        Cep85Error::InvalidTransferFilterContract,
    );

    let transfer_filter_method: Option<String> = get_optional_named_arg_with_user_errors(
        ARG_TRANSFER_FILTER_METHOD,
        Cep85Error::InvalidTransferFilterMethod,
    );

    if let Some(_contract_key) = transfer_filter_contract_key {
        if transfer_filter_method.is_none() || transfer_filter_method.as_ref().unwrap().is_empty() {
            revert(Cep85Error::InvalidTransferFilterMethod);
        }
    }

    let mut named_keys = NamedKeys::new();
    named_keys.insert(ARG_NAME.to_string(), storage::new_uref(name.clone()).into());
    named_keys.insert(ARG_URI.to_string(), storage::new_uref(uri).into());
    named_keys.insert(
        ARG_EVENTS_MODE.to_string(),
        storage::new_uref(events_mode).into(),
    );
    named_keys.insert(
        ARG_ENABLE_BURN.to_string(),
        storage::new_uref(enable_burn).into(),
    );

    let entry_points = generate_entry_points();

    let package_key_name = format!("{PREFIX_CONTRACT_PACKAGE_NAME}_{name}");
    let mut message_topics = BTreeMap::new();
    let message_topics = if EventsMode::Native == events_mode.try_into().unwrap_or_default() {
        message_topics.insert(EVENTS.to_string(), MessageTopicOperation::Add);
        Some(message_topics)
    } else {
        None
    };

    let (contract_hash, contract_version) = storage::new_contract(
        entry_points,
        Some(named_keys),
        Some(package_key_name.clone()),
        Some(format!("{PREFIX_ACCESS_KEY_NAME}_{name}")),
        message_topics,
    );

    let contract_hash_key =
        Key::AddressableEntity(EntityAddr::SmartContract(contract_hash.value()));

    runtime::put_key(&format!("{PREFIX_CONTRACT_NAME}_{name}"), contract_hash_key);
    runtime::put_key(
        &format!("{PREFIX_CONTRACT_VERSION}_{name}"),
        storage::new_uref(contract_version).into(),
    );

    let package_hash_key = runtime::get_key(&package_key_name).unwrap_or_revert();

    // Call contract to initialize it
    let mut init_args = runtime_args! {
        ARG_CONTRACT_HASH => contract_hash_key,
        ARG_PACKAGE_HASH => package_hash_key,
        ARG_TRANSFER_FILTER_CONTRACT => transfer_filter_contract_key,
        ARG_TRANSFER_FILTER_METHOD => transfer_filter_method
    };

    let admin_list: Option<Vec<Key>> =
        get_optional_named_arg_with_user_errors(ADMIN_LIST, Cep85Error::InvalidAdminList);
    let minter_list: Option<Vec<Key>> =
        get_optional_named_arg_with_user_errors(MINTER_LIST, Cep85Error::InvalidMinterList);
    let burner_list: Option<Vec<Key>> =
        get_optional_named_arg_with_user_errors(BURNER_LIST, Cep85Error::InvalidBurnerList);
    let meta_list: Option<Vec<Key>> =
        get_optional_named_arg_with_user_errors(META_LIST, Cep85Error::InvalidMetaList);
    let none_list: Option<Vec<Key>> =
        get_optional_named_arg_with_user_errors(NONE_LIST, Cep85Error::InvalidNoneList);

    if let Some(admin_list) = admin_list {
        init_args
            .insert(ADMIN_LIST, admin_list)
            .unwrap_or_revert_with(Cep85Error::FailedToInsertToSecurityList);
    }
    if let Some(minter_list) = minter_list {
        init_args
            .insert(MINTER_LIST, minter_list)
            .unwrap_or_revert_with(Cep85Error::FailedToInsertToSecurityList);
    }
    if let Some(burner_list) = burner_list {
        init_args
            .insert(BURNER_LIST, burner_list)
            .unwrap_or_revert_with(Cep85Error::FailedToInsertToSecurityList);
    }
    if let Some(meta_list) = meta_list {
        init_args
            .insert(META_LIST, meta_list)
            .unwrap_or_revert_with(Cep85Error::FailedToInsertToSecurityList);
    }
    if let Some(none_list) = none_list {
        init_args
            .insert(NONE_LIST, none_list)
            .unwrap_or_revert_with(Cep85Error::FailedToInsertToSecurityList);
    }

    runtime::call_contract::<()>(contract_hash, ENTRY_POINT_INIT, init_args);
}

fn upgrade_contract(name: &str, contract_package_hash: Key) {
    let (contract_hash, contract_version) = storage::add_contract_version(
        contract_package_hash
            .into_package_hash()
            .unwrap_or_revert_with(Cep85Error::InvalidPackageHash),
        generate_entry_points(),
        NamedKeys::new(),
        BTreeMap::new(),
    );

    let contract_hash_key =
        Key::AddressableEntity(EntityAddr::SmartContract(contract_hash.value()));

    runtime::put_key(&format!("{PREFIX_CONTRACT_NAME}_{name}"), contract_hash_key);
    runtime::put_key(
        &format!("{PREFIX_CONTRACT_VERSION}_{name}"),
        storage::new_uref(contract_version).into(),
    );

    let runtime_args = runtime_args! {
        ARG_CONTRACT_HASH => contract_hash_key,
    };

    runtime::call_contract::<()>(contract_hash, ENTRY_POINT_UPGRADE, runtime_args);
}

fn before_token_transfer(
    operator: &Key,
    from: &Key,
    to: &Key,
    ids: &[U256],
    amounts: &[U256],
    data: Option<Bytes>,
) {
    if amounts.len() != ids.len() {
        runtime::revert(Cep85Error::MismatchParamsLength);
    }

    for amount in amounts {
        if *amount == U256::zero() {
            runtime::revert(Cep85Error::InvalidAmount);
        }
    }

    if let Some(filter_contract) = get_transfer_filter_contract() {
        if let Some(filter_method) = get_transfer_filter_method() {
            let mut args = RuntimeArgs::new();
            args.insert(ARG_OPERATOR, *operator)
                .unwrap_or_revert_with(Cep85Error::FailedToCreateArg);
            args.insert(ARG_FROM, *from)
                .unwrap_or_revert_with(Cep85Error::FailedToCreateArg);
            args.insert(ARG_TO, *to)
                .unwrap_or_revert_with(Cep85Error::FailedToCreateArg);
            args.insert(ARG_IDS, ids.to_owned())
                .unwrap_or_revert_with(Cep85Error::FailedToCreateArg);
            args.insert(ARG_AMOUNTS, amounts.to_owned())
                .unwrap_or_revert_with(Cep85Error::FailedToCreateArg);
            args.insert(ARG_DATA, data)
                .unwrap_or_revert_with(Cep85Error::FailedToCreateArg);

            let result: TransferFilterContractResult =
                call_contract::<u8>(filter_contract, &filter_method, args).into();

            if TransferFilterContractResult::DenyTransfer == result {
                revert(Cep85Error::TransferFilterContractDenied);
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn call() {
    let upgrade_flag: Option<bool> =
        get_optional_named_arg_with_user_errors(ARG_UPGRADE_FLAG, Cep85Error::InvalidUpgradeFlag);

    if upgrade_flag.is_some() && upgrade_flag.unwrap() {
        let name: String =
            get_optional_named_arg_with_user_errors(ARG_NAME, Cep85Error::MissingCollectionName)
                .unwrap_or_revert_with(Cep85Error::InvalidCollectionName);
        let contract_package_hash: Key =
            get_key(&format!("{PREFIX_CONTRACT_PACKAGE_NAME}_{}", name))
                .unwrap_or_revert_with(Cep85Error::MissingPackageHash);
        upgrade_contract(&name, contract_package_hash)
    } else {
        install_contract()
    }
}
