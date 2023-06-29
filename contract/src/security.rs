use alloc::{collections::BTreeMap, vec, vec::Vec};
use casper_contract::{contract_api::runtime::revert, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{
    bytesrepr::{self, FromBytes, ToBytes},
    CLTyped, Key,
};

use crate::{
    constants::SECURITY_BADGES,
    error::Cep85Error,
    utils::{get_dictionary_value_from_key, get_verified_caller, set_dictionary_value_for_key},
};

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SecurityBadge {
    Admin = 0,
    Minter = 1,
    Burner = 2,
    Meta = 3,
    None = 4,
}

impl CLTyped for SecurityBadge {
    fn cl_type() -> casper_types::CLType {
        casper_types::CLType::U8
    }
}

impl ToBytes for SecurityBadge {
    fn to_bytes(&self) -> Result<Vec<u8>, bytesrepr::Error> {
        Ok(vec![*self as u8])
    }

    fn serialized_length(&self) -> usize {
        1
    }
}

impl FromBytes for SecurityBadge {
    fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), bytesrepr::Error> {
        Ok((
            match bytes[0] {
                0 => SecurityBadge::Admin,
                1 => SecurityBadge::Minter,
                2 => SecurityBadge::Burner,
                3 => SecurityBadge::Meta,
                4 => SecurityBadge::None,
                _ => return Err(bytesrepr::Error::LeftOverBytes),
            },
            &[],
        ))
    }
}

pub fn sec_check(allowed_badge_list: Vec<SecurityBadge>) {
    let caller = get_verified_caller().0;
    let user_badge: Option<SecurityBadge> = get_dictionary_value_from_key(
        SECURITY_BADGES,
        &hex::encode(caller.to_bytes().unwrap_or_revert()),
    );

    if !allowed_badge_list
        .contains(&user_badge.unwrap_or_revert_with(Cep85Error::InsufficientRights))
    {
        revert(Cep85Error::InsufficientRights)
    }
}

pub fn change_sec_badge(badge_map: &BTreeMap<Key, SecurityBadge>) {
    for (&user, &badge) in badge_map {
        set_dictionary_value_for_key(
            SECURITY_BADGES,
            &hex::encode(user.to_bytes().unwrap_or_revert()),
            &badge,
        );
    }
}
