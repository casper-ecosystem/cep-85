use casper_engine_test_support::DEFAULT_ACCOUNT_ADDR;
use casper_types::{bytesrepr::Bytes, EntityAddr, Key, U256};
use cep85::{constants::DEFAULT_DICT_ITEM_KEY_NAME, error::Cep85Error};

use crate::utility::{
    installer_request_builders::{
        cep85_batch_mint, cep85_batch_transfer_from, cep85_batch_transfer_from_as_contract,
        cep85_check_balance_of, cep85_check_balance_of_batch, cep85_check_is_approved,
        cep85_make_dictionary_item_key, cep85_mint, cep85_set_approval_for_all,
        cep85_transfer_from, cep85_transfer_from_as_contract, setup, TestContext, TransferData,
    },
    support::{assert_expected_error, get_test_account},
};

#[test]
fn should_not_allow_self_approval() {
    let (account_user_1_key, account_user_1_account_hash, _) = get_test_account("ACCOUNT_USER_1");

    let (
        mut builder,
        TestContext {
            cep85_contract_hash,

            cep85_test_contract_package,
            ..
        },
    ) = setup();

    let operator = account_user_1_key;
    let approved = true;

    let failing_set_approval_for_all_call = cep85_set_approval_for_all(
        &mut builder,
        &cep85_contract_hash,
        &account_user_1_account_hash,
        &operator,
        approved,
    );
    failing_set_approval_for_all_call.expect_failure();

    let error = builder.get_error().expect("must have error");

    assert_expected_error(
        error,
        Cep85Error::SelfOperatorApproval as u16,
        "setting self approval is not allowed",
    );

    let is_approved = cep85_check_is_approved(
        &mut builder,
        &cep85_test_contract_package,
        &operator,
        &operator,
    );

    assert_eq!(is_approved, !approved);
}

#[test]
fn should_allow_approval_of_an_account() {
    let (account_user_1_key, _, _) = get_test_account("ACCOUNT_USER_1");

    let (
        mut builder,
        TestContext {
            cep85_contract_hash,
            cep85_test_contract_package,
            ..
        },
    ) = setup();

    let owner = *DEFAULT_ACCOUNT_ADDR;
    let approving_account = Key::AddressableEntity(EntityAddr::Account(owner.value()));

    let operator = account_user_1_key;
    let approved = true;

    let set_approval_for_all_call = cep85_set_approval_for_all(
        &mut builder,
        &cep85_contract_hash,
        &owner,
        &operator,
        approved,
    );
    set_approval_for_all_call.expect_success().commit();

    let is_approved = cep85_check_is_approved(
        &mut builder,
        &cep85_test_contract_package,
        &approving_account,
        &operator,
    );

    assert_eq!(is_approved, approved);
}

#[test]
fn should_allow_approval_of_a_contract() {
    let (account_user_1_key, _, _) = get_test_account("ACCOUNT_USER_1");

    let (
        mut builder,
        TestContext {
            cep85_contract_hash,
            cep85_test_contract_package,
            ..
        },
    ) = setup();

    let owner = *DEFAULT_ACCOUNT_ADDR;
    let approving_account = Key::AddressableEntity(EntityAddr::Account(owner.value()));
    let operator = account_user_1_key;
    let approved = true;

    let set_approval_for_all_call = cep85_set_approval_for_all(
        &mut builder,
        &cep85_contract_hash,
        &owner,
        &operator,
        approved,
    );
    set_approval_for_all_call.expect_success().commit();

    let is_approved = cep85_check_is_approved(
        &mut builder,
        &cep85_test_contract_package,
        &approving_account,
        &operator,
    );

    assert_eq!(is_approved, approved);
}

#[test]
fn should_allow_approval_of_a_package() {
    let (
        mut builder,
        TestContext {
            cep85_contract_hash,
            cep85_test_contract_package,
            ..
        },
    ) = setup();

    let owner = *DEFAULT_ACCOUNT_ADDR;
    let approving_account = Key::AddressableEntity(EntityAddr::Account(owner.value()));
    let operator = Key::from(cep85_test_contract_package);
    let approved = true;

    let set_approval_for_all_call = cep85_set_approval_for_all(
        &mut builder,
        &cep85_contract_hash,
        &owner,
        &operator,
        approved,
    );
    set_approval_for_all_call.expect_success().commit();

    let is_approved = cep85_check_is_approved(
        &mut builder,
        &cep85_test_contract_package,
        &approving_account,
        &operator,
    );

    assert_eq!(is_approved, approved);
}

#[test]
fn should_remove_approval_of_an_account() {
    let (account_user_1_key, _, _) = get_test_account("ACCOUNT_USER_1");
    let (
        mut builder,
        TestContext {
            cep85_contract_hash,

            cep85_test_contract_package,
            ..
        },
    ) = setup();

    let owner = *DEFAULT_ACCOUNT_ADDR;
    let approving_account = Key::AddressableEntity(EntityAddr::Account(owner.value()));

    let operator = account_user_1_key;
    let approved = true;

    let set_approval_for_all_call = cep85_set_approval_for_all(
        &mut builder,
        &cep85_contract_hash,
        &DEFAULT_ACCOUNT_ADDR,
        &operator,
        approved,
    );
    set_approval_for_all_call.expect_success().commit();

    let is_approved = cep85_check_is_approved(
        &mut builder,
        &cep85_test_contract_package,
        &approving_account,
        &operator,
    );

    assert_eq!(is_approved, approved);

    let not_approved = !approved;

    let set_approval_for_all_call = cep85_set_approval_for_all(
        &mut builder,
        &cep85_contract_hash,
        &DEFAULT_ACCOUNT_ADDR,
        &operator,
        not_approved,
    );
    set_approval_for_all_call.expect_success().commit();

    let is_approved = cep85_check_is_approved(
        &mut builder,
        &cep85_test_contract_package,
        &approving_account,
        &operator,
    );

    assert_eq!(is_approved, not_approved);
}

