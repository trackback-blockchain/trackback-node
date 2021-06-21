#![cfg_attr(not(feature = "std"), no_std)]

mod did_operations;
mod utils;
mod IPFSDriver;

// Making the pallet available for other pallets
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
    use frame_system::pallet_prelude::*;
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
    pub(super) type Proofs<T: Config> =
        StorageMap<_, Blake2_128Concat, Vec<u8>, (T::AccountId, T::BlockNumber), ValueQuery>;

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
        pub fn create_proof(origin: OriginFor<T>, proof: Vec<u8>) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;
            debug::info!(
                "Request sent by: {:?} and the proof {:?}",
                sender, proof
            );
            ensure!(!Proofs::<T>::contains_key(&proof), Error::<T>::ProofExists);

            let current_block = <frame_system::Module<T>>::block_number();
            // Key -> Append AccountId + DID Document Hash, Value -> DID Document hash
            //
            Proofs::<T>::insert(&proof, (&sender, current_block));

            Self::deposit_event(Event::ProofCreated(sender, proof));
            Ok(().into())
        }
    }
}
