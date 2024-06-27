use crate::utility::{
    installer_request_builders::{
        cep85_batch_burn, cep85_batch_mint, cep85_burn, cep85_change_security,
        cep85_check_balance_of, cep85_check_balance_of_batch, cep85_check_is_approved,
        cep85_check_supply_of, cep85_check_supply_of_batch, cep85_mint, cep85_set_approval_for_all,
        cep85_set_total_supply_of, cep85_set_total_supply_of_batch, setup_with_args, SecurityLists,
        TestContext,
    },
    support::{assert_expected_error, get_test_account},
};

use casper_engine_test_support::DEFAULT_ACCOUNT_ADDR;
use casper_types::{addressable_entity::EntityKindTag, runtime_args, Key, U256};
use cep85::{
    constants::{ARG_ENABLE_BURN, BURNER_LIST},
    error::Cep85Error,
};

#[test]
fn should_burn_by_owner() {
    let (account_user_1_key, account_user_1_account_hash, _) = get_test_account("ACCOUNT_USER_1");

    let (
        mut builder,
        TestContext {
            cep18_contract_hash,
            ..
        },
    ) = setup_with_args(runtime_args! {
        ARG_ENABLE_BURN => true,
        BURNER_LIST => vec![account_user_1_key]
    });

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let minting_recipient: Key = account_user_1_key;
    let mint_amount = U256::one();
    let id = U256::one();

    let mint_call = cep85_mint(
        &mut builder,
        &cep18_contract_hash,
        &minting_account,
        &minting_recipient,
        &id,
        &mint_amount,
        None,
    );

    mint_call.expect_success().commit();

    let burning_account = minting_account;
    // owner is now last recipient account_user_1
    let owner: Key = minting_recipient;
    let burn_amount = U256::one();

    let failing_burn_call = cep85_burn(
        &mut builder,
        &cep18_contract_hash,
        &burning_account,
        &owner,
        &id,
        &burn_amount,
    );
    failing_burn_call.expect_failure();

    let error = builder.get_error().expect("must have error");

    assert_expected_error(
        error,
        Cep85Error::InvalidBurnTarget as u16,
        "only owner can burn its token",
    );

    let burning_account = account_user_1_account_hash;

    let burn_call = cep85_burn(
        &mut builder,
        &cep18_contract_hash,
        &burning_account,
        &owner,
        &id,
        &burn_amount,
    );
    burn_call.expect_success().commit();
}

