use std::convert::TryInto;

use contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert
};
use types::{CLTyped, account::AccountHash, bytesrepr::{FromBytes, ToBytes}, system::CallStackElement};

pub fn get_caller() -> Option<AccountHash> {
    match runtime::get_call_stack().first().unwrap() {
        CallStackElement::Session { account_hash }  => {
            Some(*account_hash)
        },
        CallStackElement::StoredSession { account_hash, contract_package_hash: _, contract_hash: _ } => {
            Some(*account_hash)
        },
        CallStackElement::StoredContract { contract_package_hash: _, contract_hash: _ } => {
            None
        }
    }
}

/// Getter function from context storage.
/// Returns the previously data previously stored under `name` key,
/// or returns the default value of the type expected at the end of the call.
pub fn get_key<T: FromBytes + CLTyped + Default>(name: &str) -> T {
    match runtime::get_key(name) {
        None => Default::default(),
        Some(value) => {
            let key = value.try_into().unwrap_or_revert();
            storage::read(key).unwrap_or_revert().unwrap_or_revert()
        }
    }
}

/// Creates new storage key `name` and stores `value` to it.
/// In case the key `name` already exists, overwrites it with the new data.
pub fn set_key<T: ToBytes + CLTyped>(name: &str, value: T) {
    match runtime::get_key(name) {
        Some(key) => {
            let key_ref = key.try_into().unwrap_or_revert();
            storage::write(key_ref, value);
        }
        None => {
            let key = storage::new_uref(value).into();
            runtime::put_key(name, key);
        }
    }
}