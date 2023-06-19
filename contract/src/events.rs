use alloc::string::{String, ToString};
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
    Transfer(Transfer),
    TransferFrom(TransferFrom),
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
    pub token_id: String,
    pub recipient: Key,
    pub amount: U256,
}

impl Mint {
    pub fn new(token_id: TokenIdentifier, recipient: Key, amount: U256) -> Self {
        Self {
            token_id: token_id.to_string(),
            recipient,
            amount,
        }
    }
}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct Burn {
    pub token_id: String,
    pub owner: Key,
    pub amount: U256,
}

impl Burn {
    pub fn new(owner: Key, token_id: TokenIdentifier, amount: U256) -> Self {
        Self {
            token_id: token_id.to_string(),
            owner,
            amount,
        }
    }
}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct Transfer {
    pub token_id: String,
    pub sender: Key,
    pub recipient: Key,
    pub amount: U256,
}

impl Transfer {
    pub fn new(token_id: TokenIdentifier, sender: Key, recipient: Key, amount: U256) -> Self {
        Self {
            token_id: token_id.to_string(),
            sender,
            recipient,
            amount,
        }
    }
}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct TransferFrom {
    pub token_id: String,
    pub spender: Option<Key>,
    pub owner: Key,
    pub recipient: Key,
    pub amount: U256,
}

impl TransferFrom {
    pub fn new(
        token_id: TokenIdentifier,
        owner: Key,
        spender: Option<Key>,
        recipient: Key,
        amount: U256,
    ) -> Self {
        Self {
            token_id: token_id.to_string(),
            owner,
            spender,
            recipient,
            amount,
        }
    }
}

fn ces(event: Event) {
    match event {
        Event::Mint(ev) => emit(ev),
        Event::Burn(ev) => emit(ev),
        Event::Transfer(ev) => emit(ev),
        Event::TransferFrom(ev) => emit(ev),
    }
}

pub fn init_events() {
    let events_mode = EventsMode::try_from(get_stored_value::<u8>(EVENTS_MODE)).unwrap_or_revert();

    if events_mode == EventsMode::CES {
        let schemas = Schemas::new()
            .with::<Mint>()
            .with::<Burn>()
            .with::<Transfer>()
            .with::<TransferFrom>();
        casper_event_standard::init(schemas);
    }
}
