//! TrackBack limited
//! Decentralised Identifiers Pallet Implementation TrackBack Limited
//! Features in v0.0.1
//! * Creates a decentralised identifier
//! * Revokes a decentralised identifier
//! * Checks an existence of a decentralised identifier
//! * Creates a finger print of a verifiable credential
//! * Checks an existence of a verifiable credential
//!
//! # Storage
//! ## DIDDocument
//! Stores a DID document on chain
//! * Key 1 -> AccountId + DIDDocumentHash
//! * Value -> DID structure
//!
//! # Example
//! ```no_run
//! use frame_support::pallet_prelude::{StorageMap};
//! use frame_support::Blake2_128Concat;
//! use pallet_dids::Config;
//! use frame_support::pallet;
//! #[pallet::storage]
//! #[pallet::getter(fn get_did_document)]
//! pub(super) type DIDDocument<T: Config> = StorageMap<_, Blake2_128Concat, Vec<u8>, DID<T>>;
//! ```
//!
//! ## DIDDocument
//! Keeps trails of DID documents by combination of the  Issuer/Controller Account and a unique value
//! * Key 1 -> AccountId + DIDDocumentHash
//! * Value -> DID structure
//!
//! ```no_run
//! use frame_support::pallet_prelude::StorageMap;
//! use frame_support::Blake2_128Concat;
//! use pallet_dids::Config;
//! use frame_support::pallet;
//! #[pallet::storage]
//! #[pallet::getter(fn get_did_accounts)]
//! pub(super) type DIDs<T: Config> =
//!     StorageMap<_, Blake2_128Concat, (Vec<u8>, T::AccountId), Vec<DID<T>>>;
//! ```
//! ## VerifiableCredential
//! * Stores a fingerprint of a verifiableCredential
//! * TODO: Will move to a separate pallet at MVP stage
//! ```no_run
//! use frame_support::pallet_prelude::StorageMap;
//! use frame_support::Blake2_128Concat;
//! use pallet_dids::Config;
//! use frame_support::pallet;
//! #[pallet::storage]
//! #[pallet::getter(fn get_verifiable_credential_hash)]
//! pub(super) type VC<T: Config> =
//!     StorageMap<_, Blake2_128Concat, Vec<u8>, VerifiableCredential<T>>;
//! ```

#![cfg_attr(not(feature = "std"), no_std)]

mod ipfs_driver;
mod structs;
mod utils;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {

