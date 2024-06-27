use super::constants::{
    CEP85_CONTRACT_WASM, CEP85_TEST_CONTRACT_WASM, CEP85_TEST_TOKEN_CONTRACT_NAME, TOKEN_NAME,
    TOKEN_URI,
};
use casper_engine_test_support::{
    utils::create_run_genesis_request, ExecuteRequestBuilder, LmdbWasmTestBuilder, ARG_AMOUNT,
    DEFAULT_ACCOUNTS, DEFAULT_ACCOUNT_ADDR,
};
use casper_types::{
    account::AccountHash,
    addressable_entity::EntityKindTag,
    bytesrepr::{Bytes, FromBytes},
    runtime_args,
    system::mint::{ARG_ID, ARG_TO},
    AddressableEntityHash, CLTyped, EntityAddr, Key, PackageHash, RuntimeArgs, U256,
};
use cep85::{
    constants::{
        ADMIN_LIST, ARG_ACCOUNT, ARG_ACCOUNTS, ARG_AMOUNTS, ARG_APPROVED, ARG_DATA,
        ARG_ENABLE_BURN, ARG_EVENTS_MODE, ARG_FROM, ARG_IDS, ARG_NAME, ARG_OPERATOR, ARG_OWNER,
        ARG_RECIPIENT, ARG_SESSION_NAMED_KEY_NAME, ARG_TOKEN_CONTRACT, ARG_TOTAL_SUPPLIES,
        ARG_TOTAL_SUPPLY, ARG_URI, BURNER_LIST, ENTRY_POINT_BATCH_BURN, ENTRY_POINT_BATCH_MINT,
        ENTRY_POINT_BATCH_TRANSFER_FROM, ENTRY_POINT_BURN, ENTRY_POINT_CHANGE_SECURITY,
        ENTRY_POINT_MAKE_DICTIONARY_ITEM_KEY, ENTRY_POINT_MINT, ENTRY_POINT_SET_APPROVAL_FOR_ALL,
        ENTRY_POINT_SET_MODALITIES, ENTRY_POINT_SET_TOTAL_SUPPLY_OF,
        ENTRY_POINT_SET_TOTAL_SUPPLY_OF_BATCH, ENTRY_POINT_SET_URI, ENTRY_POINT_TRANSFER_FROM,
        META_LIST, MINTER_LIST, NONE_LIST,
    },
    modalities::EventsMode,
};
use cep85_test_contract::constants::{
    CEP85_TEST_CONTRACT_NAME, CEP85_TEST_PACKAGE_NAME, ENTRY_POINT_CHECK_BALANCE_OF,
    ENTRY_POINT_CHECK_BALANCE_OF_BATCH, ENTRY_POINT_CHECK_BATCH_TRANSFER_FROM,
    ENTRY_POINT_CHECK_IS_APPROVED_FOR_ALL, ENTRY_POINT_CHECK_IS_NON_FUNGIBLE,
    ENTRY_POINT_CHECK_SUPPLY_OF, ENTRY_POINT_CHECK_SUPPLY_OF_BATCH,
    ENTRY_POINT_CHECK_TOTAL_FUNGIBLE_SUPPLY, ENTRY_POINT_CHECK_TOTAL_SUPPLY_OF,
    ENTRY_POINT_CHECK_TOTAL_SUPPLY_OF_BATCH, ENTRY_POINT_CHECK_TRANSFER_FROM,
    ENTRY_POINT_CHECK_URI, RESULT_KEY,
};

#[derive(Clone)]
pub struct TestContext {
    pub cep18_contract_hash: AddressableEntityHash,
    pub cep85_test_contract: AddressableEntityHash,
    pub cep85_test_contract_package: PackageHash,
}

impl Drop for TestContext {
    fn drop(&mut self) {}
}

fn default_args() -> RuntimeArgs {
    runtime_args! {
        ARG_NAME => TOKEN_NAME,
        ARG_URI => TOKEN_URI,
    }
}

