#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame

use frame_support::{
	decl_module, decl_storage, decl_event, decl_error, ensure, StorageMap
};
use frame_system::ensure_signed;
use sp_std::vec::Vec;
use sp_std::collections::btree_set::BTreeSet;
use frame_support::codec::{Encode, Decode};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Trait: frame_system::Trait {
	/// Because this pallet emits events, it depends on the runtime's definition of an event.
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}

// Represents a crop entry result from BELIS
#[derive(Encode, Decode, Clone, PartialEq)]
pub enum CropEntryResult {
	Accepted(Vec<u8>),
	Rejected,
}

impl Default for CropEntryResult {
	fn default() -> Self {
		CropEntryResult::Rejected
	}
}

// Represents a bci line result from BELIS
#[derive(Encode, Decode, Clone, PartialEq)]
pub enum BCILineResult {
	Accepted,
	Rejected,
}

impl Default for BCILineResult {
	fn default() -> Self {
		BCILineResult::Rejected
	}
}

// The pallet's runtime storage items.
// https://substrate.dev/docs/en/knowledgebase/runtime/storage
decl_storage! {
	// A unique name is used to ensure that the pallet's storage items are isolated.
	// This name may be updated, but each pallet in the runtime must use a unique name.
	// ---------------------------------vvvvvvvvvvvvvv
	trait Store for Module<T: Trait> as TrackbackModule {
		// Learn more about declaring storage items:
		// https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items

		// crop_entry_id to [grower, event_hash, block_number]
		CropEntries get(fn crop_entries): map hasher(blake2_128_concat) Vec<u8> => (T::AccountId, Vec<u8>, T::BlockNumber);

		// crop_entry_id to [approver, result, event_hash, block_number]
		CropEntryResults get(fn crop_entry_results): map hasher(blake2_128_concat) Vec<u8> => (T::AccountId, CropEntryResult, Vec<u8>, T::BlockNumber);

		// crn to [approver, crop_info_id, event_hash, block_number]
		CRNs get(fn crns): map hasher(blake2_128_concat) Vec<u8> => (T::AccountId, Vec<u8>, Vec<u8>, T::BlockNumber);

		// bci_line_id to [submitter, crn, event_hash, block_number]
		BCILines get(fn bci_lines): map hasher(blake2_128_concat) Vec<u8> => (T::AccountId, Vec<u8>, Vec<u8>, T::BlockNumber);

		// bci_line_id to [approver, result, event_hash, block_number]
		BCILineResults get(fn bci_line_results): map hasher(blake2_128_concat) Vec<u8> => (T::AccountId, BCILineResult, Vec<u8>, T::BlockNumber);
	}
}

// Pallets use events to inform users when important changes are made.
// https://substrate.dev/docs/en/knowledgebase/runtime/events
decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Trait>::AccountId {
		/// Seed application submitted by grower. [event_hash, who]
		SeedApplicationSubmitted(Vec<u8>, AccountId),
		/// Seed application approved by BELIS system. [event_hash, who]
		SeedApplicationApproved(Vec<u8>, AccountId),
		/// Seed application rejected by BELIS system. [event_hash, who]
		SeedApplicationRejected(Vec<u8>, AccountId),
		/// BCI line submitted [event_hash, who]
		BCILineSubmitted(Vec<u8>, AccountId),
		/// BCI line approved by BELIS system. [event_hash, who]
		BCILineApproved(Vec<u8>, AccountId),
		/// BCI line rejected by BELIS system. [event_hash, who]
		BCILineRejected(Vec<u8>, AccountId),
	}
);

// Errors inform users that something went wrong.
decl_error! {
	pub enum Error for Module<T: Trait> {
		SeedApplicationAlreadyExists,
		NoCropEntriesInApplication,
		DuplicateCropEntryIdsInApplication,
		CropEntryIdAlreadyExists,
		CropEntryNotFoundForSeedValidation,
		CropEntryAlreadyValidated,
		CRNNotFoundForBCILine,
		BCILineAlreadyExists,
		BCILineIdNotFoundForValidation,
		BCILineAlreadyValidated,
	}
}

