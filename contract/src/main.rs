#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;

use alloc::{
    format,
    string::{String, ToString},
    vec::Vec,
};
use casper_contract::{
    contract_api::{
        runtime::{self, get_key, put_key, revert},
        storage,
    },
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    contracts::NamedKeys, runtime_args, CLValue, ContractHash, Key, RuntimeArgs, U256,
};
use cep1155::{
    self,
    balances::{batch_transfer_balance, read_balance_from, transfer_balance, write_balance_to},
    constants::{
        ARG_ACCOUNT, ARG_ACCOUNTS, ARG_AMOUNT, ARG_AMOUNTS, ARG_APPROVED, ARG_DATA, ARG_FROM,
        ARG_ID, ARG_IDS, ARG_OPERATOR, ARG_OWNER, ARG_RECIPIENT, ARG_TO, BALANCES, CONTRACT_HASH,
        ENABLE_MINT_BURN, ENTRY_POINT_INIT, EVENTS_MODE, NAME, OPERATORS, PACKAGE_HASH,
        PREFIX_ACCESS_KEY_NAME, PREFIX_CONTRACT_NAME, PREFIX_CONTRACT_PACKAGE_NAME,
        PREFIX_CONTRACT_VERSION, TRANSFER_FILTER_CONTRACT,
    },
    entry_points::generate_entry_points,
    error::Cep1155Error,
    events::{self, init_events, ApprovalForAll, Burn, Event, Mint, TransferBatch, TransferSingle},
    modalities::TokenIdentifier,
    operators::{read_operator, write_operator},
    utils::{
        get_named_arg_with_user_errors, get_optional_named_arg_with_user_errors,
        get_stored_value_with_user_errors, get_token_id_from_identifier_mode,
    },
};

/// Initiates the contracts states. Only used by the installer call,
/// later calls will cause it to revert.
#[no_mangle]
pub extern "C" fn init() {
    // TODO Change to admin check
    if get_key(PACKAGE_HASH).is_some() {
        revert(Cep1155Error::ContractAlreadyInitialized);
    }

    put_key(
        PACKAGE_HASH,
        get_named_arg_with_user_errors::<Key>(
            PACKAGE_HASH,
            Cep1155Error::MissingPackageHash,
            Cep1155Error::InvalidPackageHash,
        )
        .unwrap_or_revert(),
    );

    put_key(
        CONTRACT_HASH,
        get_named_arg_with_user_errors::<Key>(
            CONTRACT_HASH,
            Cep1155Error::MissingPackageHash,
            Cep1155Error::InvalidPackageHash,
        )
        .unwrap_or_revert(),
    );

    let transfer_filter_contract_key: Option<Key> =
        get_optional_named_arg_with_user_errors::<Option<Key>>(
            TRANSFER_FILTER_CONTRACT,
            Cep1155Error::InvalidTransferFilterContract,
        )
        .unwrap_or_default();

    let transfer_filter_contract: Option<ContractHash> =
        transfer_filter_contract_key.map(|transfer_filter_contract_key| {
            ContractHash::from(transfer_filter_contract_key.into_hash().unwrap_or_revert())
        });

    runtime::put_key(
        TRANSFER_FILTER_CONTRACT,
        storage::new_uref(transfer_filter_contract).into(),
    );

    storage::new_dictionary(BALANCES).unwrap_or_revert_with(Cep1155Error::FailedToCreateDictionary);
    storage::new_dictionary(OPERATORS)
        .unwrap_or_revert_with(Cep1155Error::FailedToCreateDictionary);

    init_events();
}

