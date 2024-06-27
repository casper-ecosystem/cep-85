use crate::utility::{
    constants::{TOKEN_URI, TOKEN_URI_TEST},
    installer_request_builders::{
        cep85_burn, cep85_change_security, cep85_check_uri, cep85_mint, cep85_set_total_supply_of,
        cep85_set_uri, setup_with_args, SecurityLists, TestContext,
    },
    support::{assert_expected_error, get_test_account},
};
use casper_engine_test_support::DEFAULT_ACCOUNT_ADDR;
use casper_types::{runtime_args, Key, U256};
use cep85::{
    constants::{ADMIN_LIST, ARG_ENABLE_BURN, BURNER_LIST, META_LIST, MINTER_LIST},
    error::Cep85Error,
    utils::replace_token_id_in_uri,
};

#[test]
fn should_test_security_no_rights() {
    let (_, account_user_1_acccount_hash, _) = get_test_account("ACCOUNT_USER_1");

    let (
        mut builder,
        TestContext {
            cep18_contract_hash,
            ..
        },
    ) = setup_with_args(runtime_args! {
        ARG_ENABLE_BURN => true,
    });
    let minting_account = account_user_1_acccount_hash;
    let minting_recipient: Key = minting_account.into();

    let mint_amount = U256::one();
    let id = U256::one();

    let failing_mint_call = cep85_mint(
        &mut builder,
        &cep18_contract_hash,
        &minting_account,
        &minting_recipient,
        &id,
        &mint_amount,
        None,
    );

    // minting account is not in minter nor admin list
    failing_mint_call.expect_failure();

    let error = builder.get_error().expect("must have error");

    assert_expected_error(
        error,
        Cep85Error::InsufficientRights as u16,
        "should not allow to mint for non default admin account",
    );

    let minting_account = *DEFAULT_ACCOUNT_ADDR;

    let mint_call = cep85_mint(
        &mut builder,
        &cep18_contract_hash,
        &minting_account,
        &minting_recipient,
        &id,
        &mint_amount,
        None,
    );

    // default address is in admin list by default
    mint_call.expect_success().commit();

    // New owner is now ACCOUNT_USER_1 but is not in
    let bunrning_account = account_user_1_acccount_hash;
    let owner: Key = bunrning_account.into();

    let failing_burn_call = cep85_burn(
        &mut builder,
        &cep18_contract_hash,
        &bunrning_account,
        &owner,
        &id,
        &mint_amount,
    );

    failing_burn_call.expect_failure();

    let error = builder.get_error().expect("must have error");

    assert_expected_error(
        error,
        Cep85Error::InsufficientRights as u16,
        "should not allow to mint for non default admin account",
    );
}

#[test]
fn should_test_security_meta_rights() {
    let (account_user_1_key, account_user_1_account_hash, _) = get_test_account("ACCOUNT_USER_1");
    let (_, account_user_2_account_hash, _) = get_test_account("ACCOUNT_USER_2");

    let (
        mut builder,
        TestContext {
            cep18_contract_hash,
            cep85_test_contract_package,
            ..
        },
    ) = setup_with_args(runtime_args! {
        META_LIST => vec![account_user_1_key]
    });

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let minting_recipient: Key = account_user_1_key;
    let mint_amount = U256::from(1);
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

    // account_user_2 is not in meta list, request should fail
    let updating_account = account_user_2_account_hash;
    let new_uri = TOKEN_URI_TEST;

    let failing_meta_call = cep85_set_uri(
        &mut builder,
        &cep18_contract_hash,
        &updating_account,
        new_uri,
        Some(id),
    );

    failing_meta_call.expect_failure();

    let actual_uri = cep85_check_uri(&mut builder, &cep85_test_contract_package, Some(id)).unwrap();

    assert_eq!(actual_uri, replace_token_id_in_uri(TOKEN_URI, &id));

    // account_user_1 is in meta list, request should succeed
    let updating_account = account_user_1_account_hash;

    let meta_call = cep85_set_uri(
        &mut builder,
        &cep18_contract_hash,
        &updating_account,
        new_uri,
        Some(id),
    );

    meta_call.expect_success().commit();

    let actual_uri = cep85_check_uri(&mut builder, &cep85_test_contract_package, Some(id)).unwrap();

    assert_eq!(actual_uri, replace_token_id_in_uri(TOKEN_URI_TEST, &id));

    // default address is in admin list, request should succeed
    let updating_account = *DEFAULT_ACCOUNT_ADDR;

    let meta_call = cep85_set_uri(
        &mut builder,
        &cep18_contract_hash,
        &updating_account,
        TOKEN_URI,
        None,
    );
    meta_call.expect_success().commit();

    let actual_uri = cep85_check_uri(&mut builder, &cep85_test_contract_package, None).unwrap();

    assert_eq!(actual_uri, TOKEN_URI);
}

