#![cfg_attr(not(feature = "std"), no_std)]
/// Decentralised Pallet Implementation TrackBack Limited
mod did_operations;
mod ipfs_driver;
mod utils;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {

    use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};

    use frame_system::{
        pallet_prelude::*,
    };


    use sp_std::str;
    use sp_std::vec::Vec;
    #[allow(dead_code)]
    pub struct DID {
        did_uri: Vec<u8>,
        did_document: Vec<u8>,
        did_ref: Vec<u8>,
    }

    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_timestamp::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    /// Stores a DID document on chain
    /// Key 1 -> AccountId + DIDDocumentHash
    /// Key 2 -> Chain time
    /// Value -> DID Document(hash) + BlockNumber
    #[pallet::storage]
    #[pallet::getter(fn get_did_document)]
    pub(super) type DIDDocument<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        Vec<u8>,
        (T::Moment, Vec<u8>, T::BlockNumber, T::AccountId),
        ValueQuery,
    >;

    /// Accounts associated with a DID
    #[pallet::storage]
    #[pallet::getter(fn get_did_accounts)]
    pub(super) type DIDAccount<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, Vec<Vec<u8>>, ValueQuery>;

    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        DIDCreated(T::AccountId, Vec<u8>),
        /// Event returns DID Document hash, DID URI, Sender's AccountId
        DIDDocumentCreated(Vec<u8>, T::AccountId),

        /// DID Document revoked
        DIDDocumentRevoked(Vec<u8>, T::AccountId),
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
    }

    /// Offchain worker to support custom RPC calls to assist verifiable credentials with DIDs
    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn offchain_worker(block_number: T::BlockNumber) {

            log::info!("TrackBack OCW");
            log::info!("{:?}", block_number);
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {

        /// DID Revocation
        /// Throws DoesNotExists for a non existing DID revocation
        #[pallet::weight(0)]
        pub fn revoke_did(origin: OriginFor<T>, did_hash: Vec<u8>) -> DispatchResultWithPostInfo {

            let origin_account = ensure_signed(origin)?;

            ensure!(
                DIDDocument::<T>::contains_key(&did_hash),
                Error::<T>::DIDDoesNotExists
            );

            DIDDocument::<T>::remove(&did_hash);

            Self::deposit_event(Event::DIDDocumentRevoked(did_hash, origin_account));

            Ok(().into())
        }

        /// Updates a DID document
        #[pallet::weight(0)]
        pub fn update_did(_origin: OriginFor<T>, _did_doc: Vec<u8>) -> DispatchResultWithPostInfo {
            Ok(().into())
        }

        /// Stores a DID document
        #[pallet::weight(0)]
        pub fn insert_did_document(
            origin: OriginFor<T>,
            did_document: Vec<u8>,
            did_hash: Vec<u8>,
        ) -> DispatchResultWithPostInfo {

            let origin_account = ensure_signed(origin)?;

            let block_number = <frame_system::Module<T>>::block_number();

            let time = <pallet_timestamp::Module<T>>::get();

            ensure!(
                !DIDDocument::<T>::contains_key(&did_hash),
                Error::<T>::DIDExists
            );

            DIDDocument::<T>::insert(
                did_hash.clone(),
                (time, did_document, block_number, &origin_account),
            );

            Self::deposit_event(Event::DIDDocumentCreated(did_hash, origin_account));

            Ok(().into())
        }
    }
}
