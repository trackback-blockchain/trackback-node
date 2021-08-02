use super::*;
use crate::mock::new_test_ext;
use crate::mock::Origin;

use frame_support::{assert_ok};
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
