// Tests to be written here

use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};

#[test]
fn it_stores_fees() {
	new_test_ext().execute_with(|| {
		assert_ok!(RemittanceModule::set_deposit_fee(Origin::signed(1), 100));
		assert_eq!(RemittanceModule::deposit_fee(), 100);
	});
}

// #[test]
// fn correct_error_for_none_value() {
// 	new_test_ext().execute_with(|| {
// 		// Ensure the correct error is thrown on None value
// 		assert_noop!(
// 			RemittanceModule::cause_error(Origin::signed(1)),
// 			Error::<Test>::NoneValue
// 		);
// 	});
// }
