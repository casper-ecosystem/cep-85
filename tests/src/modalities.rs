use crate::utility::{
    installer_request_builders::{cep85_set_modalities, setup, setup_with_args, TestContext},
    support::get_event,
};
use casper_engine_test_support::DEFAULT_ACCOUNT_ADDR;
use casper_types::{runtime_args, RuntimeArgs};
use cep85::{
    constants::{ARG_ENABLE_BURN, ARG_EVENTS_MODE},
    events::SetModalities,
    modalities::EventsMode,
};

#[test]
fn should_toggle_enable_burn() {
    let (mut builder, TestContext { cep85_token, .. }) = setup();

    let contract = builder
        .get_contract(cep85_token)
        .expect("should have contract");
    let named_keys = contract.named_keys();
    assert!(named_keys.contains_key(ARG_ENABLE_BURN), "{:?}", named_keys);

    let enable_burn: bool = builder
        .query(None, cep85_token.into(), &[ARG_ENABLE_BURN.to_string()])
        .unwrap()
        .as_cl_value()
        .unwrap()
        .to_owned()
        .into_t::<bool>()
        .unwrap();
    assert!(!enable_burn);

    let owner = *DEFAULT_ACCOUNT_ADDR;
    let set_modalities_call = cep85_set_modalities(&mut builder, &cep85_token, &owner, None, None);
    set_modalities_call.expect_success().commit();

    let enable_burn: bool = builder
        .query(None, cep85_token.into(), &[ARG_ENABLE_BURN.to_string()])
        .unwrap()
        .as_cl_value()
        .unwrap()
        .to_owned()
        .into_t::<bool>()
        .unwrap();
    assert!(!enable_burn);

    let set_modalities_call =
        cep85_set_modalities(&mut builder, &cep85_token, &owner, Some(true), None);
    set_modalities_call.expect_success().commit();

    let enable_burn: bool = builder
        .query(None, cep85_token.into(), &[ARG_ENABLE_BURN.to_string()])
        .unwrap()
        .as_cl_value()
        .unwrap()
        .to_owned()
        .into_t::<bool>()
        .unwrap();
    assert!(enable_burn);

    let set_modalities_call =
        cep85_set_modalities(&mut builder, &cep85_token, &owner, Some(false), None);
    set_modalities_call.expect_success().commit();

    let enable_burn: bool = builder
        .query(None, cep85_token.into(), &[ARG_ENABLE_BURN.to_string()])
        .unwrap()
        .as_cl_value()
        .unwrap()
        .to_owned()
        .into_t::<bool>()
        .unwrap();
    assert!(!enable_burn);
}

