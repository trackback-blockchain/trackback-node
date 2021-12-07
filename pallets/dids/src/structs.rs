// This file is part of TrackBck which is released under GNU General Public License v3.0.
// See file LICENSE.md or go to https://www.gnu.org/licenses/gpl-3.0.en.html for full license details.
//! Structs to use in DID pallet
#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::codec::{Decode, Encode};
use frame_system::Config;
use sp_core::ed25519::Signature;
use sp_std::vec::Vec;
use serde::{Deserialize, Serialize};
/// Stores Signatures by DID Controllers
/// A DID can have at least a controller
#[derive(Clone, Decode, Encode, Eq, PartialEq, Debug)]
pub struct DIDSignature {
	pub public_key: Vec<u8>,
	pub proof: Signature,
	pub active: bool,
	pub created_time_stamp: u64,
	pub updated_timestamp: u64,
}

#[derive(Clone, Decode, Encode, Eq, PartialEq)]
pub struct DIDDetail<T: Config> {
	// Tracking number of issued DIDs  by the controller
	public_key: Vec<u8>,

	// Issued DID documents by the controller
	did_documents: Vec<DID<T>>,
}

#[derive(Clone, Decode, Encode, Eq, PartialEq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct DID<T: Config> {
	pub did_resolution_metadata: Option<Vec<u8>>,

	// DID Document Metadata
	pub did_document_metadata: Option<Vec<u8>>,

	// Block number
	pub block_number: <T as frame_system::Config>::BlockNumber,
	// Created  time stamp in ISO 8601 format
	pub block_time_stamp: u64,

	// Updated timestamp
	pub updated_timestamp: u64,
	// IPFS  URI of the DID document
	pub did_ref: Option<Vec<u8>>,

	// Sender AccountId
	pub sender_account_id: Vec<u8>,

	// public keys
	pub public_keys: Option<Vec<Vec<u8>>>,
}

#[derive(Clone, Decode, Encode, Eq, PartialEq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct VerifiableCredential<T: Config> {
	// Controller's AccountId
	pub account_id: Option<T::AccountId>,

	// Holder's public key
	pub public_key: Vec<u8>,

	// Created time
	pub block_time_stamp: u64,

	// active
	pub active: Option<bool>,
}

/// Defaults for VerifiableCredentials
impl<T: Config> Default for VerifiableCredential<T> {
	fn default() -> Self {
		Self { account_id: None, public_key: Vec::new(), block_time_stamp: 0, active: Some(false) }
	}
}

/// Defaults for Signature
impl Default for DIDSignature {
	fn default() -> Self {
		Self {
			public_key: Vec::new(),
			proof: Signature::from_raw([0; 64]),
			active: true,
			created_time_stamp: 0,
			updated_timestamp: 0,
		}
	}
}

// Defaults for DIDs
impl <T: Config> Default for DID<T> {
	fn default() -> Self {
		Self{
			did_resolution_metadata: None,
			did_document_metadata: None,
			block_number: <T as frame_system::Config>::BlockNumber::default(),
			block_time_stamp: 0,
			updated_timestamp: 0,
			did_ref: None,
			sender_account_id: vec![],
			public_keys: None
		}
	}
}
