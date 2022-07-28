use frame_benchmarking::{account, benchmarks, whitelisted_caller};
use frame_benchmarking::vec;
use frame_system::RawOrigin;

#[allow(unused)]
use crate::Pallet as KittiesModule;

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
		assert_eq!(KittiesModule::<T>::quantity(), 1);
	}

	transfer{
		let alice:T::AccountId = account("seller",0,0);
		let bob : T::AccountId = account("buyer",1,1);
		let price =123;
		KittiesModule::<T>::create_kitty(RawOrigin::Signed(alice.clone()).into(),price);
		let kitty_dna = &OwnerDetail::<T>::get(&alice)[0];
	}: transfer_kitty(RawOrigin::Signed(alice.clone()),kitty_dna.to_vec(),bob.clone())
	verify{
		assert_eq!(OwnerDetail::<T>::get(&bob).len(), 1);
	}

	// thực hiện benchmark với mock runtime, storage ban đầu.
	impl_benchmark_test_suite!(KittiesModule, crate::mock::new_test_ext(), crate::mock::Test);
}
