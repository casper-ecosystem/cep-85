use crate::utility::{
    constants::{ACCOUNT_USER_1, ACCOUNT_USER_2},
    installer_request_builders::{
        cep85_batch_mint, cep85_check_balance_of, cep85_check_balance_of_batch, cep85_mint, setup,
        TestContext,
    },
    support::assert_expected_error,
};
use casper_engine_test_support::{ExecuteRequestBuilder, DEFAULT_ACCOUNT_ADDR};
use casper_types::{runtime_args, Key, RuntimeArgs, U256};
use cep85::{
    constants::{ARG_ACCOUNTS, ARG_IDS},
    error::Cep85Error,
};
use cep85_test_contract::constants::ENTRY_POINT_CHECK_BALANCE_OF_BATCH;
#[test]
fn should_check_balance_of() {
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
    let recipient: Key = Key::from(*test_accounts.get(&ACCOUNT_USER_1).unwrap());
    let mint_amount = U256::one();
    let id = U256::one();

    let mint_call = cep85_mint(
        &mut builder,
        &cep85_token,
        &minting_account,
        &recipient,
        &id,
        &mint_amount,
    );

    mint_call.expect_success().commit();

    let actual_balance =
        cep85_check_balance_of(&mut builder, &cep85_test_contract_package, &recipient, &id);
    let expected_balance = U256::one();

    assert_eq!(actual_balance, expected_balance);
}

#[test]
fn should_check_balance_of_batch() {
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
    let recipient_user_1 = Key::from(*test_accounts.get(&ACCOUNT_USER_1).unwrap());
    let recipient_user_2 = Key::from(*test_accounts.get(&ACCOUNT_USER_2).unwrap());
    let mut ids: Vec<U256> = vec![U256::one(), U256::from(2)];
    let amounts: Vec<U256> = vec![U256::one(), U256::one()];

    // batch_mint is only one recipient
    let mint_call = cep85_batch_mint(
        &mut builder,
        &cep85_token,
        &minting_account,
        &recipient_user_1,
        ids.clone(),
        amounts,
    );

    mint_call.expect_success().commit();

    // Add a new recipient for token id 3, balance will be zero
    ids.push(U256::from(3));
    let recipients: Vec<Key> = vec![recipient_user_1, recipient_user_1, recipient_user_2];

    let actual_balances =
        cep85_check_balance_of_batch(&mut builder, &cep85_test_contract_package, recipients, ids);

    let expected_balances = vec![U256::one(), U256::one(), U256::zero()];

    assert_eq!(actual_balances, expected_balances);
}

#[test]
fn should_error_on_balance_of_batch_args_len_difference() {
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
    let recipient = Key::from(*test_accounts.get(&ACCOUNT_USER_1).unwrap());
    let mut ids: Vec<U256> = vec![U256::one()];
    let amounts: Vec<U256> = vec![U256::one()];

    // batch_mint is only one recipient
    let mint_call = cep85_batch_mint(
        &mut builder,
        &cep85_token,
        &minting_account,
        &recipient,
        ids.clone(),
        amounts,
    );

    mint_call.expect_success().commit();

    let accounts: Vec<Key> = vec![recipient, recipient];

    let check_balance_args = runtime_args! {
        ARG_ACCOUNTS => accounts,
        ARG_IDS => ids.clone(),
    };
    let exec_request = ExecuteRequestBuilder::versioned_contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        cep85_test_contract_package,
        None,
        ENTRY_POINT_CHECK_BALANCE_OF_BATCH,
        check_balance_args,
    )
    .build();
    builder.exec(exec_request).expect_failure();

    let error = builder.get_error().expect("must have error");

    assert_expected_error(
        error,
        Cep85Error::MismatchParamsLength as u16,
        "ids and recipients should have same length",
    );

    // check again the opposite len diff
    ids.push(U256::one());
    assert!(ids.len() == 2_usize);
    let accounts: Vec<Key> = vec![recipient];

    let check_balance_args = runtime_args! {
        ARG_ACCOUNTS => accounts,
        ARG_IDS => ids.clone(),
    };
    let exec_request = ExecuteRequestBuilder::versioned_contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        cep85_test_contract_package,
        None,
        ENTRY_POINT_CHECK_BALANCE_OF_BATCH,
        check_balance_args,
    )
    .build();
    builder.exec(exec_request).expect_failure();

    let error = builder.get_error().expect("must have error");

    assert_expected_error(
        error,
        Cep85Error::MismatchParamsLength as u16,
        "ids and recipients should have same length",
    );
}