pub fn setup() -> (LmdbWasmTestBuilder, TestContext) {
    setup_with_args(default_args())
}

pub fn setup_with_args(install_args: RuntimeArgs) -> (LmdbWasmTestBuilder, TestContext) {
    let mut builder = LmdbWasmTestBuilder::default();

    builder
        .run_genesis(create_run_genesis_request(DEFAULT_ACCOUNTS.to_vec()))
        .commit();

    let install_request_contract = ExecuteRequestBuilder::standard(
        *DEFAULT_ACCOUNT_ADDR,
        CEP85_CONTRACT_WASM,
        merge_args(install_args.clone()),
    )
    .build();

    builder
        .exec(install_request_contract)
        .expect_success()
        .commit();

    let account = builder
        .get_entity_with_named_keys_by_account_hash(*DEFAULT_ACCOUNT_ADDR)
        .expect("should have account");

    let cep18_contract_hash = account
        .named_keys()
        .get(CEP85_TEST_TOKEN_CONTRACT_NAME)
        .and_then(|key| key.into_entity_hash())
        .map(AddressableEntityHash::from)
        .expect("should have contract hash");

    let contract_key =
        Key::addressable_entity_key(EntityKindTag::SmartContract, cep18_contract_hash);

    let install_request_contract_test = ExecuteRequestBuilder::standard(
        *DEFAULT_ACCOUNT_ADDR,
        CEP85_TEST_CONTRACT_WASM,
        runtime_args! {
            ARG_TOKEN_CONTRACT => contract_key
        },
    )
    .build();

    builder
        .exec(install_request_contract_test)
        .expect_success()
        .commit();

    let account = builder
        .get_entity_with_named_keys_by_account_hash(*DEFAULT_ACCOUNT_ADDR)
        .expect("should have account");

    let cep85_test_contract = account
        .named_keys()
        .get(CEP85_TEST_CONTRACT_NAME)
        .and_then(|key| key.into_entity_hash())
        .map(AddressableEntityHash::from)
        .expect("should have contract hash");

    let cep85_test_contract_package = account
        .named_keys()
        .get(CEP85_TEST_PACKAGE_NAME)
        .and_then(|key| key.into_package_hash())
        .map(PackageHash::from)
        .expect("should have contract package hash");

    let test_context = TestContext {
        cep18_contract_hash,
        cep85_test_contract,
        cep85_test_contract_package,
    };

    (builder, test_context)
}

pub fn get_test_result<T: FromBytes + CLTyped>(
    builder: &mut LmdbWasmTestBuilder,
    cep85_test_contract_package: PackageHash,
) -> T {
    let contract_package = builder
        .get_package(cep85_test_contract_package)
        .expect("should have contract package");
    let mut enabled_versions = contract_package.enabled_versions();
    let maybe_first_enabled_version = enabled_versions.maybe_first();
    let (_version, contract_hash) = maybe_first_enabled_version
        .iter()
        .next_back()
        .expect("should have latest version");
    let contract_entity_addr = EntityAddr::new_smart_contract(contract_hash.value());
    builder.get_value(contract_entity_addr, RESULT_KEY)
}

pub fn cep85_set_uri<'a>(
    builder: &'a mut LmdbWasmTestBuilder,
    cep18_contract_hash: &'a AddressableEntityHash,
    updating_account: &'a AccountHash,
    uri: &str,
    id: Option<U256>,
) -> &'a mut LmdbWasmTestBuilder {
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
        *updating_account,
        *cep18_contract_hash,
        ENTRY_POINT_SET_URI,
        set_uri_args,
    )
    .build();
    builder.exec(set_uri_request)
}

