use std::collections::HashMap;

use super::{
    constants::{
        ACCOUNT_USER_1, ACCOUNT_USER_2, CEP85_CONTRACT_WASM, CEP85_TEST_CONTRACT_WASM,
        CEP85_TEST_TOKEN_CONTRACT_NAME, TOKEN_NAME, TOKEN_URI,
    },
    support::create_funded_dummy_account,
};
use casper_engine_test_support::{
    ExecuteRequestBuilder, InMemoryWasmTestBuilder, ARG_AMOUNT, DEFAULT_ACCOUNT_ADDR,
    PRODUCTION_RUN_GENESIS_REQUEST,
};
use casper_types::{
    account::AccountHash, bytesrepr::FromBytes, runtime_args, system::mint::ARG_ID, CLTyped,
    ContractHash, ContractPackageHash, Key, RuntimeArgs, U256,
};
use cep85::constants::{
    ARG_ACCOUNT, ARG_NAME, ARG_RECIPIENT, ARG_URI, ENTRY_POINT_MINT, TOKEN_CONTRACT,
};
use cep85_test_contract::constants::{
    CEP85_TEST_PACKAGE_NAME, ENTRY_POINT_CHECK_BALANCE_OF, RESULT_KEY,
};

#[derive(Clone)]
pub struct TestContext {
    pub cep85_token: ContractHash,
    pub cep85_test_contract_package: ContractPackageHash,
    pub test_accounts: HashMap<[u8; 32], AccountHash>,
}

pub fn setup() -> (InMemoryWasmTestBuilder, TestContext) {
    setup_with_args(
        runtime_args! {
            ARG_NAME => TOKEN_NAME,
            ARG_URI => TOKEN_URI,
        },
        None,
    )
}

pub fn setup_with_args(
    install_args: RuntimeArgs,
    test_accounts: Option<HashMap<[u8; 32], AccountHash>>,
) -> (InMemoryWasmTestBuilder, TestContext) {
    let mut builder = InMemoryWasmTestBuilder::default();
    builder.run_genesis(&PRODUCTION_RUN_GENESIS_REQUEST);

    let mut test_accounts = test_accounts.unwrap_or_default();

    test_accounts
        .entry(ACCOUNT_USER_1)
        .or_insert_with(|| create_funded_dummy_account(&mut builder, Some(ACCOUNT_USER_1)));
    test_accounts
        .entry(ACCOUNT_USER_2)
        .or_insert_with(|| create_funded_dummy_account(&mut builder, Some(ACCOUNT_USER_2)));

    let install_request_contract =
        ExecuteRequestBuilder::standard(*DEFAULT_ACCOUNT_ADDR, CEP85_CONTRACT_WASM, install_args)
            .build();

    builder
        .exec(install_request_contract)
        .expect_success()
        .commit();

    let account = builder
        .get_account(*DEFAULT_ACCOUNT_ADDR)
        .expect("should have account");

    let cep85_token = account
        .named_keys()
        .get(CEP85_TEST_TOKEN_CONTRACT_NAME)
        .and_then(|key| key.into_hash())
        .map(ContractHash::new)
        .expect("should have contract hash");

    let install_request_contract_test = ExecuteRequestBuilder::standard(
        *DEFAULT_ACCOUNT_ADDR,
        CEP85_TEST_CONTRACT_WASM,
        runtime_args! {
            TOKEN_CONTRACT => Key::from(cep85_token)
        },
    )
    .build();

    builder
        .exec(install_request_contract_test)
        .expect_success()
        .commit();

    let account = builder
        .get_account(*DEFAULT_ACCOUNT_ADDR)
        .expect("should have account");

    let cep85_test_contract_package = account
        .named_keys()
        .get(CEP85_TEST_PACKAGE_NAME)
        .and_then(|key| key.into_hash())
        .map(ContractPackageHash::new)
        .expect("should have contract package hash");

    let test_context = TestContext {
        cep85_token,
        cep85_test_contract_package,
        test_accounts,
    };

    (builder, test_context)
}

pub fn get_test_result<T: FromBytes + CLTyped>(
    builder: &mut InMemoryWasmTestBuilder,
    cep85_test_contract_package: ContractPackageHash,
) -> T {
    let contract_package = builder
        .get_contract_package(cep85_test_contract_package)
        .expect("should have contract package");
    let enabled_versions = contract_package.enabled_versions();
    let (_version, contract_hash) = enabled_versions
        .iter()
        .rev()
        .next()
        .expect("should have latest version");

    builder.get_value(*contract_hash, RESULT_KEY)
}

pub fn cep85_mint(
    builder: &mut InMemoryWasmTestBuilder,
    cep85_token: &ContractHash,
    recipient: Key,
    id: U256,
    amount: U256,
) {
    let mint_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        *cep85_token,
        ENTRY_POINT_MINT,
        runtime_args! {
            ARG_RECIPIENT => recipient,
            ARG_ID => id,
            ARG_AMOUNT => amount,
        },
    )
    .build();
    builder.exec(mint_request).expect_success().commit();
}

pub fn cep85_check_balance_of(
    builder: &mut InMemoryWasmTestBuilder,
    contract_package_hash: &ContractPackageHash,
    account: Key,
    id: U256,
) -> U256 {
    let check_balance_args = runtime_args! {
        ARG_ACCOUNT => account,
        ARG_ID => id,
    };
    let exec_request = ExecuteRequestBuilder::versioned_contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        *contract_package_hash,
        None,
        ENTRY_POINT_CHECK_BALANCE_OF,
        check_balance_args,
    )
    .build();
    builder.exec(exec_request).expect_success().commit();
    get_test_result(builder, *contract_package_hash)
}
