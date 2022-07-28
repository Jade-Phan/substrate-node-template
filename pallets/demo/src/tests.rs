use frame_support::{assert_noop, assert_ok};

use crate::{Error, mock::*};

#[test]
fn create_student_successfully() {
	new_test_ext().execute_with(|| {
		assert_ok!(Demo::create_student(Origin::signed(1),b"student-name".to_vec(),24));
		assert_eq!(Demo::student_id(), 1);
	})
}

// #[test]
// fn generate_gender() {
// 	new_test_ext().execute_with(|| {
// 		let len = b"student-name".to_vec().len();
// 		let mut gender;
// 		if len % 2 == 0 {
// 			gender = Demo::Gender::Male;
// 		} else {
// 			gender = Demo::Gender::Female;
// 		}
// 		let res = Demo::gen_gender(b"student-name".to_vec()).unwrap();
// 		assert_eq!(res, gender);
// 	})
// }

#[test]
fn create_student_fail_if_age_is_lower_than_20() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			Demo::create_student(Origin::signed(1), b"student_name".to_vec(), 10),
			Error::<Test>::TooYoung
		);
	});
}
