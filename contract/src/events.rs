use alloc::vec::Vec;
use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_event_standard::{emit, Event, Schemas};
use casper_types::{Key, U256};
use core::convert::TryFrom;

use crate::{
    constants::EVENTS_MODE,
    modalities::{EventsMode, TokenIdentifier},
    utils::get_stored_value,
};

pub enum Event {
    Mint(Mint),
    Burn(Burn),
    ApprovalForAll(ApprovalForAll),
    TransferSingle(TransferSingle),
    TransferBatch(TransferBatch),
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
    pub id: TokenIdentifier,
    pub recipient: Key,
    pub amount: U256,
}

impl Mint {
    pub fn new(id: TokenIdentifier, recipient: Key, amount: U256) -> Self {
        Self {
            id,
            recipient,
            amount,
        }
    }
}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct Burn {
    pub id: TokenIdentifier,
    pub owner: Key,
    pub amount: U256,
}

impl Burn {
    pub fn new(owner: Key, id: TokenIdentifier, amount: U256) -> Self {
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
    pub id: TokenIdentifier,
    pub value: U256,
}

impl TransferSingle {
    pub fn new(operator: Key, from: Key, to: Key, id: TokenIdentifier, value: U256) -> Self {
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
    pub ids: Vec<TokenIdentifier>,
    pub values: Vec<U256>,
}

impl TransferBatch {
    pub fn new(
        operator: Key,
        from: Key,
        to: Key,
        ids: Vec<TokenIdentifier>,
        values: Vec<U256>,
    ) -> Self {
        Self {
            operator,
            from,
            to,
            ids,
            values,
        }
    }
}

fn ces(event: Event) {
    match event {
        Event::Mint(ev) => emit(ev),
        Event::Burn(ev) => emit(ev),
        Event::ApprovalForAll(ev) => emit(ev),
        Event::TransferSingle(ev) => emit(ev),
        Event::TransferBatch(ev) => emit(ev),
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
            .with::<TransferBatch>();
        casper_event_standard::init(schemas);
    }
}
