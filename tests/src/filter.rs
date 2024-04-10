use casper_engine_test_support::{
    ExecuteRequestBuilder, InMemoryWasmTestBuilder, ARG_AMOUNT, DEFAULT_ACCOUNT_ADDR,
    PRODUCTION_RUN_GENESIS_REQUEST,
};
use casper_types::{
    bytesrepr::Bytes,
    runtime_args,
    system::mint::{ARG_ID, ARG_TO},
    ContractHash, ContractPackageHash, Key, RuntimeArgs, U256,
};
use cep85::{
    constants::{
        ARG_DATA, ARG_FROM, ARG_NAME, ARG_TOKEN_CONTRACT, ARG_TRANSFER_FILTER_CONTRACT,
        ARG_TRANSFER_FILTER_METHOD, ARG_URI, ENTRY_POINT_INIT, ENTRY_POINT_TRANSFER_FROM,
    },
    error::Cep85Error,
    modalities::TransferFilterContractResult,
};
use cep85_test_contract::constants::{
    ARG_FILTER_CONTRACT_RETURN_VALUE, CEP85_TEST_CONTRACT_NAME, CEP85_TEST_PACKAGE_NAME,
    ENTRY_POINT_SET_FILTER_CONTRACT_RETURN_VALUE, ENTRY_POINT_TRANSFER_FILTER_METHOD,
};

use crate::utility::{
    constants::{
        ACCOUNT_USER_1, ACCOUNT_USER_2, CEP85_CONTRACT_WASM, CEP85_TEST_CONTRACT_WASM,
        CEP85_TEST_TOKEN_CONTRACT_NAME, TOKEN_NAME, TOKEN_URI,
    },
    installer_request_builders::{
        cep85_batch_mint, cep85_check_balance_of, cep85_check_balance_of_batch,
        cep85_set_total_supply_of_batch, cep85_transfer_from, TransferData,
    },
    support::{assert_expected_error, create_funded_dummy_account},
};

