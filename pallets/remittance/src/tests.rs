// Tests to be written here

use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};

#[test]
fn it_stores_fees() {
	new_test_ext().execute_with(|| {
		assert_ok!(RemittanceModule::set_deposit_fee(Origin::signed(1), 100));
		assert_eq!(RemittanceModule::deposit_fee(), 100);

		assert_noop!(
			RemittanceModule::set_deposit_fee(Origin::signed(1), 0),
			Error::<Test>::NoneValue
		);

		assert_noop!(
			RemittanceModule::set_deposit_fee(Origin::signed(1), 100),
			Error::<Test>::EqualValue
		);
	});
}
