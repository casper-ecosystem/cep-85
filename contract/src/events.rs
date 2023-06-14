use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_event_standard::{emit, Event, Schemas};
use casper_types::{Key, U256};
use core::convert::TryFrom;

use crate::{constants::EVENTS_MODE, modalities::EventsMode, utils::read_from};

pub fn record_event_dictionary(event: Event) {
    let events_mode: EventsMode =
        EventsMode::try_from(read_from::<u8>(EVENTS_MODE)).unwrap_or_revert();

    match events_mode {
        EventsMode::NoEvents => {}
        EventsMode::CES => ces(event),
    }
}

pub enum Event {
    Mint(Mint),
}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct Mint {
    pub recipient: Key,
    pub amount: U256,
}

fn ces(event: Event) {
    match event {
        Event::Mint(ev) => emit(ev),
    }
}

pub fn init_events() {
    let events_mode = EventsMode::try_from(read_from::<u8>(EVENTS_MODE)).unwrap_or_revert();

    if events_mode == EventsMode::CES {
        let schemas = Schemas::new();
        casper_event_standard::init(schemas);
    }
}