// Dispatchable functions allows users to interact with the pallet and invoke state changes.
// These functions materialize as "extrinsics", which are often compared to transactions.
// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Errors must be initialized if they are used by the pallet.
		type Error = Error<T>;

		// Events must be initialized if they are used by the pallet.
		fn deposit_event() = default;

		/// Stores the crop-entry ids and hashes of submitted crop entries
		#[weight = 10_000]
		pub fn submit_crop_entries(origin, event_hash: Vec<u8>, crop_entries: Vec<Vec<u8>>) {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://substrate.dev/docs/en/knowledgebase/runtime/origin
			let sender = ensure_signed(origin)?;

			ensure!(crop_entries.len() > 0, Error::<T>::NoCropEntriesInApplication);
			let mut seen = BTreeSet::new();

			// Make sure the crop info Ids hasn't been already submitted
			for entry in &crop_entries {
				let is_unique = seen.insert(entry);
				ensure!(is_unique, Error::<T>::DuplicateCropEntryIdsInApplication);
				ensure!(!(CropEntries::<T>::get(&entry).1 ==  event_hash), Error::<T>::SeedApplicationAlreadyExists);
				ensure!(!CropEntries::<T>::contains_key(&entry), Error::<T>::CropEntryIdAlreadyExists);
			}

			// Get the block number from the FRAME System module.
			let current_block = <frame_system::Module<T>>::block_number();

			// Store hash of the entire crop info application with each crop info Ids
			// We do this after complete validation to avoid any partial inserts to the state.
			for entry in crop_entries {
				CropEntries::<T>::insert(&entry, (&sender, &event_hash, &current_block))
			}

			// Emit an event that the application was created
			Self::deposit_event(RawEvent::SeedApplicationSubmitted(event_hash, sender));
		}

		/// Accepts a crop entry if it exists and has no result already. Then store the CRN for the crop entry
		#[weight = 10_000]
		pub fn accept_crop_entry(origin, crop_entry: Vec<u8>, event_hash: Vec<u8>, crn: Vec<u8>) {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://substrate.dev/docs/en/knowledgebase/runtime/origin
			let sender = ensure_signed(origin)?;
			
			// Make sure the crop entry record exists
			ensure!(CropEntries::<T>::contains_key(&crop_entry), Error::<T>::CropEntryNotFoundForSeedValidation);
			// Make sure it hasn't been already decided
			ensure!(!CropEntryResults::<T>::contains_key(&crop_entry), Error::<T>::CropEntryAlreadyValidated);

			// Get the block number from the FRAME System module.
			let current_block = <frame_system::Module<T>>::block_number();
			
			// Store result of crop entry with its event hash
			CropEntryResults::<T>::insert(&crop_entry, (&sender, CropEntryResult::Accepted(crn.clone()), &event_hash, &current_block));

			// Store crn submitted with the crop entry approval
			CRNs::<T>::insert(&crn, (&sender, &event_hash, &crop_entry, &current_block));

			// Emit an event that the crop entry was approved
			Self::deposit_event(RawEvent::SeedApplicationApproved(event_hash, sender));
		}

		/// Rejects a crop entry if it exists and has no result already
		#[weight = 10_000]
		pub fn reject_crop_entry(origin, crop_entry: Vec<u8>, event_hash: Vec<u8>) {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://substrate.dev/docs/en/knowledgebase/runtime/origin
			let sender = ensure_signed(origin)?;
			
			// Make sure the crop entry record exists
			ensure!(CropEntries::<T>::contains_key(&crop_entry), Error::<T>::CropEntryNotFoundForSeedValidation);
			// Make sure it hasn't been already decided
			ensure!(!CropEntryResults::<T>::contains_key(&crop_entry), Error::<T>::CropEntryAlreadyValidated);

			// Get the block number from the FRAME System module.
			let current_block = <frame_system::Module<T>>::block_number();
			
			// Store result of crop entry with its event hash
			CropEntryResults::<T>::insert(&crop_entry, (&sender, CropEntryResult::Rejected, &event_hash, &current_block));

			// Emit an event that the crop entry was approved
			Self::deposit_event(RawEvent::SeedApplicationRejected(event_hash, sender));
		}

		/// Submits a BCI line against a CRN and store the event hash
		#[weight = 10_000]
		pub fn submit_bci_line(origin, crn: Vec<u8>, bci_line_id: Vec<u8>, event_hash: Vec<u8>) {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://substrate.dev/docs/en/knowledgebase/runtime/origin
			let sender = ensure_signed(origin)?;
			
			// Make sure the crn record exists
			ensure!(CRNs::<T>::contains_key(&crn), Error::<T>::CRNNotFoundForBCILine);

			// Make sure that the bci line with same Id doesn't already exists
			ensure!(!BCILines::<T>::contains_key(&bci_line_id), Error::<T>::BCILineAlreadyExists);

			// Get the block number from the FRAME System module.
			let current_block = <frame_system::Module<T>>::block_number();
			
			// Store event_hash of the entire BCI line application
			BCILines::<T>::insert(&bci_line_id, (&sender, &crn, &event_hash, &current_block));

			// Emit an event that the BCI line was submitted
			Self::deposit_event(RawEvent::BCILineSubmitted(event_hash, sender));
		}

		/// Accept a BCI line if BCI line exists and has not result already
		#[weight = 10_000]
		pub fn accept_bci_line(origin, bci_line_id: Vec<u8>, event_hash: Vec<u8>) {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://substrate.dev/docs/en/knowledgebase/runtime/origin
			let sender = ensure_signed(origin)?;
			
			// Make sure the BCI line 
			ensure!(BCILines::<T>::contains_key(&bci_line_id), Error::<T>::BCILineIdNotFoundForValidation);
			// Make sure it hasn't been already decided
			ensure!(!BCILineResults::<T>::contains_key(&bci_line_id), Error::<T>::BCILineAlreadyValidated);

			// Get the block number from the FRAME System module.
			let current_block = <frame_system::Module<T>>::block_number();
			
			// Store result against the BCI line
			BCILineResults::<T>::insert(&bci_line_id, (&sender, BCILineResult::Accepted, &event_hash, &current_block));

			// Emit an event that the BCI line was approved
			Self::deposit_event(RawEvent::BCILineApproved(event_hash, sender));
		}

		/// Reject a BCI line if BCI line exists and has not result already
		#[weight = 10_000]
		pub fn reject_bci_line(origin, bci_line_id: Vec<u8>, event_hash: Vec<u8>) {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://substrate.dev/docs/en/knowledgebase/runtime/origin
			let sender = ensure_signed(origin)?;
			
			// Make sure the BCI line 
			ensure!(BCILines::<T>::contains_key(&bci_line_id), Error::<T>::BCILineIdNotFoundForValidation);
			// Make sure it hasn't been already decided
			ensure!(!BCILineResults::<T>::contains_key(&bci_line_id), Error::<T>::BCILineAlreadyValidated);

			// Get the block number from the FRAME System module.
			let current_block = <frame_system::Module<T>>::block_number();
			
			// Store result against the BCI line
			BCILineResults::<T>::insert(&bci_line_id, (&sender, BCILineResult::Rejected, &event_hash, &current_block));

			// Emit an event that the BCI line was approved
			Self::deposit_event(RawEvent::BCILineRejected(event_hash, sender));
		}

	}
}
