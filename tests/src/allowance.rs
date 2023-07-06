use casper_engine_test_support::DEFAULT_ACCOUNT_ADDR;
use casper_types::{bytesrepr::Bytes, Key, U256};
use cep85::error::Cep85Error;

use crate::utility::{
    constants::ACCOUNT_USER_1,
    installer_request_builders::{
        cep85_check_balance_of, cep85_mint, cep85_set_approval_for_all, cep85_transfer_from, setup,
        TestContext, TransferData,
    },
    support::assert_expected_error,
};

#[test]
fn should_not_allow_self_approval() {
    let (
        mut builder,
        TestContext {
            cep85_token,
            test_accounts,
            ..
        },
    ) = setup();

    let account_user_1 = *test_accounts.get(&ACCOUNT_USER_1).unwrap();
    let operator = Key::from(account_user_1);
    let approved = true;

    let set_approval_for_all_call = cep85_set_approval_for_all(
        &mut builder,
        &cep85_token,
        &account_user_1,
        &operator,
        approved,
    );
    set_approval_for_all_call.expect_failure();

    let error = builder.get_error().expect("must have error");

    assert_expected_error(
        error,
        Cep85Error::SelfOperatorApproval as u16,
        "setting self approval is not allowed",
    );
}

#[test]
fn should_not_transfer_from_without_allowance() {
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
    let minting_recipient = Key::from(minting_account);
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

    let from = minting_recipient;
    let account_user_1 = *test_accounts.get(&ACCOUNT_USER_1).unwrap();
    let to = Key::from(account_user_1);
    let transfer_amount = U256::one();
    let data: Vec<Bytes> = vec![];

    // Let's try to send as account_user_1 a transfer request from owner to account_user_1, this
    // request should fail
    let transfer_call = cep85_transfer_from(
        &mut builder,
        &cep85_token,
        &account_user_1,
        TransferData {
            from: &from,
            to: &to,
            ids: vec![id],
            amounts: vec![transfer_amount],
            data,
        },
        None,
    );
    transfer_call.expect_failure();

    let error = builder.get_error().expect("must have error");

    assert_expected_error(
        error,
        Cep85Error::NotApproved as u16,
        "Only owner or approved operator can transfer token on behalf of token owner",
    );

    let actual_balance_from =
        cep85_check_balance_of(&mut builder, &cep85_test_contract_package, &from, &id);
    let expected_balance_from = U256::one();

    assert_eq!(actual_balance_from, expected_balance_from);

    let actual_balance_to =
        cep85_check_balance_of(&mut builder, &cep85_test_contract_package, &to, &id);
    let expected_balance_to = U256::zero();

    assert_eq!(actual_balance_to, expected_balance_to);
}

#[test]
fn should_transfer_from_without_with_allowance() {
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
    let minting_recipient = Key::from(minting_account);
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

    let account_user_1 = *test_accounts.get(&ACCOUNT_USER_1).unwrap();

    let operator = Key::from(account_user_1);
    let approved = true;

    let set_approval_for_all_call = cep85_set_approval_for_all(
        &mut builder,
        &cep85_token,
        &minting_account,
        &operator,
        approved,
    );
    set_approval_for_all_call.expect_success().commit();

    // Let's try to send as account_user_1 a transfer request from owner to account_user_1, this
    // request should succeed as account_user_1 is operator for owner DEFAULT_ACCOUNT_ADDR
    let from = minting_recipient;
    let to = operator; // operator will also also recipient of the transfer
    let transfer_amount = U256::one();
    let data: Vec<Bytes> = vec![];
    let transfer_call = cep85_transfer_from(
        &mut builder,
        &cep85_token,
        &account_user_1, // account_user_1 is now operator
        TransferData {
            from: &from,
            to: &to,
            ids: vec![id],
            amounts: vec![transfer_amount],
            data,
        },
        None,
    );
    transfer_call.expect_success().commit();

    let actual_balance_from =
        cep85_check_balance_of(&mut builder, &cep85_test_contract_package, &from, &id);
    let expected_balance_from = U256::zero();

    assert_eq!(actual_balance_from, expected_balance_from);

    let actual_balance_to =
        cep85_check_balance_of(&mut builder, &cep85_test_contract_package, &to, &id);
    let expected_balance_to = U256::one();

    assert_eq!(actual_balance_to, expected_balance_to);
}
