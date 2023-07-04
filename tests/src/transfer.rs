use casper_engine_test_support::DEFAULT_ACCOUNT_ADDR;
use casper_types::{bytesrepr::Bytes, runtime_args, Key, RuntimeArgs, U256};
use cep85::{
    constants::{ARG_ENABLE_MINT_BURN, ARG_EVENTS_MODE, ARG_NAME, ARG_URI},
    modalities::EventsMode,
};

use crate::utility::{
    constants::{ACCOUNT_USER_1, TOKEN_NAME, TOKEN_URI},
    installer_request_builders::{
        cep85_batch_mint, cep85_batch_transfer_from, cep85_check_balance_of,
        cep85_check_balance_of_batch, cep85_mint, cep85_set_total_supply_of_batch,
        cep85_transfer_from, setup_with_args, TestContext,
    },
};

#[test]
fn should_transfer_full_owned_amount() {
    let (
        mut builder,
        TestContext {
            cep85_token,
            cep85_test_contract_package,
            test_accounts,
            ..
        },
    ) = setup_with_args(
        runtime_args! {
            ARG_NAME => TOKEN_NAME,
            ARG_URI => TOKEN_URI,
            ARG_EVENTS_MODE => EventsMode::CES as u8,
            ARG_ENABLE_MINT_BURN => true
        },
        None,
    );

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let recipient: Key = Key::from(minting_account);
    let mint_amount = U256::one();
    let id = U256::one();

    let mint_call = cep85_mint(
        &mut builder,
        &cep85_token,
        minting_account,
        recipient,
        id,
        mint_amount,
    );

    mint_call.expect_success().commit();

    let actual_balance_before =
        cep85_check_balance_of(&mut builder, &cep85_test_contract_package, recipient, id);
    let expected_balance_before = U256::one();

    assert_eq!(actual_balance_before, expected_balance_before);

    let from = minting_account;
    let to = *test_accounts.get(&ACCOUNT_USER_1).unwrap();
    let transfer_amount = U256::one();
    let data: Vec<Bytes> = vec![];

    let transfer_call = cep85_transfer_from(
        &mut builder,
        &cep85_token,
        from,
        to,
        id,
        transfer_amount,
        data,
    );
    transfer_call.expect_success().commit();

    let actual_balance_after = cep85_check_balance_of(
        &mut builder,
        &cep85_test_contract_package,
        Key::from(from),
        id,
    );
    let expected_balance_after = U256::zero();

    assert_eq!(actual_balance_after, expected_balance_after);

    let actual_balance = cep85_check_balance_of(
        &mut builder,
        &cep85_test_contract_package,
        Key::from(to),
        id,
    );
    let expected_balance = U256::one();

    assert_eq!(actual_balance, expected_balance);
}

#[test]
fn should_batch_transfer_full_owned_amount() {
    let (
        mut builder,
        TestContext {
            cep85_token,
            cep85_test_contract_package,
            test_accounts,
            ..
        },
    ) = setup_with_args(
        runtime_args! {
            ARG_NAME => TOKEN_NAME,
            ARG_URI => TOKEN_URI,
            ARG_EVENTS_MODE => EventsMode::CES as u8,
            ARG_ENABLE_MINT_BURN => true
        },
        None,
    );

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let recipient: Key = Key::from(minting_account);
    let ids: Vec<U256> = vec![U256::one(), U256::from(2)];
    let amounts: Vec<U256> = vec![U256::one(), U256::from(2)];
    let from = minting_account;
    let to = *test_accounts.get(&ACCOUNT_USER_1).unwrap();
    let data: Vec<Bytes> = vec![];
    let recipients = vec![from, from, to, to];
    let expected_balance_before: Vec<U256> = [&amounts[..], &[U256::zero(), U256::zero()]].concat();
    let expected_balance_after: Vec<U256> = [&[U256::zero(), U256::zero()], &amounts[..]].concat();

    let total_supplies = amounts.clone();
    let set_total_supply_of_batch_call = cep85_set_total_supply_of_batch(
        &mut builder,
        &cep85_token,
        minting_account,
        ids.clone(),
        total_supplies,
    );

    set_total_supply_of_batch_call.expect_success().commit();

    let mint_call = cep85_batch_mint(
        &mut builder,
        &cep85_token,
        minting_account,
        recipient,
        ids.clone(),
        amounts.clone(),
    );

    mint_call.expect_success().commit();

    let actual_balances_before = cep85_check_balance_of_batch(
        &mut builder,
        &cep85_test_contract_package,
        recipients.clone().into_iter().map(Key::Account).collect(),
        vec![ids.clone(); 2_usize].into_iter().flatten().collect(),
    );

    assert_eq!(actual_balances_before, expected_balance_before);

    let transfer_call = cep85_batch_transfer_from(
        &mut builder,
        &cep85_token,
        from,
        to,
        ids.clone(),
        amounts,
        data,
    );
    transfer_call.expect_success().commit();

    let actual_balances_after = cep85_check_balance_of_batch(
        &mut builder,
        &cep85_test_contract_package,
        recipients.into_iter().map(Key::Account).collect(),
        vec![ids; 2_usize].into_iter().flatten().collect(),
    );

    assert_eq!(actual_balances_after, expected_balance_after);
}
