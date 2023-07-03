use std::collections::HashMap;

use crate::utility::{
    constants::{ACCOUNT_USER_1, TOKEN_NAME, TOKEN_URI},
    installer_request_builders::{
        cep85_batch_burn, cep85_batch_mint, cep85_burn, cep85_mint, setup_with_args, TestContext,
    },
    support::{assert_expected_error, create_dummy_key_pair, fund_account},
};

use casper_engine_test_support::DEFAULT_ACCOUNT_ADDR;
use casper_types::{runtime_args, Key, RuntimeArgs, U256};
use cep85::{
    constants::{ARG_ENABLE_MINT_BURN, ARG_EVENTS_MODE, ARG_NAME, ARG_URI, BURNER_LIST},
    error::Cep85Error,
    modalities::EventsMode,
};

#[test]
fn should_burn_by_owner() {
    let (_, public_key_account_user_1) = create_dummy_key_pair(ACCOUNT_USER_1);
    let account_user_1 = public_key_account_user_1.to_account_hash();
    let mut test_accounts = HashMap::new();
    test_accounts.insert(ACCOUNT_USER_1, account_user_1);

    let (mut builder, TestContext { cep85_token, .. }) = setup_with_args(
        runtime_args! {
            ARG_NAME => TOKEN_NAME,
            ARG_URI => TOKEN_URI,
            ARG_EVENTS_MODE => EventsMode::CES as u8,
            ARG_ENABLE_MINT_BURN => true,
            BURNER_LIST => vec![Key::from(account_user_1)]
        },
        Some(test_accounts),
    );

    // account_user_1 was created before genesis and is not yet funded so fund it
    fund_account(&mut builder, account_user_1);

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let recipient: Key = Key::from(account_user_1);
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

    let burning_account = *DEFAULT_ACCOUNT_ADDR;
    // owner is now last recipient account_user_1
    let owner: Key = recipient;
    let burn_amount = U256::one();

    let failing_burn_call = cep85_burn(
        &mut builder,
        &cep85_token,
        burning_account,
        owner,
        id,
        burn_amount,
    );
    failing_burn_call.expect_failure();

    let error = builder.get_error().expect("must have error");

    assert_expected_error(
        error,
        Cep85Error::InvalidBurnTarget as u16,
        "only owner can burn its token",
    );

    let burning_account = account_user_1;

    let burn_call = cep85_burn(
        &mut builder,
        &cep85_token,
        burning_account,
        owner,
        id,
        burn_amount,
    );
    burn_call.expect_success().commit();
}

#[test]
fn should_batch_burn_by_owner() {
    let (_, public_key_account_user_1) = create_dummy_key_pair(ACCOUNT_USER_1);
    let account_user_1 = public_key_account_user_1.to_account_hash();
    let mut test_accounts = HashMap::new();
    test_accounts.insert(ACCOUNT_USER_1, account_user_1);
    let (mut builder, TestContext { cep85_token, .. }) = setup_with_args(
        runtime_args! {
            ARG_NAME => TOKEN_NAME,
            ARG_URI => TOKEN_URI,
            ARG_EVENTS_MODE => EventsMode::CES as u8,
            ARG_ENABLE_MINT_BURN => true,
            BURNER_LIST => vec![Key::from(account_user_1)]
        },
        Some(test_accounts),
    );

    // account_user_1 was created before genesis and is not yet funded so fund it
    fund_account(&mut builder, account_user_1);

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let recipient = Key::from(account_user_1);
    let ids: Vec<U256> = vec![U256::one(), U256::from(2)];
    let amounts: Vec<U256> = vec![U256::one(), U256::one()];

    // batch_mint is only one recipient
    let mint_call = cep85_batch_mint(
        &mut builder,
        &cep85_token,
        minting_account,
        recipient,
        ids.clone(),
        amounts.clone(),
    );

    mint_call.expect_success().commit();

    let burning_account = *DEFAULT_ACCOUNT_ADDR;
    let owner: Key = recipient;

    let failing_burn_call = cep85_batch_burn(
        &mut builder,
        &cep85_token,
        burning_account,
        owner,
        ids.clone(),
        amounts.clone(),
    );

    failing_burn_call.expect_failure();

    let error = builder.get_error().expect("must have error");

    assert_expected_error(
        error,
        Cep85Error::InvalidBurnTarget as u16,
        "only owner can burn its token",
    );

    let burning_account = account_user_1;

    let burn_call = cep85_batch_burn(
        &mut builder,
        &cep85_token,
        burning_account,
        owner,
        ids,
        amounts,
    );

    burn_call.expect_success().commit();
}

