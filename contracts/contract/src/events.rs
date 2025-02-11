#[cfg(feature = "contract-support")]
mod contract_support {
    pub use crate::{
        constants::{ARG_EVENTS_MODE, EVENTS},
        error::Cep85Error,
        modalities::EventsMode,
        utils::get_stored_value,
    };
    pub use alloc::format;
    pub use casper_contract::{
        contract_api::runtime::{emit_message, get_key},
        unwrap_or_revert::UnwrapOrRevert,
    };
    pub use casper_event_standard::{emit, Schemas};
    pub use core::convert::TryFrom;
}
use crate::security::SecurityBadge;
use alloc::{collections::BTreeMap, string::String, vec::Vec};
use casper_event_standard::Event;
use casper_types::{bytesrepr::Bytes, Key, U256};
#[cfg(feature = "contract-support")]
use contract_support::*;

#[derive(Debug)]
pub enum Event {
    Mint(Mint),
    MintBatch(MintBatch),
    Burn(Burn),
    BurnBatch(BurnBatch),
    ApprovalForAll(ApprovalForAll),
    Transfer(Transfer),
    TransferBatch(TransferBatch),
    Uri(Uri),
    UriBatch(UriBatch),
    SetTotalSupply(SetTotalSupply),
    ChangeSecurity(ChangeSecurity),
    SetModalities(SetModalities),
    Upgrade(Upgrade),
    ChangeEnableBurnMode(ChangeEnableBurnMode),
    ChangeEventsMode(ChangeEventsMode),
}

#[cfg(feature = "contract-support")]
pub fn record_event_dictionary(event: Event) {
    let events_mode: EventsMode = EventsMode::try_from(get_stored_value::<u8>(ARG_EVENTS_MODE))
        .unwrap_or_revert_with(Cep85Error::InvalidEventsMode);

    match events_mode {
        EventsMode::NoEvents => {}
        EventsMode::CES => ces(event),
        EventsMode::Native => emit_message(EVENTS, &format!("{event:?}").into())
            .unwrap_or_revert_with(Cep85Error::InvalidEventsMode),
        EventsMode::NativeNCES => {
            emit_message(EVENTS, &format!("{event:?}").into())
                .unwrap_or_revert_with(Cep85Error::InvalidEventsMode);
            ces(event);
        }
    }
}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct Mint {
    pub id: U256,
    pub recipient: Key,
    pub amount: U256,
}

impl Mint {
    pub fn new(id: U256, recipient: Key, amount: U256) -> Self {
        Self {
            id,
            recipient,
            amount,
        }
    }
}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct MintBatch {
    pub ids: Vec<U256>,
    pub recipient: Key,
    pub amounts: Vec<U256>,
}

impl MintBatch {
    pub fn new(ids: Vec<U256>, recipient: Key, amounts: Vec<U256>) -> Self {
        Self {
            ids,
            recipient,
            amounts,
        }
    }
}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct Burn {
    pub id: U256,
    pub owner: Key,
    pub amount: U256,
}

impl Burn {
    pub fn new(owner: Key, id: U256, amount: U256) -> Self {
        Self { id, owner, amount }
    }
}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct BurnBatch {
    pub ids: Vec<U256>,
    pub owner: Key,
    pub amounts: Vec<U256>,
}

impl BurnBatch {
    pub fn new(ids: Vec<U256>, owner: Key, amounts: Vec<U256>) -> Self {
        Self {
            ids,
            owner,
            amounts,
        }
    }
}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct ApprovalForAll {
    pub owner: Key,
    pub operator: Key,
    pub approved: bool,
}

impl ApprovalForAll {
    pub fn new(owner: Key, operator: Key, approved: bool) -> Self {
        Self {
            owner,
            operator,
            approved,
        }
    }
}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct Transfer {
    pub operator: Key,
    pub from: Key,
    pub to: Key,
    pub id: U256,
    pub value: U256,
    pub data: Option<Bytes>,
}