pub fn cep85_check_uri(
    builder: &mut LmdbWasmTestBuilder,
    contract_package_hash: &PackageHash,
    id: Option<U256>,
) -> Option<String> {
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
    builder: &'a mut LmdbWasmTestBuilder,
    cep18_contract_hash: &'a AddressableEntityHash,
    minting_account: &'a AccountHash,
    recipient: &Key,
    id: &U256,
    amount: &U256,
    uri: Option<&str>,
) -> &'a mut LmdbWasmTestBuilder {
    let mut mint_args = runtime_args! {
        ARG_RECIPIENT => *recipient,
        ARG_ID => *id,
        ARG_AMOUNT => *amount,
    };
    if uri.is_some() {
        let _ = mint_args.insert(ARG_URI, uri.unwrap_or_default());
    }
    let mint_request = ExecuteRequestBuilder::contract_call_by_hash(
        *minting_account,
        *cep18_contract_hash,
        ENTRY_POINT_MINT,
        mint_args,
    )
    .build();
    builder.exec(mint_request)
}

pub fn cep85_batch_mint<'a>(
    builder: &'a mut LmdbWasmTestBuilder,
    cep18_contract_hash: &'a AddressableEntityHash,
    minting_account: &'a AccountHash,
    recipient: &Key,
    ids: Vec<U256>,
    amounts: Vec<U256>,
    uri: Option<&str>,
) -> &'a mut LmdbWasmTestBuilder {
    let mut batch_mint_args = runtime_args! {
        ARG_RECIPIENT => *recipient,
        ARG_IDS => ids,
        ARG_AMOUNTS => amounts,
    };
    if uri.is_some() {
        let _ = batch_mint_args.insert(ARG_URI, uri.unwrap_or_default());
    }
    let mint_request = ExecuteRequestBuilder::contract_call_by_hash(
        *minting_account,
        *cep18_contract_hash,
        ENTRY_POINT_BATCH_MINT,
        batch_mint_args,
    )
    .build();
    builder.exec(mint_request)
}

pub fn cep85_burn<'a>(
    builder: &'a mut LmdbWasmTestBuilder,
    contract_hash: &'a AddressableEntityHash,
    burning_account: &'a AccountHash,
    owner: &Key,
    id: &U256,
    amount: &U256,
) -> &'a mut LmdbWasmTestBuilder {
    let burn_request = ExecuteRequestBuilder::contract_call_by_hash(
        *burning_account,
        *contract_hash,
        ENTRY_POINT_BURN,
        runtime_args! {
            ARG_OWNER => *owner,
            ARG_ID => *id,
            ARG_AMOUNT => *amount,
        },
    )
    .build();
    builder.exec(burn_request)
}

pub fn cep85_batch_burn<'a>(
    builder: &'a mut LmdbWasmTestBuilder,
    contract_hash: &'a AddressableEntityHash,
    burning_account: &'a AccountHash,
    owner: &Key,
    ids: Vec<U256>,
    amounts: Vec<U256>,
) -> &'a mut LmdbWasmTestBuilder {
    let burn_request = ExecuteRequestBuilder::contract_call_by_hash(
        *burning_account,
        *contract_hash,
        ENTRY_POINT_BATCH_BURN,
        runtime_args! {
            ARG_OWNER => *owner,
            ARG_IDS => ids,
            ARG_AMOUNTS => amounts,
        },
    )
    .build();
    builder.exec(burn_request)
}