#[test]
fn should_not_burn_above_balance() {
    let (_, public_key_account_user_1) = create_dummy_key_pair(ACCOUNT_USER_1);
    let account_user_1 = public_key_account_user_1.to_account_hash();
    let mut test_accounts = HashMap::new();
    test_accounts.insert(ACCOUNT_USER_1, account_user_1);

    let (mut builder, TestContext { cep85_token, .. }) = setup_with_args(
        runtime_args! {
            ARG_NAME => TOKEN_NAME,
            ARG_URI => TOKEN_URI,
            ARG_EVENTS_MODE => EventsMode::CES as u8,
            ARG_ENABLE_MINT_BURN => true,
            BURNER_LIST => vec![Key::from(account_user_1)]
        },
        Some(test_accounts),
    );

    // account_user_1 was created before genesis and is not yet funded so fund it
    fund_account(&mut builder, account_user_1);

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let recipient: Key = Key::from(account_user_1);
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

    let burning_account = account_user_1;
    // owner is now last recipient account_user_1
    let owner: Key = recipient;

    // burn_amount > mint_amount, request should fail
    let burn_amount = U256::from(2);

    let failing_burn_call = cep85_burn(
        &mut builder,
        &cep85_token,
        burning_account,
        owner,
        id,
        burn_amount,
    );
    failing_burn_call.expect_failure();

    let error = builder.get_error().expect("must have error");

    assert_expected_error(
        error,
        Cep85Error::OverflowBurn as u16,
        "owner can only burn its token balance",
    );

    let burn_amount = U256::one();

    let burn_call = cep85_burn(
        &mut builder,
        &cep85_token,
        burning_account,
        owner,
        id,
        burn_amount,
    );
    burn_call.expect_success().commit();
}

#[test]
fn should_not_batch_burn_above_balance() {
    let (_, public_key_account_user_1) = create_dummy_key_pair(ACCOUNT_USER_1);
    let account_user_1 = public_key_account_user_1.to_account_hash();
    let mut test_accounts = HashMap::new();
    test_accounts.insert(ACCOUNT_USER_1, account_user_1);

    let (
        mut builder,
        TestContext {
            cep85_token,
            test_accounts,
            ..
        },
    ) = setup_with_args(
        runtime_args! {
            ARG_NAME => TOKEN_NAME,
            ARG_URI => TOKEN_URI,
            ARG_EVENTS_MODE => EventsMode::CES as u8,
            ARG_ENABLE_MINT_BURN => true,
            BURNER_LIST => vec![Key::from(account_user_1)]
        },
        Some(test_accounts),
    );

    // account_user_1 was created before genesis and is not yet funded so fund it
    fund_account(&mut builder, account_user_1);

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let recipient = Key::from(*test_accounts.get(&ACCOUNT_USER_1).unwrap());
    let ids: Vec<U256> = vec![U256::one(), U256::from(2)];
    let amounts: Vec<U256> = vec![U256::one(), U256::one()];

    // batch_mint is only one recipient
    let mint_call = cep85_batch_mint(
        &mut builder,
        &cep85_token,
        minting_account,
        recipient,
        ids.clone(),
        amounts,
    );

    mint_call.expect_success().commit();

    let burning_account = account_user_1;
    let owner: Key = recipient;

    // burn_amount > mint_amount, request should fail
    let burn_amounts: Vec<U256> = vec![U256::from(2), U256::from(2)];

    let failing_batch_burn_call = cep85_batch_burn(
        &mut builder,
        &cep85_token,
        burning_account,
        owner,
        ids.clone(),
        burn_amounts,
    );

    failing_batch_burn_call.expect_failure();

    let error = builder.get_error().expect("must have error");

    assert_expected_error(
        error,
        Cep85Error::OverflowBatchBurn as u16,
        "owner can only burn its token balance",
    );

    let burn_amounts: Vec<U256> = vec![U256::one(), U256::one()];

    let batch_burn_call = cep85_batch_burn(
        &mut builder,
        &cep85_token,
        burning_account,
        owner,
        ids,
        burn_amounts,
    );

    batch_burn_call.expect_success().commit();
}
