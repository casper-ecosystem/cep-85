use casper_engine_test_support::DEFAULT_ACCOUNT_ADDR;
use casper_types::{EntityAddr, Key, U256};

use crate::utility::installer_request_builders::{
    cep85_check_is_non_fungible, cep85_check_total_fungible_supply, cep85_mint,
    cep85_set_total_supply_of, setup, TestContext,
};

#[test]
fn should_check_if_fungible() {
    let (
        mut builder,
        TestContext {
            cep18_contract_hash,
            cep85_test_contract_package,
            ..
        },
    ) = setup();

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let minting_recipient = Key::AddressableEntity(EntityAddr::Account(minting_account.value()));
    let total_supply = U256::from(2);
    let mint_amount = U256::one();
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

    let is_non_fungible =
        cep85_check_is_non_fungible(&mut builder, &cep85_test_contract_package, &id).unwrap();

    assert!(!is_non_fungible);
}

#[test]
fn should_check_if_non_fungible() {
    let (
        mut builder,
        TestContext {
            cep18_contract_hash,
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
        &cep18_contract_hash,
        &minting_account,
        &minting_recipient,
        &id,
        &mint_amount,
        None,
    );

    mint_call.expect_success().commit();

    let is_non_fungible =
        cep85_check_is_non_fungible(&mut builder, &cep85_test_contract_package, &id).unwrap();

    assert!(is_non_fungible);
}

#[test]
fn should_check_if_non_fungible_if_total_supply_is_reduced_to_one() {
    let (
        mut builder,
        TestContext {
            cep18_contract_hash,
            cep85_test_contract_package,
            ..
        },
    ) = setup();

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let minting_recipient = Key::AddressableEntity(EntityAddr::Account(minting_account.value()));
    let total_supply = U256::from(2);
    let mint_amount = U256::one();
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

    let is_non_fungible =
        cep85_check_is_non_fungible(&mut builder, &cep85_test_contract_package, &id).unwrap();

    assert!(!is_non_fungible);

    let total_supply = U256::one();

    // Set total supply to 2 for the token to be minted
    let set_total_supply_of_call = cep85_set_total_supply_of(
        &mut builder,
        &cep18_contract_hash,
        &minting_account,
        &id,
        &total_supply,
    );

    set_total_supply_of_call.expect_success().commit();

    let is_now_non_fungible =
        cep85_check_is_non_fungible(&mut builder, &cep85_test_contract_package, &id).unwrap();

    assert!(is_now_non_fungible);
}

#[test]
fn should_get_total_fungible_supply() {
    let (
        mut builder,
        TestContext {
            cep18_contract_hash,
            cep85_test_contract_package,
            ..
        },
    ) = setup();

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let minting_recipient = Key::AddressableEntity(EntityAddr::Account(minting_account.value()));
    let total_supply = U256::from(10);
    let mint_amount = U256::from(4);
    let id = U256::one();

    // Set total supply to 10 for the 4 token to be minted
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

    let fungible_supply =
        cep85_check_total_fungible_supply(&mut builder, &cep85_test_contract_package, &id);

    // Fungible supply should be 6
    let expected_fungible_supply = total_supply - mint_amount;
    assert_eq!(expected_fungible_supply, U256::from(6));
    assert_eq!(fungible_supply, Some(expected_fungible_supply));
}