impl Transfer {
    pub fn new(
        operator: Key,
        from: Key,
        to: Key,
        id: U256,
        value: U256,
        data: Option<Bytes>,
    ) -> Self {
        Self {
            operator,
            from,
            to,
            id,
            value,
            data,
        }
    }
}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct TransferBatch {
    pub operator: Key,
    pub from: Key,
    pub to: Key,
    pub ids: Vec<U256>,
    pub values: Vec<U256>,
    pub data: Option<Bytes>,
}

impl TransferBatch {
    pub fn new(
        operator: Key,
        from: Key,
        to: Key,
        ids: Vec<U256>,
        values: Vec<U256>,
        data: Option<Bytes>,
    ) -> Self {
        Self {
            operator,
            from,
            to,
            ids,
            values,
            data,
        }
    }
}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct Uri {
    pub value: String,
    pub id: Option<U256>,
}

impl Uri {
    pub fn new(value: String, id: Option<U256>) -> Self {
        Self { value, id }
    }
}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct UriBatch {
    pub value: String,
    pub ids: Vec<U256>,
}

impl UriBatch {
    pub fn new(value: String, ids: Vec<U256>) -> Self {
        Self { value, ids }
    }
}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct SetTotalSupply {
    pub id: U256,
    pub total_supply: U256,
}

impl SetTotalSupply {
    pub fn new(id: U256, total_supply: U256) -> Self {
        Self { id, total_supply }
    }
}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct ChangeSecurity {
    pub admin: Key,
    pub sec_change_map: BTreeMap<Key, SecurityBadge>,
}

impl ChangeSecurity {
    pub fn new(admin: Key, sec_change_map: BTreeMap<Key, SecurityBadge>) -> Self {
        Self {
            admin,
            sec_change_map,
        }
    }
}

#[derive(Event, Debug, PartialEq, Eq, Default)]
pub struct SetModalities {}

impl SetModalities {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Event, Debug, PartialEq, Eq, Default)]
pub struct Upgrade {}

impl Upgrade {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct ChangeEnableBurnMode {
    pub enable_burn: bool,
}

impl ChangeEnableBurnMode {
    pub fn new(enable_burn: bool) -> Self {
        Self { enable_burn }
    }
}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct ChangeEventsMode {
    pub events_mode: u8,
}

impl ChangeEventsMode {
    pub fn new(events_mode: u8) -> Self {
        Self { events_mode }
    }
}

#[cfg(feature = "contract-support")]
fn ces(event: Event) {
    match event {
        Event::Mint(ev) => emit(ev),
        Event::MintBatch(ev) => emit(ev),
        Event::Burn(ev) => emit(ev),
        Event::BurnBatch(ev) => emit(ev),
        Event::ApprovalForAll(ev) => emit(ev),
        Event::Transfer(ev) => emit(ev),
        Event::TransferBatch(ev) => emit(ev),
        Event::Uri(ev) => emit(ev),
        Event::UriBatch(ev) => emit(ev),
        Event::SetTotalSupply(ev) => emit(ev),
        Event::ChangeSecurity(ev) => emit(ev),
        Event::SetModalities(ev) => emit(ev),
        Event::Upgrade(ev) => emit(ev),
        Event::ChangeEnableBurnMode(ev) => emit(ev),
        Event::ChangeEventsMode(ev) => emit(ev),
    }
}

#[cfg(feature = "contract-support")]
pub fn init_events() {
    let events_mode = EventsMode::try_from(get_stored_value::<u8>(ARG_EVENTS_MODE))
        .unwrap_or_revert_with(Cep85Error::InvalidEventsMode);

    if [EventsMode::CES, EventsMode::NativeNCES].contains(&events_mode)
        && get_key(casper_event_standard::EVENTS_DICT).is_none()
    {
        let schemas = Schemas::new()
            .with::<Mint>()
            .with::<MintBatch>()
            .with::<Burn>()
            .with::<BurnBatch>()
            .with::<ApprovalForAll>()
            .with::<Transfer>()
            .with::<TransferBatch>()
            .with::<Uri>()
            .with::<UriBatch>()
            .with::<SetTotalSupply>()
            .with::<ChangeSecurity>()
            .with::<SetModalities>()
            .with::<Upgrade>()
            .with::<ChangeEventsMode>()
            .with::<ChangeEnableBurnMode>();
        casper_event_standard::init(schemas);
    }
}