pub fn cep85_check_balance_of(
    builder: &mut LmdbWasmTestBuilder,
    contract_package_hash: &PackageHash,
    account: &Key,
    id: &U256,
) -> Option<U256> {
    let check_balance_args = runtime_args! {
        ARG_ACCOUNT => *account,
        ARG_ID => *id,
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

pub fn cep85_check_balance_of_batch(
    builder: &mut LmdbWasmTestBuilder,
    contract_package_hash: &PackageHash,
    accounts: Vec<Key>,
    ids: Vec<U256>,
) -> Vec<Option<U256>> {
    let check_balance_args = runtime_args! {
        ARG_ACCOUNTS => accounts,
        ARG_IDS => ids,
    };
    let exec_request = ExecuteRequestBuilder::versioned_contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        *contract_package_hash,
        None,
        ENTRY_POINT_CHECK_BALANCE_OF_BATCH,
        check_balance_args,
    )
    .build();
    builder.exec(exec_request).expect_success().commit();
    get_test_result(builder, *contract_package_hash)
}

pub fn cep85_set_total_supply_of<'a>(
    builder: &'a mut LmdbWasmTestBuilder,
    cep18_contract_hash: &'a AddressableEntityHash,
    admin_account: &'a AccountHash,
    id: &U256,
    total_supply: &U256,
) -> &'a mut LmdbWasmTestBuilder {
    let set_total_supply_request = ExecuteRequestBuilder::contract_call_by_hash(
        *admin_account,
        *cep18_contract_hash,
        ENTRY_POINT_SET_TOTAL_SUPPLY_OF,
        runtime_args! {
            ARG_ID => *id,
            ARG_TOTAL_SUPPLY => *total_supply,
        },
    )
    .build();
    builder.exec(set_total_supply_request)
}

pub fn cep85_check_total_supply_of(
    builder: &mut LmdbWasmTestBuilder,
    contract_package_hash: &PackageHash,
    id: &U256,
) -> Option<U256> {
    let check_total_supply_of_args = runtime_args! {
        ARG_ID => *id,
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
    builder: &'a mut LmdbWasmTestBuilder,
    cep18_contract_hash: &'a AddressableEntityHash,
    sender: &'a AccountHash,
    ids: Vec<U256>,
    total_supplies: Vec<U256>,
) -> &'a mut LmdbWasmTestBuilder {
    let set_total_supply_of_batch_request = ExecuteRequestBuilder::contract_call_by_hash(
        *sender,
        *cep18_contract_hash,
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
    builder: &mut LmdbWasmTestBuilder,
    contract_package_hash: &PackageHash,
    ids: Vec<U256>,
) -> Vec<Option<U256>> {
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
    builder: &mut LmdbWasmTestBuilder,
    contract_package_hash: &PackageHash,
    id: &U256,
) -> Option<U256> {
    let check_supply_of_args = runtime_args! {
        ARG_ID => *id,
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
    builder: &mut LmdbWasmTestBuilder,
    contract_package_hash: &PackageHash,
    ids: Vec<U256>,
) -> Vec<Option<U256>> {
    let check_supply_of_batch_args = runtime_args! {
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

pub fn cep85_set_approval_for_all<'a>(
    builder: &'a mut LmdbWasmTestBuilder,
    cep18_contract_hash: &'a AddressableEntityHash,
    owner: &'a AccountHash,
    operator: &'a Key,
    approved: bool,
) -> &'a mut LmdbWasmTestBuilder {
    let set_approval_for_all_request = ExecuteRequestBuilder::contract_call_by_hash(
        *owner,
        *cep18_contract_hash,
        ENTRY_POINT_SET_APPROVAL_FOR_ALL,
        runtime_args! {
            ARG_OPERATOR => *operator,
            ARG_APPROVED => approved,
        },
    )
    .build();
    builder.exec(set_approval_for_all_request)
}

pub fn cep85_check_is_approved(
    builder: &mut LmdbWasmTestBuilder,
    contract_package_hash: &PackageHash,
    account: &Key,
    operator: &Key,
) -> bool {
    let check_is_approved_args = runtime_args! {
        ARG_ACCOUNT => *account,
        ARG_OPERATOR => *operator,
    };
    let exec_request = ExecuteRequestBuilder::versioned_contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        *contract_package_hash,
        None,
        ENTRY_POINT_CHECK_IS_APPROVED_FOR_ALL,
        check_is_approved_args,
    )
    .build();
    builder.exec(exec_request).expect_success().commit();
    get_test_result(builder, *contract_package_hash)
}

pub fn cep85_make_dictionary_item_key(
    builder: &mut LmdbWasmTestBuilder,
    cep18_contract_hash: &AddressableEntityHash,
    key: &Key,
    id: Option<U256>,
    operator: Option<Key>,
    session_named_key_name: Option<String>,
) {
    let mut args = runtime_args! {
        ARG_OWNER => *key,
    };
    let _ = match id {
        Some(id) => args.insert(ARG_ID, id),
        None => Ok(()),
    };
    let _ = match operator {
        Some(operator) => args.insert(ARG_OPERATOR, operator),
        None => Ok(()),
    };
    let _ = match session_named_key_name {
        Some(session_named_key_name) => {
            args.insert(ARG_SESSION_NAMED_KEY_NAME, session_named_key_name)
        }
        None => Ok(()),
    };
    let dictionary_item_key_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        *cep18_contract_hash,
        ENTRY_POINT_MAKE_DICTIONARY_ITEM_KEY,
        args,
    )
    .build();
    builder
        .exec(dictionary_item_key_request)
        .expect_success()
        .commit();
}

