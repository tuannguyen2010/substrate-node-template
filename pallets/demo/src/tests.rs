use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

// #[test]
// fn it_works_for_default_value() {
// 	new_test_ext().execute_with(|| {
// 		// Dispatch a signed extrinsic.
// 		assert_ok!(TemplateModule::do_something(Origin::signed(1), 42));
// 		// Read pallet storage and assert an expected result.
// 		assert_eq!(TemplateModule::something(), Some(42));
// 	});
// }

#[test]
fn correct_error_for_none_value() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_noop!(TemplateModule::cause_error(Origin::signed(1)), Error::<Test>::NoneValue);
	});
}

// #[test]
// fn correct_error_for_age_lower_than_20() {
// 	new_test_ext().execute_with(|| {
// 		// Ensure the expected error is thrown when no value is present.
// 		assert_noop!(TemplateModule::cause_error(TemplateModule::create_student(Origin::signed(1), "testName", 19)), Error::<Test>::TooYoung);
// 	});
// }

//#[test]
// fn correct_create_student() {
// 	new_test_ext().execute_with(|| {
// 		// Ensure the expected error is thrown when no value is present.
// 		let before_create_id = TemplateModule::student_id();
// 		let after_create_id = before_create_id + 1;
// 		assert_ok!(TemplateModule::create_student(Origin::signed(1), "testName", 21));
// 		assert_eq!(TemplateModule::something(), Some(42));
// 		assert_noop!(TemplateModule::cause_error(TemplateModule::create_student(Origin::signed(1), "testName", 19)), Error::<Test>::TooYoung);
// 	});
// }