#[test]
fn should_test_security_minter_rights() {
    let (account_user_1_key, account_user_1_account_hash, _) = get_test_account("ACCOUNT_USER_1");
    let (account_user_2_key, account_user_2_account_hash, _) = get_test_account("ACCOUNT_USER_2");

    let (
        mut builder,
        TestContext {
            cep18_contract_hash,
            ..
        },
    ) = setup_with_args(runtime_args! {
        MINTER_LIST => vec![account_user_1_key]
    });

    let minting_account = account_user_1_account_hash;
    let minting_recipient: Key = account_user_1_key;
    let mint_amount = U256::one();
    let id = U256::one();

    // account_user_1 is in minter list, request should succeed
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

    // account_user_2 is not in minter list, request should fail
    let minting_account = account_user_2_account_hash;
    let minting_recipient: Key = account_user_2_key;

    let failing_mint_call = cep85_mint(
        &mut builder,
        &cep18_contract_hash,
        &minting_account,
        &minting_recipient,
        &id,
        &mint_amount,
        None,
    );

    failing_mint_call.expect_failure();

    let error = builder.get_error().expect("must have error");

    assert_expected_error(
        error,
        Cep85Error::InsufficientRights as u16,
        "should not allow to mint for non default admin account",
    );
}

#[test]
fn should_test_security_burner_rights() {
    let (account_user_1_key, account_user_1_account_hash, _) = get_test_account("ACCOUNT_USER_1");
    let (_, account_user_2_account_hash, _) = get_test_account("ACCOUNT_USER_2");

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

    // Set total supply to 2 for the token to be minted

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let minting_recipient: Key = account_user_1_key;
    let total_supply = U256::from(2);
    let mint_amount = U256::from(2);
    let id = U256::one();

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

    // account_user_2 is not in burner list, request should fail
    let burning_account = account_user_2_account_hash;
    // owner is now last recipient
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
        Cep85Error::InsufficientRights as u16,
        "should not allow to burn for non burner account",
    );

    // account_user_1 is in burner list, request should succeed
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

    // default address is in admin list but not funded
    let burning_account = minting_account;
    let owner: Key = burning_account.into();

    let burn_call = cep85_burn(
        &mut builder,
        &cep18_contract_hash,
        &burning_account,
        &owner,
        &id,
        &burn_amount,
    );

    burn_call.expect_failure();

    let error = builder.get_error().expect("must have error");

    assert_expected_error(
        error,
        Cep85Error::OverflowBurn as u16,
        "should not allow to mint above balance for non funded admin account",
    );
}

#[test]
fn should_test_change_security() {
    let (account_user_1_key, account_user_1_account_hash, _) = get_test_account("ACCOUNT_USER_1");
    let (account_user_2_key, account_user_2_account_hash, _) = get_test_account("ACCOUNT_USER_2");

    let (
        mut builder,
        TestContext {
            cep18_contract_hash,
            ..
        },
    ) = setup_with_args(runtime_args! {
        ADMIN_LIST => vec![account_user_1_key],
    });

    let minting_account = account_user_1_account_hash;
    let minting_recipient: Key = account_user_1_key;
    let total_supply = U256::from(2);
    let mint_amount = U256::one();
    let id = U256::one();

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

    // account_user_1 is in admin list
    mint_call.expect_success().commit();
    let minting_account = account_user_2_account_hash;
    let minting_recipient: Key = account_user_2_key;

    let security_lists = SecurityLists {
        minter_list: Some(vec![account_user_2_key]),
        burner_list: None,
        meta_list: None,
        admin_list: None,
        none_list: None,
    };

    let change_security = cep85_change_security(
        &mut builder,
        &cep18_contract_hash,
        &account_user_1_account_hash,
        security_lists,
    );

    change_security.expect_success().commit();

    let mint_call = cep85_mint(
        &mut builder,
        &cep18_contract_hash,
        &minting_account,
        &minting_recipient,
        &id,
        &mint_amount,
        None,
    );

    // account_user_2 is in minter list
    mint_call.expect_success().commit();

    let security_lists = SecurityLists {
        minter_list: None,
        burner_list: None,
        meta_list: None,
        admin_list: None,
        none_list: Some(vec![account_user_2_key]),
    };

    let change_security = cep85_change_security(
        &mut builder,
        &cep18_contract_hash,
        &account_user_1_account_hash,
        security_lists,
    );

    change_security.expect_success().commit();

    let failing_mint_call = cep85_mint(
        &mut builder,
        &cep18_contract_hash,
        &minting_account,
        &minting_recipient,
        &id,
        &mint_amount,
        None,
    );

    // minting account is in none list now, so the same mint request should fail
    failing_mint_call.expect_failure();

    let error = builder.get_error().expect("must have error");

    assert_expected_error(
        error,
        Cep85Error::InsufficientRights as u16,
        "should not allow to mint for non default admin account",
    );
}
