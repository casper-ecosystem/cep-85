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
    ADMIN_LIST, ARG_ACCOUNT, ARG_ACCOUNTS, ARG_AMOUNTS, ARG_IDS, ARG_NAME, ARG_OWNER,
    ARG_RECIPIENT, ARG_TOTAL_SUPPLIES, ARG_TOTAL_SUPPLY, ARG_URI, BURNER_LIST,
    ENTRY_POINT_BATCH_BURN, ENTRY_POINT_BATCH_MINT, ENTRY_POINT_BURN, ENTRY_POINT_CHANGE_SECURITY,
    ENTRY_POINT_MINT, ENTRY_POINT_SET_TOTAL_SUPPLY_OF, ENTRY_POINT_SET_TOTAL_SUPPLY_OF_BATCH,
    ENTRY_POINT_SET_URI, META_LIST, MINTER_LIST, NONE_LIST, TOKEN_CONTRACT,
};
use cep85_test_contract::constants::{
    CEP85_TEST_PACKAGE_NAME, ENTRY_POINT_CHECK_BALANCE_OF, ENTRY_POINT_CHECK_SUPPLY_OF,
    ENTRY_POINT_CHECK_SUPPLY_OF_BATCH, ENTRY_POINT_CHECK_TOTAL_SUPPLY_OF,
    ENTRY_POINT_CHECK_TOTAL_SUPPLY_OF_BATCH, ENTRY_POINT_CHECK_URI, RESULT_KEY,
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

pub fn cep85_set_uri<'a>(
    builder: &'a mut InMemoryWasmTestBuilder,
    cep85_token: &'a ContractHash,
    updating_account: AccountHash,
    uri: &str,
    id: Option<U256>,
) -> &'a mut InMemoryWasmTestBuilder {
    let set_uri_args = if let Some(id) = id {
        runtime_args! {
            ARG_ID => id,
            ARG_URI => uri,
        }
    } else {
        runtime_args! {
            ARG_URI => uri,
        }
    };
    let set_uri_request = ExecuteRequestBuilder::contract_call_by_hash(
        updating_account,
        *cep85_token,
        ENTRY_POINT_SET_URI,
        set_uri_args,
    )
    .build();
    builder.exec(set_uri_request)
}

pub fn cep85_check_uri(
    builder: &mut InMemoryWasmTestBuilder,
    contract_package_hash: &ContractPackageHash,
    id: Option<U256>,
) -> String {
    let exec_request = ExecuteRequestBuilder::versioned_contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        *contract_package_hash,
        None,
        ENTRY_POINT_CHECK_URI,
        runtime_args! {
            ARG_ID => id,
        },
    )
    .build();
    builder.exec(exec_request).expect_success().commit();
    get_test_result(builder, *contract_package_hash)
}

pub fn cep85_mint<'a>(
    builder: &'a mut InMemoryWasmTestBuilder,
    cep85_token: &'a ContractHash,
    minting_account: AccountHash,
    recipient: Key,
    id: U256,
    amount: U256,
) -> &'a mut casper_engine_test_support::WasmTestBuilder<
    casper_execution_engine::storage::global_state::in_memory::InMemoryGlobalState,
> {
    let mint_request = ExecuteRequestBuilder::contract_call_by_hash(
        minting_account,
        *cep85_token,
        ENTRY_POINT_MINT,
        runtime_args! {
            ARG_RECIPIENT => recipient,
            ARG_ID => id,
            ARG_AMOUNT => amount,
        },
    )
    .build();
    builder.exec(mint_request)
}

pub fn cep85_batch_mint<'a>(
    builder: &'a mut InMemoryWasmTestBuilder,
    cep85_token: &'a ContractHash,
    minting_account: AccountHash,
    recipient: Key,
    ids: Vec<U256>,
    amounts: Vec<U256>,
) -> &'a mut InMemoryWasmTestBuilder {
    let mint_request = ExecuteRequestBuilder::contract_call_by_hash(
        minting_account,
        *cep85_token,
        ENTRY_POINT_BATCH_MINT,
        runtime_args! {
            ARG_RECIPIENT => recipient,
            ARG_IDS => ids,
            ARG_AMOUNTS => amounts,
        },
    )
    .build();
    builder.exec(mint_request)
}

pub fn cep85_burn<'a>(
    builder: &'a mut InMemoryWasmTestBuilder,
    cep85_token: &'a ContractHash,
    burning_account: AccountHash,
    owner: Key,
    id: U256,
    amount: U256,
) -> &'a mut casper_engine_test_support::WasmTestBuilder<
    casper_execution_engine::storage::global_state::in_memory::InMemoryGlobalState,
> {
    let burn_request = ExecuteRequestBuilder::contract_call_by_hash(
        burning_account,
        *cep85_token,
        ENTRY_POINT_BURN,
        runtime_args! {
            ARG_OWNER => owner,
            ARG_ID => id,
            ARG_AMOUNT => amount,
        },
    )
    .build();
    builder.exec(burn_request)
}

pub fn cep85_batch_burn<'a>(
    builder: &'a mut InMemoryWasmTestBuilder,
    cep85_token: &'a ContractHash,
    burning_account: AccountHash,
    owner: Key,
    ids: Vec<U256>,
    amounts: Vec<U256>,
) -> &'a mut InMemoryWasmTestBuilder {
    let burn_request = ExecuteRequestBuilder::contract_call_by_hash(
        burning_account,
        *cep85_token,
        ENTRY_POINT_BATCH_BURN,
        runtime_args! {
            ARG_OWNER => owner,
            ARG_IDS => ids,
            ARG_AMOUNTS => amounts,
        },
    )
    .build();
    builder.exec(burn_request)
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

