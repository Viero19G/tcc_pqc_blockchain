use crate::{mock::*, Error, Event, Pallet as Pqc, PqcKeys};
use frame_support::{assert_noop, assert_ok};
use frame_system::Pallet as System;
use pqc_crypto::{
	mldsa::MlDsaKeypair,
	mlkem::MlKemKeypair,
	SignatureScheme,
};

#[test]
fn register_keys_works() {
	new_test_ext().execute_with(|| {
		System::<Test>::set_block_number(1);
		let kp = MlDsaKeypair::generate();
		let kem = MlKemKeypair::generate();

		assert_ok!(Pqc::<Test>::register_keys(
			RuntimeOrigin::signed(1),
			kp.public,
			Some(kem.public)
		));

		assert!(PqcKeys::<Test>::contains_key(1));
		System::<Test>::assert_last_event(
			Event::KeysRegistered { who: 1, scheme: SignatureScheme::MlDsa65, has_kem: true }
				.into(),
		);
	});
}

#[test]
fn register_keys_twice_fails() {
	new_test_ext().execute_with(|| {
		let kp = MlDsaKeypair::generate();
		assert_ok!(Pqc::<Test>::register_keys(RuntimeOrigin::signed(1), kp.public, None));
		assert_noop!(
			Pqc::<Test>::register_keys(RuntimeOrigin::signed(1), kp.public, None),
			Error::<Test>::KeysAlreadyRegistered
		);
	});
}

#[test]
fn verify_signature_works() {
	new_test_ext().execute_with(|| {
		let kp = MlDsaKeypair::generate();
		assert_ok!(Pqc::<Test>::register_keys(RuntimeOrigin::signed(1), kp.public, None));

		let msg = b"hello entangle".to_vec();
		let sig = kp.sign(&msg);

		assert_ok!(Pqc::<Test>::verify_signature(RuntimeOrigin::signed(2), 1, msg, sig));
	});
}

#[test]
fn verify_signature_fails_with_wrong_message() {
	new_test_ext().execute_with(|| {
		let kp = MlDsaKeypair::generate();
		assert_ok!(Pqc::<Test>::register_keys(RuntimeOrigin::signed(1), kp.public, None));

		let sig = kp.sign(b"correct message");
		assert_noop!(
			Pqc::<Test>::verify_signature(
				RuntimeOrigin::signed(2),
				1,
				b"wrong message".to_vec(),
				sig
			),
			Error::<Test>::InvalidSignature
		);
	});
}

#[test]
fn remove_keys_works() {
	new_test_ext().execute_with(|| {
		let kp = MlDsaKeypair::generate();
		assert_ok!(Pqc::<Test>::register_keys(RuntimeOrigin::signed(1), kp.public, None));
		assert_ok!(Pqc::<Test>::remove_keys(RuntimeOrigin::signed(1)));
		assert!(!PqcKeys::<Test>::contains_key(1));
	});
}

#[test]
fn register_validator_keys_works() {
	new_test_ext().execute_with(|| {
		let kp = MlDsaKeypair::generate();
		let kem = MlKemKeypair::generate();

		assert_ok!(Pqc::<Test>::register_validator_keys(
			RuntimeOrigin::signed(1),
			kp.public,
			Some(kem.public)
		));

		assert!(Pqc::<Test>::validator_keys_of(&1).is_some());
	});
}