#[test]
fn should_remove_approval_of_a_contract() {
    let (account_user_1_key, _, _) = get_test_account("ACCOUNT_USER_1");
    let (
        mut builder,
        TestContext {
            cep85_contract_hash,
            cep85_test_contract_package,
            ..
        },
    ) = setup();

    let owner = *DEFAULT_ACCOUNT_ADDR;
    let approving_account = Key::AddressableEntity(EntityAddr::Account(owner.value()));
    let operator = account_user_1_key;
    let approved = true;

    let set_approval_for_all_call = cep85_set_approval_for_all(
        &mut builder,
        &cep85_contract_hash,
        &owner,
        &operator,
        approved,
    );
    set_approval_for_all_call.expect_success().commit();

    let is_approved = cep85_check_is_approved(
        &mut builder,
        &cep85_test_contract_package,
        &approving_account,
        &operator,
    );

    assert_eq!(is_approved, approved);

    let not_approved = !approved;

    let set_approval_for_all_call = cep85_set_approval_for_all(
        &mut builder,
        &cep85_contract_hash,
        &DEFAULT_ACCOUNT_ADDR,
        &operator,
        not_approved,
    );
    set_approval_for_all_call.expect_success().commit();

    let is_approved = cep85_check_is_approved(
        &mut builder,
        &cep85_test_contract_package,
        &approving_account,
        &operator,
    );

    assert_eq!(is_approved, not_approved);
}

#[test]
fn should_remove_approval_of_a_package() {
    let (
        mut builder,
        TestContext {
            cep85_contract_hash,
            cep85_test_contract_package,
            ..
        },
    ) = setup();

    let owner = *DEFAULT_ACCOUNT_ADDR;
    let approving_account = Key::AddressableEntity(EntityAddr::Account(owner.value()));
    let operator = Key::from(cep85_test_contract_package);
    let approved = true;

    let set_approval_for_all_call = cep85_set_approval_for_all(
        &mut builder,
        &cep85_contract_hash,
        &owner,
        &operator,
        approved,
    );
    set_approval_for_all_call.expect_success().commit();

    let is_approved = cep85_check_is_approved(
        &mut builder,
        &cep85_test_contract_package,
        &approving_account,
        &operator,
    );

    assert_eq!(is_approved, approved);

    let not_approved = !approved;

    let set_approval_for_all_call = cep85_set_approval_for_all(
        &mut builder,
        &cep85_contract_hash,
        &DEFAULT_ACCOUNT_ADDR,
        &operator,
        not_approved,
    );
    set_approval_for_all_call.expect_success().commit();

    let is_approved = cep85_check_is_approved(
        &mut builder,
        &cep85_test_contract_package,
        &approving_account,
        &operator,
    );

    assert_eq!(is_approved, not_approved);
}

#[test]
fn should_not_transfer_from_account_to_account_without_allowance() {
    let (account_user_1_key, account_user_1_account_hash, _) = get_test_account("ACCOUNT_USER_1");

    let (
        mut builder,
        TestContext {
            cep85_contract_hash,
            cep85_test_contract_package,
            ..
        },
    ) = setup();

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let minting_recipient = Key::AddressableEntity(EntityAddr::Account(minting_account.value()));
    let mint_amount = U256::one();
    let id = U256::one();

    let mint_call = cep85_mint(
        &mut builder,
        &cep85_contract_hash,
        &minting_account,
        &minting_recipient,
        &id,
        &mint_amount,
        None,
    );

    mint_call.expect_success().commit();

    let from = minting_recipient;

    let to = account_user_1_key;
    let transfer_amount = U256::one();
    let data = Some(Bytes::from("Casper Labs free bytes".as_bytes()));

    // Let's try to send as account_user_1 a transfer request from owner to account_user_1, this
    // request should fail
    let failing_transfer_call = cep85_transfer_from(
        &mut builder,
        &cep85_contract_hash,
        &account_user_1_account_hash,
        TransferData {
            from: &from,
            to: &to,
            ids: vec![id],
            amounts: vec![transfer_amount],
            data,
        },
        None,
    );
    failing_transfer_call.expect_failure();

    let error = builder.get_error().expect("must have error");

    assert_expected_error(
        error,
        Cep85Error::NotApproved as u16,
        "Only owner or approved operator can transfer token on behalf of token owner",
    );

    let actual_balance_from =
        cep85_check_balance_of(&mut builder, &cep85_test_contract_package, &from, &id).unwrap();
    let expected_balance_from = U256::one();

    assert_eq!(actual_balance_from, expected_balance_from);

    let actual_balance_to =
        cep85_check_balance_of(&mut builder, &cep85_test_contract_package, &to, &id).unwrap();
    let expected_balance_to = U256::zero();

    assert_eq!(actual_balance_to, expected_balance_to);
}

