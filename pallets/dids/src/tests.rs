use crate::mock::new_test_ext;
use crate::mock::Origin;

use rstest::*;

use crate::mock::DIDModule;
use frame_support::pallet_prelude::DispatchError;
use frame_support::sp_runtime::app_crypto::sp_core::Hasher;
use frame_support::{assert_err, assert_ok};
use sp_core::Blake2Hasher;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;


#[fixture]
pub fn did_ref() -> Option<Vec<u8>> {
    Some(r#"{QmcNYMJBhvbrH8oTo5QGNUFA5rhKpBVXHBpfiecxso7D8P}"#.as_bytes().to_vec())
}

#[fixture]
pub fn public_keys() -> Option<Vec<Vec<u8>>> {
    let mut public_keys = Vec::new();
    for _ in 0..10 {
        let pk = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(60)
            .collect::<Vec<_>>();
        public_keys.push(pk);
    }
    Some(public_keys)
}


#[fixture]
pub fn did_document_metadata() -> Option<Vec<u8>> {
    Some(r#"{
        "created": "2002-01-01T20:20:20Z",
        "updated": "2002-02-01T20:20:20Z",
        "deactivated": "2002-03-01T20:20:20Z",
        "versionId": "1",
    }"#.as_bytes().to_vec())
}

#[fixture]
pub fn did_resolution_metadata() -> Option<Vec<u8>> {
    Some(r#"{
        "accept": "application/did+ld+json"
    }"#.as_bytes().to_vec())
}

#[fixture]
pub fn did_document() -> &'static str {
    r#"{
      "@context": [
        "https://www.w3.org/ns/did/v1",
        "https://w3id.org/security/suites/ed25519-2020/v1"
      ]
      "id": "did:trackback.dev:123456789abcdefghi",
      "authentication": [{
        "id": "did:trackback.dev:123456789abcdefghi#keys-1",
        "type": "Ed25519VerificationKey2020",
        "controller": "did:trackback.dev:123456789abcdefghi",
        "publicKeyMultibase": "zH3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV"
      }]
    }"#
}

#[fixture]
pub fn did_uri(did_document: &'static str) -> Vec<u8> {
    Blake2Hasher::hash(did_document.as_ref()).as_bytes().to_vec()
}

#[fixture]
pub fn public_key() ->Vec<u8> {
    vec![
        0, 1, 217, 200, 51, 244, 152, 125, 173, 92, 30, 224, 60, 141, 221, 44, 65, 132, 45, 94,
        199, 150, 116, 108, 95, 18, 118, 246, 86, 167, 64, 132, 76,
    ]
}

#[fixture]
pub fn vc_hash() -> Vec<u8> {
    "Hash".as_bytes().to_vec()
}

#[rstest]
fn create_vc(public_key: Vec<u8>,vc_hash: Vec<u8>) {

    new_test_ext().execute_with(|| {
        assert_ok!(DIDModule::create_vc_fingerprint(
            Origin::signed(1),
            public_key,
            vc_hash,
            Some(true)
        ));
    });
}

#[rstest]
fn create_vc_exists(public_key: Vec<u8>, vc_hash: Vec<u8>) {

    new_test_ext().execute_with(|| {
        DIDModule::create_vc_fingerprint(
            Origin::signed(1),
            public_key.clone(),
            vc_hash.clone(),
            Some(true),
        ).ok();

        assert_err!(
            DIDModule::create_vc_fingerprint(Origin::signed(1), public_key, vc_hash, Some(true)),
            DispatchError::Module {
                index: 1,
                error: 4,
                message: Some("VerifiableCredentialExists")
            }
        );
    });
}

#[rstest]
fn create_did(
    did_document_metadata: Option<Vec<u8>>,
    did_resolution_metadata: Option<Vec<u8>>,
    did_document: &'static str,
    did_uri: Vec<u8>,
    did_ref: Option<Vec<u8>>,
    public_key: Vec<u8>,
    public_keys: Option<Vec<Vec<u8>>>
) {

    new_test_ext().execute_with(|| {
        assert_ok!(DIDModule::insert_did_document(
            Origin::signed(1),
            did_document.as_bytes().to_vec(),
            did_document_metadata,
            did_resolution_metadata,
            public_key,
            did_uri,
            did_ref,
            public_keys
        ));
    });
}

