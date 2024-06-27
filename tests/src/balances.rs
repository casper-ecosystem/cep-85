use crate::utility::{
    installer_request_builders::{
        cep85_batch_mint, cep85_check_balance_of, cep85_check_balance_of_batch,
        cep85_make_dictionary_item_key, cep85_mint, setup, TestContext,
    },
    support::{assert_expected_error, get_dictionary_value_from_key, get_test_account},
};
use casper_engine_test_support::{ExecuteRequestBuilder, DEFAULT_ACCOUNT_ADDR};
use casper_types::{runtime_args, EntityAddr, Key, U256};
use cep85::{
    constants::{ARG_ACCOUNTS, ARG_IDS, DEFAULT_DICT_ITEM_KEY_NAME, DICT_BALANCES},
    error::Cep85Error,
};
use cep85_test_contract::constants::ENTRY_POINT_CHECK_BALANCE_OF_BATCH;
#[test]
fn should_check_balance_of() {
    let (account_user_1_key, _, _) = get_test_account("ACCOUNT_USER_1");

    let (
        mut builder,
        TestContext {
            cep18_contract_hash,
            cep85_test_contract_package,
            ..
        },
    ) = setup();

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

    let actual_balance = cep85_check_balance_of(
        &mut builder,
        &cep85_test_contract_package,
        &minting_recipient,
        &id,
    )
    .unwrap();
    let expected_balance = U256::one();

    assert_eq!(actual_balance, expected_balance);
}

#[test]
fn should_return_none_getting_balance_of_non_existing_token() {
    let (account_user_1_key, _, _) = get_test_account("ACCOUNT_USER_1");

    let (
        mut builder,
        TestContext {
            cep85_test_contract_package,
            ..
        },
    ) = setup();

    let id = U256::one();
    let owner = account_user_1_key;

    let actual_balance =
        cep85_check_balance_of(&mut builder, &cep85_test_contract_package, &owner, &id);
    let expected_balance = None;

    assert_eq!(actual_balance, expected_balance);
}

#[test]
fn should_check_balance_of_batch() {
    let (account_user_1_key, _, _) = get_test_account("ACCOUNT_USER_1");
    let (account_user_2_key, _, _) = get_test_account("ACCOUNT_USER_2");

    let (
        mut builder,
        TestContext {
            cep18_contract_hash,
            cep85_test_contract_package,
            ..
        },
    ) = setup();

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let recipient_user_1 = account_user_1_key;
    let recipient_user_2 = account_user_2_key;
    let ids: Vec<U256> = vec![U256::one(), U256::from(2)];
    let amounts: Vec<U256> = vec![U256::one(), U256::one()];

    // batch_mint is only one recipient
    let mint_call = cep85_batch_mint(
        &mut builder,
        &cep18_contract_hash,
        &minting_account,
        &recipient_user_1,
        ids.clone(),
        amounts,
        None,
    );

    mint_call.expect_success().commit();

    let recipients: Vec<Key> = vec![recipient_user_1, recipient_user_2];

    let actual_balances =
        cep85_check_balance_of_batch(&mut builder, &cep85_test_contract_package, recipients, ids);

    let expected_balances = [U256::one(), U256::zero()];

    assert_eq!(
        actual_balances,
        expected_balances
            .iter()
            .map(|&amount| Some(amount))
            .collect::<Vec<Option<U256>>>()
    );
}

