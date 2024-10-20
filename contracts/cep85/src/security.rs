#[cfg(feature = "contract-support")]
use alloc::collections::BTreeMap;
use alloc::{vec, vec::Vec};
#[cfg(feature = "contract-support")]
use casper_contract::{contract_api::runtime::revert, unwrap_or_revert::UnwrapOrRevert};
#[cfg(feature = "contract-support")]
use casper_types::Key;
use casper_types::{
    bytesrepr::{self, FromBytes, ToBytes},
    CLTyped,
};

#[cfg(feature = "contract-support")]
use crate::{
    constants::DICT_SECURITY_BADGES,
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

#[cfg(feature = "contract-support")]
pub fn sec_check(allowed_badge_list: Vec<SecurityBadge>) {
    let (caller, caller_package) = get_verified_caller();
    let caller_badge = get_security_badge(&caller);
    let package_badge = caller_package.and_then(|package| get_security_badge(&package));

    if let Some(badge) = caller_badge.or(package_badge) {
        if allowed_badge_list.contains(&badge)
            || (allowed_badge_list.contains(&SecurityBadge::Burner)
                && badge == SecurityBadge::Burner)
        {
            return;
        }
    }
    revert(Cep85Error::InsufficientRights);
}

#[cfg(feature = "contract-support")]
fn get_security_badge(entity: &Key) -> Option<SecurityBadge> {
    get_dictionary_value_from_key(
        DICT_SECURITY_BADGES,
        &hex::encode(
            entity
                .to_bytes()
                .unwrap_or_revert_with(Cep85Error::FailedToConvertBytes),
        ),
    )
}

#[cfg(feature = "contract-support")]
pub fn change_sec_badge(badge_map: &BTreeMap<Key, SecurityBadge>) {
    for (&user, &badge) in badge_map {
        set_dictionary_value_for_key(
            DICT_SECURITY_BADGES,
            &hex::encode(
                user.to_bytes()
                    .unwrap_or_revert_with(Cep85Error::FailedToConvertBytes),
            ),
            &badge,
        );
    }
}
