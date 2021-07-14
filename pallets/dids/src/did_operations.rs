#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    codec::{Decode, Encode}
};
use frame_system::ensure_signed;
use sp_runtime::RuntimeDebug;
/// DID operations
/// Create
/// Delete
/// PK invocation/ revocation

/// Holds information about a DID document
/// Associate account
/// <Key, Value>
#[derive(Encode, Debug, Decode, Default, Clone, PartialEq)]
pub struct DIDAnchor {}

pub trait ValidateDID {}

pub trait DeleteDID {}
