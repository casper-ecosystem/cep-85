use alloc::{string::String, vec::Vec, collections::BTreeMap};
use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_event_standard::{emit, Event, Schemas};
use casper_types::{Key, U256};
use core::convert::TryFrom;

use crate::{constants::EVENTS_MODE, modalities::EventsMode, utils::get_stored_value, security::SecurityBadge};

pub enum Event {
    Mint(Mint),
    Burn(Burn),
    ApprovalForAll(ApprovalForAll),
    TransferSingle(TransferSingle),
    TransferBatch(TransferBatch),
    Uri(Uri),
    SetTotalSupply(SetTotalSupply),
    ChangeSecurity(ChangeSecurity),
}

pub fn record_event_dictionary(event: Event) {
    let events_mode: EventsMode =
        EventsMode::try_from(get_stored_value::<u8>(EVENTS_MODE)).unwrap_or_revert();

    match events_mode {
        EventsMode::NoEvents => {}
        EventsMode::CES => ces(event),
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
pub struct TransferSingle {
    pub operator: Key,
    pub from: Key,
    pub to: Key,
    pub id: U256,
    pub value: U256,
}

impl TransferSingle {
    pub fn new(operator: Key, from: Key, to: Key, id: U256, value: U256) -> Self {
        Self {
            operator,
            from,
            to,
            id,
            value,
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
}

impl TransferBatch {
    pub fn new(operator: Key, from: Key, to: Key, ids: Vec<U256>, values: Vec<U256>) -> Self {
        Self {
            operator,
            from,
            to,
            ids,
            values,
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
        Self { admin, sec_change_map }
    }
}

fn ces(event: Event) {
    match event {
        Event::Mint(ev) => emit(ev),
        Event::Burn(ev) => emit(ev),
        Event::ApprovalForAll(ev) => emit(ev),
        Event::TransferSingle(ev) => emit(ev),
        Event::TransferBatch(ev) => emit(ev),
        Event::Uri(ev) => emit(ev),
        Event::SetTotalSupply(ev) => emit(ev),
        Event::ChangeSecurity(ev) => emit(ev),
    }
}

pub fn init_events() {
    let events_mode = EventsMode::try_from(get_stored_value::<u8>(EVENTS_MODE)).unwrap_or_revert();

    if events_mode == EventsMode::CES {
        let schemas = Schemas::new()
            .with::<Mint>()
            .with::<Burn>()
            .with::<ApprovalForAll>()
            .with::<TransferSingle>()
            .with::<TransferBatch>()
            .with::<Uri>()
            .with::<SetTotalSupply>();
        casper_event_standard::init(schemas);
    }
}
