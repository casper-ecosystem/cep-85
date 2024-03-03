use crate::{
    constants::{ARG_TRANSFER_FILTER_CONTRACT, ARG_TRANSFER_FILTER_METHOD},
    error::Cep85Error,
};
use alloc::{
    borrow::ToOwned,
    format,
    string::{String, ToString},
    vec,
    vec::Vec,
};
use casper_contract::{
    contract_api::{self, runtime, storage},
    ext_ffi,
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    account::AccountHash,
    api_error,
    bytesrepr::{self, FromBytes, ToBytes},
    system::CallStackElement,
    ApiError, CLTyped, ContractHash, ContractPackageHash, Key, URef, U256,
};
use core::{convert::TryInto, mem::MaybeUninit};

pub enum Caller {
    Session(AccountHash),
    StoredCaller(ContractHash, ContractPackageHash),
}

pub fn get_verified_caller() -> (Key, Option<Key>) {
    let get_verified_caller: Result<Caller, Cep85Error> = match *runtime::get_call_stack()
        .iter()
        .nth_back(1)
        .to_owned()
        .unwrap_or_revert()
    {
        CallStackElement::Session {
            account_hash: calling_account_hash,
        } => Ok(Caller::Session(calling_account_hash)),
        CallStackElement::StoredSession {
            contract_hash,
            contract_package_hash,
            ..
        }
        | CallStackElement::StoredContract {
            contract_hash,
            contract_package_hash,
        } => Ok(Caller::StoredCaller(contract_hash, contract_package_hash)),
    };

    match get_verified_caller.unwrap_or_revert() {
        Caller::Session(account_hash) => (account_hash.into(), None),
        Caller::StoredCaller(contract_hash, package_hash) => {
            (contract_hash.into(), Some(package_hash.into()))
        }
    }
}

pub fn get_stored_value<T>(name: &str) -> T
where
    T: FromBytes + CLTyped,
{
    let uref = get_uref(name);
    let value: T = storage::read(uref).unwrap_or_revert().unwrap_or_revert();
    value
}

pub fn get_named_arg_with_user_errors<T: FromBytes>(
    name: &str,
    missing: Cep85Error,
    invalid: Cep85Error,
) -> Result<T, Cep85Error> {
    let arg_size = get_named_arg_size(name).ok_or(missing)?;
    let arg_bytes = if arg_size > 0 {
        let res = {
            let data_non_null_ptr = contract_api::alloc_bytes(arg_size);
            let ret = unsafe {
                ext_ffi::casper_get_named_arg(
                    name.as_bytes().as_ptr(),
                    name.len(),
                    data_non_null_ptr.as_ptr(),
                    arg_size,
                )
            };
            let data =
                unsafe { Vec::from_raw_parts(data_non_null_ptr.as_ptr(), arg_size, arg_size) };
            api_error::result_from(ret).map(|_| data)
        };
        // Assumed to be safe as `get_named_arg_size` checks the argument already
        res.unwrap_or_revert_with(Cep85Error::FailedToGetArgBytes)
    } else {
        // Avoids allocation with 0 bytes and a call to get_named_arg
        Vec::new()
    };

    bytesrepr::deserialize(arg_bytes).map_err(|_| invalid)
}

pub fn get_optional_named_arg_with_user_errors<T: FromBytes>(
    name: &str,
    invalid: Cep85Error,
) -> Option<T> {
    match get_named_arg_with_user_errors::<T>(name, Cep85Error::Phantom, invalid) {
        Ok(val) => Some(val),
        Err(_) => None,
    }
}

pub fn get_stored_value_with_user_errors<T: CLTyped + FromBytes>(
    name: &str,
    missing: Cep85Error,
    invalid: Cep85Error,
) -> T {
    let uref = get_uref_with_user_errors(name, missing, invalid);
    read_with_user_errors(uref, missing, invalid)
}

pub fn stringify_key<T: CLTyped>(key: Key) -> String {
    match key {
        Key::Account(account_hash) => account_hash.to_string(),
        Key::Hash(hash_addr) => ContractHash::new(hash_addr).to_string(),
        _ => runtime::revert(Cep85Error::InvalidKey),
    }
}

pub fn make_dictionary_item_key<T: CLTyped + ToBytes, V: CLTyped + ToBytes>(
    key: &T,
    value: &V,
) -> String {
    let mut bytes_a = key.to_bytes().unwrap_or_revert();
    let mut bytes_b = value.to_bytes().unwrap_or_revert();

    bytes_a.append(&mut bytes_b);

    let bytes = runtime::blake2b(bytes_a);
    hex::encode(bytes)
}