#[no_mangle]
pub extern "C" fn balance_of() {
    let account: Key = get_named_arg_with_user_errors(
        ARG_ACCOUNT,
        Cep1155Error::MissingAccount,
        Cep1155Error::InvalidAccount,
    )
    .unwrap_or_revert();
    let id: U256 =
        get_named_arg_with_user_errors(ARG_ID, Cep1155Error::MissingId, Cep1155Error::InvalidId)
            .unwrap_or_revert();

    let token_id: TokenIdentifier = get_token_id_from_identifier_mode(&id);
    let balance: U256 = read_balance_from(&account, &token_id);
    runtime::ret(CLValue::from_t(balance).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn balance_of_batch() {
    let accounts: Vec<Key> = get_named_arg_with_user_errors(
        ARG_ACCOUNTS,
        Cep1155Error::MissingAccounts,
        Cep1155Error::InvalidAccounts,
    )
    .unwrap_or_revert();
    let ids: Vec<U256> =
        get_named_arg_with_user_errors(ARG_IDS, Cep1155Error::MissingIds, Cep1155Error::InvalidIds)
            .unwrap_or_revert();

    if accounts.len() != ids.len() {
        runtime::revert(Cep1155Error::MismatchParamsLength);
    }

    let mut batch_balances = Vec::new();

    for i in 0..accounts.len() {
        let token_id: TokenIdentifier = get_token_id_from_identifier_mode(&ids[i]);
        let balance: U256 = read_balance_from(&accounts[i], &token_id);
        batch_balances.push(balance);
    }

    runtime::ret(CLValue::from_t(batch_balances).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn is_approved_for_all() {
    let owner: Key = get_named_arg_with_user_errors(
        ARG_OWNER,
        Cep1155Error::MissingOwner,
        Cep1155Error::InvalidOwner,
    )
    .unwrap_or_revert();

    let operator: Key = get_named_arg_with_user_errors(
        ARG_OPERATOR,
        Cep1155Error::MissingOperator,
        Cep1155Error::InvalidOperator,
    )
    .unwrap_or_revert();

    let is_approved_for_all: bool = read_operator(&owner, &operator);

    runtime::ret(CLValue::from_t(is_approved_for_all).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn set_approval_for_all() {
    let operator: Key = get_named_arg_with_user_errors(
        ARG_OPERATOR,
        Cep1155Error::MissingOperator,
        Cep1155Error::InvalidOperator,
    )
    .unwrap_or_revert();

    // TODO get_verified_caller
    let caller = Key::from(runtime::get_caller());

    // If caller tries to approve itself as operator that's probably a mistake and we revert.
    if caller == operator {
        runtime::revert(Cep1155Error::SelfOperatorApproveal);
    }

    let approved: bool = get_named_arg_with_user_errors(
        ARG_APPROVED,
        Cep1155Error::MissingOperator,
        Cep1155Error::InvalidOperator,
    )
    .unwrap_or_revert();

    write_operator(&caller, &operator, approved);
    events::record_event_dictionary(Event::ApprovalForAll(ApprovalForAll {
        owner: caller,
        operator,
        approved,
    }));
}

/// Transfer a specified amount of tokens from the `sender` to the `recipient`.
///
/// This function should only be called by an approved operator or by the sender themselves.
#[no_mangle]
pub extern "C" fn safe_transfer_from() {
    let from: Key = get_named_arg_with_user_errors(
        ARG_FROM,
        Cep1155Error::MissingFrom,
        Cep1155Error::InvalidFrom,
    )
    .unwrap_or_revert();

    // TODO get_verified_caller ?
    let caller = Key::from(runtime::get_caller());

    // Check if the caller is the spender or an operator
    let is_approved: bool = read_operator(&from, &caller);
    if from != caller && !is_approved {
        runtime::revert(Cep1155Error::NotApproved);
    }

    let to: Key =
        get_named_arg_with_user_errors(ARG_TO, Cep1155Error::MissingTo, Cep1155Error::InvalidTo)
            .unwrap_or_revert();

    let id: U256 =
        get_named_arg_with_user_errors(ARG_ID, Cep1155Error::MissingId, Cep1155Error::InvalidId)
            .unwrap_or_revert();
    let token_id: TokenIdentifier = get_token_id_from_identifier_mode(&id);

    let amount: U256 = get_named_arg_with_user_errors(
        ARG_AMOUNT,
        Cep1155Error::MissingAmount,
        Cep1155Error::InvalidAmount,
    )
    .unwrap_or_revert();

    // TODO
    let _data: Vec<u8> = get_named_arg_with_user_errors(
        ARG_DATA,
        Cep1155Error::MissingData,
        Cep1155Error::InvalidData,
    )
    .unwrap_or_revert();

    transfer_balance(&from, &to, &token_id, &amount)
        .unwrap_or_revert_with(Cep1155Error::FailToTransferBalance);
    events::record_event_dictionary(Event::TransferSingle(TransferSingle {
        operator: caller,
        from,
        to,
        id: token_id,
        value: amount,
    }));
}

/// Batch transfer specified amounts of multiple tokens from the `sender` to the `recipient`.
///
/// This function should only be called by an approved operator or by the sender themselves.
#[no_mangle]
pub extern "C" fn safe_batch_transfer_from() {
    let ids: Vec<U256> =
        get_named_arg_with_user_errors(ARG_IDS, Cep1155Error::MissingIds, Cep1155Error::InvalidIds)
            .unwrap_or_revert();

    let amounts: Vec<U256> = get_named_arg_with_user_errors(
        ARG_AMOUNTS,
        Cep1155Error::MissingAmounts,
        Cep1155Error::InvalidAmounts,
    )
    .unwrap_or_revert();

    if ids.len() != amounts.len() {
        runtime::revert(Cep1155Error::MismatchParamsLength);
    }

    let from: Key = get_named_arg_with_user_errors(
        ARG_FROM,
        Cep1155Error::MissingFrom,
        Cep1155Error::InvalidFrom,
    )
    .unwrap_or_revert();

    // TODO get_verified_caller ?
    let caller = Key::from(runtime::get_caller());

    // Check if the caller is the spender or an operator
    let is_approved: bool = read_operator(&from, &caller);
    if from != caller && !is_approved {
        runtime::revert(Cep1155Error::NotApproved);
    }

    let mut token_ids: Vec<TokenIdentifier> = Vec::new();
    for id in &ids {
        let token_id: TokenIdentifier = get_token_id_from_identifier_mode(&id);
        token_ids.push(token_id)
    }

    // TODO
    let _data: Vec<u8> = get_named_arg_with_user_errors(
        ARG_DATA,
        Cep1155Error::MissingData,
        Cep1155Error::InvalidData,
    )
    .unwrap_or_revert();

    let to: Key =
        get_named_arg_with_user_errors(ARG_TO, Cep1155Error::MissingTo, Cep1155Error::InvalidTo)
            .unwrap_or_revert();

    batch_transfer_balance(&from, &to, &token_ids, &amounts)
        .unwrap_or_revert_with(Cep1155Error::FailToBatchTransferBalance);

    events::record_event_dictionary(Event::TransferBatch(TransferBatch {
        operator: caller,
        from,
        to,
        ids: token_ids,
        values: amounts,
    }));
}

#[no_mangle]
pub extern "C" fn mint() {
    if 0 == get_stored_value_with_user_errors::<u8>(
        ENABLE_MINT_BURN,
        Cep1155Error::MissingEnableMBFlag,
        Cep1155Error::InvalidEnableMBFlag,
    ) {
        revert(Cep1155Error::MintBurnDisabled);
    };

    // TODO ADMIN
    // sec_check(vec![SecurityBadge::Admin, SecurityBadge::Minter]);

    let recipient: Key = get_named_arg_with_user_errors(
        ARG_RECIPIENT,
        Cep1155Error::MissingRecipient,
        Cep1155Error::InvalidRecipient,
    )
    .unwrap_or_revert();

    let id: U256 =
        get_named_arg_with_user_errors(ARG_ID, Cep1155Error::MissingId, Cep1155Error::InvalidId)
            .unwrap_or_revert();
    let token_id: TokenIdentifier = get_token_id_from_identifier_mode(&id);

    let amount: U256 = get_named_arg_with_user_errors(
        ARG_AMOUNT,
        Cep1155Error::MissingAmount,
        Cep1155Error::InvalidAmount,
    )
    .unwrap_or_revert();

    // TODO check if token_id already exists

    let recipient_balance = read_balance_from(&recipient, &token_id);
    let new_recipient_balance = recipient_balance.checked_add(amount).unwrap_or_revert();
    write_balance_to(&recipient, &token_id, &new_recipient_balance);

    events::record_event_dictionary(Event::Mint(Mint {
        id: token_id,
        recipient,
        amount,
    }))
}

#[no_mangle]
pub extern "C" fn batch_mint() {
    if 0_u8
        == get_stored_value_with_user_errors::<u8>(
            ENABLE_MINT_BURN,
            Cep1155Error::MissingEnableMBFlag,
            Cep1155Error::InvalidEnableMBFlag,
        )
    {
        revert(Cep1155Error::MintBurnDisabled);
    };

    // TODO ADMIN
    // sec_check(vec![SecurityBadge::Admin, SecurityBadge::Minter]);

    let recipient: Key = get_named_arg_with_user_errors(
        ARG_RECIPIENT,
        Cep1155Error::MissingRecipient,
        Cep1155Error::InvalidRecipient,
    )
    .unwrap_or_revert();

    let ids: Vec<U256> =
        get_named_arg_with_user_errors(ARG_IDS, Cep1155Error::MissingIds, Cep1155Error::InvalidIds)
            .unwrap_or_revert();

    let amounts: Vec<U256> = get_named_arg_with_user_errors(
        ARG_AMOUNTS,
        Cep1155Error::MissingAmounts,
        Cep1155Error::InvalidAmounts,
    )
    .unwrap_or_revert();

    // Vérifier si les vecteurs ids et amounts ont la même longueur
    if ids.len() != amounts.len() {
        revert(Cep1155Error::MismatchParamsLength);
    }

    // Parcourir les vecteurs ids et amounts et effectuer les mintings
    for (i, &id) in ids.iter().enumerate() {
        let token_id: TokenIdentifier = get_token_id_from_identifier_mode(&id);
        let amount = amounts[i];

        // TODO check if token_id already exists

        let recipient_balance = read_balance_from(&recipient, &token_id);
        let new_recipient_balance = recipient_balance.checked_add(amount).unwrap_or_revert();
        write_balance_to(&recipient, &token_id, &new_recipient_balance);

        events::record_event_dictionary(Event::Mint(Mint {
            id: token_id,
            recipient,
            amount,
        }));
    }
}

#[no_mangle]
pub extern "C" fn burn() {
    if 0 == get_stored_value_with_user_errors::<u8>(
        ENABLE_MINT_BURN,
        Cep1155Error::MissingEnableMBFlag,
        Cep1155Error::InvalidEnableMBFlag,
    ) {
        revert(Cep1155Error::MintBurnDisabled);
    };

    // TODO ADMIN
    // sec_check(vec![SecurityBadge::Admin, SecurityBadge::Burner]);

    let owner: Key = get_named_arg_with_user_errors(
        ARG_OWNER,
        Cep1155Error::MissingOwner,
        Cep1155Error::InvalidOwner,
    )
    .unwrap_or_revert();

    let id: U256 =
        get_named_arg_with_user_errors(ARG_ID, Cep1155Error::MissingId, Cep1155Error::InvalidId)
            .unwrap_or_revert();
    let token_id: TokenIdentifier = get_token_id_from_identifier_mode(&id);

    let amount: U256 = get_named_arg_with_user_errors(
        ARG_AMOUNT,
        Cep1155Error::MissingAmount,
        Cep1155Error::InvalidAmount,
    )
    .unwrap_or_revert();

    let owner_balance = read_balance_from(&owner, &token_id);
    let new_owner_balance = owner_balance.checked_sub(amount).unwrap_or_revert();
    write_balance_to(&owner, &token_id, &new_owner_balance);

    events::record_event_dictionary(Event::Burn(Burn {
        id: token_id,
        owner,
        amount,
    }));
}

#[no_mangle]
pub extern "C" fn batch_burn() {
    if 0 == get_stored_value_with_user_errors::<u8>(
        ENABLE_MINT_BURN,
        Cep1155Error::MissingEnableMBFlag,
        Cep1155Error::InvalidEnableMBFlag,
    ) {
        revert(Cep1155Error::MintBurnDisabled);
    };

    // TODO ADMIN
    // sec_check(vec![SecurityBadge::Admin, SecurityBadge::Burner]);

    let owner: Key = get_named_arg_with_user_errors(
        ARG_OWNER,
        Cep1155Error::MissingOwner,
        Cep1155Error::InvalidOwner,
    )
    .unwrap_or_revert();

    let ids: Vec<U256> =
        get_named_arg_with_user_errors(ARG_IDS, Cep1155Error::MissingIds, Cep1155Error::InvalidIds)
            .unwrap_or_revert();

    let amounts: Vec<U256> = get_named_arg_with_user_errors(
        ARG_AMOUNTS,
        Cep1155Error::MissingAmounts,
        Cep1155Error::InvalidAmounts,
    )
    .unwrap_or_revert();

    if ids.len() != amounts.len() {
        revert(Cep1155Error::MismatchParamsLength);
    }

    for (i, id) in ids.iter().enumerate() {
        let amount = amounts[i];
        let token_id: TokenIdentifier = get_token_id_from_identifier_mode(&id);
        let owner_balance = read_balance_from(&owner, &token_id);
        let new_owner_balance = owner_balance.checked_sub(amount).unwrap_or_revert();
        write_balance_to(&owner, &token_id, &new_owner_balance);

        events::record_event_dictionary(Event::Burn(Burn {
            id: token_id,
            owner,
            amount,
        }));
    }
}

fn install_contract() {
    let name: String = get_named_arg_with_user_errors(
        NAME,
        Cep1155Error::MissingCollectionName,
        Cep1155Error::InvalidCollectionName,
    )
    .unwrap_or_revert();

    let events_mode: u8 =
        get_optional_named_arg_with_user_errors(EVENTS_MODE, Cep1155Error::InvalidEventsMode)
            .unwrap_or_default();

    let enable_mint_burn: u8 = get_optional_named_arg_with_user_errors(
        ENABLE_MINT_BURN,
        Cep1155Error::InvalidEnableMBFlag,
    )
    .unwrap_or_default();

    let transfer_filter_contract_key: Option<Key> = get_optional_named_arg_with_user_errors(
        TRANSFER_FILTER_CONTRACT,
        Cep1155Error::InvalidTransferFilterContract,
    );

    let mut named_keys = NamedKeys::new();
    named_keys.insert(NAME.to_string(), storage::new_uref(name.clone()).into());
    named_keys.insert(
        EVENTS_MODE.to_string(),
        storage::new_uref(events_mode).into(),
    );
    named_keys.insert(
        ENABLE_MINT_BURN.to_string(),
        storage::new_uref(enable_mint_burn).into(),
    );

    let entry_points = generate_entry_points();

    let package_key_name = format!("{PREFIX_CONTRACT_PACKAGE_NAME}_{name}");

    let (contract_hash, contract_version) = storage::new_contract(
        entry_points,
        Some(named_keys),
        Some(package_key_name.clone()),
        Some(format!("{PREFIX_ACCESS_KEY_NAME}_{name}")),
    );

    let contract_hash_key = Key::from(contract_hash);

    runtime::put_key(&format!("{PREFIX_CONTRACT_NAME}_{name}"), contract_hash_key);
    runtime::put_key(
        &format!("{PREFIX_CONTRACT_VERSION}_{name}"),
        storage::new_uref(contract_version).into(),
    );

    let package_hash_key = runtime::get_key(&package_key_name).unwrap_or_revert();

    // Call contract to initialize it
    let init_args = runtime_args! {
        CONTRACT_HASH => contract_hash_key,
        PACKAGE_HASH => package_hash_key,
    };

    runtime::call_contract::<()>(contract_hash, ENTRY_POINT_INIT, init_args);
}

fn before_token_transfer(
    operator: Key,
    from: Key,
    to: Key,
    ids: Vec<TokenIdentifier>,
    amounts: Vec<U256>,
    data: Vec<u8>,
) {
    if amounts.len() != ids.len() {
        runtime::revert(Cep1155Error::MismatchParamsLength);
    }

    for amount in amounts {
        if amount == U256::zero() {
            runtime::revert(Cep1155Error::InvalidAmount);
        }
    }
}

#[no_mangle]
pub extern "C" fn call() {
    install_contract()
}