#[test]
fn should_batch_burn_by_owner() {
    let (account_user_1_key, account_user_1_account_hash, _) = get_test_account("ACCOUNT_USER_1");

    let (
        mut builder,
        TestContext {
            cep18_contract_hash,
            ..
        },
    ) = setup_with_args(runtime_args! {
        ARG_ENABLE_BURN => true,
        BURNER_LIST => vec![account_user_1_key]
    });

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let minting_recipient = account_user_1_key;
    let ids: Vec<U256> = vec![U256::one(), U256::from(2)];
    let amounts: Vec<U256> = vec![U256::one(), U256::one()];

    // batch_mint is only one recipient
    let mint_call = cep85_batch_mint(
        &mut builder,
        &cep18_contract_hash,
        &minting_account,
        &minting_recipient,
        ids.clone(),
        amounts.clone(),
        None,
    );

    mint_call.expect_success().commit();

    let burning_account = minting_account;
    let owner: Key = minting_recipient;

    let failing_burn_call = cep85_batch_burn(
        &mut builder,
        &cep18_contract_hash,
        &burning_account,
        &owner,
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

    let burning_account = account_user_1_account_hash;

    let burn_call = cep85_batch_burn(
        &mut builder,
        &cep18_contract_hash,
        &burning_account,
        &owner,
        ids,
        amounts,
    );

    burn_call.expect_success().commit();
}

#[test]
fn should_not_burn_above_balance_with_default_supply() {
    let (account_user_1_key, account_user_1_account_hash, _) = get_test_account("ACCOUNT_USER_1");

    let (
        mut builder,
        TestContext {
            cep18_contract_hash,
            ..
        },
    ) = setup_with_args(runtime_args! {
        ARG_ENABLE_BURN => true,
        BURNER_LIST => vec![account_user_1_key]
    });

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let minting_recipient: Key = account_user_1_key;
    let mint_amount = U256::one();
    let id = U256::one();

    let mint_call = cep85_mint(
        &mut builder,
        &cep18_contract_hash,
        &minting_account,
        &minting_recipient,
        &id,
        &mint_amount,
        None,
    );

    mint_call.expect_success().commit();

    let burning_account = account_user_1_account_hash;
    // owner is now last recipient account_user_1
    let owner: Key = minting_recipient;

    // burn_amount > mint_amount, request should fail
    let burn_amount = U256::from(2);

    let failing_burn_call = cep85_burn(
        &mut builder,
        &cep18_contract_hash,
        &burning_account,
        &owner,
        &id,
        &burn_amount,
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
        &cep18_contract_hash,
        &burning_account,
        &owner,
        &id,
        &burn_amount,
    );
    burn_call.expect_success().commit();
}

#[test]
fn should_not_burn_above_balance_with_custom_supply() {
    let (account_user_1_key, account_user_1_account_hash, _) = get_test_account("ACCOUNT_USER_1");
    let (account_user_2_key, _, _) = get_test_account("ACCOUNT_USER_2");

    let (
        mut builder,
        TestContext {
            cep18_contract_hash,
            ..
        },
    ) = setup_with_args(runtime_args! {
        ARG_ENABLE_BURN => true,
        BURNER_LIST => vec![account_user_1_key]
    });

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let minting_recipient: Key = account_user_1_key;
    let mint_amount = U256::from(10);
    let id = U256::one();

    let mint_call = cep85_mint(
        &mut builder,
        &cep18_contract_hash,
        &minting_account,
        &minting_recipient,
        &id,
        &mint_amount,
        None,
    );

    mint_call.expect_success().commit();

    let total_supply = U256::from(20);

    let set_total_supply_of_call = cep85_set_total_supply_of(
        &mut builder,
        &cep18_contract_hash,
        &minting_account,
        &id,
        &total_supply,
    );

    set_total_supply_of_call.expect_success().commit();

    let mint_call = cep85_mint(
        &mut builder,
        &cep18_contract_hash,
        &minting_account,
        &account_user_2_key,
        &id,
        &mint_amount,
        None,
    );

    mint_call.expect_success().commit();

    let burning_account = account_user_1_account_hash;
    // owner is now last recipient account_user_1
    let owner: Key = minting_recipient;

    // burn_amount > mint_amount, request should fail
    let burn_amount = U256::from(15);

    let failing_burn_call = cep85_burn(
        &mut builder,
        &cep18_contract_hash,
        &burning_account,
        &owner,
        &id,
        &burn_amount,
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
        &cep18_contract_hash,
        &burning_account,
        &owner,
        &id,
        &burn_amount,
    );
    burn_call.expect_success().commit();
}

#[test]
fn should_not_batch_burn_above_balance_with_default_supply() {
    let (account_user_1_key, account_user_1_account_hash, _) = get_test_account("ACCOUNT_USER_1");

    let (
        mut builder,
        TestContext {
            cep18_contract_hash,
            ..
        },
    ) = setup_with_args(runtime_args! {
        ARG_ENABLE_BURN => true,
        BURNER_LIST => vec![account_user_1_key]
    });

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let minting_recipient = account_user_1_key;
    let ids: Vec<U256> = vec![U256::one(), U256::from(2)];
    let amounts: Vec<U256> = vec![U256::one(), U256::one()];

    // batch_mint is only one recipient
    let mint_call = cep85_batch_mint(
        &mut builder,
        &cep18_contract_hash,
        &minting_account,
        &minting_recipient,
        ids.clone(),
        amounts,
        None,
    );

    mint_call.expect_success().commit();

    let burning_account = account_user_1_account_hash;
    let owner: Key = minting_recipient;

    // burn_amount > mint_amount, request should fail
    let burn_amounts: Vec<U256> = vec![U256::from(2), U256::from(2)];

    let failing_batch_burn_call = cep85_batch_burn(
        &mut builder,
        &cep18_contract_hash,
        &burning_account,
        &owner,
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
        &cep18_contract_hash,
        &burning_account,
        &owner,
        ids,
        burn_amounts,
    );

    batch_burn_call.expect_success().commit();
}

#[test]
fn should_not_batch_burn_above_balance_with_custom_supply() {
    let (account_user_1_key, account_user_1_account_hash, _) = get_test_account("ACCOUNT_USER_1");

    let (
        mut builder,
        TestContext {
            cep18_contract_hash,
            ..
        },
    ) = setup_with_args(runtime_args! {
        ARG_ENABLE_BURN => true,
        BURNER_LIST => vec![account_user_1_key]
    });

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let minting_recipient = account_user_1_key;
    let ids: Vec<U256> = vec![U256::one(), U256::from(2)];
    let amounts: Vec<U256> = vec![U256::one(), U256::one()];
    let total_supplies: Vec<U256> = vec![U256::from(10), U256::from(20)];

    // Set total supply to total supplies for the token to be minted
    let set_total_supply_of_batch_call = cep85_set_total_supply_of_batch(
        &mut builder,
        &cep18_contract_hash,
        &minting_account,
        ids.clone(),
        total_supplies,
    );

    set_total_supply_of_batch_call.expect_success().commit();

    // batch_mint is only one recipient
    let mint_call = cep85_batch_mint(
        &mut builder,
        &cep18_contract_hash,
        &minting_account,
        &minting_recipient,
        ids.clone(),
        amounts,
        None,
    );

    mint_call.expect_success().commit();

    let burning_account = account_user_1_account_hash;
    let owner: Key = minting_recipient;

    // burn_amount > mint_amount, request should fail
    let burn_amounts: Vec<U256> = vec![U256::from(20), U256::from(30)];

    let failing_batch_burn_call = cep85_batch_burn(
        &mut builder,
        &cep18_contract_hash,
        &burning_account,
        &owner,
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
        &cep18_contract_hash,
        &burning_account,
        &owner,
        ids,
        burn_amounts,
    );

    batch_burn_call.expect_success().commit();
}

#[test]
fn should_reduce_supply_on_burn() {
    let (account_user_1_key, account_user_1_account_hash, _) = get_test_account("ACCOUNT_USER_1");
    let (
        mut builder,
        TestContext {
            cep18_contract_hash,
            cep85_test_contract_package,
            ..
        },
    ) = setup_with_args(runtime_args! {
        ARG_ENABLE_BURN => true,
        BURNER_LIST => vec![account_user_1_key]
    });

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let minting_recipient = account_user_1_key;
    let total_supply = U256::from(2);
    let mint_amount = U256::from(2);
    let id = U256::one();

    // Set total supply to 2 for the token to be minted
    let set_total_supply_of_call = cep85_set_total_supply_of(
        &mut builder,
        &cep18_contract_hash,
        &minting_account,
        &id,
        &total_supply,
    );

    set_total_supply_of_call.expect_success().commit();

    let mint_call = cep85_mint(
        &mut builder,
        &cep18_contract_hash,
        &minting_account,
        &minting_recipient,
        &id,
        &mint_amount,
        None,
    );

    mint_call.expect_success().commit();

    let actual_supply =
        cep85_check_supply_of(&mut builder, &cep85_test_contract_package, &id).unwrap();

    assert_eq!(actual_supply, mint_amount);

    let actual_balance = cep85_check_balance_of(
        &mut builder,
        &cep85_test_contract_package,
        &minting_recipient,
        &id,
    )
    .unwrap();
    let expected_balance = mint_amount;

    assert_eq!(actual_balance, expected_balance);

    let burning_account = account_user_1_account_hash;
    let owner: Key = minting_recipient;
    let burn_amount = U256::one();

    let burn_call = cep85_burn(
        &mut builder,
        &cep18_contract_hash,
        &burning_account,
        &owner,
        &id,
        &burn_amount,
    );
    burn_call.expect_success().commit();

    let actual_supply =
        cep85_check_supply_of(&mut builder, &cep85_test_contract_package, &id).unwrap();
    let expected_supply = U256::one();
    assert_eq!(actual_supply, expected_supply);

    let actual_balance =
        cep85_check_balance_of(&mut builder, &cep85_test_contract_package, &owner, &id).unwrap();
    let expected_balance = U256::one();

    assert_eq!(actual_balance, expected_balance);
}

#[test]
fn should_reduce_supply_on_batch_burn() {
    let (account_user_1_key, account_user_1_account_hash, _) = get_test_account("ACCOUNT_USER_1");

    let (
        mut builder,
        TestContext {
            cep18_contract_hash,
            cep85_test_contract_package,
            ..
        },
    ) = setup_with_args(runtime_args! {
        ARG_ENABLE_BURN => true,
        BURNER_LIST => vec![account_user_1_key]
    });

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let minting_recipient = account_user_1_key;
    let ids: Vec<U256> = vec![U256::one(), U256::from(2)];
    let mint_amounts: Vec<U256> = vec![U256::one(), U256::from(2)];
    let total_supplies: Vec<U256> = vec![U256::one(), U256::from(2)];

    // Set total supply to total supplies for the token to be minted
    let set_total_supply_of_batch_call = cep85_set_total_supply_of_batch(
        &mut builder,
        &cep18_contract_hash,
        &minting_account,
        ids.clone(),
        total_supplies,
    );

    set_total_supply_of_batch_call.expect_success().commit();

    // batch_mint is only one recipient
    let mint_call = cep85_batch_mint(
        &mut builder,
        &cep18_contract_hash,
        &minting_account,
        &minting_recipient,
        ids.clone(),
        mint_amounts.clone(),
        None,
    );

    mint_call.expect_success().commit();

    // Get the supply of each ID using batch function
    let actual_supplies =
        cep85_check_supply_of_batch(&mut builder, &cep85_test_contract_package, ids.clone());

    // Verify the supplies
    assert_eq!(actual_supplies[0], Some(mint_amounts[0]));
    assert_eq!(actual_supplies[1], Some(mint_amounts[1]));

    let actual_balances = cep85_check_balance_of_batch(
        &mut builder,
        &cep85_test_contract_package,
        vec![minting_recipient, minting_recipient],
        ids.clone(),
    );

    let expected_balances = [mint_amounts[0], mint_amounts[1]];

    assert_eq!(
        actual_balances,
        expected_balances
            .iter()
            .map(|&amount| Some(amount))
            .collect::<Vec<Option<U256>>>()
    );

    let burning_account = account_user_1_account_hash;
    let owner: Key = minting_recipient;

    let burn_amounts: Vec<U256> = vec![U256::one(), U256::one()];

    let batch_burn_call = cep85_batch_burn(
        &mut builder,
        &cep18_contract_hash,
        &burning_account,
        &owner,
        ids.clone(),
        burn_amounts,
    );

    batch_burn_call.expect_success().commit();

    // Get the supply of each ID using batch function
    let actual_supplies =
        cep85_check_supply_of_batch(&mut builder, &cep85_test_contract_package, ids.clone());

    let expected_remaining_amounts: Vec<U256> = vec![U256::zero(), U256::one()];
    // Verify the supplies
    assert_eq!(actual_supplies[0], Some(expected_remaining_amounts[0]));
    assert_eq!(actual_supplies[1], Some(expected_remaining_amounts[1]));

    let actual_balances = cep85_check_balance_of_batch(
        &mut builder,
        &cep85_test_contract_package,
        vec![owner, owner],
        ids,
    );

    let expected_remaining_balances = [U256::zero(), U256::one()];
    assert_eq!(
        actual_balances,
        expected_remaining_balances
            .iter()
            .map(|&amount| Some(amount))
            .collect::<Vec<Option<U256>>>()
    );
}

#[test]
fn should_not_burn_previously_burnt_token() {
    let (account_user_1_key, account_user_1_account_hash, _) = get_test_account("ACCOUNT_USER_1");

    let (
        mut builder,
        TestContext {
            cep18_contract_hash,
            ..
        },
    ) = setup_with_args(runtime_args! {
        ARG_ENABLE_BURN => true,
        BURNER_LIST => vec![account_user_1_key]
    });

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let minting_recipient: Key = account_user_1_key;
    let mint_amount = U256::one();
    let id = U256::one();

    let mint_call = cep85_mint(
        &mut builder,
        &cep18_contract_hash,
        &minting_account,
        &minting_recipient,
        &id,
        &mint_amount,
        None,
    );

    mint_call.expect_success().commit();

    // owner is now last recipient account_user_1
    let owner: Key = minting_recipient;
    let burn_amount = U256::one();

    let burning_account = account_user_1_account_hash;

    let burn_call = cep85_burn(
        &mut builder,
        &cep18_contract_hash,
        &burning_account,
        &owner,
        &id,
        &burn_amount,
    );
    burn_call.expect_success().commit();

    let failing_burn_call = cep85_burn(
        &mut builder,
        &cep18_contract_hash,
        &burning_account,
        &owner,
        &id,
        &burn_amount,
    );
    failing_burn_call.expect_failure();

    let error = builder.get_error().expect("must have error");

    assert_expected_error(
        error,
        Cep85Error::OverflowBurn as u16,
        "should disallow burning of previously burnt token",
    );
}

#[test]
fn should_not_batch_burn_previously_burnt_token() {
    let (account_user_1_key, account_user_1_account_hash, _) = get_test_account("ACCOUNT_USER_1");

    let (
        mut builder,
        TestContext {
            cep18_contract_hash,
            ..
        },
    ) = setup_with_args(runtime_args! {
        ARG_ENABLE_BURN => true,
        BURNER_LIST => vec![account_user_1_key]
    });

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let minting_recipient = account_user_1_key;
    let ids: Vec<U256> = vec![U256::one(), U256::from(2)];
    let amounts: Vec<U256> = vec![U256::one(), U256::one()];

    // batch_mint is only one recipient
    let mint_call = cep85_batch_mint(
        &mut builder,
        &cep18_contract_hash,
        &minting_account,
        &minting_recipient,
        ids.clone(),
        amounts.clone(),
        None,
    );

    mint_call.expect_success().commit();

    let owner: Key = minting_recipient;
    let burning_account = account_user_1_account_hash;

    let burn_call = cep85_batch_burn(
        &mut builder,
        &cep18_contract_hash,
        &burning_account,
        &owner,
        ids.clone(),
        amounts.clone(),
    );

    burn_call.expect_success().commit();

    let failing_batch_burn_call = cep85_batch_burn(
        &mut builder,
        &cep18_contract_hash,
        &burning_account,
        &owner,
        ids,
        amounts,
    );

    failing_batch_burn_call.expect_failure();

    let error = builder.get_error().expect("must have error");

    assert_expected_error(
        error,
        Cep85Error::OverflowBatchBurn as u16,
        "should disallow burning of previously burnt token",
    );
}

#[test]
fn should_return_expected_error_when_burning_non_existing_token() {
    let (account_user_1_key, account_user_1_account_hash, _) = get_test_account("ACCOUNT_USER_1");

    let (
        mut builder,
        TestContext {
            cep18_contract_hash,
            ..
        },
    ) = setup_with_args(runtime_args! {
        ARG_ENABLE_BURN => true,
        BURNER_LIST => vec![account_user_1_key]
    });

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let minting_recipient: Key = account_user_1_key;
    let mint_amount = U256::one();
    let id = U256::one();

    let mint_call = cep85_mint(
        &mut builder,
        &cep18_contract_hash,
        &minting_account,
        &minting_recipient,
        &id,
        &mint_amount,
        None,
    );

    mint_call.expect_success().commit();

    // owner is now last recipient account_user_1
    let owner: Key = minting_recipient;
    let burn_amount = U256::one();
    let burning_account = account_user_1_account_hash;
    // This id was not minted
    let id = U256::from(2);

    let failing_burn_call = cep85_burn(
        &mut builder,
        &cep18_contract_hash,
        &burning_account,
        &owner,
        &id,
        &burn_amount,
    );
    failing_burn_call.expect_failure();

    let error = builder.get_error().expect("must have error");

    assert_expected_error(
        error,
        Cep85Error::NonSuppliedTokenId as u16,
        "should return error when trying to burn a non_existing token",
    );
}

#[test]
fn should_return_expected_error_when_batch_burning_non_existing_token() {
    let (account_user_1_key, account_user_1_account_hash, _) = get_test_account("ACCOUNT_USER_1");

    let (
        mut builder,
        TestContext {
            cep18_contract_hash,
            ..
        },
    ) = setup_with_args(runtime_args! {
        ARG_ENABLE_BURN => true,
        BURNER_LIST => vec![account_user_1_key]
    });

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let minting_recipient = account_user_1_key;
    let ids: Vec<U256> = vec![U256::one(), U256::from(2)];
    let amounts: Vec<U256> = vec![U256::one(), U256::one()];

    // batch_mint is only one recipient
    let mint_call = cep85_batch_mint(
        &mut builder,
        &cep18_contract_hash,
        &minting_account,
        &minting_recipient,
        ids,
        amounts.clone(),
        None,
    );

    mint_call.expect_success().commit();

    let owner: Key = minting_recipient;
    let burning_account = account_user_1_account_hash;

    let ids: Vec<U256> = vec![U256::from(3), U256::from(4)];

    let failing_batch_burn_call = cep85_batch_burn(
        &mut builder,
        &cep18_contract_hash,
        &burning_account,
        &owner,
        ids,
        amounts,
    );

    failing_batch_burn_call.expect_failure();

    let error = builder.get_error().expect("must have error");

    assert_expected_error(
        error,
        Cep85Error::NonSuppliedTokenId as u16,
        "should return error when trying to burn a non_existing token",
    );
}

#[test]
fn should_return_expected_error_burning_of_others_users_token() {
    let (account_user_1_key, _, _) = get_test_account("ACCOUNT_USER_1");
    let (
        mut builder,
        TestContext {
            cep18_contract_hash,
            ..
        },
    ) = setup_with_args(runtime_args! {
        ARG_ENABLE_BURN => true,
        BURNER_LIST => vec![account_user_1_key]
    });

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let minting_recipient: Key = account_user_1_key;
    let mint_amount = U256::one();
    let id = U256::one();

    let mint_call = cep85_mint(
        &mut builder,
        &cep18_contract_hash,
        &minting_account,
        &minting_recipient,
        &id,
        &mint_amount,
        None,
    );

    mint_call.expect_success().commit();

    // owner is now last recipient account_user_1
    let owner: Key = minting_recipient;
    let burn_amount = U256::one();
    let burning_account = minting_account;

    let failing_burn_call = cep85_burn(
        &mut builder,
        &cep18_contract_hash,
        &burning_account,
        &owner,
        &id,
        &burn_amount,
    );
    failing_burn_call.expect_failure();

    let error = builder.get_error().expect("must have error");

    assert_expected_error(
        error,
        Cep85Error::InvalidBurnTarget as u16,
        "should disallow burning of not owned token",
    );
}

#[test]
fn should_return_expected_error_batch_burning_of_others_users_token() {
    let (account_user_1_key, _, _) = get_test_account("ACCOUNT_USER_1");
    let (
        mut builder,
        TestContext {
            cep18_contract_hash,
            ..
        },
    ) = setup_with_args(runtime_args! {
        ARG_ENABLE_BURN => true,
        BURNER_LIST => vec![account_user_1_key]
    });

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let minting_recipient: Key = account_user_1_key;
    let mint_amount = U256::one();
    let id = U256::one();

    let mint_call = cep85_mint(
        &mut builder,
        &cep18_contract_hash,
        &minting_account,
        &minting_recipient,
        &id,
        &mint_amount,
        None,
    );

    mint_call.expect_success().commit();

    // owner is now last recipient account_user_1
    let owner: Key = minting_recipient;
    let burn_amount = U256::one();
    let burning_account = minting_account;

    let failing_burn_call = cep85_burn(
        &mut builder,
        &cep18_contract_hash,
        &burning_account,
        &owner,
        &id,
        &burn_amount,
    );
    failing_burn_call.expect_failure();

    let error = builder.get_error().expect("must have error");

    assert_expected_error(
        error,
        Cep85Error::InvalidBurnTarget as u16,
        "should disallow burning of not owned token",
    );
}

#[test]
fn should_allow_contract_to_burn_token() {
    let (
        mut builder,
        TestContext {
            cep18_contract_hash,
            cep85_test_contract,
            ..
        },
    ) = setup_with_args(runtime_args! {
        ARG_ENABLE_BURN => true,
    });
    let contract_key =
        Key::addressable_entity_key(EntityKindTag::SmartContract, cep18_contract_hash);
    let security_lists = SecurityLists {
        burner_list: Some(vec![contract_key]),
        minter_list: None,
        meta_list: None,
        admin_list: None,
        none_list: None,
    };

    let change_security = cep85_change_security(
        &mut builder,
        &cep18_contract_hash,
        &DEFAULT_ACCOUNT_ADDR,
        security_lists,
    );

    // Add test contract to burner list
    change_security.expect_success().commit();

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let minting_recipient = Key::Hash(cep85_test_contract.value());
    let mint_amount = U256::one();
    let id = U256::one();

    let mint_call = cep85_mint(
        &mut builder,
        &cep18_contract_hash,
        &minting_account,
        &minting_recipient,
        &id,
        &mint_amount,
        None,
    );

    mint_call.expect_success().commit();

    let burning_account = minting_account;
    // owner is now last recipient cep85_test_contract
    let owner: Key = minting_recipient;
    let burn_amount = U256::one();

    let burn_call = cep85_burn(
        &mut builder,
        &cep85_test_contract,
        &burning_account,
        &owner,
        &id,
        &burn_amount,
    );
    burn_call.expect_success().commit();
}

#[test]
fn should_allow_contract_to_batch_burn_token() {
    let (
        mut builder,
        TestContext {
            cep18_contract_hash,
            cep85_test_contract,
            ..
        },
    ) = setup_with_args(runtime_args! {
        ARG_ENABLE_BURN => true,
    });

    let contract_key =
        Key::addressable_entity_key(EntityKindTag::SmartContract, cep18_contract_hash);

    let security_lists = SecurityLists {
        burner_list: Some(vec![contract_key]),
        minter_list: None,
        meta_list: None,
        admin_list: None,
        none_list: None,
    };

    let change_security = cep85_change_security(
        &mut builder,
        &cep18_contract_hash,
        &DEFAULT_ACCOUNT_ADDR,
        security_lists,
    );

    // Add test contract to burner list
    change_security.expect_success().commit();

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let minting_recipient = Key::Hash(cep85_test_contract.value());
    let ids: Vec<U256> = vec![U256::one(), U256::from(2)];
    let amounts: Vec<U256> = vec![U256::one(), U256::one()];

    // batch_mint is only one recipient
    let mint_call = cep85_batch_mint(
        &mut builder,
        &cep18_contract_hash,
        &minting_account,
        &minting_recipient,
        ids.clone(),
        amounts.clone(),
        None,
    );

    mint_call.expect_success().commit();

    let burning_account = minting_account;
    // owner is now last recipient cep85_test_contract
    let owner: Key = minting_recipient;

    let burn_call = cep85_batch_burn(
        &mut builder,
        &cep85_test_contract,
        &burning_account,
        &owner,
        ids,
        amounts,
    );
    burn_call.expect_success().commit();
}

#[test]
fn should_allow_contract_package_to_burn_token() {
    let (
        mut builder,
        TestContext {
            cep18_contract_hash,
            cep85_test_contract,
            cep85_test_contract_package,
            ..
        },
    ) = setup_with_args(runtime_args! {
        ARG_ENABLE_BURN => true,
    });

    let security_lists = SecurityLists {
        burner_list: Some(vec![Key::from(cep85_test_contract_package)]),
        minter_list: None,
        meta_list: None,
        admin_list: None,
        none_list: None,
    };

    let change_security = cep85_change_security(
        &mut builder,
        &cep18_contract_hash,
        &DEFAULT_ACCOUNT_ADDR,
        security_lists,
    );

    // Add test contract package to burner list
    change_security.expect_success().commit();

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let minting_recipient = Key::Hash(cep85_test_contract_package.value());
    let mint_amount = U256::one();
    let id = U256::one();

    let mint_call = cep85_mint(
        &mut builder,
        &cep18_contract_hash,
        &minting_account,
        &minting_recipient,
        &id,
        &mint_amount,
        None,
    );

    mint_call.expect_success().commit();

    let burning_account = minting_account;
    // owner is now last recipient cep85_test_contract_package
    let owner: Key = minting_recipient;
    let burn_amount = U256::one();

    let burn_call = cep85_burn(
        &mut builder,
        &cep85_test_contract,
        &burning_account,
        &owner,
        &id,
        &burn_amount,
    );
    burn_call.expect_success().commit();
}

#[test]
fn should_allow_contract_package_to_batch_burn_token() {
    let (
        mut builder,
        TestContext {
            cep18_contract_hash,
            cep85_test_contract,
            cep85_test_contract_package,
            ..
        },
    ) = setup_with_args(runtime_args! {
        ARG_ENABLE_BURN => true,
    });

    let security_lists = SecurityLists {
        burner_list: Some(vec![Key::from(cep85_test_contract_package)]),
        minter_list: None,
        meta_list: None,
        admin_list: None,
        none_list: None,
    };

    let change_security = cep85_change_security(
        &mut builder,
        &cep18_contract_hash,
        &DEFAULT_ACCOUNT_ADDR,
        security_lists,
    );

    // Add test contract package to burner list
    change_security.expect_success().commit();

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let minting_recipient = Key::Hash(cep85_test_contract_package.value());
    let ids: Vec<U256> = vec![U256::one(), U256::from(2)];
    let amounts: Vec<U256> = vec![U256::one(), U256::one()];

    // batch_mint is only one recipient
    let mint_call = cep85_batch_mint(
        &mut builder,
        &cep18_contract_hash,
        &minting_account,
        &minting_recipient,
        ids.clone(),
        amounts.clone(),
        None,
    );

    mint_call.expect_success().commit();

    let burning_account = minting_account;
    // owner is now last recipient cep85_test_contract_package
    let owner: Key = minting_recipient;

    let burn_call = cep85_batch_burn(
        &mut builder,
        &cep85_test_contract,
        &burning_account,
        &owner,
        ids,
        amounts,
    );
    burn_call.expect_success().commit();
}

#[test]
fn should_allow_operator_to_burn_token() {
    let (account_user_1_key, account_user_1_account_hash, _) = get_test_account("ACCOUNT_USER_1");

    let (
        mut builder,
        TestContext {
            cep18_contract_hash,
            cep85_test_contract_package,
            ..
        },
    ) = setup_with_args(runtime_args! {
        ARG_ENABLE_BURN => true,
        BURNER_LIST => vec![account_user_1_key]
    });

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let minting_recipient: Key =
        Key::AddressableEntity(casper_types::EntityAddr::Account(minting_account.value()));
    let mint_amount = U256::one();
    let id = U256::one();

    let mint_call = cep85_mint(
        &mut builder,
        &cep18_contract_hash,
        &minting_account,
        &minting_recipient,
        &id,
        &mint_amount,
        None,
    );

    mint_call.expect_success().commit();

    let operator = account_user_1_key;
    let burning_account = account_user_1_account_hash;
    let owner: Key = minting_recipient;
    let burn_amount = U256::one();

    let failing_burn_call = cep85_burn(
        &mut builder,
        &cep18_contract_hash,
        &burning_account,
        &owner,
        &id,
        &burn_amount,
    );
    failing_burn_call.expect_failure();

    let error = builder.get_error().expect("must have error");

    assert_expected_error(
        error,
        Cep85Error::InvalidBurnTarget as u16,
        "only owner can burn its token",
    );

    let approved = true;

    let set_approval_for_all_call = cep85_set_approval_for_all(
        &mut builder,
        &cep18_contract_hash,
        &minting_account,
        &operator,
        approved,
    );
    set_approval_for_all_call.expect_success().commit();

    let is_approved = cep85_check_is_approved(
        &mut builder,
        &cep85_test_contract_package,
        &owner,
        &operator,
    );

    assert_eq!(is_approved, approved);

    let burn_call = cep85_burn(
        &mut builder,
        &cep18_contract_hash,
        &burning_account,
        &owner,
        &id,
        &burn_amount,
    );
    burn_call.expect_success().commit();
}

#[test]
fn should_allow_operator_to_batch_burn_token() {
    let (account_user_1_key, account_user_1_account_hash, _) = get_test_account("ACCOUNT_USER_1");

    let (
        mut builder,
        TestContext {
            cep18_contract_hash,
            cep85_test_contract_package,
            ..
        },
    ) = setup_with_args(runtime_args! {
        ARG_ENABLE_BURN => true,
        BURNER_LIST => vec![account_user_1_key]
    });

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let minting_recipient: Key =
        Key::AddressableEntity(casper_types::EntityAddr::Account(minting_account.value()));
    let ids: Vec<U256> = vec![U256::one(), U256::from(2)];
    let amounts: Vec<U256> = vec![U256::one(), U256::one()];

    // batch_mint is only one recipient
    let mint_call = cep85_batch_mint(
        &mut builder,
        &cep18_contract_hash,
        &minting_account,
        &minting_recipient,
        ids.clone(),
        amounts.clone(),
        None,
    );

    mint_call.expect_success().commit();

    let operator = account_user_1_key;
    let burning_account = account_user_1_account_hash;
    let owner: Key = minting_recipient;

    let failing_burn_call = cep85_batch_burn(
        &mut builder,
        &cep18_contract_hash,
        &burning_account,
        &owner,
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

    let approved = true;

    let set_approval_for_all_call = cep85_set_approval_for_all(
        &mut builder,
        &cep18_contract_hash,
        &minting_account,
        &operator,
        approved,
    );
    set_approval_for_all_call.expect_success().commit();

    let is_approved = cep85_check_is_approved(
        &mut builder,
        &cep85_test_contract_package,
        &owner,
        &operator,
    );

    assert_eq!(is_approved, approved);

    let burn_call = cep85_batch_burn(
        &mut builder,
        &cep18_contract_hash,
        &burning_account,
        &owner,
        ids,
        amounts,
    );

    burn_call.expect_success().commit();
}

#[test]
fn should_allow_contract_as_operator_to_burn_token() {
    let (account_user_1_key, account_user_1_account_hash, _) = get_test_account("ACCOUNT_USER_1");

    let (
        mut builder,
        TestContext {
            cep18_contract_hash,
            cep85_test_contract,
            cep85_test_contract_package,
            ..
        },
    ) = setup_with_args(runtime_args! {
        ARG_ENABLE_BURN => true,
    });

    let contract_key =
        Key::addressable_entity_key(EntityKindTag::SmartContract, cep18_contract_hash);

    let security_lists = SecurityLists {
        burner_list: Some(vec![contract_key]),
        minter_list: None,
        meta_list: None,
        admin_list: None,
        none_list: None,
    };

    let change_security = cep85_change_security(
        &mut builder,
        &cep18_contract_hash,
        &DEFAULT_ACCOUNT_ADDR,
        security_lists,
    );

    // Add test contract to burner list
    change_security.expect_success().commit();

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let minting_recipient = account_user_1_key;
    let mint_amount = U256::one();
    let id = U256::one();

    let mint_call = cep85_mint(
        &mut builder,
        &cep18_contract_hash,
        &minting_account,
        &minting_recipient,
        &id,
        &mint_amount,
        None,
    );

    mint_call.expect_success().commit();

    let operator = Key::addressable_entity_key(EntityKindTag::SmartContract, cep85_test_contract);
    let burning_account = minting_account;
    let owner = account_user_1_key;
    let burn_amount = U256::one();

    let failing_burn_call = cep85_burn(
        &mut builder,
        &cep18_contract_hash,
        &burning_account,
        &owner,
        &id,
        &burn_amount,
    );
    failing_burn_call.expect_failure();

    let error = builder.get_error().expect("must have error");

    assert_expected_error(
        error,
        Cep85Error::InvalidBurnTarget as u16,
        "only owner or operator can burn its token",
    );

    let approved = true;
    let approving_account = account_user_1_account_hash;

    let set_approval_for_all_call = cep85_set_approval_for_all(
        &mut builder,
        &cep18_contract_hash,
        &approving_account,
        &operator,
        approved,
    );
    set_approval_for_all_call.expect_success().commit();

    let is_approved = cep85_check_is_approved(
        &mut builder,
        &cep85_test_contract_package,
        &owner,
        &operator,
    );

    assert_eq!(is_approved, approved);

    let burn_call = cep85_burn(
        &mut builder,
        &cep85_test_contract,
        &burning_account,
        &owner,
        &id,
        &burn_amount,
    );
    burn_call.expect_success().commit();
}

#[test]
fn should_allow_contract_as_operator_to_batch_burn_token() {
    let (account_user_1_key, account_user_1_account_hash, _) = get_test_account("ACCOUNT_USER_1");

    let (
        mut builder,
        TestContext {
            cep18_contract_hash,
            cep85_test_contract,
            cep85_test_contract_package,
            ..
        },
    ) = setup_with_args(runtime_args! {
        ARG_ENABLE_BURN => true,
    });

    let contract_key =
        Key::addressable_entity_key(EntityKindTag::SmartContract, cep18_contract_hash);

    let security_lists = SecurityLists {
        burner_list: Some(vec![contract_key]),
        minter_list: None,
        meta_list: None,
        admin_list: None,
        none_list: None,
    };

    let change_security = cep85_change_security(
        &mut builder,
        &cep18_contract_hash,
        &DEFAULT_ACCOUNT_ADDR,
        security_lists,
    );

    // Add test contract to burner list
    change_security.expect_success().commit();

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let minting_recipient = account_user_1_key;
    let ids: Vec<U256> = vec![U256::one(), U256::from(2)];
    let amounts: Vec<U256> = vec![U256::one(), U256::one()];

    // batch_mint is only one recipient
    let mint_call = cep85_batch_mint(
        &mut builder,
        &cep18_contract_hash,
        &minting_account,
        &minting_recipient,
        ids.clone(),
        amounts.clone(),
        None,
    );

    mint_call.expect_success().commit();

    let operator = Key::addressable_entity_key(EntityKindTag::SmartContract, cep85_test_contract);
    let burning_account = minting_account;
    let owner = account_user_1_key;

    let failing_burn_call = cep85_batch_burn(
        &mut builder,
        &cep85_test_contract,
        &burning_account,
        &owner,
        ids.clone(),
        amounts.clone(),
    );
    failing_burn_call.expect_failure();

    let error = builder.get_error().expect("must have error");

    assert_expected_error(
        error,
        Cep85Error::InvalidBurnTarget as u16,
        "only owner or operator can burn its token",
    );

    let approved = true;
    let approving_account = account_user_1_account_hash;

    let set_approval_for_all_call = cep85_set_approval_for_all(
        &mut builder,
        &cep18_contract_hash,
        &approving_account,
        &operator,
        approved,
    );
    set_approval_for_all_call.expect_success().commit();

    let is_approved = cep85_check_is_approved(
        &mut builder,
        &cep85_test_contract_package,
        &owner,
        &operator,
    );

    assert_eq!(is_approved, approved);

    let burn_call = cep85_batch_burn(
        &mut builder,
        &cep85_test_contract,
        &burning_account,
        &owner,
        ids,
        amounts,
    );
    burn_call.expect_success().commit();
}

#[test]
fn should_allow_contract_package_as_operator_to_burn_token() {
    let (account_user_1_key, account_user_1_account_hash, _) = get_test_account("ACCOUNT_USER_1");

    let (
        mut builder,
        TestContext {
            cep18_contract_hash,
            cep85_test_contract,
            cep85_test_contract_package,
            ..
        },
    ) = setup_with_args(runtime_args! {
        ARG_ENABLE_BURN => true,
    });

    let security_lists = SecurityLists {
        burner_list: Some(vec![Key::from(cep85_test_contract_package)]),
        minter_list: None,
        meta_list: None,
        admin_list: None,
        none_list: None,
    };

    let change_security = cep85_change_security(
        &mut builder,
        &cep18_contract_hash,
        &DEFAULT_ACCOUNT_ADDR,
        security_lists,
    );

    // Add test contract package to burner list
    change_security.expect_success().commit();

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let minting_recipient = account_user_1_key;
    let mint_amount = U256::one();
    let id = U256::one();

    let mint_call = cep85_mint(
        &mut builder,
        &cep18_contract_hash,
        &minting_account,
        &minting_recipient,
        &id,
        &mint_amount,
        None,
    );

    mint_call.expect_success().commit();

    let operator = Key::from(cep85_test_contract_package);
    let burning_account = minting_account;
    let owner = account_user_1_key;
    let burn_amount = U256::one();

    let failing_burn_call = cep85_burn(
        &mut builder,
        &cep18_contract_hash,
        &burning_account,
        &owner,
        &id,
        &burn_amount,
    );
    failing_burn_call.expect_failure();

    let error = builder.get_error().expect("must have error");

    assert_expected_error(
        error,
        Cep85Error::InvalidBurnTarget as u16,
        "only owner or operator can burn its token",
    );

    let approved = true;
    let approving_account = account_user_1_account_hash;

    let set_approval_for_all_call = cep85_set_approval_for_all(
        &mut builder,
        &cep18_contract_hash,
        &approving_account,
        &operator,
        approved,
    );
    set_approval_for_all_call.expect_success().commit();

    let is_approved = cep85_check_is_approved(
        &mut builder,
        &cep85_test_contract_package,
        &owner,
        &operator,
    );

    assert_eq!(is_approved, approved);

    let burn_call = cep85_burn(
        &mut builder,
        &cep85_test_contract,
        &burning_account,
        &owner,
        &id,
        &burn_amount,
    );
    burn_call.expect_success().commit();
}

#[test]
fn should_allow_contract_package_as_operator_to_batch_burn_token() {
    let (account_user_1_key, account_user_1_account_hash, _) = get_test_account("ACCOUNT_USER_1");

    let (
        mut builder,
        TestContext {
            cep18_contract_hash,
            cep85_test_contract,
            cep85_test_contract_package,
            ..
        },
    ) = setup_with_args(runtime_args! {
        ARG_ENABLE_BURN => true,
    });

    let security_lists = SecurityLists {
        burner_list: Some(vec![Key::from(cep85_test_contract_package)]),
        minter_list: None,
        meta_list: None,
        admin_list: None,
        none_list: None,
    };

    let change_security = cep85_change_security(
        &mut builder,
        &cep18_contract_hash,
        &DEFAULT_ACCOUNT_ADDR,
        security_lists,
    );

    // Add test contract package to burner list
    change_security.expect_success().commit();

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let minting_recipient = account_user_1_key;
    let ids: Vec<U256> = vec![U256::one(), U256::from(2)];
    let amounts: Vec<U256> = vec![U256::one(), U256::one()];

    // batch_mint is only one recipient
    let mint_call = cep85_batch_mint(
        &mut builder,
        &cep18_contract_hash,
        &minting_account,
        &minting_recipient,
        ids.clone(),
        amounts.clone(),
        None,
    );

    mint_call.expect_success().commit();

    let operator = Key::from(cep85_test_contract_package);
    let burning_account = minting_account;
    let owner = account_user_1_key;

    let failing_burn_call = cep85_batch_burn(
        &mut builder,
        &cep85_test_contract,
        &burning_account,
        &owner,
        ids.clone(),
        amounts.clone(),
    );
    failing_burn_call.expect_failure();

    let error = builder.get_error().expect("must have error");

    assert_expected_error(
        error,
        Cep85Error::InvalidBurnTarget as u16,
        "only owner or operator can burn its token",
    );

    let approved = true;
    let approving_account = account_user_1_account_hash;

    let set_approval_for_all_call = cep85_set_approval_for_all(
        &mut builder,
        &cep18_contract_hash,
        &approving_account,
        &operator,
        approved,
    );
    set_approval_for_all_call.expect_success().commit();

    let is_approved = cep85_check_is_approved(
        &mut builder,
        &cep85_test_contract_package,
        &owner,
        &operator,
    );

    assert_eq!(is_approved, approved);

    let burn_call = cep85_batch_burn(
        &mut builder,
        &cep85_test_contract,
        &burning_account,
        &owner,
        ids,
        amounts,
    );
    burn_call.expect_success().commit();
}

#[test]
fn should_not_burn_in_non_burn_mode() {
    let (
        mut builder,
        TestContext {
            cep18_contract_hash,
            ..
        },
    ) = setup_with_args(runtime_args! {
        ARG_ENABLE_BURN => false,
    });

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let minting_recipient: Key =
        Key::AddressableEntity(casper_types::EntityAddr::Account(minting_account.value()));
    let mint_amount = U256::one();
    let id = U256::one();

    let mint_call = cep85_mint(
        &mut builder,
        &cep18_contract_hash,
        &minting_account,
        &minting_recipient,
        &id,
        &mint_amount,
        None,
    );

    mint_call.expect_success().commit();

    let burning_account = minting_account;
    // owner is now last recipient DEFAULT_ACCOUNT_ADDR
    let owner: Key = minting_recipient;
    let burn_amount = U256::one();

    let failing_burn_call = cep85_burn(
        &mut builder,
        &cep18_contract_hash,
        &burning_account,
        &owner,
        &id,
        &burn_amount,
    );
    failing_burn_call.expect_failure();

    let error = builder.get_error().expect("must have error");

    assert_expected_error(
        error,
        Cep85Error::BurnDisabled as u16,
        "Can not burn when burn mode is disabled",
    );
}
