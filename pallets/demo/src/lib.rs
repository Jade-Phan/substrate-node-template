#![cfg_attr(not(feature = "std"), no_std)]

pub use sp_std::vec::Vec;

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::dispatch::fmt;
	use frame_support::log;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	pub use super::*;

	//use sp_runtime::generic::BlockId::Number;
	pub type Id = u32;

	#[derive(TypeInfo, Encode, Decode, Debug, Clone)]
	pub enum Gender {
		Male,
		Female,
	}

	#[derive(TypeInfo, Default, Encode, Decode)]
	#[scale_info(skip_type_params(T))]
	pub struct Student<T: Config> {
		name: Vec<u8>,
		age: u8,
		gender: Gender,
		account: T::AccountId,
	}

	impl<T: Config> fmt::Debug for Student<T> {
		fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
			f.debug_struct("Student")
				.field("name", &self.name)
				.field("age", &self.age)
				.field("gender", &self.gender)
				.field("account", &self.account)
				.finish()
		}
	}

	impl Default for Gender {
		fn default() -> Self {
			Gender::Female
		}
	}

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub (super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn student_id)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
	pub type StudentIds<T> = StorageValue<_, Id, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn students)]
	// Key :Id, Value: Student
	pub(super) type Students<T: Config> =
	StorageMap<_, Blake2_128Concat, Id, Student<T>, OptionQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub (super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		CreatedStudent(Vec<u8>, u8),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		TooYoung,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn create_student(origin: OriginFor<T>, name: Vec<u8>, age: u8) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(age > 20, Error::<T>::TooYoung);
			let gender = Self::gen_gender(name.clone())?;
			let student = Student { name: name.clone(), age, gender: gender.clone(), account: who };
			// let current_id = Self::student_id();
			// let current_id = StudentIds::<T>::get();
			let mut current_id = <StudentIds<T>>::get();
			log::info!("Current id: {}", current_id);
			log::info!("Gender: {:?}", gender);
			log::info!("Student: {:?}", student);
			// Students::<T>::insert(current_id, student);
			<Students<T>>::insert(current_id, student);

			current_id = current_id + 1;

			StudentIds::<T>::put(current_id);
			Self::deposit_event(Event::CreatedStudent(name, age));
			Ok(())
		}
	}
}

//helper function
impl<T> Pallet<T> {
	fn gen_gender(name: Vec<u8>) -> Result<Gender, Error<T>> {
		let mut result = Gender::Female;
		if name.len() % 2 == 0 {
			result = Gender::Male
		}
		Ok(result)
	}
}