#[test]
fn should_toggle_events_mode() {
    let (mut builder, TestContext { cep85_token, .. }) = setup();

    let contract = builder
        .get_contract(cep85_token)
        .expect("should have contract");
    let named_keys = contract.named_keys();
    assert!(named_keys.contains_key(ARG_EVENTS_MODE), "{:?}", named_keys);

    let events_mode = builder
        .query(None, cep85_token.into(), &[ARG_EVENTS_MODE.to_string()])
        .unwrap()
        .as_cl_value()
        .unwrap()
        .to_owned()
        .into_t::<u8>()
        .unwrap_or_default();

    assert_eq!(events_mode, EventsMode::NoEvents as u8);

    let owner = *DEFAULT_ACCOUNT_ADDR;
    let set_modalities_call = cep85_set_modalities(&mut builder, &cep85_token, &owner, None, None);
    set_modalities_call.expect_success().commit();

    let events_mode = builder
        .query(None, cep85_token.into(), &[ARG_EVENTS_MODE.to_string()])
        .unwrap()
        .as_cl_value()
        .unwrap()
        .to_owned()
        .into_t::<u8>()
        .unwrap_or_default();

    assert_eq!(events_mode, EventsMode::NoEvents as u8);

    let set_modalities_call = cep85_set_modalities(
        &mut builder,
        &cep85_token,
        &owner,
        None,
        Some(EventsMode::CES),
    );
    set_modalities_call.expect_success().commit();

    let events_mode = builder
        .query(None, cep85_token.into(), &[ARG_EVENTS_MODE.to_string()])
        .unwrap()
        .as_cl_value()
        .unwrap()
        .to_owned()
        .into_t::<u8>()
        .unwrap_or_default();

    assert_eq!(events_mode, EventsMode::CES as u8);

    // Expect SetModalities event
    let expected_event = SetModalities::new();
    let actual_event: SetModalities = get_event(&builder, &cep85_token.into(), 0);
    assert_eq!(
        actual_event, expected_event,
        "Expected SetModalities event."
    );

    let set_modalities_call = cep85_set_modalities(
        &mut builder,
        &cep85_token,
        &owner,
        None,
        Some(EventsMode::NoEvents),
    );
    set_modalities_call.expect_success().commit();

    let events_mode = builder
        .query(None, cep85_token.into(), &[ARG_EVENTS_MODE.to_string()])
        .unwrap()
        .as_cl_value()
        .unwrap()
        .to_owned()
        .into_t::<u8>()
        .unwrap_or_default();

    assert_eq!(events_mode, EventsMode::NoEvents as u8);

    // Expect No SetModalities event after one SetModalities event
    let dictionary_seed_uref = *builder
        .query(None, cep85_token.into(), &[])
        .expect("must have contract")
        .as_contract()
        .expect("must convert contract")
        .named_keys()
        .get(casper_event_standard::EVENTS_DICT)
        .expect("must have key")
        .as_uref()
        .expect("must convert to dictionary seed uref");

    builder
        .query_dictionary_item(None, dictionary_seed_uref, "1")
        .expect_err("should not have dictionary value for a second SetModalities event");
}

#[test]
fn should_emit_event_on_set_modalities_with_events_mode_ces() {
    let (mut builder, TestContext { cep85_token, .. }) = setup_with_args(
        runtime_args! {
            ARG_EVENTS_MODE => EventsMode::CES as u8,
        },
        None,
    );

    let contract = builder
        .get_contract(cep85_token)
        .expect("should have contract");
    let named_keys = contract.named_keys();
    assert!(named_keys.contains_key(ARG_ENABLE_BURN), "{:?}", named_keys);

    let enable_burn: bool = builder
        .query(None, cep85_token.into(), &[ARG_ENABLE_BURN.to_string()])
        .unwrap()
        .as_cl_value()
        .unwrap()
        .to_owned()
        .into_t::<bool>()
        .unwrap();
    assert!(!enable_burn);

    let owner = *DEFAULT_ACCOUNT_ADDR;
    let set_modalities_call = cep85_set_modalities(&mut builder, &cep85_token, &owner, None, None);
    set_modalities_call.expect_success().commit();

    let enable_burn: bool = builder
        .query(None, cep85_token.into(), &[ARG_ENABLE_BURN.to_string()])
        .unwrap()
        .as_cl_value()
        .unwrap()
        .to_owned()
        .into_t::<bool>()
        .unwrap();
    assert!(!enable_burn);

    let set_modalities_call =
        cep85_set_modalities(&mut builder, &cep85_token, &owner, Some(true), None);
    set_modalities_call.expect_success().commit();

    let enable_burn: bool = builder
        .query(None, cep85_token.into(), &[ARG_ENABLE_BURN.to_string()])
        .unwrap()
        .as_cl_value()
        .unwrap()
        .to_owned()
        .into_t::<bool>()
        .unwrap();
    assert!(enable_burn);

    // Expect SetModalities event
    let expected_event = SetModalities::new();
    let actual_event: SetModalities = get_event(&builder, &cep85_token.into(), 0);
    assert_eq!(
        actual_event, expected_event,
        "Expected SetModalities event."
    );
}
