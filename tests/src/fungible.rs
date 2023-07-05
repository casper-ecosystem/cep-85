use casper_engine_test_support::DEFAULT_ACCOUNT_ADDR;
use casper_types::{runtime_args, Key, RuntimeArgs, U256};
use cep85::{
    constants::{ARG_ENABLE_MINT_BURN, ARG_EVENTS_MODE, ARG_NAME, ARG_URI},
    modalities::EventsMode,
};

use crate::utility::{
    constants::{TOKEN_NAME, TOKEN_URI},
    installer_request_builders::{
        cep85_check_is_non_fungible, cep85_check_total_fungible_supply, cep85_mint,
        cep85_set_total_supply_of, setup_with_args, TestContext,
    },
};

#[test]
fn should_check_if_fungible() {
    let (
        mut builder,
        TestContext {
            cep85_token,
            cep85_test_contract_package,
            ..
        },
    ) = setup_with_args(
        runtime_args! {
            ARG_NAME => TOKEN_NAME,
            ARG_URI => TOKEN_URI,
            ARG_EVENTS_MODE => EventsMode::CES as u8,
            ARG_ENABLE_MINT_BURN => true,
        },
        None,
    );

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let minting_recipient = Key::from(minting_account);
    let total_supply = U256::from(2);
    let mint_amount = U256::one();
    let id = U256::one();

    // Set total supply to 2 for the token to be minted
    let set_total_supply_of_call = cep85_set_total_supply_of(
        &mut builder,
        &cep85_token,
        minting_account,
        id,
        total_supply,
    );

    set_total_supply_of_call.expect_success().commit();

    let mint_call = cep85_mint(
        &mut builder,
        &cep85_token,
        minting_account,
        minting_recipient,
        id,
        mint_amount,
    );

    mint_call.expect_success().commit();

    let is_non_fungible =
        cep85_check_is_non_fungible(&mut builder, &cep85_test_contract_package, id);

    assert!(!is_non_fungible);
}

#[test]
fn should_check_if_non_fungible() {
    let (
        mut builder,
        TestContext {
            cep85_token,
            cep85_test_contract_package,
            ..
        },
    ) = setup_with_args(
        runtime_args! {
            ARG_NAME => TOKEN_NAME,
            ARG_URI => TOKEN_URI,
            ARG_EVENTS_MODE => EventsMode::CES as u8,
            ARG_ENABLE_MINT_BURN => true,
        },
        None,
    );

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let minting_recipient = Key::from(minting_account);
    let mint_amount = U256::one();
    let id = U256::one();

    let mint_call = cep85_mint(
        &mut builder,
        &cep85_token,
        minting_account,
        minting_recipient,
        id,
        mint_amount,
    );

    mint_call.expect_success().commit();

    let is_non_fungible =
        cep85_check_is_non_fungible(&mut builder, &cep85_test_contract_package, id);

    assert!(is_non_fungible);
}

#[test]
fn should_check_if_non_fungible_if_total_supply_is_reduced_to_one() {
    let (
        mut builder,
        TestContext {
            cep85_token,
            cep85_test_contract_package,
            ..
        },
    ) = setup_with_args(
        runtime_args! {
            ARG_NAME => TOKEN_NAME,
            ARG_URI => TOKEN_URI,
            ARG_EVENTS_MODE => EventsMode::CES as u8,
            ARG_ENABLE_MINT_BURN => true,
        },
        None,
    );

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let minting_recipient = Key::from(minting_account);
    let total_supply = U256::from(2);
    let mint_amount = U256::one();
    let id = U256::one();

    // Set total supply to 2 for the token to be minted
    let set_total_supply_of_call = cep85_set_total_supply_of(
        &mut builder,
        &cep85_token,
        minting_account,
        id,
        total_supply,
    );

    set_total_supply_of_call.expect_success().commit();

    let mint_call = cep85_mint(
        &mut builder,
        &cep85_token,
        minting_account,
        minting_recipient,
        id,
        mint_amount,
    );

    mint_call.expect_success().commit();

    let is_non_fungible =
        cep85_check_is_non_fungible(&mut builder, &cep85_test_contract_package, id);

    assert!(!is_non_fungible);

    let total_supply = U256::one();

    // Set total supply to 2 for the token to be minted
    let set_total_supply_of_call = cep85_set_total_supply_of(
        &mut builder,
        &cep85_token,
        minting_account,
        id,
        total_supply,
    );

    set_total_supply_of_call.expect_success().commit();

    let is_now_non_fungible =
        cep85_check_is_non_fungible(&mut builder, &cep85_test_contract_package, id);

    assert!(is_now_non_fungible);
}

#[test]
fn should_get_total_fungible_supply() {
    let (
        mut builder,
        TestContext {
            cep85_token,
            cep85_test_contract_package,
            ..
        },
    ) = setup_with_args(
        runtime_args! {
            ARG_NAME => TOKEN_NAME,
            ARG_URI => TOKEN_URI,
            ARG_EVENTS_MODE => EventsMode::CES as u8,
            ARG_ENABLE_MINT_BURN => true,
        },
        None,
    );

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let minting_recipient = Key::from(minting_account);
    let total_supply = U256::from(10);
    let mint_amount = U256::from(4);
    let id = U256::one();

    // Set total supply to 10 for the 4 token to be minted
    let set_total_supply_of_call = cep85_set_total_supply_of(
        &mut builder,
        &cep85_token,
        minting_account,
        id,
        total_supply,
    );

    set_total_supply_of_call.expect_success().commit();

    let mint_call = cep85_mint(
        &mut builder,
        &cep85_token,
        minting_account,
        minting_recipient,
        id,
        mint_amount,
    );

    mint_call.expect_success().commit();

    let fungible_supply =
        cep85_check_total_fungible_supply(&mut builder, &cep85_test_contract_package, id);

    // Fungible supply should be 6
    let expected_fungible_supply = total_supply - mint_amount;
    assert_eq!(expected_fungible_supply, U256::from(6));
    assert_eq!(fungible_supply, expected_fungible_supply);
}