pub struct TransferData<'a> {
    pub from: &'a Key,
    pub to: &'a Key,
    pub ids: Vec<U256>,
    pub amounts: Vec<U256>,
    pub data: Option<Bytes>,
}

pub fn cep85_transfer_from<'a>(
    builder: &'a mut LmdbWasmTestBuilder,
    cep18_contract_hash: &'a AddressableEntityHash,
    sender: &'a AccountHash,
    transfer_data: TransferData<'a>,
    direct_call_test_contract: Option<bool>,
) -> &'a mut LmdbWasmTestBuilder {
    let TransferData {
        from,
        to,
        ids,
        amounts,
        data,
    } = transfer_data;

    let transfer_request = match from {
        Key::Account(_hash) => {
            let mut args = runtime_args! {
                ARG_FROM => *from,
                ARG_TO => *to,
                ARG_ID => ids[0],
                ARG_AMOUNT => amounts[0],
            };

            if let Some(data) = data {
                let _ = args.insert(ARG_DATA, data);
            }

            ExecuteRequestBuilder::contract_call_by_hash(
                *sender, /* We do not use above _hash here because from and sender could be
                          * different */
                *cep18_contract_hash,
                ENTRY_POINT_TRANSFER_FROM,
                args,
            )
            .build()
        }
        Key::Hash(hash) => {
            let hash_bytes: &[u8; 32] = hash.as_slice().try_into().expect("Hash must be 32 bytes");
            let call_package =
                direct_call_test_contract.is_none() || direct_call_test_contract == Some(false);
            if call_package {
                let contract_package_hash = PackageHash::from(*hash_bytes);
                let args = runtime_args! {
                    ARG_FROM => *from,
                    ARG_TO => *to,
                    ARG_ID => ids[0],
                    ARG_AMOUNT => amounts[0],
                    ARG_DATA => data,
                };

                ExecuteRequestBuilder::versioned_contract_call_by_hash(
                    *sender,
                    contract_package_hash,
                    None,
                    ENTRY_POINT_CHECK_TRANSFER_FROM,
                    args,
                )
                .build()
            } else {
                let contract_hash = AddressableEntityHash::from(*hash_bytes);
                let args = runtime_args! {
                    ARG_FROM => *from,
                    ARG_TO => *to,
                    ARG_ID => ids[0],
                    ARG_AMOUNT => amounts[0],
                    ARG_DATA => data,
                };

                ExecuteRequestBuilder::contract_call_by_hash(
                    *sender,
                    contract_hash,
                    ENTRY_POINT_CHECK_TRANSFER_FROM,
                    args,
                )
                .build()
            }
        }
        _ => panic!("Unknown variant"),
    };
    builder.exec(transfer_request)
}