#[test]
fn should_return_none_getting_balance_of_batch_non_existing_token() {
    let (account_user_1_key, _, _) = get_test_account("ACCOUNT_USER_1");
    let (account_user_2_key, _, _) = get_test_account("ACCOUNT_USER_2");
    let (
        mut builder,
        TestContext {
            cep18_contract_hash,
            cep85_test_contract_package,
            ..
        },
    ) = setup();

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let recipient_user_1 = account_user_1_key;
    let recipient_user_2 = account_user_2_key;
    let mut ids: Vec<U256> = vec![U256::one(), U256::from(2)];
    let amounts: Vec<U256> = vec![U256::one(), U256::one()];

    // batch_mint is only one recipient
    let mint_call = cep85_batch_mint(
        &mut builder,
        &cep18_contract_hash,
        &minting_account,
        &recipient_user_1,
        ids.clone(),
        amounts,
        None,
    );

    mint_call.expect_success().commit();

    let recipients: Vec<Key> = vec![recipient_user_1, recipient_user_2, recipient_user_2];
    // Add a new recipient for token id 3, balance will be None
    ids.push(U256::from(3));

    let actual_balances =
        cep85_check_balance_of_batch(&mut builder, &cep85_test_contract_package, recipients, ids);

    let expected_balances = vec![Some(U256::one()), Some(U256::zero()), None];

    assert_eq!(actual_balances, expected_balances);
}

#[test]
fn should_error_on_balance_of_batch_args_len_difference() {
    let (account_user_1_key, _, _) = get_test_account("ACCOUNT_USER_1");

    let (
        mut builder,
        TestContext {
            cep18_contract_hash,
            cep85_test_contract_package,
            ..
        },
    ) = setup();

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let minting_recipient = account_user_1_key;
    let mut ids: Vec<U256> = vec![U256::one()];
    let amounts: Vec<U256> = vec![U256::one()];

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

    let accounts: Vec<Key> = vec![minting_recipient, minting_recipient];

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
    let accounts: Vec<Key> = vec![minting_recipient];

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

#[test]
fn should_make_dictionary_item_key_for_dict_balances_queries() {
    let (
        mut builder,
        TestContext {
            cep18_contract_hash,
            ..
        },
    ) = setup();

    let key = Key::AddressableEntity(EntityAddr::Account(DEFAULT_ACCOUNT_ADDR.value()));
    let id = U256::one();

    cep85_make_dictionary_item_key(
        &mut builder,
        &cep18_contract_hash,
        &key,
        Some(id),
        None,
        None,
    );

    let dictionary_item_key = builder
        .query(
            None,
            Key::AddressableEntity(EntityAddr::Account(DEFAULT_ACCOUNT_ADDR.value())),
            &[DEFAULT_DICT_ITEM_KEY_NAME.to_string()],
        )
        .unwrap()
        .as_cl_value()
        .unwrap()
        .to_owned()
        .into_t::<String>()
        .unwrap();

    // This is the dictionary item key to query balances dictionary with casper-client-rs
    assert_eq!(
        dictionary_item_key,
        "d4cc85d3e1ba5d7e915ccd9083dcf81cd7d6e1e8d8ea4e431edd85b5a7bf9360".to_string()
    );

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let minting_recipient: Key =
        Key::AddressableEntity(EntityAddr::Account(minting_account.value()));
    let mint_amount = U256::from(2);

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

    let balance = get_dictionary_value_from_key::<U256>(
        &mut builder,
        &cep18_contract_hash,
        DICT_BALANCES,
        &dictionary_item_key,
    );

    assert_eq!(balance, mint_amount)
}

#[test]
fn should_make_dictionary_item_key_for_dict_balances_queries_with_specific_session_named_key_name()
{
    let (
        mut builder,
        TestContext {
            cep18_contract_hash,
            ..
        },
    ) = setup();

    let key = Key::AddressableEntity(EntityAddr::Account(DEFAULT_ACCOUNT_ADDR.value()));
    let id = U256::one();
    let session_named_key_name = "my_session_named_key_name".to_string();

    cep85_make_dictionary_item_key(
        &mut builder,
        &cep18_contract_hash,
        &key,
        Some(id),
        None,
        Some(session_named_key_name.clone()),
    );

    let dictionary_item_key = builder
        .query(None, key, &[session_named_key_name])
        .unwrap()
        .as_cl_value()
        .unwrap()
        .to_owned()
        .into_t::<String>()
        .unwrap();

    // This is the dictionary item key to query balances dictionary with casper-client-rs
    assert_eq!(
        dictionary_item_key,
        "d4cc85d3e1ba5d7e915ccd9083dcf81cd7d6e1e8d8ea4e431edd85b5a7bf9360".to_string()
    );
}