pub fn cep85_set_total_supply_of<'a>(
    builder: &'a mut InMemoryWasmTestBuilder,
    cep85_token: &'a ContractHash,
    admin_account: AccountHash,
    id: U256,
    total_supply: U256,
) -> &'a mut InMemoryWasmTestBuilder {
    let set_total_supply_request = ExecuteRequestBuilder::contract_call_by_hash(
        admin_account,
        *cep85_token,
        ENTRY_POINT_SET_TOTAL_SUPPLY_OF,
        runtime_args! {
            ARG_ID => id,
            ARG_TOTAL_SUPPLY => total_supply,
        },
    )
    .build();
    builder.exec(set_total_supply_request)
}

pub fn cep85_check_total_supply_of(
    builder: &mut InMemoryWasmTestBuilder,
    contract_package_hash: &ContractPackageHash,
    id: U256,
) -> U256 {
    let check_total_supply_of_args = runtime_args! {
        ARG_ID => id,
    };
    let exec_request = ExecuteRequestBuilder::versioned_contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        *contract_package_hash,
        None,
        ENTRY_POINT_CHECK_TOTAL_SUPPLY_OF,
        check_total_supply_of_args,
    )
    .build();
    builder.exec(exec_request).expect_success().commit();
    get_test_result(builder, *contract_package_hash)
}

pub fn cep85_set_total_supply_of_batch<'a>(
    builder: &'a mut InMemoryWasmTestBuilder,
    cep85_token: &'a ContractHash,
    burning_account: AccountHash,
    ids: Vec<U256>,
    total_supplies: Vec<U256>,
) -> &'a mut casper_engine_test_support::WasmTestBuilder<
    casper_execution_engine::storage::global_state::in_memory::InMemoryGlobalState,
> {
    let set_total_supply_of_batch_request = ExecuteRequestBuilder::contract_call_by_hash(
        burning_account,
        *cep85_token,
        ENTRY_POINT_SET_TOTAL_SUPPLY_OF_BATCH,
        runtime_args! {
            ARG_IDS => ids,
            ARG_TOTAL_SUPPLIES => total_supplies,
        },
    )
    .build();
    builder.exec(set_total_supply_of_batch_request)
}

pub fn cep85_check_total_supply_of_batch(
    builder: &mut InMemoryWasmTestBuilder,
    contract_package_hash: &ContractPackageHash,
    ids: Vec<U256>,
) -> Vec<U256> {
    let check_total_supply_batch_of_args = runtime_args! {
        ARG_IDS => ids,
    };
    let exec_request = ExecuteRequestBuilder::versioned_contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        *contract_package_hash,
        None,
        ENTRY_POINT_CHECK_TOTAL_SUPPLY_OF_BATCH,
        check_total_supply_batch_of_args,
    )
    .build();
    builder.exec(exec_request).expect_success().commit();
    get_test_result(builder, *contract_package_hash)
}

pub fn cep85_check_supply_of(
    builder: &mut InMemoryWasmTestBuilder,
    contract_package_hash: &ContractPackageHash,
    id: U256,
) -> U256 {
    let check_supply_of_args = runtime_args! {
        ARG_ID => id,
    };
    let exec_request = ExecuteRequestBuilder::versioned_contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        *contract_package_hash,
        None,
        ENTRY_POINT_CHECK_SUPPLY_OF,
        check_supply_of_args,
    )
    .build();
    builder.exec(exec_request).expect_success().commit();
    get_test_result(builder, *contract_package_hash)
}

pub fn cep85_check_supply_of_batch(
    builder: &mut InMemoryWasmTestBuilder,
    contract_package_hash: &ContractPackageHash,
    accounts: Vec<Key>,
    ids: Vec<U256>,
) -> Vec<U256> {
    let check_supply_of_batch_args = runtime_args! {
        ARG_ACCOUNTS => accounts,
        ARG_IDS => ids,
    };
    let exec_request = ExecuteRequestBuilder::versioned_contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        *contract_package_hash,
        None,
        ENTRY_POINT_CHECK_SUPPLY_OF_BATCH,
        check_supply_of_batch_args,
    )
    .build();
    builder.exec(exec_request).expect_success().commit();
    get_test_result(builder, *contract_package_hash)
}

pub struct SecurityLists {
    pub minter_list: Option<Vec<Key>>,
    pub burner_list: Option<Vec<Key>>,
    pub meta_list: Option<Vec<Key>>,
    pub admin_list: Option<Vec<Key>>,
    pub none_list: Option<Vec<Key>>,
}

pub fn cep85_change_security<'a>(
    builder: &'a mut InMemoryWasmTestBuilder,
    cep85_token: &'a ContractHash,
    admin_account: AccountHash,
    security_lists: SecurityLists,
) -> &'a mut InMemoryWasmTestBuilder {
    let SecurityLists {
        minter_list,
        burner_list,
        meta_list,
        admin_list,
        none_list,
    } = security_lists;

    let change_security_request = ExecuteRequestBuilder::contract_call_by_hash(
        admin_account,
        *cep85_token,
        ENTRY_POINT_CHANGE_SECURITY,
        runtime_args! {
            MINTER_LIST => minter_list.unwrap_or_default(),
            BURNER_LIST => burner_list.unwrap_or_default(),
            META_LIST => meta_list.unwrap_or_default(),
            ADMIN_LIST => admin_list.unwrap_or_default(),
            NONE_LIST => none_list.unwrap_or_default(),
        },
    )
    .build();
    builder.exec(change_security_request)
}
