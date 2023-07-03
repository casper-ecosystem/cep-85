use casper_engine_test_support::DEFAULT_ACCOUNT_ADDR;
use casper_types::{bytesrepr::ToBytes, runtime_args, ContractPackageHash, Key, RuntimeArgs, U256};
use cep85::{
    constants::{ARG_ENABLE_MINT_BURN, ARG_EVENTS_MODE, ARG_NAME, ARG_URI},
    modalities::EventsMode,
};

use crate::utility::{
    constants::{TOKEN_NAME, TOKEN_URI},
    installer_request_builders::{
        cep85_batch_burn, cep85_batch_mint, cep85_check_supply_of, cep85_check_supply_of_batch,
        cep85_check_total_supply_of, cep85_check_total_supply_of_batch, cep85_mint,
        cep85_set_total_supply_of, cep85_set_total_supply_of_batch, setup_with_args, TestContext,
    },
};

#[test]
fn should_set_total_supply_of_id() {
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
    let total_supply = U256::from(2);
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

    let actual_total_supply =
        cep85_check_total_supply_of(&mut builder, &cep85_test_contract_package, id);

    assert_eq!(actual_total_supply, total_supply);
}

#[test]
fn should_set_total_supply_batch_for_ids() {
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
    let ids = vec![U256::from(1), U256::from(2)];
    let total_supplies = vec![U256::from(2), U256::from(3)];

    let set_total_supply_of_batch_call = cep85_set_total_supply_of_batch(
        &mut builder,
        &cep85_token,
        minting_account,
        ids.clone(),
        total_supplies.clone(),
    );

    set_total_supply_of_batch_call.expect_success().commit();

    let actual_total_supply =
        cep85_check_total_supply_of_batch(&mut builder, &cep85_test_contract_package, ids);

    assert_eq!(actual_total_supply.len(), 2);
    assert_eq!(actual_total_supply[0], total_supplies[0]);
    assert_eq!(actual_total_supply[1], total_supplies[1]);
}

#[test]
fn should_get_supply_of_id() {
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
    let recipient: Key = minting_account.into();
    let total_supply = U256::from(2);
    let mint_amount = U256::from(2);
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
        recipient,
        id,
        mint_amount,
    );

    mint_call.expect_success().commit();

    let actual_supply = cep85_check_supply_of(&mut builder, &cep85_test_contract_package, id);

    assert_eq!(actual_supply, mint_amount);
}

#[test]
fn should_get_supply_of_batch_for_ids() {
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
    let ids = vec![U256::one(), U256::from(2)];
    let recipient: Key = minting_account.into();
    let mint_amount = U256::from(2);
    let total_supplies = vec![U256::from(2), U256::from(3)];

    // Set total supply for each ID using batch function
    let set_total_supply_of_batch_call = cep85_set_total_supply_of_batch(
        &mut builder,
        &cep85_token,
        minting_account,
        ids.clone(),
        total_supplies,
    );
    set_total_supply_of_batch_call.expect_success().commit();

    // Mint tokens for each ID using batch function
    let batch_mint_call = cep85_batch_mint(
        &mut builder,
        &cep85_token,
        minting_account,
        recipient,
        ids.clone(),
        vec![mint_amount; ids.len()],
    );

    batch_mint_call.expect_success().commit();

    // Get the supply of each ID using batch function
    let actual_supplies = cep85_check_supply_of_batch(
        &mut builder,
        &cep85_test_contract_package,
        vec![minting_account.into(); ids.len()],
        ids.clone(),
    );

    // Verify the supplies
    for (index, _) in ids.iter().enumerate() {
        assert_eq!(actual_supplies[index], mint_amount);
    }

    let burning_account = *DEFAULT_ACCOUNT_ADDR;
    // Owner is now last recipient
    let owner = recipient;

    // Perform a batch burn call
    let batch_burn_call = cep85_batch_burn(
        &mut builder,
        &cep85_token,
        burning_account,
        owner,
        ids.clone(),
        vec![mint_amount; ids.len()],
    );
    batch_burn_call.expect_success().commit();

    // Get the supply of each ID using batch function
    let actual_supplies = cep85_check_supply_of_batch(
        &mut builder,
        &cep85_test_contract_package,
        vec![burning_account.into(); ids.len()],
        ids.clone(),
    );

    // Verify the supplies equal zero
    for (index, _) in ids.iter().enumerate() {
        assert_eq!(actual_supplies[index], U256::zero());
    }
}
