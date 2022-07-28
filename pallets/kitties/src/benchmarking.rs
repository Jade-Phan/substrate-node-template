use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_benchmarking::vec;
use frame_system::RawOrigin;

#[allow(unused)]
use crate::Pallet as Kitties;

use super::*;

benchmarks! {
	// tên của benchmark
	create_kitty {
		// khởi tạo các tham số cho extrinsic benchmark
		let price = 123;

		let caller: T::AccountId = whitelisted_caller();
	}: create_kitty(RawOrigin::Signed(caller), price)

	// kiểm tra lại trạng thái storage khi thực hiện extrinsic xem đúng chưa
	verify {
		assert_eq!(Kitties::<T>::quantity(), 1);
	}

	// thực hiện benchmark với mock runtime, storage ban đầu.
	impl_benchmark_test_suite!(Kitties, crate::mock::new_test_ext(), crate::mock::Test);
}
