#![cfg_attr(not(feature = "std"), no_std)]

mod did_operations;
mod ipfs_driver;
mod utils;

// Making the pallet available for other pallets
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
    use pallet_timestamp as timestamp;
    use sp_runtime::{
        offchain as oc,
        offchain::{
            storage::StorageValueRef,
            storage_lock::{BlockAndTime, StorageLock, Time},
        },
        DispatchResult,
    };

    use frame_system::{
        offchain::{
            AppCrypto, CreateSignedTransaction, SendSignedTransaction, SendUnsignedTransaction,
            SignedPayload, Signer, SigningTypes, SubmitTransaction,
        },
        pallet_prelude::*,
    };

    use sp_std::str;
    use sp_std::vec::Vec;

    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_timestamp::Config{
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn dids)]
    pub(super) type DIDs<T: Config> =
        StorageMap<_, Blake2_128Concat, Vec<u8>, (T::AccountId, T::BlockNumber), ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn did_docs)]
    pub(super) type DIDDocument<T: Config> =
        StorageMap<_, Blake2_128Concat, Vec<u8>, (T::AccountId, T::BlockNumber), ValueQuery>;

    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        DIDCreated(T::AccountId, Vec<u8>),
    }

    #[pallet::error]
    pub enum Error<T> {
        DIDExists,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn offchain_worker(block_number: T::BlockNumber) {
            log::info!("Hello World from offchain workers!");
            log::info!("{:?}", block_number);
            let request = oc::http::Request::get(
                "https://api.github.com/repos/octocat/hello-world/stats/commit_activity",
            );
        }
    }

    // Functions that are callable from outside the runtime
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        // fn offchain_worker(block_number: OriginFor<T>::BlockNumber){
        //     log::info!("Hello World from offchain workers!");
        // }

        #[pallet::weight(0)]
        pub fn revoke_did(origin: OriginFor<T>, did_key: Vec<u8>) -> DispatchResultWithPostInfo {
            Ok(().into())
        }

        #[pallet::weight(0)]
        pub fn update_did(origin: OriginFor<T>, did_doc: Vec<u8>) -> DispatchResultWithPostInfo {
            Ok(().into())
        }

        #[pallet::weight(0)]
        pub fn insert_did_document(
            origin: OriginFor<T>,
            did_document: Vec<u8>,
            did_hex: Vec<u8>,
            did_hash: Vec<u8>,
        ) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin);

            // let acc = match sender {
            //     | Ok(v) => v,
            //     | _ => Err(BadOrigin),
            // };
            // let did_str = match str::from_utf8(origin) {
            //     | Ok(v) => v,
            //     | Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
            // };

            let block_number = <frame_system::Module<T>>::block_number();
            let parent_hash = <frame_system::Module<T>>::parent_hash();
            let block_hash = <frame_system::Module<T>>::block_hash(block_number);

            // type did_key = sender +

            Ok(().into())
        }

        #[pallet::weight(0)]
        pub fn create_did(
            origin: OriginFor<T>,
            mut did_doc: Vec<u8>,
        ) -> DispatchResultWithPostInfo {
            // T::AccountId
            let sender = ensure_signed(origin)?;

            let did_str = match str::from_utf8(&*did_doc) {
                | Ok(v) => v,
                | Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
            };

            debug::info!(
                "Request sent by: {:?} \n \
                and the proof {:?} \n\
                and the DID document {:?}",
                sender,
                did_doc,
                did_str
            );
            ensure!(!DIDs::<T>::contains_key(&did_doc), Error::<T>::DIDExists);

            let current_block = <frame_system::Module<T>>::block_number();
            // Key -> Append AccountId + DID Document Hash, Value -> DID Document hash

            DIDs::<T>::insert(&did_doc, (&sender, current_block));

            Self::deposit_event(Event::DIDCreated(sender, did_doc));
            Ok(().into())
        }
    }
}