pub fn cep85_transfer_from_as_contract<'a>(
    builder: &'a mut LmdbWasmTestBuilder,
    contract_package_hash: &'a PackageHash,
    sender: &'a AccountHash,
    transfer_data: TransferData<'a>,
) -> &'a mut LmdbWasmTestBuilder {
    let TransferData {
        from,
        to,
        ids,
        amounts,
        data,
    } = transfer_data;

    let transfer_request = ExecuteRequestBuilder::versioned_contract_call_by_hash(
        *sender,
        *contract_package_hash,
        None,
        ENTRY_POINT_CHECK_TRANSFER_FROM,
        runtime_args! {
            ARG_FROM => *from,
            ARG_TO => *to,
            ARG_ID => ids[0],
            ARG_AMOUNT => amounts[0],
            ARG_DATA => data,
        },
    )
    .build();

    builder.exec(transfer_request)
}

pub fn cep85_batch_transfer_from<'a>(
    builder: &'a mut LmdbWasmTestBuilder,
    cep18_contract_hash: &'a AddressableEntityHash,
    sender: &'a AccountHash,
    transfer_data: TransferData<'a>,
    direct_call_test_contract: Option<bool>,
) -> &'a mut LmdbWasmTestBuilder {
    let TransferData {
        from,
        to,
        ids,
        amounts,
        data,
    } = transfer_data;

    let transfer_request = match from {
        Key::Account(_hash) => {
            let mut args = runtime_args! {
                ARG_FROM => *from,
                ARG_TO => *to,
                ARG_IDS => ids,
                ARG_AMOUNTS => amounts,
            };

            if let Some(data) = data {
                let _ = args.insert(ARG_DATA, data);
            }
            ExecuteRequestBuilder::contract_call_by_hash(
                *sender, /* We do not use above _hash here because from and sender could be
                          * different */
                *cep18_contract_hash,
                ENTRY_POINT_BATCH_TRANSFER_FROM,
                args,
            )
            .build()
        }
        Key::Hash(hash) => {
            let hash_bytes: &[u8; 32] = hash.as_slice().try_into().expect("Hash must be 32 bytes");
            let call_package =
                direct_call_test_contract.is_none() || direct_call_test_contract == Some(false);
            if call_package {
                let contract_package_hash = PackageHash::from(*hash_bytes);
                ExecuteRequestBuilder::versioned_contract_call_by_hash(
                    *sender,
                    contract_package_hash,
                    None,
                    ENTRY_POINT_CHECK_BATCH_TRANSFER_FROM,
                    runtime_args! {
                        ARG_FROM => *from,
                        ARG_TO => *to,
                        ARG_IDS => ids,
                        ARG_AMOUNTS => amounts,
                        ARG_DATA => data,
                    },
                )
                .build()
            } else {
                let contract_hash = AddressableEntityHash::from(*hash_bytes);
                ExecuteRequestBuilder::contract_call_by_hash(
                    *sender,
                    contract_hash,
                    ENTRY_POINT_CHECK_BATCH_TRANSFER_FROM,
                    runtime_args! {
                        ARG_FROM => *from,
                        ARG_TO => *to,
                        ARG_IDS => ids,
                        ARG_AMOUNTS => amounts,
                        ARG_DATA => data,
                    },
                )
                .build()
            }
        }
        _ => panic!("Unknown variant"),
    };

    builder.exec(transfer_request)
}

pub fn cep85_batch_transfer_from_as_contract<'a>(
    builder: &'a mut LmdbWasmTestBuilder,
    contract_package_hash: &'a PackageHash,
    sender: &'a AccountHash,
    transfer_data: TransferData<'a>,
) -> &'a mut LmdbWasmTestBuilder {
    let TransferData {
        from,
        to,
        ids,
        amounts,
        data,
    } = transfer_data;

    let transfer_request = ExecuteRequestBuilder::versioned_contract_call_by_hash(
        *sender,
        *contract_package_hash,
        None,
        ENTRY_POINT_CHECK_BATCH_TRANSFER_FROM,
        runtime_args! {
            ARG_FROM => *from,
            ARG_TO => *to,
            ARG_IDS => ids,
            ARG_AMOUNTS => amounts,
            ARG_DATA => data,
        },
    )
    .build();

    builder.exec(transfer_request)
}