#[test]
fn should_not_batch_transfer_from_account_to_account_without_allowance() {
    let (account_user_1_key, account_user_1_account_hash, _) = get_test_account("ACCOUNT_USER_1");
    let (
        mut builder,
        TestContext {
            cep85_contract_hash,
            cep85_test_contract_package,
            ..
        },
    ) = setup();

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let minting_recipient = Key::AddressableEntity(EntityAddr::Account(minting_account.value()));
    let ids: Vec<U256> = vec![U256::one(), U256::from(2)];
    let amounts: Vec<U256> = vec![U256::one(), U256::one()];
    let from = minting_recipient;

    let to = account_user_1_key;
    let data = Some(Bytes::from("Casper Labs free bytes".as_bytes()));
    let recipients = vec![from, from, to, to];
    let expected_balances_after: Vec<U256> = [&amounts[..], &[U256::zero(), U256::zero()]].concat();

    let mint_call = cep85_batch_mint(
        &mut builder,
        &cep85_contract_hash,
        &minting_account,
        &minting_recipient,
        ids.clone(),
        amounts.clone(),
        None,
    );

    mint_call.expect_success().commit();

    // Let's try to send as account_user_1 a transfer request from owner to account_user_1, this
    // request should fail
    let failing_transfer_call = cep85_batch_transfer_from(
        &mut builder,
        &cep85_contract_hash,
        &account_user_1_account_hash,
        TransferData {
            from: &from,
            to: &to,
            ids: ids.clone(),
            amounts,
            data,
        },
        None,
    );
    failing_transfer_call.expect_failure();

    let error = builder.get_error().expect("must have error");

    assert_expected_error(
        error,
        Cep85Error::NotApproved as u16,
        "Only owner or approved operator can transfer token on behalf of token owner",
    );

    let actual_balances_after: Vec<Option<U256>> = cep85_check_balance_of_batch(
        &mut builder,
        &cep85_test_contract_package,
        recipients,
        vec![ids; 2_usize].into_iter().flatten().collect(),
    );

    assert_eq!(
        actual_balances_after,
        expected_balances_after
            .iter()
            .map(|&amount| Some(amount))
            .collect::<Vec<Option<U256>>>()
    );
}

