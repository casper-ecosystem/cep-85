use casper_engine_test_support::DEFAULT_ACCOUNT_ADDR;
use casper_types::{runtime_args, Key, RuntimeArgs, U256};
use cep85::{constants::ARG_ENABLE_BURN, error::Cep85Error};

use crate::utility::{
    installer_request_builders::{
        cep85_batch_burn, cep85_batch_mint, cep85_check_supply_of, cep85_check_supply_of_batch,
        cep85_check_total_supply_of, cep85_check_total_supply_of_batch, cep85_mint,
        cep85_set_total_supply_of, cep85_set_total_supply_of_batch, setup, setup_with_args,
        TestContext,
    },
    support::assert_expected_error,
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
    ) = setup();

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let total_supply = U256::from(2);
    let id = U256::one();

    // Set total supply to 2 for the token to be minted
    let set_total_supply_of_call = cep85_set_total_supply_of(
        &mut builder,
        &cep85_token,
        &minting_account,
        &id,
        &total_supply,
    );

    set_total_supply_of_call.expect_success().commit();

    let actual_total_supply =
        cep85_check_total_supply_of(&mut builder, &cep85_test_contract_package, &id);

    assert_eq!(actual_total_supply, total_supply);
}

#[test]
fn should_not_set_total_supply_of_id_below_current_supply() {
    let (
        mut builder,
        TestContext {
            cep85_token,
            cep85_test_contract_package,
            ..
        },
    ) = setup();

    let total_supply = U256::from(2);
    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let minting_recipient = Key::from(minting_account);
    let mint_amount = U256::from(2);
    let id = U256::one();

    // Set total supply to 2 for the token to be minted
    let set_total_supply_of_call = cep85_set_total_supply_of(
        &mut builder,
        &cep85_token,
        &minting_account,
        &id,
        &total_supply,
    );

    set_total_supply_of_call.expect_success().commit();

    let actual_total_supply =
        cep85_check_total_supply_of(&mut builder, &cep85_test_contract_package, &id);

    assert_eq!(actual_total_supply, total_supply);

    let mint_call = cep85_mint(
        &mut builder,
        &cep85_token,
        &minting_account,
        &minting_recipient,
        &id,
        &mint_amount,
    );

    mint_call.expect_success().commit();

    // Set total supply to 1 for the token should fail
    let new_total_supply = U256::one();
    let failing_set_total_supply_of_call = cep85_set_total_supply_of(
        &mut builder,
        &cep85_token,
        &minting_account,
        &id,
        &new_total_supply,
    );

    failing_set_total_supply_of_call.expect_failure();

    let error = builder.get_error().expect("must have error");

    assert_expected_error(
        error,
        Cep85Error::InvalidTotalSupply as u16,
        "should not allow to set total supply below current supply",
    );

    let actual_total_supply =
        cep85_check_total_supply_of(&mut builder, &cep85_test_contract_package, &id);

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
    ) = setup();

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let ids = vec![U256::from(1), U256::from(2)];
    let total_supplies = vec![U256::from(2), U256::from(3)];

    let set_total_supply_of_batch_call = cep85_set_total_supply_of_batch(
        &mut builder,
        &cep85_token,
        &minting_account,
        ids.clone(),
        total_supplies.clone(),
    );

    set_total_supply_of_batch_call.expect_success().commit();

    let actual_total_supplies =
        cep85_check_total_supply_of_batch(&mut builder, &cep85_test_contract_package, ids);

    assert_eq!(actual_total_supplies.len(), 2);
    assert_eq!(actual_total_supplies[0], total_supplies[0]);
    assert_eq!(actual_total_supplies[1], total_supplies[1]);
}