pub fn cep85_check_is_non_fungible(
    builder: &mut LmdbWasmTestBuilder,
    contract_package_hash: &PackageHash,
    id: &U256,
) -> Option<bool> {
    let check_is_non_fungible_args = runtime_args! {
        ARG_ID => *id,
    };
    let exec_request = ExecuteRequestBuilder::versioned_contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        *contract_package_hash,
        None,
        ENTRY_POINT_CHECK_IS_NON_FUNGIBLE,
        check_is_non_fungible_args,
    )
    .build();
    builder.exec(exec_request).expect_success().commit();
    get_test_result(builder, *contract_package_hash)
}

pub fn cep85_check_total_fungible_supply(
    builder: &mut LmdbWasmTestBuilder,
    contract_package_hash: &PackageHash,
    id: &U256,
) -> Option<U256> {
    let check_total_fungible_supply_args = runtime_args! {
        ARG_ID => *id,
    };
    let exec_request = ExecuteRequestBuilder::versioned_contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        *contract_package_hash,
        None,
        ENTRY_POINT_CHECK_TOTAL_FUNGIBLE_SUPPLY,
        check_total_fungible_supply_args,
    )
    .build();
    builder.exec(exec_request).expect_success().commit();
    get_test_result(builder, *contract_package_hash)
}

pub fn cep85_set_modalities<'a>(
    builder: &'a mut LmdbWasmTestBuilder,
    cep18_contract_hash: &'a AddressableEntityHash,
    owner: &'a AccountHash,
    burn_enable: Option<bool>,
    events_mode: Option<EventsMode>,
) -> &'a mut LmdbWasmTestBuilder {
    let mut args = runtime_args! {};
    if let Some(burn_enable) = burn_enable {
        let _ = args.insert(ARG_ENABLE_BURN, burn_enable);
    };
    if let Some(events_mode) = events_mode {
        let _ = args.insert(ARG_EVENTS_MODE, events_mode as u8);
    };
    let set_modalities_request = ExecuteRequestBuilder::contract_call_by_hash(
        *owner,
        *cep18_contract_hash,
        ENTRY_POINT_SET_MODALITIES,
        args,
    )
    .build();
    builder.exec(set_modalities_request)
}

pub struct SecurityLists {
    pub minter_list: Option<Vec<Key>>,
    pub burner_list: Option<Vec<Key>>,
    pub meta_list: Option<Vec<Key>>,
    pub admin_list: Option<Vec<Key>>,
    pub none_list: Option<Vec<Key>>,
}

pub fn cep85_change_security<'a>(
    builder: &'a mut LmdbWasmTestBuilder,
    cep18_contract_hash: &'a AddressableEntityHash,
    admin_account: &'a AccountHash,
    security_lists: SecurityLists,
) -> &'a mut LmdbWasmTestBuilder {
    let SecurityLists {
        minter_list,
        burner_list,
        meta_list,
        admin_list,
        none_list,
    } = security_lists;

    let change_security_request = ExecuteRequestBuilder::contract_call_by_hash(
        *admin_account,
        *cep18_contract_hash,
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

fn merge_args(install_args: RuntimeArgs) -> RuntimeArgs {
    let mut merged_args = install_args;

    if merged_args.get(ARG_NAME).is_none() {
        if let Some(default_name_value) = default_args().get(ARG_NAME) {
            merged_args.insert_cl_value(ARG_NAME, default_name_value.clone());
        }
    }
    if merged_args.get(ARG_URI).is_none() {
        if let Some(default_uri_value) = default_args().get(ARG_URI) {
            merged_args.insert_cl_value(ARG_URI, default_uri_value.clone());
        }
    }
    merged_args
}
