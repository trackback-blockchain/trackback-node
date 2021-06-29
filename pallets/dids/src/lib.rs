#![cfg_attr(not(feature = "std"), no_std)]

mod did_operations;
mod utils;
mod ipfs_driver;

// Making the pallet available for other pallets
pub use pallet::*;
// use pallet_timestamp as timestamp;
#[frame_support::pallet]
pub mod pallet {
    use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};

    use frame_system::{pallet_prelude::*, offchain::{
        AppCrypto, CreateSignedTransaction, SendSignedTransaction,
        SendUnsignedTransaction, SignedPayload, Signer, SigningTypes, SubmitTransaction
    }};

    // use frame_system::pallet_prelude::*;
    // File hash retrieves as a vector
    // TODO:This should be a CBOR.
    use sp_std::vec::Vec;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn dids)]
    pub(super) type DIDs<T: Config> =
        StorageMap<_, Blake2_128Concat, Vec<u8>, (T::AccountId, T::BlockNumber), ValueQuery>;

    // #[pallet::pallet]

    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        ProofCreated(T::AccountId, Vec<u8>),
    }

    #[pallet::error]
    pub enum Error<T> {
        ProofExists,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {

        #[pallet::weight(0)]
        pub fn ipfs_connector(){
            todo!()
        }

        #[pallet::weight(0)]
        pub fn revoke_did(origin:OriginFor<T>, did_key: Vec<u8>) -> DispatchResultWithPostInfo {
            Ok(().into())
        }

        #[pallet::weight(0)]
        pub fn update_did(origin:OriginFor<T>, did_doc: Vec<u8>) -> DispatchResultWithPostInfo {
            Ok(().into())
        }

        #[pallet::weight(0)]
        pub fn create_did(origin: OriginFor<T>, did_doc: Vec<u8>) -> DispatchResultWithPostInfo {
            // T::AccountId
            let sender = ensure_signed(origin)?;
            // T::AccountId::decode(sender.0).unwrap_or_default();
            // let p = T::AccountId::decode(&mut sender.clone()).unwrap_or_default();
            debug::info!(
                "Request sent by: {:?} and the proof {:?}",
                sender, did_doc
            );
            ensure!(!DIDs::<T>::contains_key(&did_doc), Error::<T>::ProofExists);

            let current_block = <frame_system::Module<T>>::block_number();
            // Key -> Append AccountId + DID Document Hash, Value -> DID Document hash
            //
            DIDs::<T>::insert(&did_doc, (&sender, current_block));

            Self::deposit_event(Event::ProofCreated(sender, did_doc));
            Ok(().into())
        }
    }
}