#[test]
fn should_not_set_total_supply_batch_of_id_below_current_supply() {
    let (
        mut builder,
        TestContext {
            cep85_token,
            cep85_test_contract_package,
            ..
        },
    ) = setup();

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let minting_recipient = Key::from(minting_account);
    let ids: Vec<U256> = vec![U256::one()];
    let amounts: Vec<U256> = vec![U256::from(2)];
    let total_supplies = amounts.clone();

    // Set total supply to 2 for the token to be minted
    let set_total_supply_of_batch_call = cep85_set_total_supply_of_batch(
        &mut builder,
        &cep85_token,
        &minting_account,
        ids.clone(),
        total_supplies.clone(),
    );

    set_total_supply_of_batch_call.expect_success().commit();

    // Batch mint tokens with initial total supplies

    let batch_mint_call = cep85_batch_mint(
        &mut builder,
        &cep85_token,
        &minting_account,
        &minting_recipient,
        ids.clone(),
        amounts,
    );
    batch_mint_call.expect_success().commit();

    let actual_total_supplies =
        cep85_check_total_supply_of_batch(&mut builder, &cep85_test_contract_package, ids.clone());

    assert_eq!(actual_total_supplies, total_supplies);

    // Attempt to set total supply below current supply should fail
    let new_total_supplies = vec![U256::from(1)];
    let failing_set_total_supply_of_batch_call = cep85_set_total_supply_of_batch(
        &mut builder,
        &cep85_token,
        &minting_account,
        ids.clone(),
        new_total_supplies,
    );

    failing_set_total_supply_of_batch_call.expect_failure();

    let error = builder.get_error().expect("must have error");

    assert_expected_error(
        error,
        Cep85Error::InvalidTotalSupply as u16,
        "should not allow to set total supply below current supply",
    );

    let actual_total_supplies =
        cep85_check_total_supply_of_batch(&mut builder, &cep85_test_contract_package, ids);

    assert_eq!(actual_total_supplies, total_supplies);
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
    ) = setup();

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let minting_recipient: Key = minting_account.into();
    let total_supply = U256::from(2);
    let mint_amount = U256::from(2);
    let id = U256::one();

    // Set total supply to 2 for the token to be minted
    let set_total_supply_of_call = cep85_set_total_supply_of(
        &mut builder,
        &cep85_token,
        &minting_account,
        &id,
        &total_supply,
    );

    set_total_supply_of_call.expect_success().commit();

    let mint_call = cep85_mint(
        &mut builder,
        &cep85_token,
        &minting_account,
        &minting_recipient,
        &id,
        &mint_amount,
    );

    mint_call.expect_success().commit();

    let actual_supply = cep85_check_supply_of(&mut builder, &cep85_test_contract_package, &id);

    assert_eq!(actual_supply, mint_amount);

    let actual_total_supply =
        cep85_check_total_supply_of(&mut builder, &cep85_test_contract_package, &id);
    assert_eq!(actual_total_supply, total_supply);
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
            ARG_ENABLE_BURN => true,
        },
        None,
    );
    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let ids = vec![U256::one(), U256::from(2)];
    let minting_recipient: Key = minting_account.into();
    let mint_amounts = vec![U256::from(2), U256::from(3)];
    let total_supplies = vec![U256::from(2), U256::from(3)];

    // Set total supply for each ID using batch function
    let set_total_supply_of_batch_call = cep85_set_total_supply_of_batch(
        &mut builder,
        &cep85_token,
        &minting_account,
        ids.clone(),
        total_supplies.clone(),
    );
    set_total_supply_of_batch_call.expect_success().commit();

    // Mint tokens for each ID using batch function
    let batch_mint_call = cep85_batch_mint(
        &mut builder,
        &cep85_token,
        &minting_account,
        &minting_recipient,
        ids.clone(),
        mint_amounts.clone(),
    );

    batch_mint_call.expect_success().commit();

    // Get the supply of each ID using batch function
    let actual_supplies =
        cep85_check_supply_of_batch(&mut builder, &cep85_test_contract_package, ids.clone());

    // Verify the supplies
    assert_eq!(actual_supplies.len(), 2);
    assert_eq!(actual_supplies[0], mint_amounts[0]);
    assert_eq!(actual_supplies[1], mint_amounts[1]);

    let burning_account = minting_account;
    // Owner is now last recipient
    let owner = minting_recipient;

    // Perform a batch burn call
    let batch_burn_call = cep85_batch_burn(
        &mut builder,
        &cep85_token,
        &burning_account,
        &owner,
        ids.clone(),
        mint_amounts,
    );
    batch_burn_call.expect_success().commit();

    // Get the supply of each ID using batch function
    let actual_supplies =
        cep85_check_supply_of_batch(&mut builder, &cep85_test_contract_package, ids.clone());

    // Verify the supplies equal zero
    for (index, _) in ids.iter().enumerate() {
        assert_eq!(actual_supplies[index], U256::zero());
    }

    let actual_total_supplies =
        cep85_check_total_supply_of_batch(&mut builder, &cep85_test_contract_package, ids);

    assert_eq!(actual_total_supplies.len(), 2);
    assert_eq!(actual_total_supplies[0], total_supplies[0]);
    assert_eq!(actual_total_supplies[1], total_supplies[1]);
}

#[test]
fn should_get_no_supplies_of_non_existent_id() {
    let (
        mut builder,
        TestContext {
            cep85_test_contract_package,
            ..
        },
    ) = setup();

    let id = U256::one();

    let actual_supply = cep85_check_supply_of(&mut builder, &cep85_test_contract_package, &id);

    assert_eq!(actual_supply, U256::zero());

    let actual_total_supply =
        cep85_check_total_supply_of(&mut builder, &cep85_test_contract_package, &id);
    assert_eq!(actual_total_supply, U256::zero());
}

#[test]
fn should_get_no_supplies_of_batch_for_non_existent_ids() {
    let (
        mut builder,
        TestContext {
            cep85_test_contract_package,
            ..
        },
    ) = setup_with_args(
        runtime_args! {
            ARG_ENABLE_BURN => true,
        },
        None,
    );

    let ids = vec![U256::one(), U256::from(2)];

    // Get the supply of each ID using batch function
    let actual_supplies =
        cep85_check_supply_of_batch(&mut builder, &cep85_test_contract_package, ids.clone());

    // Verify the supplies equal zero
    for (index, _) in ids.iter().enumerate() {
        assert_eq!(actual_supplies[index], U256::zero());
    }

    let actual_total_supplies =
        cep85_check_total_supply_of_batch(&mut builder, &cep85_test_contract_package, ids);

    assert_eq!(actual_total_supplies.len(), 2);
    assert_eq!(actual_total_supplies[0], U256::zero());
    assert_eq!(actual_total_supplies[1], U256::zero());
}
