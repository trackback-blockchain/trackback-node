//! Structs to use in DID pallet
#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::codec::{Decode, Encode};
use frame_system::Config;
use sp_std::vec::Vec;

#[derive(Clone, Decode, Encode, Eq, PartialEq)]
pub struct DIDDetail<T: Config> {
	// Tracking number of issued DIDs  by the controller
	public_key: Vec<u8>,

	// Issued DID documents by the controller
	did_documents: Vec<DID<T>>,
}

#[derive(Clone, Decode, Encode, Eq, PartialEq, Debug)]
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

impl<T: Config> Default for VerifiableCredential<T> {
	fn default() -> Self {
		Self { account_id: None, public_key: Vec::new(), block_time_stamp: 0, active: Some(false) }
	}
}
