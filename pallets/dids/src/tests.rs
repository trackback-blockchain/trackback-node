// This file is part of TrackBck which is released under GNU General Public License v3.0.
// See file LICENSE.md or go to https://www.gnu.org/licenses/gpl-3.0.en.html for full license details.
use crate::mock::{new_test_ext, Origin};

use rstest::*;

use crate::{mock::DIDModule, structs::DIDSignature};
use codec::Encode;
use frame_support::{
	assert_err, assert_ok,
	pallet_prelude::DispatchError,
	sp_runtime::app_crypto::{sp_core::Hasher, Pair},
};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use sp_core::{ed25519, ed25519::Pair as KeyPair, Blake2Hasher};

/// Fixture to generate a keypair, secret and a peerId
#[fixture]
pub fn key_pair() -> KeyPair {
	ed25519::Pair::generate().0
}

/// Creates a signature for a DID document
/// This performs by the Controller or the Issuer
#[fixture]
pub fn signature(key_pair: KeyPair, did_document: &'static str) -> Vec<DIDSignature> {
	// let k = ed25519::Pair::from_seed("Alice".as_bytes().to_vec());
	// public key
	let public_key = key_pair.public();

	// Converts a public key in to byte array
	let public_key_to_bytes = public_key.encode();

	// Digital Signature
	let signed = key_pair.sign(&*did_document.as_bytes().to_vec());

	let mut signatures: Vec<DIDSignature> = Vec::new();
	signatures.push(DIDSignature {
		public_key: Vec::from(public_key_to_bytes),
		proof: signed,
		active: true,
		created_time_stamp: 0,
		updated_timestamp: 0,
	});

	signatures
}

#[fixture]
pub fn did_ref() -> Option<Vec<u8>> {
	Some(r#"{QmcNYMJBhvbrH8oTo5QGNUFA5rhKpBVXHBpfiecxso7D8P}"#.as_bytes().to_vec())
}

#[fixture]
pub fn public_keys() -> Option<Vec<Vec<u8>>> {
	let mut public_keys = Vec::new();
	for _ in 0..10 {
		let pk = thread_rng().sample_iter(&Alphanumeric).take(60).collect::<Vec<_>>();
		public_keys.push(pk);
	}
	Some(public_keys)
}

#[fixture]
pub fn did_document_metadata() -> Option<Vec<u8>> {
	Some(
		r#"{
        "created": "2002-01-01T20:20:20Z",
        "updated": "2002-02-01T20:20:20Z",
        "deactivated": "2002-03-01T20:20:20Z",
        "versionId": "1",
    }"#
		.as_bytes()
		.to_vec(),
	)
}

#[fixture]
pub fn did_resolution_metadata() -> Option<Vec<u8>> {
	Some(
		r#"{
        "accept": "application/did+ld+json"
    }"#
		.as_bytes()
		.to_vec(),
	)
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

/// Vec<u8> representation of a publicKey
#[fixture]
pub fn public_key() -> Vec<u8> {
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
fn create_vc(public_key: Vec<u8>, vc_hash: Vec<u8>) {
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
		)
		.ok();

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

/// Creates a DID with Valid Signature
/// Single Controller for a DID Document
#[rstest]
fn create_did(
	did_document_metadata: Option<Vec<u8>>,
	did_resolution_metadata: Option<Vec<u8>>,
	did_document: &'static str,
	did_uri: Vec<u8>,
	did_ref: Option<Vec<u8>>,
	public_key: Vec<u8>,
	signature: Vec<DIDSignature>,
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
			signature
		));
	});
}

/// Creates a DID Document, the proof is not matching with the signed private key of the controller
#[rstest]
#[case(ed25519::Pair::generate().0,5,"DIDProofVerificationFailed")]
fn creates_a_did_with_invalid_signature(
	#[case] key_pair: KeyPair,
	#[case] error_num: u8,
	#[case] message: &'static str,
	did_document_metadata: Option<Vec<u8>>,
	did_resolution_metadata: Option<Vec<u8>>,
	did_document: &'static str,
	did_uri: Vec<u8>,
	did_ref: Option<Vec<u8>>,
	public_key: Vec<u8>,
	mut signature: Vec<DIDSignature>,
) {
	// Signed with a new Keypair
	let signed = key_pair.sign(&*did_document.as_bytes().to_vec());
	signature[0].proof = signed;

	new_test_ext().execute_with(|| {
		assert_err!(
			DIDModule::insert_did_document(
				Origin::signed(1),
				did_document.as_bytes().to_vec(),
				did_document_metadata,
				did_resolution_metadata,
				public_key,
				did_uri,
				did_ref,
				signature
			),
			DispatchError::Module { index: 1, error: error_num, message: Some(message) }
		);
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
	signature: Vec<DIDSignature>,
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
			signature.clone(),
		)
		.ok();

		assert_err!(
			DIDModule::insert_did_document(
				Origin::signed(1),
				did_document.as_bytes().to_vec(),
				did_document_metadata,
				did_resolution_metadata,
				public_key,
				did_uri.clone(),
				did_ref,
				signature
			),
			DispatchError::Module { index: 1, error: 0, message: Some("DIDExists") }
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
	signature: Vec<DIDSignature>,
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
			signature,
		)
		.ok();

		assert_ok!(DIDModule::revoke_did(Origin::signed(1), did_uri));
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
	signature: Vec<DIDSignature>,
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
			signature,
		)
		.ok();

		DIDModule::revoke_did(Origin::signed(1), did_uri.clone()).ok();
	});
}

#[rstest]
fn revoke_a_revoked_did(did_uri: Vec<u8>) {
	new_test_ext().execute_with(|| {
		assert_err!(
			DIDModule::revoke_did(Origin::signed(1), did_uri),
			DispatchError::Module { index: 1, error: 1, message: Some("DIDDoesNotExists") }
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
	signature: Vec<DIDSignature>,
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
			signature.clone(),
		)
		.ok();
		assert_ok!(DIDModule::update_did(
			Origin::signed(1),
			did_document.as_bytes().to_vec(),
			did_uri.clone(),
			did_resolution_metadata,
			did_document_metadata,
			None,
			signature
		));
	});
}

#[rstest]
fn update_non_exsited_did(
	did_document_metadata: Option<Vec<u8>>,
	did_resolution_metadata: Option<Vec<u8>>,
	did_document: &'static str,
	did_uri: Vec<u8>,
	public_key: Vec<u8>,
	did_ref: Option<Vec<u8>>,
	signature: Vec<DIDSignature>,
) {
	let non_existed_did_uri = Blake2Hasher::hash("non_existed".as_ref()).as_bytes().to_vec();
	new_test_ext().execute_with(|| {
		DIDModule::insert_did_document(
			Origin::signed(1),
			did_document.as_bytes().to_vec(),
			did_document_metadata.clone(),
			did_resolution_metadata.clone(),
			public_key,
			did_uri.clone(),
			did_ref,
			signature.clone(),
		)
		.ok();

		assert_err!(
			DIDModule::update_did(
				Origin::signed(1),
				did_document.as_bytes().to_vec(),
				non_existed_did_uri,
				did_resolution_metadata,
				did_document_metadata,
				None,
				signature
			),
			DispatchError::Module { index: 1, error: 1, message: Some("DIDDoesNotExists") }
		);
	});
}