#[rstest]
fn create_an_existing_did(
    did_document_metadata: Option<Vec<u8>>,
    did_resolution_metadata: Option<Vec<u8>>,
    did_document: &'static str,
    did_uri: Vec<u8>,
    did_ref: Option<Vec<u8>>,
    public_key: Vec<u8>,
    public_keys: Option<Vec<Vec<u8>>>
) {

    new_test_ext().execute_with(|| {
        DIDModule::insert_did_document(
            Origin::signed(1),
            did_document.as_bytes().to_vec(),
            did_document_metadata.clone(),
            did_resolution_metadata.clone(),
            public_key.clone(),
            did_uri.clone(),
            did_ref.clone(),
            public_keys.clone()
        ).ok();

        assert_err!(
            DIDModule::insert_did_document(
                Origin::signed(1),
                did_document.as_bytes().to_vec(),
                did_document_metadata,
                did_resolution_metadata,
                public_key,
                did_uri.clone(),
                did_ref,
                public_keys.clone()
            ),
            DispatchError::Module {
                index: 1,
                error: 0,
                message: Some("DIDExists")
            }
        );
    });
}

#[rstest]
fn revoke_a_did(
    did_document_metadata: Option<Vec<u8>>,
    did_resolution_metadata: Option<Vec<u8>>,
    did_document: &'static str,
    did_uri: Vec<u8>,
    public_key: Vec<u8>,
    did_ref: Option<Vec<u8>>,
    public_keys: Option<Vec<Vec<u8>>>
) {

    new_test_ext().execute_with(|| {
        DIDModule::insert_did_document(
            Origin::signed(1),
            did_document.as_bytes().to_vec(),
            did_document_metadata.clone(),
            did_resolution_metadata.clone(),
            public_key,
            did_uri.clone(),
            did_ref,
            public_keys
        ).ok();

        assert_ok!(
            DIDModule::revoke_did(
                Origin::signed(1),
                did_uri
            )
        );
    });
}

#[rstest]
fn revoke_non_existing_did(
    did_document_metadata: Option<Vec<u8>>,
    did_resolution_metadata: Option<Vec<u8>>,
    did_document: &'static str,
    did_uri: Vec<u8>,
    public_key: Vec<u8>,
    did_ref: Option<Vec<u8>>,
    public_keys: Option<Vec<Vec<u8>>>
) {

    new_test_ext().execute_with(|| {
        DIDModule::insert_did_document(
            Origin::signed(1),
            did_document.as_bytes().to_vec(),
            did_document_metadata.clone(),
            did_resolution_metadata.clone(),
            public_key,
            did_uri.clone(),
            did_ref,
            public_keys
        ).ok();

        DIDModule::revoke_did(
            Origin::signed(1),
            did_uri.clone()
        ).ok();

    });
}

#[rstest]
fn revoke_a_revoked_did(did_uri: Vec<u8>)
{
    new_test_ext().execute_with(|| {
        assert_err!(
            DIDModule::revoke_did(
                Origin::signed(1),
                did_uri
            ),
            DispatchError::Module {
                index: 1,
                error: 1,
                message: Some("DIDDoesNotExists")
            }
        );
    });
}

#[rstest]
fn update_did(
    did_document_metadata: Option<Vec<u8>>,
    did_resolution_metadata: Option<Vec<u8>>,
    did_document: &'static str,
    did_uri: Vec<u8>,
    public_key: Vec<u8>,
    did_ref: Option<Vec<u8>>,
    public_keys: Option<Vec<Vec<u8>>>
) {

    new_test_ext().execute_with(|| {
        DIDModule::insert_did_document(
            Origin::signed(1),
            did_document.as_bytes().to_vec(),
            did_document_metadata.clone(),
            did_resolution_metadata.clone(),
            public_key,
            did_uri.clone(),
            did_ref.clone(),
            public_keys
        ).ok();
        assert_ok!(
            DIDModule::update_did(
                Origin::signed(1),
                did_uri.clone(),
                did_resolution_metadata,
                did_document_metadata,
                None,
                None
                )
            );
    }
    );
}

#[rstest]
fn update_non_exsited_did(
    did_document_metadata: Option<Vec<u8>>,
    did_resolution_metadata: Option<Vec<u8>>,
    did_document: &'static str,
    did_uri: Vec<u8>,
    public_key: Vec<u8>,
    did_ref: Option<Vec<u8>>,
    public_keys: Option<Vec<Vec<u8>>>
) {
    let non_existed_did_uri =Blake2Hasher::hash("non_existed".as_ref()).as_bytes().to_vec();
    new_test_ext().execute_with(|| {
        DIDModule::insert_did_document(
            Origin::signed(1),
            did_document.as_bytes().to_vec(),
            did_document_metadata.clone(),
            did_resolution_metadata.clone(),
            public_key,
            did_uri.clone(),
            did_ref,
            public_keys
        ).ok();

        assert_err!(
            DIDModule::update_did(
                Origin::signed(1),
                non_existed_did_uri,
                did_resolution_metadata,
                did_document_metadata,
                None,
                None,
                ),
            DispatchError::Module {
                index: 1,
                error: 1,
                message: Some("DIDDoesNotExists")
            }
            );
    }
    );
}
