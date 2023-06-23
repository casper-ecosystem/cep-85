use casper_engine_test_support::{
    ExecuteRequestBuilder, InMemoryWasmTestBuilder, DEFAULT_ACCOUNT_ADDR,
    PRODUCTION_RUN_GENESIS_REQUEST,
};
use casper_types::{runtime_args, ContractHash, ContractPackageHash, Key, RuntimeArgs};
use cep1155::constants::{ARG_NAME, ARG_URI, TOKEN_CONTRACT};

use super::constants::{
    CEP1155_CONTRACT_WASM, CEP1155_TEST_CONTRACT_WASM, CEP1155_TEST_TOKEN_CONTRACT_NAME,
    TOKEN_NAME, TOKEN_URI,
};

#[derive(Copy, Clone)]
pub(crate) struct TestContext {
    pub(crate) cep1155_token: ContractHash,
    pub(crate) cep1155_test_contract_package: ContractPackageHash,
}

pub(crate) fn setup() -> (InMemoryWasmTestBuilder, TestContext) {
    setup_with_args(runtime_args! {
        ARG_NAME => TOKEN_NAME,
        ARG_URI => TOKEN_URI,
    })
}
pub(crate) fn setup_with_args(install_args: RuntimeArgs) -> (InMemoryWasmTestBuilder, TestContext) {
    let mut builder = InMemoryWasmTestBuilder::default();
    builder.run_genesis(&PRODUCTION_RUN_GENESIS_REQUEST);

    let install_request_contract =
        ExecuteRequestBuilder::standard(*DEFAULT_ACCOUNT_ADDR, CEP1155_CONTRACT_WASM, install_args)
            .build();

    builder
        .exec(install_request_contract)
        .expect_success()
        .commit();

    let account = builder
        .get_account(*DEFAULT_ACCOUNT_ADDR)
        .expect("should have account");

    let cep1155_token = account
        .named_keys()
        .get(CEP1155_TEST_TOKEN_CONTRACT_NAME)
        .and_then(|key| key.into_hash())
        .map(ContractHash::new)
        .expect("should have contract hash");

    let install_request_contract_test = ExecuteRequestBuilder::standard(
        *DEFAULT_ACCOUNT_ADDR,
        CEP1155_TEST_CONTRACT_WASM,
        runtime_args! {
            TOKEN_CONTRACT => Key::from(cep1155_token)
        },
    )
    .build();

    builder
        .exec(install_request_contract_test)
        .expect_success()
        .commit();

    let cep1155_test_contract_package = account
        .named_keys()
        .get(CEP1155_TEST_TOKEN_CONTRACT_NAME)
        .and_then(|key| key.into_hash())
        .map(ContractPackageHash::new)
        .expect("should have contract package hash");

    let test_context = TestContext {
        cep1155_token,
        cep1155_test_contract_package,
    };

    (builder, test_context)
}
