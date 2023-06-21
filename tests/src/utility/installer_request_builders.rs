use casper_engine_test_support::{
    ExecuteRequestBuilder, InMemoryWasmTestBuilder, DEFAULT_ACCOUNT_ADDR,
    PRODUCTION_RUN_GENESIS_REQUEST,
};
use casper_types::{runtime_args, ContractHash, ContractPackageHash, RuntimeArgs};
use cep1155::constants::NAME;

use super::constants::{
    CEP1155_CONTRACT_WASM, CEP1155_TEST_CONTRACT_WASM, CEP1155_TEST_TOKEN_CONTRACT_NAME, TOKEN_NAME,
};

#[derive(Copy, Clone)]
pub(crate) struct TestContext {
    pub(crate) cep1155_token: ContractHash,
    pub(crate) cep1155_test_contract_package: ContractPackageHash,
}

pub(crate) fn setup() -> (InMemoryWasmTestBuilder, TestContext) {
    setup_with_args(runtime_args! {
        NAME => TOKEN_NAME,
    })
}
pub(crate) fn setup_with_args(install_args: RuntimeArgs) -> (InMemoryWasmTestBuilder, TestContext) {
    let mut builder = InMemoryWasmTestBuilder::default();
    builder.run_genesis(&PRODUCTION_RUN_GENESIS_REQUEST);

    let install_request_1 =
        ExecuteRequestBuilder::standard(*DEFAULT_ACCOUNT_ADDR, CEP1155_CONTRACT_WASM, install_args)
            .build();

    let install_request_2 = ExecuteRequestBuilder::standard(
        *DEFAULT_ACCOUNT_ADDR,
        CEP1155_TEST_CONTRACT_WASM,
        RuntimeArgs::default(),
    )
    .build();

    builder.exec(install_request_1).expect_success().commit();
    builder.exec(install_request_2).expect_success().commit();

    let account = builder
        .get_account(*DEFAULT_ACCOUNT_ADDR)
        .expect("should have account");

    let cep1155_token = account
        .named_keys()
        .get(CEP1155_TEST_TOKEN_CONTRACT_NAME)
        .and_then(|key| key.into_hash())
        .map(ContractHash::new)
        .expect("should have contract hash");

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
