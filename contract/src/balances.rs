// //! Implementation of balances.



use casper_types::{URef};

use crate::{constants::BALANCES, utils};

/// Getter for the "balances" dictionary URef.
pub fn get_balances_uref() -> URef {
    utils::get_uref(BALANCES)
}

// /// Writes token balance of a specified account into a dictionary.
// pub(crate) fn write_balance_to(balances_uref: URef, address: Key, amount: U256) {
//     let dictionary_item_key = make_dictionary_item_key(address);
//     storage::dictionary_put(balances_uref, &dictionary_item_key, amount);
// }

// /// Reads token balance of a specified account.
// ///
// /// If a given account does not have balances in the system, then a 0 is returned.
// pub(crate) fn read_balance_from(balances_uref: URef, address: Key) -> U256 {
//     let dictionary_item_key = make_dictionary_item_key(address);

//     storage::dictionary_get(balances_uref, &dictionary_item_key)
//         .unwrap_or_revert()
//         .unwrap_or_default()
// }

// /// Transfer tokens from the `sender` to the `recipient`.
// ///
// /// This function should not be used directly by contract's entrypoint as it does not validate
// the /// sender.
// pub(crate) fn transfer_balance(
//     sender: Key,
//     recipient: Key,
//     amount: U256,
// ) -> Result<(), Cep18Error> {
//     if sender == recipient || amount.is_zero() {
//         return Ok(());
//     }

//     let balances_uref = get_balances_uref();
//     let new_sender_balance = {
//         let sender_balance = read_balance_from(balances_uref, sender);
//         sender_balance
//             .checked_sub(amount)
//             .ok_or(Cep18Error::InsufficientBalance)?
//     };

//     let new_recipient_balance = {
//         let recipient_balance = read_balance_from(balances_uref, recipient);
//         recipient_balance
//             .checked_add(amount)
//             .ok_or(Cep18Error::Overflow)?
//     };

//     write_balance_to(balances_uref, sender, new_sender_balance);
//     write_balance_to(balances_uref, recipient, new_recipient_balance);

//     Ok(())
// }
