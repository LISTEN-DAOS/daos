use frame_benchmarking::{account, benchmarks, benchmarks_instance, impl_benchmark_test_suite, whitelisted_caller};
use crate::Pallet as Dao;
use crate::Config;
use frame_system::RawOrigin as SystemOrigin;
use primitives::AccountIdConversion;
use super::*;
fn get_alice<T: Config>() -> T::AccountId {
	account("alice", 1, 1)
}

fn get_dao_account<T: Config>(second_id: T::SecondId) -> T::AccountId {
	second_id.into_account()
}

fn creat_dao<T: Config>() -> (T::DaoId, T::SecondId) {
	let alice = get_alice::<T>();
	let dao_id = T::DaoId::default();
	let second_id: T::SecondId = Default::default();
	assert!(Dao::<T>::create_dao(SystemOrigin::Signed(alice).into(), dao_id, second_id.clone()).is_ok());
	(dao_id, second_id)
}


benchmarks! {
	create_dao {
		let alice = get_alice::<T>();
		let dao_id = T::DaoId::default();
		let second_id = Default::default();
	}:_(SystemOrigin::Signed(alice), dao_id, second_id)
	verify {
		assert!(Dao::<T>::daos(dao_id).is_some());
	}

	dao_remark {
		let (dao_id, second_id) = creat_dao::<T>();
		let dao_account = get_dao_account::<T>(second_id);
		let remark = vec![1; 50];
	}:_(SystemOrigin::Signed(dao_account), dao_id, remark)
}