#[test]
fn should_transfer_from_account_to_account_with_allowance() {
    let (account_user_1_key, account_user_1_account_hash, _) = get_test_account("ACCOUNT_USER_1");
    let (
        mut builder,
        TestContext {
            cep85_contract_hash,
            cep85_test_contract_package,
            ..
        },
    ) = setup();

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let minting_recipient = Key::AddressableEntity(EntityAddr::Account(minting_account.value()));
    let mint_amount = U256::one();
    let id = U256::one();

    let mint_call = cep85_mint(
        &mut builder,
        &cep85_contract_hash,
        &minting_account,
        &minting_recipient,
        &id,
        &mint_amount,
        None,
    );

    mint_call.expect_success().commit();

    let operator = account_user_1_key;
    let approved = true;

    let set_approval_for_all_call = cep85_set_approval_for_all(
        &mut builder,
        &cep85_contract_hash,
        &minting_account,
        &operator,
        approved,
    );
    set_approval_for_all_call.expect_success().commit();

    // Let's try to send as account_user_1 a transfer request from owner to account_user_1, this
    // request should succeed as account_user_1 is operator for owner DEFAULT_ACCOUNT_ADDR
    let from = minting_recipient;
    let to = operator; // operator will also be recipient of the transfer
    let transfer_amount = U256::one();
    let data = Some(Bytes::from("Casper Labs free bytes".as_bytes()));
    let transfer_call = cep85_transfer_from(
        &mut builder,
        &cep85_contract_hash,
        &account_user_1_account_hash, // account_user_1 is now operator
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
        cep85_check_balance_of(&mut builder, &cep85_test_contract_package, &from, &id).unwrap();
    let expected_balance_from = U256::zero();

    assert_eq!(actual_balance_from, expected_balance_from);

    let actual_balance_to =
        cep85_check_balance_of(&mut builder, &cep85_test_contract_package, &to, &id).unwrap();
    let expected_balance_to = U256::one();

    assert_eq!(actual_balance_to, expected_balance_to);
}

#[test]
fn should_batch_transfer_from_account_to_account_with_allowance() {
    let (account_user_1_key, account_user_1_account_hash, _) = get_test_account("ACCOUNT_USER_1");

    let (
        mut builder,
        TestContext {
            cep85_contract_hash,
            cep85_test_contract_package,
            ..
        },
    ) = setup();

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let minting_recipient = Key::AddressableEntity(EntityAddr::Account(minting_account.value()));
    let ids: Vec<U256> = vec![U256::one(), U256::from(2)];
    let amounts: Vec<U256> = vec![U256::one(), U256::one()];

    let mint_call = cep85_batch_mint(
        &mut builder,
        &cep85_contract_hash,
        &minting_account,
        &minting_recipient,
        ids.clone(),
        amounts.clone(),
        None,
    );

    mint_call.expect_success().commit();

    let operator = account_user_1_key;
    let approved = true;

    let set_approval_for_all_call = cep85_set_approval_for_all(
        &mut builder,
        &cep85_contract_hash,
        &minting_account,
        &operator,
        approved,
    );
    set_approval_for_all_call.expect_success().commit();

    let from = minting_recipient;
    let to = operator; // operator will also be recipient of the transfer
    let data = Some(Bytes::from("Casper Labs free bytes".as_bytes()));
    let recipients = vec![from, from, to, to];
    let expected_balances_after: Vec<U256> = [&[U256::zero(), U256::zero()], &amounts[..]].concat();

    // Let's try to send as account_user_1 a transfer request from owner to account_user_1, this
    // request should succeed as account_user_1 is operator for owner DEFAULT_ACCOUNT_ADDR
    let transfer_call = cep85_batch_transfer_from(
        &mut builder,
        &cep85_contract_hash,
        &account_user_1_account_hash, // account_user_1 is now operator
        TransferData {
            from: &from,
            to: &to,
            ids: ids.clone(),
            amounts,
            data,
        },
        None,
    );
    transfer_call.expect_success().commit();

    let actual_balances_after = cep85_check_balance_of_batch(
        &mut builder,
        &cep85_test_contract_package,
        recipients,
        vec![ids; 2_usize].into_iter().flatten().collect(),
    );

    assert_eq!(
        actual_balances_after,
        expected_balances_after
            .iter()
            .map(|&amount| Some(amount))
            .collect::<Vec<Option<U256>>>()
    );
}

#[test]
fn should_not_transfer_from_account_to_account_through_contract_without_allowance() {
    let (account_user_1_key, _, _) = get_test_account("ACCOUNT_USER_1");
    let (account_user_2_key, _, _) = get_test_account("ACCOUNT_USER_2");

    let (
        mut builder,
        TestContext {
            cep85_contract_hash,
            cep85_test_contract_package,
            ..
        },
    ) = setup();

    let minting_account = *DEFAULT_ACCOUNT_ADDR;

    let minting_recipient = account_user_1_key;
    let mint_amount = U256::one();
    let id = U256::one();

    let mint_call = cep85_mint(
        &mut builder,
        &cep85_contract_hash,
        &minting_account,
        &minting_recipient,
        &id,
        &mint_amount,
        None,
    );

    mint_call.expect_success().commit();

    let from = minting_recipient;
    let to = account_user_2_key;
    let transfer_amount = U256::one();
    let data = Some(Bytes::from("Casper Labs free bytes".as_bytes()));

    // Let's try to send as a contract a transfer request from owner account_user_1 to
    // account_user_2, this request should fail
    let failing_transfer_call = cep85_transfer_from_as_contract(
        &mut builder,
        &cep85_test_contract_package,
        &minting_account,
        TransferData {
            from: &from,
            to: &to,
            ids: vec![id],
            amounts: vec![transfer_amount],
            data,
        },
    );
    failing_transfer_call.expect_failure();

    let error = builder.get_error().expect("must have error");

    assert_expected_error(
        error,
        Cep85Error::NotApproved as u16,
        "Only owner or approved operator can transfer token on behalf of token owner",
    );

    let actual_balance_from =
        cep85_check_balance_of(&mut builder, &cep85_test_contract_package, &from, &id).unwrap();
    let expected_balance_from = U256::one();

    assert_eq!(actual_balance_from, expected_balance_from);

    let actual_balance_to =
        cep85_check_balance_of(&mut builder, &cep85_test_contract_package, &to, &id).unwrap();
    let expected_balance_to = U256::zero();

    assert_eq!(actual_balance_to, expected_balance_to);
}

#[test]
fn should_not_batch_transfer_from_account_to_account_through_contract_without_allowance() {
    let (account_user_1_key, account_user_1_account_hash, _) = get_test_account("ACCOUNT_USER_1");
    let (account_user_2_key, _, _) = get_test_account("ACCOUNT_USER_2");
    let (
        mut builder,
        TestContext {
            cep85_contract_hash,
            cep85_test_contract_key,
            cep85_test_contract_package,
            ..
        },
    ) = setup();

    let minting_account = *DEFAULT_ACCOUNT_ADDR;

    let minting_recipient = account_user_1_key;
    let ids: Vec<U256> = vec![U256::one(), U256::from(2)];
    let amounts: Vec<U256> = vec![U256::one(), U256::one()];

    let mint_call = cep85_batch_mint(
        &mut builder,
        &cep85_contract_hash,
        &minting_account,
        &minting_recipient,
        ids.clone(),
        amounts.clone(),
        None,
    );

    mint_call.expect_success().commit();

    let owner = account_user_1_account_hash;
    let approving_account = Key::AddressableEntity(EntityAddr::Account(owner.value()));
    let operator = cep85_test_contract_key;
    let not_approved = false;

    let is_approved = cep85_check_is_approved(
        &mut builder,
        &cep85_test_contract_package,
        &approving_account,
        &operator,
    );

    assert_eq!(is_approved, not_approved);

    let from = minting_recipient;
    let to = account_user_2_key;
    let data = Some(Bytes::from("Casper Labs free bytes".as_bytes()));
    let recipients = vec![from, from, to, to];
    let expected_balances_after: Vec<U256> = [&amounts[..], &[U256::zero(), U256::zero()]].concat();

    // Let's try to send as account_user_1 a transfer request from owner to account_user_1, this
    // request should fail as not approved
    let failing_transfer_call = cep85_batch_transfer_from_as_contract(
        &mut builder,
        &cep85_test_contract_package,
        &minting_account,
        TransferData {
            from: &from,
            to: &to,
            ids: ids.clone(),
            amounts,
            data,
        },
    );
    failing_transfer_call.expect_failure();

    let error = builder.get_error().expect("must have error");

    assert_expected_error(
        error,
        Cep85Error::NotApproved as u16,
        "Only owner or approved operator can transfer token on behalf of token owner",
    );

    let actual_balances_after = cep85_check_balance_of_batch(
        &mut builder,
        &cep85_test_contract_package,
        recipients,
        vec![ids; 2_usize].into_iter().flatten().collect(),
    );

    assert_eq!(
        actual_balances_after,
        expected_balances_after
            .iter()
            .map(|&amount| Some(amount))
            .collect::<Vec<Option<U256>>>()
    );
}

#[test]
fn should_transfer_from_account_to_account_through_contract_with_allowance() {
    let (account_user_1_key, account_user_1_account_hash, _) = get_test_account("ACCOUNT_USER_1");
    let (account_user_2_key, _, _) = get_test_account("ACCOUNT_USER_2");
    let (
        mut builder,
        TestContext {
            cep85_contract_hash,
            cep85_test_contract_key,
            cep85_test_contract_package,
            ..
        },
    ) = setup();
    let minting_account = *DEFAULT_ACCOUNT_ADDR;

    let minting_recipient = account_user_1_key;
    let mint_amount = U256::one();
    let id = U256::one();

    let mint_call = cep85_mint(
        &mut builder,
        &cep85_contract_hash,
        &minting_account,
        &minting_recipient,
        &id,
        &mint_amount,
        None,
    );

    mint_call.expect_success().commit();

    let from = minting_recipient;
    let to = account_user_2_key;
    let transfer_amount = U256::one();
    let data = Some(Bytes::from("Casper Labs free bytes".as_bytes()));

    let owner = account_user_1_account_hash;
    let approving_account = Key::AddressableEntity(EntityAddr::Account(owner.value()));
    let operator = cep85_test_contract_key;
    let approved = true;

    let set_approval_for_all_call = cep85_set_approval_for_all(
        &mut builder,
        &cep85_contract_hash,
        &owner,
        &operator,
        approved,
    );
    set_approval_for_all_call.expect_success().commit();

    let is_approved = cep85_check_is_approved(
        &mut builder,
        &cep85_test_contract_package,
        &approving_account,
        &operator,
    );

    assert_eq!(is_approved, approved);

    // Let's try to send as a contract a transfer request from owner account_user_1 to
    // account_user_2, this request should succeed as contract is operator for owner account_user_1
    let transfer_call = cep85_transfer_from_as_contract(
        &mut builder,
        &cep85_test_contract_package,
        &minting_account,
        TransferData {
            from: &from,
            to: &to,
            ids: vec![id],
            amounts: vec![transfer_amount],
            data,
        },
    );
    transfer_call.expect_success().commit();

    // let actual_balance_from =
    //     cep85_check_balance_of(&mut builder, &cep85_test_contract_package, &from, &id).unwrap();
    // let expected_balance_from = U256::zero();

    // assert_eq!(actual_balance_from, expected_balance_from);

    // let actual_balance_to =
    //     cep85_check_balance_of(&mut builder, &cep85_test_contract_package, &to, &id).unwrap();
    // let expected_balance_to = U256::one();

    // assert_eq!(actual_balance_to, expected_balance_to);
}

#[test]
fn should_transfer_from_account_to_account_through_package_with_allowance() {
    let (account_user_1_key, account_user_1_account_hash, _) = get_test_account("ACCOUNT_USER_1");
    let (account_user_2_key, _, _) = get_test_account("ACCOUNT_USER_2");
    let (
        mut builder,
        TestContext {
            cep85_contract_hash,
            cep85_test_contract_package,
            ..
        },
    ) = setup();
    let minting_account = *DEFAULT_ACCOUNT_ADDR;

    let minting_recipient = account_user_1_key;
    let mint_amount = U256::one();
    let id = U256::one();

    let mint_call = cep85_mint(
        &mut builder,
        &cep85_contract_hash,
        &minting_account,
        &minting_recipient,
        &id,
        &mint_amount,
        None,
    );

    mint_call.expect_success().commit();

    let from = minting_recipient;
    let to = account_user_2_key;
    let transfer_amount = U256::one();
    let data = Some(Bytes::from("Casper Labs free bytes".as_bytes()));

    let owner = account_user_1_account_hash;
    let approving_account = Key::AddressableEntity(EntityAddr::Account(owner.value()));

    // Here we approve a package and not a contract only
    let operator = Key::from(cep85_test_contract_package);
    let approved = true;

    let set_approval_for_all_call = cep85_set_approval_for_all(
        &mut builder,
        &cep85_contract_hash,
        &owner,
        &operator,
        approved,
    );
    set_approval_for_all_call.expect_success().commit();

    let is_approved = cep85_check_is_approved(
        &mut builder,
        &cep85_test_contract_package,
        &approving_account,
        &operator,
    );

    assert_eq!(is_approved, approved);

    // Let's try to send as a contract a transfer request from owner account_user_1 to
    // account_user_2, this request should succeed as package is operator for owner account_user_1
    let transfer_call = cep85_transfer_from_as_contract(
        &mut builder,
        &cep85_test_contract_package,
        &minting_account,
        TransferData {
            from: &from,
            to: &to,
            ids: vec![id],
            amounts: vec![transfer_amount],
            data,
        },
    );
    transfer_call.expect_success().commit();

    let actual_balance_from =
        cep85_check_balance_of(&mut builder, &cep85_test_contract_package, &from, &id).unwrap();
    let expected_balance_from = U256::zero();

    assert_eq!(actual_balance_from, expected_balance_from);

    let actual_balance_to =
        cep85_check_balance_of(&mut builder, &cep85_test_contract_package, &to, &id).unwrap();
    let expected_balance_to = U256::one();

    assert_eq!(actual_balance_to, expected_balance_to);
}

#[test]
fn should_batch_transfer_from_account_to_account_through_contract_with_allowance() {
    let (account_user_1_key, account_user_1_account_hash, _) = get_test_account("ACCOUNT_USER_1");
    let (account_user_2_key, _, _) = get_test_account("ACCOUNT_USER_2");

    let (
        mut builder,
        TestContext {
            cep85_contract_hash,
            cep85_test_contract_key,
            cep85_test_contract_package,
            ..
        },
    ) = setup();

    let minting_account = *DEFAULT_ACCOUNT_ADDR;

    let minting_recipient = account_user_1_key;
    let ids: Vec<U256> = vec![U256::one(), U256::from(2)];
    let amounts: Vec<U256> = vec![U256::one(), U256::one()];

    let mint_call = cep85_batch_mint(
        &mut builder,
        &cep85_contract_hash,
        &minting_account,
        &minting_recipient,
        ids.clone(),
        amounts.clone(),
        None,
    );

    mint_call.expect_success().commit();

    let owner = account_user_1_account_hash;
    let approving_account = Key::AddressableEntity(EntityAddr::Account(owner.value()));
    let operator = cep85_test_contract_key;
    let approved = true;

    let set_approval_for_all_call = cep85_set_approval_for_all(
        &mut builder,
        &cep85_contract_hash,
        &owner,
        &operator,
        approved,
    );
    set_approval_for_all_call.expect_success().commit();

    let is_approved = cep85_check_is_approved(
        &mut builder,
        &cep85_test_contract_package,
        &approving_account,
        &operator,
    );

    assert_eq!(is_approved, approved);

    let from = minting_recipient;
    let to = account_user_2_key;
    let data = Some(Bytes::from("Casper Labs free bytes".as_bytes()));
    let recipients = vec![from, from, to, to];
    let expected_balances_after: Vec<U256> = [&[U256::zero(), U256::zero()], &amounts[..]].concat();

    // Let's try to send as account_user_1 a transfer request from owner to account_user_1, this
    // request should succeed as account_user_1 is operator for owner DEFAULT_ACCOUNT_ADDR
    let transfer_call = cep85_batch_transfer_from_as_contract(
        &mut builder,
        &cep85_test_contract_package,
        &minting_account,
        TransferData {
            from: &from,
            to: &to,
            ids: ids.clone(),
            amounts,
            data,
        },
    );
    transfer_call.expect_success().commit();

    let actual_balances_after = cep85_check_balance_of_batch(
        &mut builder,
        &cep85_test_contract_package,
        recipients,
        vec![ids; 2_usize].into_iter().flatten().collect(),
    );

    assert_eq!(
        actual_balances_after,
        expected_balances_after
            .iter()
            .map(|&amount| Some(amount))
            .collect::<Vec<Option<U256>>>()
    );
}

#[test]
fn should_batch_transfer_from_account_to_account_through_package_with_allowance() {
    let (account_user_1_key, account_user_1_account_hash, _) = get_test_account("ACCOUNT_USER_1");
    let (account_user_2_key, _, _) = get_test_account("ACCOUNT_USER_2");

    let (
        mut builder,
        TestContext {
            cep85_contract_hash,
            cep85_test_contract_package,
            ..
        },
    ) = setup();

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let minting_recipient = account_user_1_key;
    let ids: Vec<U256> = vec![U256::one(), U256::from(2)];
    let amounts: Vec<U256> = vec![U256::one(), U256::one()];

    let mint_call = cep85_batch_mint(
        &mut builder,
        &cep85_contract_hash,
        &minting_account,
        &minting_recipient,
        ids.clone(),
        amounts.clone(),
        None,
    );

    mint_call.expect_success().commit();

    let owner = account_user_1_account_hash;
    let approving_account = Key::AddressableEntity(EntityAddr::Account(owner.value()));
    let operator = Key::from(cep85_test_contract_package);
    let approved = true;

    let set_approval_for_all_call = cep85_set_approval_for_all(
        &mut builder,
        &cep85_contract_hash,
        &owner,
        &operator,
        approved,
    );
    set_approval_for_all_call.expect_success().commit();

    let is_approved = cep85_check_is_approved(
        &mut builder,
        &cep85_test_contract_package,
        &approving_account,
        &operator,
    );

    assert_eq!(is_approved, approved);

    let from = minting_recipient;
    let to = account_user_2_key;
    let data = Some(Bytes::from("Casper Labs free bytes".as_bytes()));
    let recipients = vec![from, from, to, to];
    let expected_balances_after: Vec<U256> = [&[U256::zero(), U256::zero()], &amounts[..]].concat();

    // Let's try to send as account_user_1 a transfer request from owner to account_user_1, this
    // request should succeed as account_user_1 is operator for owner DEFAULT_ACCOUNT_ADDR
    let transfer_call = cep85_batch_transfer_from_as_contract(
        &mut builder,
        &cep85_test_contract_package,
        &minting_account,
        TransferData {
            from: &from,
            to: &to,
            ids: ids.clone(),
            amounts,
            data,
        },
    );
    transfer_call.expect_success().commit();

    let actual_balances_after = cep85_check_balance_of_batch(
        &mut builder,
        &cep85_test_contract_package,
        recipients,
        vec![ids; 2_usize].into_iter().flatten().collect(),
    );

    assert_eq!(
        actual_balances_after,
        expected_balances_after
            .iter()
            .map(|&amount| Some(amount))
            .collect::<Vec<Option<U256>>>()
    );
}

#[test]
#[ignore = "Should not transfer to contract"]
fn should_transfer_from_account_to_contract_through_contract_with_allowance() {
    let (account_user_1_key, account_user_1_account_hash, _) = get_test_account("ACCOUNT_USER_1");

    let (
        mut builder,
        TestContext {
            cep85_contract_hash,
            cep85_test_contract_key,
            cep85_test_contract_package,
            ..
        },
    ) = setup();
    let minting_account = *DEFAULT_ACCOUNT_ADDR;

    let minting_recipient = account_user_1_key;
    let mint_amount = U256::one();
    let id = U256::one();

    let mint_call = cep85_mint(
        &mut builder,
        &cep85_contract_hash,
        &minting_account,
        &minting_recipient,
        &id,
        &mint_amount,
        None,
    );

    mint_call.expect_success().commit();

    let from = minting_recipient;
    let to = cep85_test_contract_key;
    let transfer_amount = U256::one();
    let data = Some(Bytes::from("Casper Labs free bytes".as_bytes()));

    let owner = account_user_1_account_hash;
    let approving_account = Key::AddressableEntity(EntityAddr::Account(owner.value()));
    let operator = cep85_test_contract_key;
    let approved = true;

    let set_approval_for_all_call = cep85_set_approval_for_all(
        &mut builder,
        &cep85_contract_hash,
        &owner,
        &operator,
        approved,
    );
    set_approval_for_all_call.expect_success().commit();

    let is_approved = cep85_check_is_approved(
        &mut builder,
        &cep85_test_contract_package,
        &approving_account,
        &operator,
    );

    assert_eq!(is_approved, approved);

    // Let's try to send as a contract a transfer request from owner account_user_1 to
    // account_user_2, this request should succeed as contract is operator for owner account_user_1
    let transfer_call = cep85_transfer_from_as_contract(
        &mut builder,
        &cep85_test_contract_package,
        &minting_account,
        TransferData {
            from: &from,
            to: &to,
            ids: vec![id],
            amounts: vec![transfer_amount],
            data,
        },
    );
    transfer_call.expect_success().commit();

    let actual_balance_from =
        cep85_check_balance_of(&mut builder, &cep85_test_contract_package, &from, &id).unwrap();
    let expected_balance_from = U256::zero();

    assert_eq!(actual_balance_from, expected_balance_from);

    let actual_balance_to =
        cep85_check_balance_of(&mut builder, &cep85_test_contract_package, &to, &id).unwrap();
    let expected_balance_to = U256::one();

    assert_eq!(actual_balance_to, expected_balance_to);
}

#[test]
#[ignore = "Should not transfer to contract"]
fn should_transfer_from_account_to_contract_through_package_with_allowance() {
    let (account_user_1_key, account_user_1_account_hash, _) = get_test_account("ACCOUNT_USER_1");

    let (
        mut builder,
        TestContext {
            cep85_contract_hash,
            cep85_test_contract_key,
            cep85_test_contract_package,
            ..
        },
    ) = setup();
    let minting_account = *DEFAULT_ACCOUNT_ADDR;

    let minting_recipient = account_user_1_key;
    let mint_amount = U256::one();
    let id = U256::one();

    let mint_call = cep85_mint(
        &mut builder,
        &cep85_contract_hash,
        &minting_account,
        &minting_recipient,
        &id,
        &mint_amount,
        None,
    );

    mint_call.expect_success().commit();

    let from = minting_recipient;
    let to = cep85_test_contract_key;
    let transfer_amount = U256::one();
    let data = Some(Bytes::from("Casper Labs free bytes".as_bytes()));

    let owner = account_user_1_account_hash;
    let approving_account = Key::AddressableEntity(EntityAddr::Account(owner.value()));
    let operator = Key::from(cep85_test_contract_package);
    let approved = true;

    let set_approval_for_all_call = cep85_set_approval_for_all(
        &mut builder,
        &cep85_contract_hash,
        &owner,
        &operator,
        approved,
    );
    set_approval_for_all_call.expect_success().commit();

    let is_approved = cep85_check_is_approved(
        &mut builder,
        &cep85_test_contract_package,
        &approving_account,
        &operator,
    );

    assert_eq!(is_approved, approved);

    // Let's try to send as a contract a transfer request from owner account_user_1 to
    // account_user_2, this request should succeed as contract is operator for owner account_user_1
    let transfer_call = cep85_transfer_from_as_contract(
        &mut builder,
        &cep85_test_contract_package,
        &minting_account,
        TransferData {
            from: &from,
            to: &to,
            ids: vec![id],
            amounts: vec![transfer_amount],
            data,
        },
    );
    transfer_call.expect_success().commit();

    let actual_balance_from =
        cep85_check_balance_of(&mut builder, &cep85_test_contract_package, &from, &id).unwrap();
    let expected_balance_from = U256::zero();

    assert_eq!(actual_balance_from, expected_balance_from);

    let actual_balance_to =
        cep85_check_balance_of(&mut builder, &cep85_test_contract_package, &to, &id).unwrap();
    let expected_balance_to = U256::one();

    assert_eq!(actual_balance_to, expected_balance_to);
}

#[test]
#[ignore = "Should not transfer to contract"]
fn should_batch_transfer_from_account_to_contract_through_contract_with_allowance() {
    let (account_user_1_key, account_user_1_account_hash, _) = get_test_account("ACCOUNT_USER_1");

    let (
        mut builder,
        TestContext {
            cep85_contract_hash,
            cep85_test_contract_key,
            cep85_test_contract_package,
            ..
        },
    ) = setup();

    let minting_account = *DEFAULT_ACCOUNT_ADDR;

    let minting_recipient = account_user_1_key;
    let ids: Vec<U256> = vec![U256::one(), U256::from(2)];
    let amounts: Vec<U256> = vec![U256::one(), U256::one()];

    let mint_call = cep85_batch_mint(
        &mut builder,
        &cep85_contract_hash,
        &minting_account,
        &minting_recipient,
        ids.clone(),
        amounts.clone(),
        None,
    );

    mint_call.expect_success().commit();

    let owner = account_user_1_account_hash;
    let approving_account = Key::AddressableEntity(EntityAddr::Account(owner.value()));
    let operator = cep85_test_contract_key;
    let approved = true;

    let set_approval_for_all_call = cep85_set_approval_for_all(
        &mut builder,
        &cep85_contract_hash,
        &owner,
        &operator,
        approved,
    );
    set_approval_for_all_call.expect_success().commit();

    let is_approved = cep85_check_is_approved(
        &mut builder,
        &cep85_test_contract_package,
        &approving_account,
        &operator,
    );

    assert_eq!(is_approved, approved);

    let from = minting_recipient;
    let to = cep85_test_contract_key;
    let data = Some(Bytes::from("Casper Labs free bytes".as_bytes()));
    let recipients = vec![from, from, to, to];
    let expected_balances_after: Vec<U256> = [&[U256::zero(), U256::zero()], &amounts[..]].concat();

    // Let's try to send as account_user_1 a transfer request from owner to account_user_1, this
    // request should succeed as account_user_1 is operator for owner DEFAULT_ACCOUNT_ADDR
    let transfer_call = cep85_batch_transfer_from_as_contract(
        &mut builder,
        &cep85_test_contract_package,
        &minting_account,
        TransferData {
            from: &from,
            to: &to,
            ids: ids.clone(),
            amounts,
            data,
        },
    );
    transfer_call.expect_success().commit();

    let actual_balances_after = cep85_check_balance_of_batch(
        &mut builder,
        &cep85_test_contract_package,
        recipients,
        vec![ids; 2_usize].into_iter().flatten().collect(),
    );

    assert_eq!(
        actual_balances_after,
        expected_balances_after
            .iter()
            .map(|&amount| Some(amount))
            .collect::<Vec<Option<U256>>>()
    );
}

#[test]
#[ignore = "Should not transfer to contract"]
fn should_batch_transfer_from_account_to_contract_through_package_with_allowance() {
    let (account_user_1_key, account_user_1_account_hash, _) = get_test_account("ACCOUNT_USER_1");

    let (
        mut builder,
        TestContext {
            cep85_contract_hash,
            cep85_test_contract_key,
            cep85_test_contract_package,
            ..
        },
    ) = setup();

    let minting_account = *DEFAULT_ACCOUNT_ADDR;

    let minting_recipient = account_user_1_key;
    let ids: Vec<U256> = vec![U256::one(), U256::from(2)];
    let amounts: Vec<U256> = vec![U256::one(), U256::one()];

    let mint_call = cep85_batch_mint(
        &mut builder,
        &cep85_contract_hash,
        &minting_account,
        &minting_recipient,
        ids.clone(),
        amounts.clone(),
        None,
    );

    mint_call.expect_success().commit();

    let owner = account_user_1_account_hash;
    let approving_account = Key::AddressableEntity(EntityAddr::Account(owner.value()));
    let operator = Key::from(cep85_test_contract_package);
    let approved = true;

    let set_approval_for_all_call = cep85_set_approval_for_all(
        &mut builder,
        &cep85_contract_hash,
        &owner,
        &operator,
        approved,
    );
    set_approval_for_all_call.expect_success().commit();

    let is_approved = cep85_check_is_approved(
        &mut builder,
        &cep85_test_contract_package,
        &approving_account,
        &operator,
    );

    assert_eq!(is_approved, approved);

    let from = minting_recipient;
    let to = cep85_test_contract_key;
    let data = Some(Bytes::from("Casper Labs free bytes".as_bytes()));
    let recipients = vec![from, from, to, to];
    let expected_balances_after: Vec<U256> = [&[U256::zero(), U256::zero()], &amounts[..]].concat();

    // Let's try to send as account_user_1 a transfer request from owner to account_user_1, this
    // request should succeed as account_user_1 is operator for owner DEFAULT_ACCOUNT_ADDR
    let transfer_call = cep85_batch_transfer_from_as_contract(
        &mut builder,
        &cep85_test_contract_package,
        &minting_account,
        TransferData {
            from: &from,
            to: &to,
            ids: ids.clone(),
            amounts,
            data,
        },
    );
    transfer_call.expect_success().commit();

    let actual_balances_after = cep85_check_balance_of_batch(
        &mut builder,
        &cep85_test_contract_package,
        recipients,
        vec![ids; 2_usize].into_iter().flatten().collect(),
    );

    assert_eq!(
        actual_balances_after,
        expected_balances_after
            .iter()
            .map(|&amount| Some(amount))
            .collect::<Vec<Option<U256>>>()
    );
}

#[test]
fn should_make_dictionary_item_key_for_dict_operators_queries() {
    let (account_user_1_key, _, _) = get_test_account("ACCOUNT_USER_1");

    let (mut builder, ..) = setup();

    let key = Key::AddressableEntity(EntityAddr::Account(DEFAULT_ACCOUNT_ADDR.value()));

    let value = account_user_1_key;

    cep85_make_dictionary_item_key(&mut builder, &key, None, Some(value), None);

    let dictionary_item_key = builder
        .query(
            None,
            Key::Account(*DEFAULT_ACCOUNT_ADDR),
            &[DEFAULT_DICT_ITEM_KEY_NAME.to_string()],
        )
        .unwrap()
        .as_cl_value()
        .unwrap()
        .to_owned()
        .into_t::<String>()
        .unwrap();

    // This is the dictionary item key to query operators dictionary with casper-client-rs
    assert_eq!(
        dictionary_item_key,
        "98d1c8029184fcbe2abc467f1a9c8c439e54af97ee7f610bfd7de207ae6d0d1f".to_string()
    );
}
