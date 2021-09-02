use super::*;
use crate::mock::new_test_ext;
use crate::mock::Origin;

use crate::mock::DIDModule;
use frame_support::pallet_prelude::DispatchError;
use frame_support::sp_runtime::app_crypto::sp_core::Hasher;
use frame_support::{assert_err, assert_ok};
use sp_core::Blake2Hasher;

#[test]
fn create_vc() {
    let public_key = vec![
        0, 1, 217, 200, 51, 244, 152, 125, 173, 92, 30, 224, 60, 141, 221, 44, 65, 132, 45, 94,
        199, 150, 116, 108, 95, 18, 118, 246, 86, 167, 64, 132, 76,
    ];

    let hash = "Hash".as_bytes().to_vec();

    new_test_ext().execute_with(|| {
        let res = DIDModule::create_vc_fingerprint(
            Origin::signed(1),
            public_key,
            hash,
            Some(true)
        );
        let p = 0;
        // assert_ok!(DIDModule::create_vc_fingerprint(
        //     Origin::signed(1),
        //     public_key,
        //     hash,
        //     Some(true)
        // ));
        // assert_ok!(DIDModule::create_vc_fingerprint(
        //     Origin::signed(1),
        //     public_key,
        //     hash,
        //     Some(true)
        // ));
    });
}

#[test]
fn create_vc_exists() {
    let public_key = vec![
        0, 1, 217, 200, 51, 244, 152, 125, 173, 92, 30, 224, 60, 141, 221, 44, 65, 132, 45, 94,
        199, 150, 116, 108, 95, 18, 118, 246, 86, 167, 64, 132, 76,
    ];

    let hash = "Hash".as_bytes().to_vec();

    new_test_ext().execute_with(|| {
        DIDModule::create_vc_fingerprint(
            Origin::signed(1),
            public_key.clone(),
            hash.clone(),
            Some(true),
        ).ok();

        assert_err!(
            DIDModule::create_vc_fingerprint(Origin::signed(1), public_key, hash, Some(true)),
            DispatchError::Module {
                index: 1,
                error: 4,
                message: Some("VerifiableCredentialExists")
            }
        );
    });
}

#[test]
fn create_did() {
    let did_document = r#"{
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
    }"#;

    let did_hash = Blake2Hasher::hash(did_document.as_ref())
        .as_bytes()
        .to_vec();

    new_test_ext().execute_with(|| {
        assert_ok!(DIDModule::insert_did_document(
            Origin::signed(1),
            did_document.as_bytes().to_vec(),
            did_hash
        ));
    });
}

#[test]
fn create_an_existing_did() {
    let did_document = r#"{
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
    }"#;

    let did_hash = Blake2Hasher::hash(did_document.as_ref())
        .as_bytes()
        .to_vec();

    new_test_ext().execute_with(|| {
        DIDModule::insert_did_document(
            Origin::signed(1),
            did_document.clone().as_bytes().to_vec(),
            did_hash.clone(),
        ).ok();

        assert_err!(
            DIDModule::insert_did_document(
                Origin::signed(1),
                did_document.as_bytes().to_vec(),
                did_hash
            ),
            DispatchError::Module {
                index: 1,
                error: 0,
                message: Some("DIDExists")
            }
        );
    });
}

#[test]
fn revoke_a_did() {
    let did_document = r#"{
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
    }"#;

    let did_hash = Blake2Hasher::hash(did_document.as_ref())
        .as_bytes()
        .to_vec();

    new_test_ext().execute_with(|| {
        DIDModule::insert_did_document(
            Origin::signed(1),
            did_document.clone().as_bytes().to_vec(),
            did_hash.clone(),
        ).ok();

        assert_ok!(
            DIDModule::revoke_did(
                Origin::signed(1),
                did_hash
            )
        );
    });
}

#[test]
fn revoke_non_existing_did() {
    let did_document = r#"{
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
    }"#;

    let did_hash = Blake2Hasher::hash(did_document.as_ref())
        .as_bytes()
        .to_vec();

    new_test_ext().execute_with(|| {
        DIDModule::insert_did_document(
            Origin::signed(1),
            did_document.clone().as_bytes().to_vec(),
            did_hash.clone(),
        ).ok();
        DIDModule::revoke_did(
            Origin::signed(1),
            did_hash.clone()
        ).ok();
        assert_err!(
            DIDModule::revoke_did(
                Origin::signed(1),
                did_hash
            ),
            DispatchError::Module {
                index: 1,
                error: 1,
                message: Some("DIDDoesNotExists")
            }
        );
    });
}

#[test]
fn revoke_a_revoked_did() {
    let did_document = r#"{
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
    }"#;

    let did_hash = Blake2Hasher::hash(did_document.as_ref())
        .as_bytes()
        .to_vec();

    new_test_ext().execute_with(|| {
        assert_err!(
            DIDModule::revoke_did(
                Origin::signed(1),
                did_hash
            ),
            DispatchError::Module {
                index: 1,
                error: 1,
                message: Some("DIDDoesNotExists")
            }
        );
    });
}
