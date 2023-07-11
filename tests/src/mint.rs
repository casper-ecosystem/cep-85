use crate::utility::{
    constants::ACCOUNT_USER_1,
    installer_request_builders::{
        cep85_batch_mint, cep85_check_balance_of, cep85_check_balance_of_batch, cep85_mint,
        cep85_set_total_supply_of, cep85_set_total_supply_of_batch, setup, TestContext,
    },
    support::assert_expected_error,
};

use casper_engine_test_support::DEFAULT_ACCOUNT_ADDR;
use casper_types::{Key, U256};
use cep85::error::Cep85Error;

#[test]
fn should_mint() {
    let (
        mut builder,
        TestContext {
            cep85_token,
            cep85_test_contract_package,
            test_accounts,
            ..
        },
    ) = setup();

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let minting_recipient: Key = Key::from(*test_accounts.get(&ACCOUNT_USER_1).unwrap());
    let mint_amount = U256::one();
    let id = U256::one();

    let mint_call = cep85_mint(
        &mut builder,
        &cep85_token,
        &minting_account,
        &minting_recipient,
        &id,
        &mint_amount,
    );

    mint_call.expect_success().commit();

    let actual_balance = cep85_check_balance_of(
        &mut builder,
        &cep85_test_contract_package,
        &minting_recipient,
        &id,
    );
    let expected_balance = U256::one();

    assert_eq!(actual_balance, expected_balance);
}

#[test]
fn should_batch_mint() {
    let (
        mut builder,
        TestContext {
            cep85_token,
            cep85_test_contract_package,
            test_accounts,
            ..
        },
    ) = setup();

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let minting_recipient = Key::from(*test_accounts.get(&ACCOUNT_USER_1).unwrap());
    let ids: Vec<U256> = vec![U256::one(), U256::from(2)];
    let amounts: Vec<U256> = vec![U256::one(), U256::one()];

    // batch_mint is only one recipient
    let mint_call = cep85_batch_mint(
        &mut builder,
        &cep85_token,
        &minting_account,
        &minting_recipient,
        ids.clone(),
        amounts,
    );

    mint_call.expect_success().commit();

    let recipients: Vec<Key> = vec![minting_recipient, minting_recipient];

    let actual_balances =
        cep85_check_balance_of_batch(&mut builder, &cep85_test_contract_package, recipients, ids);

    let expected_balances = vec![U256::one(), U256::one()];

    assert_eq!(actual_balances, expected_balances);
}

#[test]
fn should_not_mint_above_total_supply() {
    let (
        mut builder,
        TestContext {
            cep85_token,
            test_accounts,
            cep85_test_contract_package,
            ..
        },
    ) = setup();

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let minting_recipient: Key = Key::from(*test_accounts.get(&ACCOUNT_USER_1).unwrap());
    let id = U256::one();
    // Token total supply has not been raised to 2, mint request should fail
    let mint_amount = U256::from(2);

    let failing_mint_call = cep85_mint(
        &mut builder,
        &cep85_token,
        &minting_account,
        &minting_recipient,
        &id,
        &mint_amount,
    );

    failing_mint_call.expect_failure();

    let error = builder.get_error().expect("must have error");

    assert_expected_error(
        error,
        Cep85Error::ExceededMaxTotalSupply as u16,
        "should not allow to mint above total supply",
    );

    // Set total supply to 2 for the token to be minted
    let total_supply = U256::from(2);
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

    let actual_balance = cep85_check_balance_of(
        &mut builder,
        &cep85_test_contract_package,
        &minting_recipient,
        &id,
    );
    let expected_balance = total_supply;

    assert_eq!(actual_balance, expected_balance);
}

#[test]
fn should_not_batch_mint_above_total_supply() {
    let (
        mut builder,
        TestContext {
            cep85_token,
            test_accounts,
            cep85_test_contract_package,
            ..
        },
    ) = setup();

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let minting_recipient = Key::from(*test_accounts.get(&ACCOUNT_USER_1).unwrap());
    let ids: Vec<U256> = vec![U256::one(), U256::from(2)];

    // Token total supply has not been raised to 2, batch mint request should fail
    let amounts: Vec<U256> = vec![U256::from(2), U256::from(3)];

    // Batch_mint is only one recipient
    let failing_mint_call = cep85_batch_mint(
        &mut builder,
        &cep85_token,
        &minting_account,
        &minting_recipient,
        ids.clone(),
        amounts.clone(),
    );

    failing_mint_call.expect_failure();

    let error = builder.get_error().expect("must have error");

    assert_expected_error(
        error,
        Cep85Error::ExceededMaxTotalSupply as u16,
        "should not allow to mint above total supply",
    );

    // Set total supply for each ID using batch function
    let total_supplies = vec![U256::from(2), U256::from(3)];
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
        amounts,
    );

    batch_mint_call.expect_success().commit();

    let recipients: Vec<Key> = vec![minting_recipient, minting_recipient];

    let actual_balances =
        cep85_check_balance_of_batch(&mut builder, &cep85_test_contract_package, recipients, ids);

    let expected_balances = total_supplies;

    assert_eq!(actual_balances, expected_balances);
}
