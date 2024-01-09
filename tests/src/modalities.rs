use crate::utility::installer_request_builders::{cep85_set_modalities, setup, TestContext};
use casper_engine_test_support::DEFAULT_ACCOUNT_ADDR;
use cep85::{
    constants::{ARG_ENABLE_BURN, ARG_EVENTS_MODE},
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
}
