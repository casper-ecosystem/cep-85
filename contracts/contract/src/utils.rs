#[cfg(feature = "contract-support")]
use crate::{
    constants::{ARG_TRANSFER_FILTER_CONTRACT, ARG_TRANSFER_FILTER_METHOD},
    error::Cep85Error,
};
use alloc::{format, string::String};
#[cfg(feature = "contract-support")]
use alloc::{vec, vec::Vec};
#[cfg(feature = "contract-support")]
use casper_contract::{
    contract_api::{
        alloc_bytes,
        runtime::{
            blake2b, get_immediate_caller as casper_get_immediate_caller, get_key,
            get_protocol_version, revert,
        },
        storage::{dictionary_get, dictionary_put, read},
    },
    ext_ffi::{
        casper_get_key, casper_get_named_arg, casper_get_named_arg_size, casper_read_host_buffer,
        casper_read_value,
    },
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::U256;
#[cfg(feature = "contract-support")]
use casper_types::{
    account::AccountHash,
    api_error::result_from,
    bytesrepr::{deserialize, FromBytes, ToBytes},
    contracts::{ContractHash, ContractPackageHash, ContractVersionKey},
    AddressableEntityHash, ApiError, CLTyped, EntityAddr, Key, PackageHash, URef,
};
#[cfg(feature = "contract-support")]
use core::{convert::TryInto, mem::MaybeUninit};
#[cfg(feature = "contract-support")]
use hex::encode;

#[cfg(feature = "contract-support")]
pub fn get_immediate_caller() -> (Key, Option<Key>) {
    const ACCOUNT: u8 = 0;
    const PACKAGE: u8 = 1;
    const CONTRACT_PACKAGE: u8 = 2;
    const ENTITY: u8 = 3;
    const CONTRACT: u8 = 4;

    let caller_info = casper_get_immediate_caller().unwrap_or_revert();

    match caller_info.kind() {
        ACCOUNT => {
            let account_hash = caller_info
                .get_field_by_index(ACCOUNT)
                .unwrap()
                .to_t::<Option<AccountHash>>()
                .unwrap_or_revert()
                .unwrap_or_revert_with(Cep85Error::UnexpectedKeyVariant);
            (
                Key::AddressableEntity(EntityAddr::Account(account_hash.value())),
                None,
            )
        }
        PACKAGE => {
            let package_hash = caller_info
                .get_field_by_index(PACKAGE)
                .unwrap()
                .to_t::<Option<PackageHash>>()
                .unwrap_or_revert()
                .unwrap_or_revert_with(Cep85Error::UnexpectedKeyVariant);
            let contract_hash = caller_info
                .get_field_by_index(CONTRACT)
                .unwrap()
                .to_t::<Option<ContractHash>>()
                .unwrap_or_revert()
                .unwrap_or_revert_with(Cep85Error::UnexpectedKeyVariant);
            (
                Key::contract_entity_key(contract_hash.into()),
                Some(Key::SmartContract(package_hash.value())),
            )
        }
        CONTRACT_PACKAGE => {
            let contract_package_hash = caller_info
                .get_field_by_index(CONTRACT_PACKAGE)
                .unwrap()
                .to_t::<Option<ContractPackageHash>>()
                .unwrap_or_revert()
                .unwrap_or_revert_with(Cep85Error::UnexpectedKeyVariant);
            let contract_hash = caller_info
                .get_field_by_index(CONTRACT)
                .unwrap()
                .to_t::<Option<ContractHash>>()
                .unwrap_or_revert()
                .unwrap_or_revert_with(Cep85Error::UnexpectedKeyVariant);
            (
                Key::contract_entity_key(contract_hash.into()),
                Some(Key::SmartContract(contract_package_hash.value())),
            )
        }
        ENTITY => {
            let entity_addr = caller_info
                .get_field_by_index(ENTITY)
                .unwrap()
                .to_t::<Option<EntityAddr>>()
                .unwrap_or_revert()
                .unwrap_or_revert_with(Cep85Error::UnexpectedKeyVariant);
            (Key::from(entity_addr), None)
        }
        CONTRACT => {
            let contract_hash = caller_info
                .get_field_by_index(CONTRACT)
                .unwrap()
                .to_t::<Option<ContractHash>>()
                .unwrap_or_revert()
                .unwrap_or_revert_with(Cep85Error::UnexpectedKeyVariant);
            let contract_package_hash = caller_info
                .get_field_by_index(CONTRACT_PACKAGE)
                .unwrap()
                .to_t::<Option<ContractPackageHash>>()
                .unwrap_or_revert()
                .unwrap_or_revert_with(Cep85Error::UnexpectedKeyVariant);
            (
                Key::contract_entity_key(contract_hash.into()),
                Some(Key::SmartContract(contract_package_hash.value())),
            )
        }
        _ => revert(Cep85Error::UnexpectedKeyVariant),
    }
}

#[cfg(feature = "contract-support")]
pub fn get_contract_version_key(contract_version: u32) -> ContractVersionKey {
    let (major, _, _) = get_protocol_version().destructure();
    ContractVersionKey::new(major, contract_version)
}

#[cfg(feature = "contract-support")]
pub fn get_stored_value<T>(name: &str) -> T
where
    T: FromBytes + CLTyped,
{
    let uref = get_uref(name);
    read(uref)
        .unwrap_or_revert_with(Cep85Error::UrefNotFound)
        .unwrap_or_revert_with(Cep85Error::FailedToReadFromStorage)
}

#[cfg(feature = "contract-support")]
pub fn get_named_arg_with_user_errors<T: FromBytes>(
    name: &str,
    missing: Cep85Error,
    invalid: Cep85Error,
) -> Result<T, Cep85Error> {
    let arg_size = get_named_arg_size(name).ok_or(missing)?;
    let arg_bytes = if arg_size > 0 {
        let res = {
            let data_non_null_ptr = alloc_bytes(arg_size);
            let ret = unsafe {
                casper_get_named_arg(
                    name.as_bytes().as_ptr(),
                    name.len(),
                    data_non_null_ptr.as_ptr(),
                    arg_size,
                )
            };
            let data =
                unsafe { Vec::from_raw_parts(data_non_null_ptr.as_ptr(), arg_size, arg_size) };
            result_from(ret).map(|_| data)
        };
        // Assumed to be safe as `get_named_arg_size` checks the argument already
        res.unwrap_or_revert_with(Cep85Error::FailedToGetArgBytes)
    } else {
        // Avoids allocation with 0 bytes and a call to get_named_arg
        Vec::new()
    };

    deserialize(arg_bytes).map_err(|_| invalid)
}

#[cfg(feature = "contract-support")]
pub fn get_optional_named_arg_with_user_errors<T: FromBytes>(
    name: &str,
    invalid: Cep85Error,
) -> Option<T> {
    match get_named_arg_with_user_errors::<T>(name, Cep85Error::Phantom, invalid) {
        Ok(val) => Some(val),
        Err(Cep85Error::Phantom) => None,
        Err(e) => revert(e),
    }
}

#[cfg(feature = "contract-support")]
pub fn get_stored_value_with_user_errors<T: CLTyped + FromBytes>(
    name: &str,
    missing: Cep85Error,
    invalid: Cep85Error,
) -> T {
    let uref = get_uref_with_user_errors(name, missing, invalid);
    read_with_user_errors(uref, missing, invalid)
}

#[cfg(feature = "contract-support")]
pub fn make_dictionary_item_key<T: CLTyped + ToBytes, V: CLTyped + ToBytes>(
    key: &T,
    value: &V,
) -> String {
    let mut bytes_a = key
        .to_bytes()
        .unwrap_or_revert_with(Cep85Error::FailedToConvertBytes);
    let mut bytes_b = value
        .to_bytes()
        .unwrap_or_revert_with(Cep85Error::FailedToConvertBytes);

    bytes_a.append(&mut bytes_b);

    let bytes = blake2b(bytes_a);
    encode(bytes)
}

#[cfg(feature = "contract-support")]
pub fn get_dictionary_value_from_key<T: CLTyped + FromBytes>(
    dictionary_name: &str,
    key: &str,
) -> Option<T> {
    let seed_uref = get_uref_with_user_errors(
        dictionary_name,
        Cep85Error::MissingStorageUref,
        Cep85Error::InvalidStorageUref,
    );

    match dictionary_get::<T>(seed_uref, key) {
        Ok(maybe_value) => maybe_value,
        Err(error) => revert(error),
    }
}

#[cfg(feature = "contract-support")]
pub fn set_dictionary_value_for_key<T: CLTyped + ToBytes + Copy>(
    dictionary_name: &str,
    key: &str,
    value: &T,
) {
    let seed_uref = get_uref_with_user_errors(
        dictionary_name,
        Cep85Error::MissingStorageUref,
        Cep85Error::InvalidStorageUref,
    );
    dictionary_put::<T>(seed_uref, key, *value)
}

#[cfg(feature = "contract-support")]
pub fn get_transfer_filter_contract() -> Option<AddressableEntityHash> {
    get_stored_value_with_user_errors(
        ARG_TRANSFER_FILTER_CONTRACT,
        Cep85Error::MissingTransferFilterContract,
        Cep85Error::InvalidTransferFilterContract,
    )
}

#[cfg(feature = "contract-support")]
pub fn get_transfer_filter_method() -> Option<String> {
    get_stored_value_with_user_errors(
        ARG_TRANSFER_FILTER_METHOD,
        Cep85Error::MissingTransferFilterMethod,
        Cep85Error::InvalidTransferFilterMethod,
    )
}

pub fn replace_token_id_in_uri(raw_uri: &str, id: &U256) -> String {
    raw_uri.replace("{id}", &format!("{}", id))
}

#[cfg(feature = "contract-support")]
pub fn get_uref_with_user_errors(name: &str, missing: Cep85Error, invalid: Cep85Error) -> URef {
    let key = get_key_with_user_errors(name, missing, invalid);
    key.into_uref()
        .unwrap_or_revert_with(Cep85Error::UnexpectedKeyVariant)
}

#[cfg(feature = "contract-support")]
fn get_uref(name: &str) -> URef {
    let key = get_key(name)
        .ok_or(ApiError::MissingKey)
        .unwrap_or_revert_with(Cep85Error::FailedToGetKey);
    key.try_into()
        .unwrap_or_revert_with(Cep85Error::InvalidKeyType)
}

#[cfg(feature = "contract-support")]
fn get_key_with_user_errors(name: &str, missing: Cep85Error, invalid: Cep85Error) -> Key {
    let (name_ptr, name_size, _bytes) = to_ptr(name);
    let mut key_bytes = vec![0u8; Key::max_serialized_length()];
    let mut total_bytes: usize = 0;
    let ret = unsafe {
        casper_get_key(
            name_ptr,
            name_size,
            key_bytes.as_mut_ptr(),
            key_bytes.len(),
            &mut total_bytes as *mut usize,
        )
    };
    match result_from(ret) {
        Ok(_) => {}
        Err(ApiError::MissingKey) => revert(missing),
        Err(e) => revert(e),
    }
    key_bytes.truncate(total_bytes);

    deserialize(key_bytes).unwrap_or_revert_with(invalid)
}

#[cfg(feature = "contract-support")]
fn read_with_user_errors<T: CLTyped + FromBytes>(
    uref: URef,
    missing: Cep85Error,
    invalid: Cep85Error,
) -> T {
    let key: Key = uref.into();
    let (key_ptr, key_size, _bytes) = to_ptr(key);

    // Get the size of the value
    let value_size = {
        let mut value_size = MaybeUninit::uninit();
        let ret = unsafe { casper_read_value(key_ptr, key_size, value_size.as_mut_ptr()) };
        match result_from(ret) {
            Ok(_) => unsafe { value_size.assume_init() },
            Err(ApiError::ValueNotFound) => revert(missing),
            Err(e) => revert(e),
        }
    };

    // Allocate a buffer to store the value
    let mut buffer = vec![0u8; value_size];
    let mut bytes_written = 0usize;

    let ret = unsafe {
        casper_read_host_buffer(
            buffer.as_mut_ptr(),
            value_size,
            &mut bytes_written as *mut usize,
        )
    };

    // Check for errors
    match result_from(ret) {
        Ok(_) => {}
        Err(e) => revert(e),
    }

    if bytes_written != value_size {
        revert(ApiError::UnexpectedKeyVariant);
    }

    deserialize(buffer).unwrap_or_revert_with(invalid)
}

#[cfg(feature = "contract-support")]
fn to_ptr<T: ToBytes>(t: T) -> (*const u8, usize, Vec<u8>) {
    let bytes = t.into_bytes().unwrap_or_revert();
    let ptr = bytes.as_ptr();
    let size = bytes.len();
    (ptr, size, bytes)
}

#[cfg(feature = "contract-support")]
fn get_named_arg_size(name: &str) -> Option<usize> {
    let mut arg_size: usize = 0;
    let ret = unsafe {
        casper_get_named_arg_size(
            name.as_bytes().as_ptr(),
            name.len(),
            &mut arg_size as *mut usize,
        )
    };
    match result_from(ret) {
        Ok(_) => Some(arg_size),
        Err(ApiError::MissingArgument) => None,
        Err(e) => revert(e),
    }
}
