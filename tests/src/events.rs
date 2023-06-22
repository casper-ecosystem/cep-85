use crate::utility::{
    constants::{TOKEN_NAME, TOKEN_URI},
    installer_request_builders::{setup_with_args, TestContext},
};
use casper_event_standard::{Schemas, EVENTS_SCHEMA};
use casper_types::{runtime_args, RuntimeArgs};
use cep1155::{
    constants::{EVENTS_MODE, NAME, URI},
    events::{ApprovalForAll, Burn, Mint, TransferBatch, TransferSingle},
    modalities::EventsMode,
};

#[test]
fn should_have_events_schema_in_events_mode() {
    let (mut builder, TestContext { cep1155_token, .. }) = setup_with_args(runtime_args! {
        NAME => TOKEN_NAME,
        URI => TOKEN_URI,
        EVENTS_MODE => EventsMode::CES as u8
    });
    let expected_schemas = Schemas::new()
        .with::<Mint>()
        .with::<Burn>()
        .with::<ApprovalForAll>()
        .with::<TransferSingle>()
        .with::<TransferBatch>();
    let actual_schemas: Schemas = builder.get_value(cep1155_token, EVENTS_SCHEMA);
    assert_eq!(actual_schemas, expected_schemas, "Schemas mismatch.");
}

// #[test]
fn _should_record_events_in_events_mode() {}

// #[test]
#[should_panic]
fn _should_not_record_events_in_no_events_mode() {}
