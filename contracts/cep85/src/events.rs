use crate::security::SecurityBadge;
#[cfg(feature = "contract-support")]
use crate::{constants::ARG_EVENTS_MODE, modalities::EventsMode, utils::get_stored_value};
use alloc::{collections::BTreeMap, string::String, vec::Vec};
#[cfg(feature = "contract-support")]
use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_event_standard::Event;
#[cfg(feature = "contract-support")]
use casper_event_standard::{emit, Schemas};
use casper_types::{bytesrepr::Bytes, Key, U256};
#[cfg(feature = "contract-support")]
use core::convert::TryFrom;

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
}

#[cfg(feature = "contract-support")]
pub fn record_event_dictionary(event: Event) {
    let events_mode: EventsMode =
        EventsMode::try_from(get_stored_value::<u8>(ARG_EVENTS_MODE)).unwrap_or_revert();

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
    }
}

#[cfg(feature = "contract-support")]
pub fn init_events() {
    let events_mode =
        EventsMode::try_from(get_stored_value::<u8>(ARG_EVENTS_MODE)).unwrap_or_revert();

    if events_mode == EventsMode::CES {
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
            .with::<Upgrade>();
        casper_event_standard::init(schemas);
    }
}
