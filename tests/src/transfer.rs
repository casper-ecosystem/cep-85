use crate::utility::{
    constants::{ACCOUNT_USER_1, ACCOUNT_USER_2},
    installer_request_builders::{
        cep85_batch_mint, cep85_batch_transfer_from, cep85_check_balance_of,
        cep85_check_balance_of_batch, cep85_mint, cep85_set_total_supply_of_batch,
        cep85_transfer_from, setup, setup_with_args, TestContext, TransferData,
    },
    support::{assert_expected_error, get_event},
};
use casper_engine_test_support::DEFAULT_ACCOUNT_ADDR;
use casper_types::{bytesrepr::Bytes, runtime_args, Key, RuntimeArgs, U256};
use cep85::{
    constants::ARG_EVENTS_MODE,
    error::Cep85Error,
    events::{Transfer, TransferBatch},
    modalities::EventsMode,
};

#[test]
fn should_transfer_full_owned_amount() {
    let (
        mut builder,
        TestContext {
            cep85_token,
            cep85_test_contract_package,
            ref test_accounts,
            ..
        },
    ) = setup_with_args(
        runtime_args! {
            ARG_EVENTS_MODE => EventsMode::CES as u8,
        },
        None,
    );

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let minting_recipient = Key::from(minting_account);
    let mint_amount = U256::one();
    let id = U256::one();

    let mint_call = cep85_mint(
        &mut builder,
        &cep85_token,
        &minting_account,
        &minting_recipient,
        &id,
        &mint_amount,
        None,
    );

    mint_call.expect_success().commit();

    let actual_balance_before = cep85_check_balance_of(
        &mut builder,
        &cep85_test_contract_package,
        &minting_recipient,
        &id,
    )
    .unwrap();
    let expected_balance_before = U256::one();

    assert_eq!(actual_balance_before, expected_balance_before);

    let from = Key::from(minting_account);
    let to = Key::from(*test_accounts.get(&ACCOUNT_USER_1).unwrap());
    let transfer_amount = U256::one();
    let data = Some(Bytes::from("Casper Labs free bytes".as_bytes()));

    let transfer_call = cep85_transfer_from(
        &mut builder,
        &cep85_token,
        &minting_account,
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

    // Expect Transfer event
    let expected_event = Transfer::new(from, from, to, id, transfer_amount, data);
    let event_index = 1; // (Mint + transfer)
    let actual_event: Transfer = get_event(&builder, &cep85_token.into(), event_index);
    assert_eq!(actual_event, expected_event, "Expected Transfer event.");

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
fn should_batch_transfer_full_owned_amount() {
    let (
        mut builder,
        TestContext {
            cep85_token,
            cep85_test_contract_package,
            ref test_accounts,
            ..
        },
    ) = setup_with_args(
        runtime_args! {
            ARG_EVENTS_MODE => EventsMode::CES as u8,
        },
        None,
    );

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let minting_recipient = Key::from(minting_account);
    let ids: Vec<U256> = vec![U256::one(), U256::from(2)];
    let amounts: Vec<U256> = vec![U256::one(), U256::from(2)];
    let from = minting_recipient;
    let to = Key::from(*test_accounts.get(&ACCOUNT_USER_1).unwrap());
    let data = Some(Bytes::from("Casper Labs free bytes".as_bytes()));
    let recipients = vec![from, from, to, to];
    let expected_balances_before: Vec<U256> =
        [&amounts[..], &[U256::zero(), U256::zero()]].concat();
    let expected_balances_after: Vec<U256> = [&[U256::zero(), U256::zero()], &amounts[..]].concat();

    let mint_call = cep85_batch_mint(
        &mut builder,
        &cep85_token,
        &minting_account,
        &minting_recipient,
        ids.clone(),
        amounts.clone(),
        None,
    );

    mint_call.expect_success().commit();

    let actual_balances_before = cep85_check_balance_of_batch(
        &mut builder,
        &cep85_test_contract_package,
        recipients.clone(),
        vec![ids.clone(); 2_usize].into_iter().flatten().collect(),
    );

    assert_eq!(
        actual_balances_before,
        expected_balances_before
            .iter()
            .map(|&amount| Some(amount))
            .collect::<Vec<Option<U256>>>()
    );

    let transfer_call = cep85_batch_transfer_from(
        &mut builder,
        &cep85_token,
        &minting_account,
        TransferData {
            from: &from,
            to: &to,
            ids: ids.clone(),
            amounts: amounts.clone(),
            data: data.clone(),
        },
        None,
    );
    transfer_call.expect_success().commit();

    // Expect Transfer event
    let expected_event = TransferBatch::new(from, from, to, ids.clone(), amounts, data);
    let event_index = 1; // (Mint + transfer)
    let actual_event: TransferBatch = get_event(&builder, &cep85_token.into(), event_index);
    assert_eq!(
        actual_event, expected_event,
        "Expected TransferBatch event."
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
fn should_not_transfer_more_than_owned_balance() {
    let (
        mut builder,
        TestContext {
            cep85_token,
            cep85_test_contract_package,
            ref test_accounts,
            ..
        },
    ) = setup();

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let minting_recipient = Key::from(minting_account);
    let mint_amount = U256::one();
    let id = U256::one();

    let mint_call = cep85_mint(
        &mut builder,
        &cep85_token,
        &minting_account,
        &minting_recipient,
        &id,
        &mint_amount,
        None,
    );

    mint_call.expect_success().commit();

    let actual_balance_before = cep85_check_balance_of(
        &mut builder,
        &cep85_test_contract_package,
        &minting_recipient,
        &id,
    )
    .unwrap();
    let expected_balance_before = U256::one();

    assert_eq!(actual_balance_before, expected_balance_before);

    let from = Key::from(minting_account);
    let to = Key::from(*test_accounts.get(&ACCOUNT_USER_1).unwrap());
    let transfer_amount = U256::from(2);
    let data = Some(Bytes::from("Casper Labs free bytes".as_bytes()));

    let failing_transfer_call = cep85_transfer_from(
        &mut builder,
        &cep85_token,
        &minting_account,
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
        Cep85Error::InsufficientBalance as u16,
        "should not allow to transfer above balance",
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
fn should_not_batch_transfer_more_than_owned_balance() {
    let (
        mut builder,
        TestContext {
            cep85_token,
            cep85_test_contract_package,
            ref test_accounts,
            ..
        },
    ) = setup();

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let minting_recipient = Key::from(minting_account);
    let ids: Vec<U256> = vec![U256::one(), U256::from(2)];
    let mint_amounts: Vec<U256> = vec![U256::one(), U256::from(2)];
    let transfer_amounts: Vec<U256> = vec![U256::from(2), U256::from(3)];
    let from = minting_recipient;
    let to = Key::from(*test_accounts.get(&ACCOUNT_USER_1).unwrap());
    let data = Some(Bytes::from("Casper Labs free bytes".as_bytes()));
    let recipients = vec![from, from, to, to];
    let expected_balances: Vec<U256> = [&mint_amounts[..], &[U256::zero(), U256::zero()]].concat();

    let total_supplies = mint_amounts.clone();
    let set_total_supply_of_batch_call = cep85_set_total_supply_of_batch(
        &mut builder,
        &cep85_token,
        &minting_account,
        ids.clone(),
        total_supplies,
    );

    set_total_supply_of_batch_call.expect_success().commit();

    let mint_call = cep85_batch_mint(
        &mut builder,
        &cep85_token,
        &minting_account,
        &minting_recipient,
        ids.clone(),
        mint_amounts,
        None,
    );

    mint_call.expect_success().commit();

    let actual_balances_before = cep85_check_balance_of_batch(
        &mut builder,
        &cep85_test_contract_package,
        recipients.clone(),
        vec![ids.clone(); 2_usize].into_iter().flatten().collect(),
    );

    assert_eq!(
        actual_balances_before,
        expected_balances
            .iter()
            .map(|&amount| Some(amount))
            .collect::<Vec<Option<U256>>>()
    );

    let failing_transfer_call = cep85_batch_transfer_from(
        &mut builder,
        &cep85_token,
        &minting_account,
        TransferData {
            from: &from,
            to: &to,
            ids: ids.clone(),
            amounts: transfer_amounts,
            data,
        },
        None,
    );

    failing_transfer_call.expect_failure();

    let error = builder.get_error().expect("must have error");

    assert_expected_error(
        error,
        Cep85Error::InsufficientBalance as u16,
        "should not allow to transfer above balance",
    );

    let actual_balances_after = cep85_check_balance_of_batch(
        &mut builder,
        &cep85_test_contract_package,
        recipients,
        vec![ids; 2_usize].into_iter().flatten().collect(),
    );

    assert_eq!(
        actual_balances_after,
        expected_balances
            .iter()
            .map(|&amount| Some(amount))
            .collect::<Vec<Option<U256>>>()
    );
}

#[test]
fn should_not_be_able_to_own_transfer() {
    let (
        mut builder,
        TestContext {
            cep85_token,
            cep85_test_contract_package,
            ..
        },
    ) = setup();

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let minting_recipient = Key::from(minting_account);
    let mint_amount = U256::one();
    let id = U256::one();

    let mint_call = cep85_mint(
        &mut builder,
        &cep85_token,
        &minting_account,
        &minting_recipient,
        &id,
        &mint_amount,
        None,
    );

    mint_call.expect_success().commit();

    let actual_balance_before = cep85_check_balance_of(
        &mut builder,
        &cep85_test_contract_package,
        &minting_recipient,
        &id,
    )
    .unwrap();
    let expected_balance_before = U256::one();

    assert_eq!(actual_balance_before, expected_balance_before);

    let from = Key::from(minting_account);
    // let's try to self transfer
    let to = from;

    let transfer_amount = U256::one();
    let data = Some(Bytes::from("Casper Labs free bytes".as_bytes()));

    let failing_transfer_call = cep85_transfer_from(
        &mut builder,
        &cep85_token,
        &minting_account,
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
        Cep85Error::SelfTransfer as u16,
        "should not allow to self transfer",
    );

    let actual_balance_from =
        cep85_check_balance_of(&mut builder, &cep85_test_contract_package, &from, &id).unwrap();
    let expected_balance_from = U256::one();

    assert_eq!(actual_balance_from, expected_balance_from);

    let actual_balance_to =
        cep85_check_balance_of(&mut builder, &cep85_test_contract_package, &to, &id).unwrap();
    let expected_balance_to = U256::one();

    assert_eq!(actual_balance_to, expected_balance_to);
}

#[test]
fn should_not_be_able_to_own_batch_transfer() {
    let (
        mut builder,
        TestContext {
            cep85_token,
            cep85_test_contract_package,
            ..
        },
    ) = setup();

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let minting_recipient = Key::from(minting_account);
    let ids: Vec<U256> = vec![U256::one(), U256::from(2)];
    let mint_amounts: Vec<U256> = vec![U256::one(), U256::from(2)];
    let transfer_amounts: Vec<U256> = vec![U256::from(2), U256::from(3)];
    let from = minting_recipient;
    // let's try to self transfer
    let to = from;
    let data = Some(Bytes::from("Casper Labs free bytes".as_bytes()));
    let recipients = vec![from, from, to, to];
    let expected_balances: Vec<U256> = [&mint_amounts[..], &mint_amounts[..]].concat();

    let total_supplies = mint_amounts.clone();
    let set_total_supply_of_batch_call = cep85_set_total_supply_of_batch(
        &mut builder,
        &cep85_token,
        &minting_account,
        ids.clone(),
        total_supplies,
    );

    set_total_supply_of_batch_call.expect_success().commit();

    let mint_call = cep85_batch_mint(
        &mut builder,
        &cep85_token,
        &minting_account,
        &minting_recipient,
        ids.clone(),
        mint_amounts,
        None,
    );

    mint_call.expect_success().commit();

    let actual_balances_before = cep85_check_balance_of_batch(
        &mut builder,
        &cep85_test_contract_package,
        recipients.clone(),
        vec![ids.clone(); 2_usize].into_iter().flatten().collect(),
    );

    assert_eq!(
        actual_balances_before,
        expected_balances
            .iter()
            .map(|&amount| Some(amount))
            .collect::<Vec<Option<U256>>>()
    );

    let failing_transfer_call = cep85_batch_transfer_from(
        &mut builder,
        &cep85_token,
        &minting_account,
        TransferData {
            from: &from,
            to: &to,
            ids: ids.clone(),
            amounts: transfer_amounts,
            data,
        },
        None,
    );

    failing_transfer_call.expect_failure();

    let error = builder.get_error().expect("must have error");

    assert_expected_error(
        error,
        Cep85Error::SelfTransfer as u16,
        "should not allow to self transfer",
    );

    let actual_balances_after = cep85_check_balance_of_batch(
        &mut builder,
        &cep85_test_contract_package,
        recipients,
        vec![ids; 2_usize].into_iter().flatten().collect(),
    );

    assert_eq!(
        actual_balances_after,
        expected_balances
            .iter()
            .map(|&amount| Some(amount))
            .collect::<Vec<Option<U256>>>()
    );
}

#[test]
fn should_verify_zero_amount_transfer_is_noop() {
    let (
        mut builder,
        TestContext {
            cep85_token,
            cep85_test_contract_package,
            ref test_accounts,
            ..
        },
    ) = setup();

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let minting_recipient = Key::from(minting_account);
    let mint_amount = U256::one();
    let id = U256::one();

    let mint_call = cep85_mint(
        &mut builder,
        &cep85_token,
        &minting_account,
        &minting_recipient,
        &id,
        &mint_amount,
        None,
    );

    mint_call.expect_success().commit();

    let actual_balance_before = cep85_check_balance_of(
        &mut builder,
        &cep85_test_contract_package,
        &minting_recipient,
        &id,
    )
    .unwrap();
    let expected_balance_before = U256::one();

    assert_eq!(actual_balance_before, expected_balance_before);

    let from = Key::from(minting_account);
    let to = Key::from(*test_accounts.get(&ACCOUNT_USER_1).unwrap());
    let transfer_amount = U256::zero();
    let data = Some(Bytes::from("Casper Labs free bytes".as_bytes()));

    let failing_transfer_call = cep85_transfer_from(
        &mut builder,
        &cep85_token,
        &minting_account,
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
        Cep85Error::InvalidAmount as u16,
        "should not allow to transfer 0 amount",
    );

    let actual_balance_from =
        cep85_check_balance_of(&mut builder, &cep85_test_contract_package, &from, &id).unwrap();
    let expected_balance_from = U256::one();

    assert_eq!(actual_balance_from, expected_balance_from);

    let actual_balance =
        cep85_check_balance_of(&mut builder, &cep85_test_contract_package, &to, &id).unwrap();
    let expected_balance = U256::zero();

    assert_eq!(actual_balance, expected_balance);
}

#[test]
fn should_verify_zero_amount_batch_transfer_is_noop() {
    let (
        mut builder,
        TestContext {
            cep85_token,
            cep85_test_contract_package,
            ref test_accounts,
            ..
        },
    ) = setup();

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let minting_recipient = Key::from(minting_account);
    let ids: Vec<U256> = vec![U256::one(), U256::from(2)];
    let mint_amounts: Vec<U256> = vec![U256::one(), U256::from(2)];
    let transfer_amounts: Vec<U256> = vec![U256::from(2), U256::zero()];
    let from = minting_recipient;
    let to = Key::from(*test_accounts.get(&ACCOUNT_USER_1).unwrap());
    let data = Some(Bytes::from("Casper Labs free bytes".as_bytes()));
    let recipients = vec![from, from, to, to];
    let expected_balances: Vec<U256> = [&mint_amounts[..], &[U256::zero(), U256::zero()]].concat();

    let total_supplies = mint_amounts.clone();
    let set_total_supply_of_batch_call = cep85_set_total_supply_of_batch(
        &mut builder,
        &cep85_token,
        &minting_account,
        ids.clone(),
        total_supplies,
    );

    set_total_supply_of_batch_call.expect_success().commit();

    let mint_call = cep85_batch_mint(
        &mut builder,
        &cep85_token,
        &minting_account,
        &minting_recipient,
        ids.clone(),
        mint_amounts,
        None,
    );

    mint_call.expect_success().commit();

    let actual_balances_before = cep85_check_balance_of_batch(
        &mut builder,
        &cep85_test_contract_package,
        recipients.clone(),
        vec![ids.clone(); 2_usize].into_iter().flatten().collect(),
    );

    assert_eq!(
        actual_balances_before,
        expected_balances
            .iter()
            .map(|&amount| Some(amount))
            .collect::<Vec<Option<U256>>>()
    );

    let failing_transfer_call = cep85_batch_transfer_from(
        &mut builder,
        &cep85_token,
        &minting_account,
        TransferData {
            from: &from,
            to: &to,
            ids: ids.clone(),
            amounts: transfer_amounts,
            data,
        },
        None,
    );

    failing_transfer_call.expect_failure();

    let error = builder.get_error().expect("must have error");

    assert_expected_error(
        error,
        Cep85Error::InvalidAmount as u16,
        "should not allow to transfer 0 amount",
    );

    let actual_balances_after = cep85_check_balance_of_batch(
        &mut builder,
        &cep85_test_contract_package,
        recipients,
        vec![ids; 2_usize].into_iter().flatten().collect(),
    );

    assert_eq!(
        actual_balances_after,
        expected_balances
            .iter()
            .map(|&amount| Some(amount))
            .collect::<Vec<Option<U256>>>()
    );
}

#[test]
fn should_transfer_account_to_account() {
    let (
        mut builder,
        TestContext {
            cep85_token,
            cep85_test_contract_package,
            ref test_accounts,
            ..
        },
    ) = setup();

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let account_user_1 = *test_accounts.get(&ACCOUNT_USER_1).unwrap();
    let from = Key::from(account_user_1);
    let minting_recipient = from;
    let to = Key::from(*test_accounts.get(&ACCOUNT_USER_2).unwrap());
    let mint_amount = U256::one();
    let id = U256::one();
    let transfer_amount = U256::one();
    let data = Some(Bytes::from("Casper Labs free bytes".as_bytes()));

    let mint_call = cep85_mint(
        &mut builder,
        &cep85_token,
        &minting_account,
        &minting_recipient,
        &id,
        &mint_amount,
        None,
    );

    mint_call.expect_success().commit();

    let actual_balance_before = cep85_check_balance_of(
        &mut builder,
        &cep85_test_contract_package,
        &minting_recipient,
        &id,
    )
    .unwrap();
    let expected_balance_before = U256::one();

    assert_eq!(actual_balance_before, expected_balance_before);

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
fn should_batch_transfer_account_to_account() {
    let (
        mut builder,
        TestContext {
            cep85_token,
            cep85_test_contract_package,
            ref test_accounts,
            ..
        },
    ) = setup();

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let ids: Vec<U256> = vec![U256::one(), U256::from(2)];
    let amounts: Vec<U256> = vec![U256::one(), U256::from(2)];
    let account_user_1 = *test_accounts.get(&ACCOUNT_USER_1).unwrap();
    let from = Key::from(account_user_1);
    let minting_recipient = from;
    let to = Key::from(*test_accounts.get(&ACCOUNT_USER_2).unwrap());
    let data = Some(Bytes::from("Casper Labs free bytes".as_bytes()));
    let recipients = vec![from, from, to, to];
    let expected_balances_before: Vec<U256> =
        [&amounts[..], &[U256::zero(), U256::zero()]].concat();
    let expected_balances_after: Vec<U256> = [&[U256::zero(), U256::zero()], &amounts[..]].concat();

    let total_supplies = amounts.clone();
    let set_total_supply_of_batch_call = cep85_set_total_supply_of_batch(
        &mut builder,
        &cep85_token,
        &minting_account,
        ids.clone(),
        total_supplies,
    );

    set_total_supply_of_batch_call.expect_success().commit();

    let mint_call = cep85_batch_mint(
        &mut builder,
        &cep85_token,
        &minting_account,
        &minting_recipient,
        ids.clone(),
        amounts.clone(),
        None,
    );

    mint_call.expect_success().commit();

    let actual_balances_before = cep85_check_balance_of_batch(
        &mut builder,
        &cep85_test_contract_package,
        recipients.clone(),
        vec![ids.clone(); 2_usize].into_iter().flatten().collect(),
    );

    assert_eq!(
        actual_balances_before,
        expected_balances_before
            .iter()
            .map(|&amount| Some(amount))
            .collect::<Vec<Option<U256>>>()
    );

    let transfer_call = cep85_batch_transfer_from(
        &mut builder,
        &cep85_token,
        &account_user_1,
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
fn should_transfer_account_to_contract_package() {
    let (
        mut builder,
        TestContext {
            cep85_token,
            cep85_test_contract_package,
            ref test_accounts,
            ..
        },
    ) = setup();

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let account_user_1 = *test_accounts.get(&ACCOUNT_USER_1).unwrap();
    let from = Key::from(account_user_1);
    let minting_recipient = from;
    let to = Key::Hash(cep85_test_contract_package.value());
    let mint_amount = U256::one();
    let id = U256::one();
    let transfer_amount = U256::one();
    let data = Some(Bytes::from("Casper Labs free bytes".as_bytes()));

    let mint_call = cep85_mint(
        &mut builder,
        &cep85_token,
        &minting_account,
        &minting_recipient,
        &id,
        &mint_amount,
        None,
    );

    mint_call.expect_success().commit();

    let actual_balance_before = cep85_check_balance_of(
        &mut builder,
        &cep85_test_contract_package,
        &minting_recipient,
        &id,
    )
    .unwrap();
    let expected_balance_before = U256::one();

    assert_eq!(actual_balance_before, expected_balance_before);

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
fn should_batch_transfer_account_to_contract_package() {
    let (
        mut builder,
        TestContext {
            cep85_token,
            cep85_test_contract_package,
            ref test_accounts,
            ..
        },
    ) = setup();

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let ids: Vec<U256> = vec![U256::one(), U256::from(2)];
    let amounts: Vec<U256> = vec![U256::one(), U256::from(2)];
    let account_user_1 = *test_accounts.get(&ACCOUNT_USER_1).unwrap();
    let from = Key::from(account_user_1);
    let minting_recipient = from;
    let to = Key::Hash(cep85_test_contract_package.value());
    let data = Some(Bytes::from("Casper Labs free bytes".as_bytes()));
    let recipients = vec![from, from, to, to];
    let expected_balances_before: Vec<U256> =
        [&amounts[..], &[U256::zero(), U256::zero()]].concat();
    let expected_balances_after: Vec<U256> = [&[U256::zero(), U256::zero()], &amounts[..]].concat();

    let total_supplies = amounts.clone();
    let set_total_supply_of_batch_call = cep85_set_total_supply_of_batch(
        &mut builder,
        &cep85_token,
        &minting_account,
        ids.clone(),
        total_supplies,
    );

    set_total_supply_of_batch_call.expect_success().commit();

    let mint_call = cep85_batch_mint(
        &mut builder,
        &cep85_token,
        &minting_account,
        &minting_recipient,
        ids.clone(),
        amounts.clone(),
        None,
    );

    mint_call.expect_success().commit();

    let actual_balances_before = cep85_check_balance_of_batch(
        &mut builder,
        &cep85_test_contract_package,
        recipients.clone(),
        vec![ids.clone(); 2_usize].into_iter().flatten().collect(),
    );

    assert_eq!(
        actual_balances_before,
        expected_balances_before
            .iter()
            .map(|&amount| Some(amount))
            .collect::<Vec<Option<U256>>>()
    );

    let transfer_call = cep85_batch_transfer_from(
        &mut builder,
        &cep85_token,
        &account_user_1,
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
fn should_transfer_contract_package_to_contract() {
    let (
        mut builder,
        TestContext {
            cep85_token,
            cep85_test_contract_package,
            ..
        },
    ) = setup();

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let from = Key::Hash(cep85_test_contract_package.value());
    let minting_recipient = from;
    let to = Key::Hash([42; 32]);
    let mint_amount = U256::one();
    let id = U256::one();
    let transfer_amount = U256::one();
    let data = Some(Bytes::from("Casper Labs free bytes".as_bytes()));

    let mint_call = cep85_mint(
        &mut builder,
        &cep85_token,
        &minting_account,
        &minting_recipient,
        &id,
        &mint_amount,
        None,
    );

    mint_call.expect_success().commit();

    let actual_balance_before = cep85_check_balance_of(
        &mut builder,
        &cep85_test_contract_package,
        &minting_recipient,
        &id,
    )
    .unwrap();
    let expected_balance_before = U256::one();

    assert_eq!(actual_balance_before, expected_balance_before);

    let transfer_call = cep85_transfer_from(
        &mut builder,
        &cep85_token,
        &minting_account,
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
fn should_batch_transfer_contract_package_to_contract() {
    let (
        mut builder,
        TestContext {
            cep85_token,
            cep85_test_contract_package,
            ..
        },
    ) = setup();

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let ids: Vec<U256> = vec![U256::one(), U256::from(2)];
    let amounts: Vec<U256> = vec![U256::one(), U256::from(2)];
    let from = Key::Hash(cep85_test_contract_package.value());
    let minting_recipient = from;
    let to = Key::Hash([42; 32]);
    let data = Some(Bytes::from("Casper Labs free bytes".as_bytes()));
    let recipients = vec![from, from, to, to];
    let expected_balances_before: Vec<U256> =
        [&amounts[..], &[U256::zero(), U256::zero()]].concat();
    let expected_balances_after: Vec<U256> = [&[U256::zero(), U256::zero()], &amounts[..]].concat();

    let total_supplies = amounts.clone();
    let set_total_supply_of_batch_call = cep85_set_total_supply_of_batch(
        &mut builder,
        &cep85_token,
        &minting_account,
        ids.clone(),
        total_supplies,
    );

    set_total_supply_of_batch_call.expect_success().commit();

    let mint_call = cep85_batch_mint(
        &mut builder,
        &cep85_token,
        &minting_account,
        &minting_recipient,
        ids.clone(),
        amounts.clone(),
        None,
    );

    mint_call.expect_success().commit();

    let actual_balances_before = cep85_check_balance_of_batch(
        &mut builder,
        &cep85_test_contract_package,
        recipients.clone(),
        vec![ids.clone(); 2_usize].into_iter().flatten().collect(),
    );

    assert_eq!(
        actual_balances_before,
        expected_balances_before
            .iter()
            .map(|&amount| Some(amount))
            .collect::<Vec<Option<U256>>>()
    );

    let transfer_call = cep85_batch_transfer_from(
        &mut builder,
        &cep85_token,
        &minting_account,
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
fn should_transfer_account_to_contract() {
    let (
        mut builder,
        TestContext {
            cep85_token,
            cep85_test_contract,
            cep85_test_contract_package,
            ref test_accounts,
            ..
        },
    ) = setup();

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let account_user_1 = *test_accounts.get(&ACCOUNT_USER_1).unwrap();
    let from = Key::from(account_user_1);
    let minting_recipient = from;
    let to = Key::Hash(cep85_test_contract.value());
    let mint_amount = U256::one();
    let id = U256::one();
    let transfer_amount = U256::one();
    let data = Some(Bytes::from("Casper Labs free bytes".as_bytes()));

    let mint_call = cep85_mint(
        &mut builder,
        &cep85_token,
        &minting_account,
        &minting_recipient,
        &id,
        &mint_amount,
        None,
    );

    mint_call.expect_success().commit();

    let actual_balance_before = cep85_check_balance_of(
        &mut builder,
        &cep85_test_contract_package,
        &minting_recipient,
        &id,
    )
    .unwrap();
    let expected_balance_before = U256::one();

    assert_eq!(actual_balance_before, expected_balance_before);

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
fn should_batch_transfer_account_to_contract_() {
    let (
        mut builder,
        TestContext {
            cep85_token,
            cep85_test_contract,
            cep85_test_contract_package,
            ref test_accounts,
            ..
        },
    ) = setup();

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let ids: Vec<U256> = vec![U256::one(), U256::from(2)];
    let amounts: Vec<U256> = vec![U256::one(), U256::from(2)];
    let account_user_1 = *test_accounts.get(&ACCOUNT_USER_1).unwrap();
    let from = Key::from(account_user_1);
    let minting_recipient = from;
    let to = Key::Hash(cep85_test_contract.value());
    let data = Some(Bytes::from("Casper Labs free bytes".as_bytes()));
    let recipients = vec![from, from, to, to];
    let expected_balances_before: Vec<U256> =
        [&amounts[..], &[U256::zero(), U256::zero()]].concat();
    let expected_balances_after: Vec<U256> = [&[U256::zero(), U256::zero()], &amounts[..]].concat();

    let total_supplies = amounts.clone();
    let set_total_supply_of_batch_call = cep85_set_total_supply_of_batch(
        &mut builder,
        &cep85_token,
        &minting_account,
        ids.clone(),
        total_supplies,
    );

    set_total_supply_of_batch_call.expect_success().commit();

    let mint_call = cep85_batch_mint(
        &mut builder,
        &cep85_token,
        &minting_account,
        &minting_recipient,
        ids.clone(),
        amounts.clone(),
        None,
    );

    mint_call.expect_success().commit();

    let actual_balances_before = cep85_check_balance_of_batch(
        &mut builder,
        &cep85_test_contract_package,
        recipients.clone(),
        vec![ids.clone(); 2_usize].into_iter().flatten().collect(),
    );

    assert_eq!(
        actual_balances_before,
        expected_balances_before
            .iter()
            .map(|&amount| Some(amount))
            .collect::<Vec<Option<U256>>>()
    );

    let transfer_call = cep85_batch_transfer_from(
        &mut builder,
        &cep85_token,
        &account_user_1,
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
fn should_transfer_contract_to_contract() {
    let (
        mut builder,
        TestContext {
            cep85_token,
            cep85_test_contract,
            cep85_test_contract_package,
            ..
        },
    ) = setup();

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let from = Key::Hash(cep85_test_contract.value());
    let minting_recipient = from;
    let to = Key::Hash([42; 32]);
    let mint_amount = U256::one();
    let id = U256::one();
    let transfer_amount = U256::one();
    let data = Some(Bytes::from("Casper Labs free bytes".as_bytes()));

    let mint_call = cep85_mint(
        &mut builder,
        &cep85_token,
        &minting_account,
        &minting_recipient,
        &id,
        &mint_amount,
        None,
    );

    mint_call.expect_success().commit();

    let actual_balance_before = cep85_check_balance_of(
        &mut builder,
        &cep85_test_contract_package,
        &minting_recipient,
        &id,
    )
    .unwrap();
    let expected_balance_before = U256::one();

    assert_eq!(actual_balance_before, expected_balance_before);

    let transfer_call = cep85_transfer_from(
        &mut builder,
        &cep85_token,
        &minting_account,
        TransferData {
            from: &from,
            to: &to,
            ids: vec![id],
            amounts: vec![transfer_amount],
            data,
        },
        Some(true),
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
fn should_batch_transfer_contract_to_contract() {
    let (
        mut builder,
        TestContext {
            cep85_token,
            cep85_test_contract,
            cep85_test_contract_package,
            ..
        },
    ) = setup();

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let ids: Vec<U256> = vec![U256::one(), U256::from(2)];
    let amounts: Vec<U256> = vec![U256::one(), U256::from(2)];
    let from = Key::Hash(cep85_test_contract.value());
    let minting_recipient = from;
    let to = Key::Hash([42; 32]);
    let data = Some(Bytes::from("Casper Labs free bytes".as_bytes()));
    let recipients = vec![from, from, to, to];
    let expected_balances_before: Vec<U256> =
        [&amounts[..], &[U256::zero(), U256::zero()]].concat();
    let expected_balances_after: Vec<U256> = [&[U256::zero(), U256::zero()], &amounts[..]].concat();

    let total_supplies = amounts.clone();
    let set_total_supply_of_batch_call = cep85_set_total_supply_of_batch(
        &mut builder,
        &cep85_token,
        &minting_account,
        ids.clone(),
        total_supplies,
    );

    set_total_supply_of_batch_call.expect_success().commit();

    let mint_call = cep85_batch_mint(
        &mut builder,
        &cep85_token,
        &minting_account,
        &minting_recipient,
        ids.clone(),
        amounts.clone(),
        None,
    );

    mint_call.expect_success().commit();

    let actual_balances_before = cep85_check_balance_of_batch(
        &mut builder,
        &cep85_test_contract_package,
        recipients.clone(),
        vec![ids.clone(); 2_usize].into_iter().flatten().collect(),
    );

    assert_eq!(
        actual_balances_before,
        expected_balances_before
            .iter()
            .map(|&amount| Some(amount))
            .collect::<Vec<Option<U256>>>()
    );

    let transfer_call = cep85_batch_transfer_from(
        &mut builder,
        &cep85_token,
        &minting_account,
        TransferData {
            from: &from,
            to: &to,
            ids: ids.clone(),
            amounts,
            data,
        },
        Some(true),
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
fn should_transfer_account_to_contract_package_to_account() {
    let (
        mut builder,
        TestContext {
            cep85_token,
            cep85_test_contract_package,
            ref test_accounts,
            ..
        },
    ) = setup();

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let account_user_1 = *test_accounts.get(&ACCOUNT_USER_1).unwrap();
    let from = Key::from(account_user_1);
    let minting_recipient = from;
    let to = Key::Hash(cep85_test_contract_package.value());
    let mint_amount = U256::one();
    let id = U256::one();
    let transfer_amount = U256::one();
    let data = Some(Bytes::from("Casper Labs free bytes".as_bytes()));

    let mint_call = cep85_mint(
        &mut builder,
        &cep85_token,
        &minting_account,
        &minting_recipient,
        &id,
        &mint_amount,
        None,
    );

    mint_call.expect_success().commit();

    let actual_balance_before = cep85_check_balance_of(
        &mut builder,
        &cep85_test_contract_package,
        &minting_recipient,
        &id,
    )
    .unwrap();
    let expected_balance_before = U256::one();

    assert_eq!(actual_balance_before, expected_balance_before);

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
        cep85_check_balance_of(&mut builder, &cep85_test_contract_package, &from, &id).unwrap();
    let expected_balance_from = U256::zero();

    assert_eq!(actual_balance_from, expected_balance_from);

    let actual_balance_to =
        cep85_check_balance_of(&mut builder, &cep85_test_contract_package, &to, &id).unwrap();
    let expected_balance_to = U256::one();

    assert_eq!(actual_balance_to, expected_balance_to);

    // Let's check package can transfer
    let account_user_2 = *test_accounts.get(&ACCOUNT_USER_2).unwrap();
    let from = Key::Hash(cep85_test_contract_package.value());
    let to = Key::from(account_user_2);

    let transfer_call = cep85_transfer_from(
        &mut builder,
        &cep85_token,
        &DEFAULT_ACCOUNT_ADDR,
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
fn should_revert_on_transfer_of_non_existent_token() {
    let (
        mut builder,
        TestContext {
            cep85_token,
            cep85_test_contract_package,
            ref test_accounts,
            ..
        },
    ) = setup();

    let id = U256::one();

    let actual_balance_before = cep85_check_balance_of(
        &mut builder,
        &cep85_test_contract_package,
        &Key::from(*DEFAULT_ACCOUNT_ADDR),
        &id,
    );
    let expected_balance_before = None;

    assert_eq!(actual_balance_before, expected_balance_before);

    let from = Key::from(*DEFAULT_ACCOUNT_ADDR);
    let to = Key::from(*test_accounts.get(&ACCOUNT_USER_1).unwrap());
    let transfer_amount = U256::one();
    let data = Some(Bytes::from("Casper Labs free bytes".as_bytes()));

    let transfer_call = cep85_transfer_from(
        &mut builder,
        &cep85_token,
        &DEFAULT_ACCOUNT_ADDR,
        TransferData {
            from: &from,
            to: &to,
            ids: vec![id],
            amounts: vec![transfer_amount],
            data,
        },
        None,
    );
    transfer_call.expect_failure();

    let error = builder.get_error().expect("must have error");

    assert_expected_error(
        error,
        Cep85Error::NonSuppliedTokenId as u16,
        "should return error when trying to burn a non_existing token",
    );
}

#[test]
fn should_revert_on_transfer_batch_of_non_existent_token() {
    let (
        mut builder,
        TestContext {
            cep85_token,
            ref test_accounts,
            ..
        },
    ) = setup();

    let ids: Vec<U256> = vec![U256::one(), U256::from(2)];
    let amounts: Vec<U256> = vec![U256::one(), U256::from(2)];
    let from = Key::from(*DEFAULT_ACCOUNT_ADDR);
    let to = Key::from(*test_accounts.get(&ACCOUNT_USER_1).unwrap());
    let data = Some(Bytes::from("Casper Labs free bytes".as_bytes()));

    let transfer_call = cep85_batch_transfer_from(
        &mut builder,
        &cep85_token,
        &DEFAULT_ACCOUNT_ADDR,
        TransferData {
            from: &from,
            to: &to,
            ids,
            amounts,
            data,
        },
        None,
    );

    transfer_call.expect_failure();

    let error = builder.get_error().expect("must have error");

    assert_expected_error(
        error,
        Cep85Error::NonSuppliedTokenId as u16,
        "should return error when trying to burn a non_existing token",
    );
}
