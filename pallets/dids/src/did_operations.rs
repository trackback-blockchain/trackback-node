#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::codec::{Decode, Encode};



/// DID operations
/// Create
/// Delete
/// PK invocation/ revocation

/// Holds information about a DID document
/// Associate account
/// <Key, Value>
#[derive(Encode, Debug, Decode, Default, Clone, PartialEq)]
pub struct DIDDocument {}

#[derive(Encode, Debug, Decode, Default, Clone, PartialEq)]
pub struct DIDController {}

#[derive(Encode, Debug, Decode, Default, Clone, PartialEq)]
pub struct DIDIssuer {}
