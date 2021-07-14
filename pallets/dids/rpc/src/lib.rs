#![cfg_attr(not(feature = "std"), no_std)]
use codec::Codec;
use pallet_dids::*;

sp_api::decl_runrime_apis! {
    #[api_version(1)]
    pub trait DIDAPI {
        fn ping() -> u64
    }
}


sp_api::impl_runtime_apis!{
    impl Self::DIDAPI for Runtime{
        fn ping() -> u64 {
            1
        }
    }
}

pub const VERSION: sp_version::RuntimeVersion = sp_version::RuntimeVersion {
    spec_name: create_runtime_str!("node"),
    impl_name: create_runtime_str!("test-node"),
    authoring_version: 1,
    spec_version: 1,
    impl_version: 0,
    // Here we are exposing the runtime api versions.
    apis: RUNTIME_API_VERSIONS,
    transaction_version: 1,
};