pub fn get_dictionary_value_from_key<T: CLTyped + FromBytes>(
    dictionary_name: &str,
    key: &str,
) -> Option<T> {
    let seed_uref = get_uref_with_user_errors(
        dictionary_name,
        Cep85Error::MissingStorageUref,
        Cep85Error::InvalidStorageUref,
    );

    match storage::dictionary_get::<T>(seed_uref, key) {
        Ok(maybe_value) => maybe_value,
        Err(error) => runtime::revert(error),
    }
}

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
    storage::dictionary_put::<T>(seed_uref, key, *value)
}

pub fn get_transfer_filter_contract() -> Option<ContractHash> {
    get_stored_value_with_user_errors(
        ARG_TRANSFER_FILTER_CONTRACT,
        Cep85Error::MissingTransferFilterContract,
        Cep85Error::InvalidTransferFilterContract,
    )
}

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

fn get_uref(name: &str) -> URef {
    let key = runtime::get_key(name)
        .ok_or(ApiError::MissingKey)
        .unwrap_or_revert();
    key.try_into().unwrap_or_revert()
}

fn get_uref_with_user_errors(name: &str, missing: Cep85Error, invalid: Cep85Error) -> URef {
    let key = get_key_with_user_errors(name, missing, invalid);
    key.into_uref()
        .unwrap_or_revert_with(Cep85Error::UnexpectedKeyVariant)
}

fn get_key_with_user_errors(name: &str, missing: Cep85Error, invalid: Cep85Error) -> Key {
    let (name_ptr, name_size, _bytes) = to_ptr(name);
    let mut key_bytes = vec![0u8; Key::max_serialized_length()];
    let mut total_bytes: usize = 0;
    let ret = unsafe {
        ext_ffi::casper_get_key(
            name_ptr,
            name_size,
            key_bytes.as_mut_ptr(),
            key_bytes.len(),
            &mut total_bytes as *mut usize,
        )
    };
    match api_error::result_from(ret) {
        Ok(_) => {}
        Err(ApiError::MissingKey) => runtime::revert(missing),
        Err(e) => runtime::revert(e),
    }
    key_bytes.truncate(total_bytes);

    bytesrepr::deserialize(key_bytes).unwrap_or_revert_with(invalid)
}

fn read_with_user_errors<T: CLTyped + FromBytes>(
    uref: URef,
    missing: Cep85Error,
    invalid: Cep85Error,
) -> T {
    let key: Key = uref.into();
    let (key_ptr, key_size, _bytes) = to_ptr(key);

    let value_size = {
        let mut value_size = MaybeUninit::uninit();
        let ret = unsafe { ext_ffi::casper_read_value(key_ptr, key_size, value_size.as_mut_ptr()) };
        match api_error::result_from(ret) {
            Ok(_) => unsafe { value_size.assume_init() },
            Err(ApiError::ValueNotFound) => runtime::revert(missing),
            Err(e) => runtime::revert(e),
        }
    };

    let value_bytes = read_host_buffer(value_size).unwrap_or_revert();

    bytesrepr::deserialize(value_bytes).unwrap_or_revert_with(invalid)
}

fn read_host_buffer(size: usize) -> Result<Vec<u8>, ApiError> {
    let mut dest: Vec<u8> = if size == 0 {
        Vec::new()
    } else {
        let bytes_non_null_ptr = contract_api::alloc_bytes(size);
        unsafe { Vec::from_raw_parts(bytes_non_null_ptr.as_ptr(), size, size) }
    };
    read_host_buffer_into(&mut dest)?;
    Ok(dest)
}

fn read_host_buffer_into(dest: &mut [u8]) -> Result<usize, ApiError> {
    let mut bytes_written = MaybeUninit::uninit();
    let ret = unsafe {
        ext_ffi::casper_read_host_buffer(dest.as_mut_ptr(), dest.len(), bytes_written.as_mut_ptr())
    };
    // NOTE: When rewriting below expression as `result_from(ret).map(|_| unsafe { ... })`, and the
    // caller ignores the return value, execution of the contract becomes unstable and ultimately
    // leads to `Unreachable` error.
    api_error::result_from(ret)?;
    Ok(unsafe { bytes_written.assume_init() })
}

fn to_ptr<T: ToBytes>(t: T) -> (*const u8, usize, Vec<u8>) {
    let bytes = t.into_bytes().unwrap_or_revert();
    let ptr = bytes.as_ptr();
    let size = bytes.len();
    (ptr, size, bytes)
}

fn get_named_arg_size(name: &str) -> Option<usize> {
    let mut arg_size: usize = 0;
    let ret = unsafe {
        ext_ffi::casper_get_named_arg_size(
            name.as_bytes().as_ptr(),
            name.len(),
            &mut arg_size as *mut usize,
        )
    };
    match api_error::result_from(ret) {
        Ok(_) => Some(arg_size),
        Err(ApiError::MissingArgument) => None,
        Err(e) => runtime::revert(e),
    }
}