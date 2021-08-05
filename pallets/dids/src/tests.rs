use super::*;
use crate::mock::new_test_ext;
use crate::mock::Origin;

use frame_support::{assert_ok, assert_err};
use frame_support::pallet_prelude::DispatchError;
use crate::mock::DIDModule;

#[test]
fn create_vc() {
    let public_key = vec![
        0, 1, 217, 200, 51, 244, 152, 125, 173, 92, 30, 224, 60, 141, 221, 44, 65, 132, 45, 94,
        199, 150, 116, 108, 95, 18, 118, 246, 86, 167, 64, 132, 76,
    ];
    let hash = "Hash".as_bytes().to_vec();
    new_test_ext().execute_with(||{
        assert_ok!(
            DIDModule::create_vc_fingerprint(Origin::signed(1), public_key, hash, Some(true))
        );
    });
}

#[test]
fn create_vc_exists() {
    let public_key = vec![
        0, 1, 217, 200, 51, 244, 152, 125, 173, 92, 30, 224, 60, 141, 221, 44, 65, 132, 45, 94,
        199, 150, 116, 108, 95, 18, 118, 246, 86, 167, 64, 132, 76,
    ];
    let hash = "Hash".as_bytes().to_vec();
    new_test_ext().execute_with(||{
        DIDModule::create_vc_fingerprint(Origin::signed(1), public_key.clone(), hash.clone(), Some(true));
        assert_err!(
            DIDModule::create_vc_fingerprint(Origin::signed(1), public_key, hash, Some(true)),
            DispatchError::Module { index: 1, error: 4, message: Some("VerifiableCredentialExists") } );
    });
}

#[test]
fn create_did() {
    let public_key = vec![
        0, 1, 217, 200, 51, 244, 152, 125, 173, 92, 30, 224, 60, 141, 221, 44, 65, 132, 45, 94,
        199, 150, 116, 108, 95, 18, 118, 246, 86, 167, 64, 132, 76,
    ];
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

    let did_document_hash = "";
}