    use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};

    use frame_system::pallet_prelude::*;

    use crate::structs::{VerifiableCredential, DID};
    use sp_std::str;
    use sp_std::vec::Vec;
    #[allow(dead_code)]
    use frame_support::traits::UnixTime;


    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_timestamp::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type TimeProvider: UnixTime;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    /// Stores a DID document on chain
    /// Key 1 -> AccountId + DIDDocumentHash
    /// Value -> DID structure
    #[pallet::storage]
    #[pallet::getter(fn get_did_document)]
    pub(super) type DIDDocument<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        Vec<u8>,
        DID<T>
    >;

    /// Accounts associated with a DID
    #[pallet::storage]
    #[pallet::getter(fn get_did_accounts)]
    pub(super) type DIDs<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        // public key + Controller Account
        (Vec<u8>, T::AccountId),
        Vec<DID<T>>,
    >;

    /// Stores a verifiable credential finger print
    #[pallet::storage]
    #[pallet::getter(fn get_verifiable_credential_hash)]
    pub(super) type VC<T: Config> =
        StorageMap<_, Blake2_128Concat, Vec<u8>, VerifiableCredential<T>>;

    /// # Pallet Events
    /// * DIDDocumentCreated
    /// - Returns the created    DID hash and the AccountId `(Vec<u8>, T::AccountId)`
    /// * DIDDocumentRevoked
    /// - Triggers when a DID revoked by a controller or a delegated authority `(Vec<u8>, T::AccountId)`
    /// * VerifiableCredentialFingerprintCreated
    /// - Returns Holder's Account, Issuer/Controller's Account and the verifiable credential hash
    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Event returns DID Document hash, DID URI, Sender's AccountId
        DIDDocumentCreated(Vec<u8>, T::AccountId),

        /// DID Document revoked
        DIDDocumentRevoked(Vec<u8>, T::AccountId),

        /// Verifiable credential fingerprint created
        VerifiableCredentialFingerPrintCreated(Vec<u8>, T::AccountId, Vec<u8>),

        /// DID Document updated
        DIDDocumentUpdated(Vec<u8>)
    }

    #[pallet::error]
    pub enum Error<T> {
        /// DID Document exists
        DIDExists,

        /// DID Document does not exists
        DIDDoesNotExists,

        /// Disputed DID Document
        DIDDispute,

        /// DID Document locked
        DIDLocked,

        /// Verifiable credential exists
        VerifiableCredentialExists,
    }

    /// Offchain worker to support custom RPC calls to assist verifiable credentials with DIDs
    /// TODO: Functionality will be implemented in MVP stage
    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn offchain_worker(block_number: T::BlockNumber) {
            log::info!("TrackBack OCW");
            log::info!("{:?}", block_number);
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Stores hashes of verifiable credentials issued per issuer's account (aka controller)
        /// Does not store any verifiable credential or user centric data on-chain store
        #[pallet::weight(0)]
        pub fn create_vc_fingerprint(
            origin: OriginFor<T>,
            public_key: Vec<u8>,
            vc_hash: Vec<u8>,
            active: Option<bool>,
        ) -> DispatchResultWithPostInfo {
            let origin_account = ensure_signed(origin)?;

            // Ensures a verifiable credential finger print does not exist
            ensure!(
                !VC::<T>::contains_key(&vc_hash),
                Error::<T>::VerifiableCredentialExists
            );

            let _account =
                T::AccountId::decode(&mut &public_key[..]).map_err(|_| "could not convert")?;
            let time = T::TimeProvider::now().as_secs();

            VC::<T>::insert(
                vc_hash.clone(),
                VerifiableCredential {
                    account_id: Some(_account),
                    public_key: public_key.clone(),
                    block_time_stamp: time,
                    active,
                },
            );
            Self::deposit_event(Event::VerifiableCredentialFingerPrintCreated(
                vc_hash,
                origin_account,
                public_key,
            ));
            Ok(().into())
        }

        /// DID Revocation
        /// Throws DoesNotExists for a non existing DID revocation
        #[pallet::weight(0)]
        pub fn revoke_did(origin: OriginFor<T>, did_uri: Vec<u8>) -> DispatchResultWithPostInfo {
            let origin_account = ensure_signed(origin)?;

            ensure!(
                DIDDocument::<T>::contains_key(&did_uri),
                Error::<T>::DIDDoesNotExists
            );

            DIDDocument::<T>::remove(&did_uri);

            Self::deposit_event(Event::DIDDocumentRevoked(did_uri, origin_account));

            Ok(().into())
        }

        /// Updates a DID document
        #[pallet::weight(0)]
        pub fn update_did(
            origin: OriginFor<T>,
            did_uri: Vec<u8>,
            did_resolution_metadata: Option<Vec<u8>>,
            did_document_metadata: Option<Vec<u8>>,
            did_ref: Option<Vec<u8>>,
            public_keys: Option<Vec<Vec<u8>>>,
        ) -> DispatchResultWithPostInfo {
            let _origin_account = ensure_signed(origin)?;

            // Checks the DID document contains the section `Capability Delegation`
            // Reference :- https://www.w3.org/TR/did-core/#capability-delegation

            // let doc = str::from_utf8(&did_document).unwrap();
            // let sanitised = doc.replace("\n", "").replace(" ", "");

            DIDDocument::<T>::mutate(did_uri.clone(), |did| {
                match did {
                    None => {
                        return Err(Error::<T>::DIDDoesNotExists)
                    }
                    Some(d) => {
                        d.did_resolution_metadata = did_resolution_metadata;
                        d.did_document_metadata = did_document_metadata;
                        d.public_keys = public_keys;
                        d.did_ref = did_ref;
                        d.updated_timestamp = T::TimeProvider::now().as_secs();
                        Ok(())
                    }
                }
            })?;
            Self::deposit_event(Event::DIDDocumentUpdated(did_uri));

            Ok(().into())
        }

        /// Stores a DID document
        #[pallet::weight(0)]
        pub fn insert_did_document(
            origin: OriginFor<T>,
            did_document: Vec<u8>,
            did_document_metadata: Option<Vec<u8>>,
            did_resolution_metadata: Option<Vec<u8>>,
            sender_account_id: Vec<u8>,
            did_uri: Vec<u8>,
            did_ref: Option<Vec<u8>>,
            public_keys: Option<Vec<Vec<u8>>>
        ) -> DispatchResultWithPostInfo {
            let origin_account = ensure_signed(origin)?;

            let block_number = <frame_system::Module<T>>::block_number();

            let time = T::TimeProvider::now().as_secs();

            ensure!(
                !DIDDocument::<T>::contains_key(&did_uri),
                Error::<T>::DIDExists
            );

            // Checks the DID document contains the section `Capability Delegation`
            // Reference :- https://www.w3.org/TR/did-core/#capability-delegation

            let doc = str::from_utf8(&did_document).unwrap();
            let _sanitised = doc.replace("\n", "").replace(" ", "");


            DIDDocument::<T>::insert(
                did_uri.clone(),
                DID {
                    did_document_metadata,
                    did_resolution_metadata,
                    block_number,
                    block_time_stamp: time.clone(),
                    updated_timestamp: time,
                    did_ref,
                    sender_account_id,
                    public_keys,
                },
            );

            Self::deposit_event(Event::DIDDocumentCreated(did_uri, origin_account));

            Ok(().into())
        }
    }
}
