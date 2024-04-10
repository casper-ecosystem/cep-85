// //! Implementation of balances.

use alloc::vec::Vec;
use casper_contract::{
    contract_api::runtime::{self, get_key},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{Key, U256};

use crate::{
    constants::{ARG_CONTRACT_HASH, DICT_BALANCES},
    error::Cep85Error,
    utils::{
        get_dictionary_value_from_key, make_dictionary_item_key, set_dictionary_value_for_key,
    },
};

/// Writes token balance of a specified account into a dictionary.
pub fn write_balance_to(account: &Key, id: &U256, amount: &U256) {
    set_dictionary_value_for_key(
        DICT_BALANCES,
        &make_dictionary_item_key(account, id),
        amount,
    )
}

/// Reads token balance of a specified account.
///
/// If a given account does not have balances in the system, then a 0 is returned.
pub fn read_balance_from(account: &Key, id: &U256) -> U256 {
    get_dictionary_value_from_key(DICT_BALANCES, &make_dictionary_item_key(account, id))
        .unwrap_or_default()
}

/// Transfer tokens from the `sender` to the `recipient`.
///
/// This function should not be used directly by contract's entrypoint as it does not validate
// the sender.
pub fn transfer_balance(sender: &Key, recipient: &Key, id: &U256, amount: &U256) {
    if amount.is_zero() {
        runtime::revert(Cep85Error::InvalidAmount);
    }

    if sender == recipient {
        runtime::revert(Cep85Error::SelfTransfer);
    }

    // Check if the recipient is a an account or a contract
    if (*recipient).into_account().is_none() && (*recipient).into_hash().is_none() {
        runtime::revert(Cep85Error::InvalidRecipient);
    }

    // Check if the recipient is the contract address
    let contract_key =
        get_key(ARG_CONTRACT_HASH).unwrap_or_revert_with(Cep85Error::MissingContractHash);
    if &contract_key == recipient {
        runtime::revert(Cep85Error::InvalidRecipient);
    }

    let new_sender_balance = {
        let sender_balance = read_balance_from(sender, id);
        sender_balance
            .checked_sub(*amount)
            .unwrap_or_revert_with(Cep85Error::InsufficientBalance)
    };

    let new_recipient_balance = {
        let recipient_balance = read_balance_from(recipient, id);
        recipient_balance
            .checked_add(*amount)
            .unwrap_or_revert_with(Cep85Error::Overflow)
    };

    write_balance_to(sender, id, &new_sender_balance);
    write_balance_to(recipient, id, &new_recipient_balance);
}

/// Transfer multiple tokens from the `sender` to the `recipient`.
///
/// This function performs the batch transfer logic by calling `transfer_balance` for each token.
pub fn batch_transfer_balance(
    sender: &Key,
    recipient: &Key,
    ids: &Vec<U256>,
    amounts: &Vec<U256>,
) -> Result<(), Cep85Error> {
    if sender == recipient {
        runtime::revert(Cep85Error::SelfTransfer);
    }

    if ids.len() != amounts.len() {
        runtime::revert(Cep85Error::MismatchParamsLength);
    }

    for (i, &id) in ids.iter().enumerate() {
        if let Some(&amount) = amounts.get(i) {
            if amount.is_zero() {
                continue;
            }

            transfer_balance(sender, recipient, &id, &amount);
        } else {
            runtime::revert(Cep85Error::MismatchParamsLength);
        }
    }

    Ok(())
}