#[test]
fn check_transfers_with_transfer_filter_contract() {
    let mut builder = InMemoryWasmTestBuilder::default();
    builder.run_genesis(&PRODUCTION_RUN_GENESIS_REQUEST);

    let account_user_1 = create_funded_dummy_account(&mut builder, Some(ACCOUNT_USER_1));
    let account_user_2 = create_funded_dummy_account(&mut builder, Some(ACCOUNT_USER_2));

    // Install filter contract first with empty TOKEN_CONTRACT value, we will update it after token
    // installation
    let install_request_contract_test = ExecuteRequestBuilder::standard(
        *DEFAULT_ACCOUNT_ADDR,
        CEP85_TEST_CONTRACT_WASM,
        runtime_args! {
            ARG_TOKEN_CONTRACT => Key::from(ContractHash::from([0u8; 32])),
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

    let transfer_filter_contract = account
        .named_keys()
        .get(CEP85_TEST_CONTRACT_NAME)
        .and_then(|key| key.into_hash())
        .map(ContractHash::new)
        .expect("should have contract hash");

    let install_args = runtime_args! {
        ARG_NAME => TOKEN_NAME,
        ARG_URI => TOKEN_URI,
        ARG_TRANSFER_FILTER_CONTRACT => Key::from(transfer_filter_contract),
        ARG_TRANSFER_FILTER_METHOD => ENTRY_POINT_TRANSFER_FILTER_METHOD
    };

    // Install token
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

    let cep85_test_contract_package = account
        .named_keys()
        .get(CEP85_TEST_PACKAGE_NAME)
        .and_then(|key| key.into_hash())
        .map(ContractPackageHash::new)
        .expect("should have contract package hash");

    let transfer_filter_contract_stored: ContractHash = builder
        .get_value::<Option<ContractHash>>(cep85_token, ARG_TRANSFER_FILTER_CONTRACT)
        .unwrap();
    let transfer_filter_method_stored: String = builder
        .get_value::<Option<String>>(cep85_token, ARG_TRANSFER_FILTER_METHOD)
        .unwrap();

    assert_eq!(transfer_filter_contract_stored, transfer_filter_contract);
    assert_eq!(
        transfer_filter_method_stored,
        ENTRY_POINT_TRANSFER_FILTER_METHOD
    );

    // Update test contract TOKEN_CONTRACT value
    let set_token_contract_request_for_transfer_filter_contract =
        ExecuteRequestBuilder::contract_call_by_hash(
            *DEFAULT_ACCOUNT_ADDR,
            transfer_filter_contract,
            ENTRY_POINT_INIT,
            runtime_args! {
                ARG_TOKEN_CONTRACT => Key::from(cep85_token)
            },
        )
        .build();

    builder
        .exec(set_token_contract_request_for_transfer_filter_contract)
        .expect_success()
        .commit();

    let contract = builder
        .get_contract(transfer_filter_contract)
        .expect("should have contract");
    let named_keys = contract.named_keys();
    let token_contract_stored = *named_keys.get(ARG_TOKEN_CONTRACT).unwrap();

    assert_eq!(token_contract_stored, Key::from(cep85_token));

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let recipient_user_1 = Key::from(account_user_1);
    let ids: Vec<U256> = vec![U256::one(), U256::from(2)];
    let amounts: Vec<U256> = vec![U256::one(), U256::from(2)];
    let total_supplies = amounts.clone();

    let set_total_supply_of_batch_call = cep85_set_total_supply_of_batch(
        &mut builder,
        &cep85_token,
        &minting_account,
        ids.clone(),
        total_supplies,
    );

    set_total_supply_of_batch_call.expect_success().commit();

    // batch_mint is only one recipient
    let mint_call = cep85_batch_mint(
        &mut builder,
        &cep85_token,
        &minting_account,
        &recipient_user_1,
        ids.clone(),
        amounts,
        None,
    );

    mint_call.expect_success().commit();

    let recipients: Vec<Key> = vec![recipient_user_1, recipient_user_1];

    let actual_balances = cep85_check_balance_of_batch(
        &mut builder,
        &cep85_test_contract_package,
        recipients,
        ids.clone(),
    );

    let expected_balances = vec![U256::one(), U256::from(2)];

    assert_eq!(actual_balances, expected_balances);

    let id = ids[0];
    let from = recipient_user_1;
    let to = Key::from(account_user_2);
    let transfer_amount = U256::one();
    let data = Some(Bytes::default());

    let failing_transfer_call = cep85_transfer_from(
        &mut builder,
        &cep85_token,
        &account_user_1,
        TransferData {
            from: &from,
            to: &to,
            ids: vec![id],
            amounts: vec![transfer_amount],
            data: data.clone(),
        },
        None,
    );
    failing_transfer_call.expect_failure();

    let error = builder.get_error().expect("must have error");

    assert_expected_error(
        error,
        Cep85Error::TransferFilterContractDenied as u16,
        "should not allow transfer with default TransferFilterContractResult::DenyTransfer",
    );

    let actual_balance_from =
        cep85_check_balance_of(&mut builder, &cep85_test_contract_package, &from, &id);
    let expected_balance_from = U256::one();

    assert_eq!(actual_balance_from, expected_balance_from);

    let actual_balance_to =
        cep85_check_balance_of(&mut builder, &cep85_test_contract_package, &to, &id);
    let expected_balance_to = U256::zero();

    assert_eq!(actual_balance_to, expected_balance_to);

    let transfer_filter_contract_set_return_value_request =
        ExecuteRequestBuilder::contract_call_by_hash(
            *DEFAULT_ACCOUNT_ADDR,
            transfer_filter_contract,
            ENTRY_POINT_SET_FILTER_CONTRACT_RETURN_VALUE,
            runtime_args! {
                ARG_FILTER_CONTRACT_RETURN_VALUE => TransferFilterContractResult::ProceedTransfer
            },
        )
        .build();

    builder
        .exec(transfer_filter_contract_set_return_value_request)
        .expect_success()
        .commit();

    let transfer_call = cep85_transfer_from(
        &mut builder,
        &cep85_token,
        &account_user_1,
        TransferData {
            from: &from,
            to: &to,
            ids: vec![id],
            amounts: vec![transfer_amount],
            data: data.clone(),
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

    // NB: token_receiver and token_owner are swapped
    let failing_transfer_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        cep85_token,
        ENTRY_POINT_TRANSFER_FROM,
        runtime_args! {
            ARG_FROM => to,
            ARG_TO => from,
            ARG_ID => id,
            ARG_AMOUNT => transfer_amount,
            ARG_DATA => data.clone(),
        },
    )
    .build();
    let failing_transfer_call = builder.exec(failing_transfer_request);
    failing_transfer_call.expect_failure();

    let error = builder.get_error().expect("must have error");

    assert_expected_error(
        error,
        Cep85Error::NotApproved as u16,
        "should not allow transfer when from operator is not owner of the token",
    );

    let id = ids[1];
    let transfer_amount = U256::from(2);

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
    transfer_call.expect_success().commit();
}
