#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;
pub use sp_std::vec::Vec;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	pub use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use frame_support::traits::Randomness;

	//use sp_runtime::generic::BlockId::Number;

	#[derive(TypeInfo, Encode, Decode, Debug,Clone)]
	pub enum Gender{
		Male, Female,
	}

	#[derive(TypeInfo, Default, Encode, Decode)]
	#[scale_info(skip_type_params(T))]
	pub struct Kitty<T:Config>{
		dna: Vec<u8>,
		price : u32,
		gender: Gender,
		owner: T::AccountId
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
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn numberOfKitty)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
	pub type Kitties<T> = StorageValue<_, u8, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn kittyDetail)]
	// Key :Id, Value: Student
	pub(super) type KittyDetail<T:Config> = StorageMap<_,Blake2_128Concat,Vec<u8>, Kitty<T>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn ownership)]
	// Key :Id, Value: Student
	pub(super) type OwnerDetail<T:Config> = StorageMap<_,Blake2_128Concat,T::AccountId, Vec<Vec<u8>>, OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		CreatedKitty(Vec<u8>,T::AccountId),
	}

	#[pallet::error]
	pub enum Error<T> {
		PriceTooLow,
		AlreadyExisted,
		NoneExisted,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000+ T::DbWeight::get().writes(1))]
		pub fn create_kitty(origin:OriginFor<T>,dna: Vec<u8>,price:u32) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(price>0, Error::<T>::PriceTooLow);
			let gender = Self::kitty_gender(dna.clone())?;
			let kitty = Kitty{
				dna: dna.clone(),
				price: price,
				gender: gender,
				owner: who.clone(),
			};

			let mut current_number = Self::numberOfKitty();
			<KittyDetail<T>>::insert(&dna, kitty);

			current_number = current_number + 1;

			Kitties::<T>::put(current_number);
			let mut list_kitty = OwnerDetail::<T>::get(&who).unwrap_or_else(|| {Vec::new()});
			list_kitty.push(dna.clone());
			OwnerDetail::<T>::mutate(&who, list_kitty);

			Self::deposit_event(Event::CreatedKitty(dna,who));
			Ok(())
		}

	}
}

//helper function
impl<T> Pallet<T>{
	fn kitty_gender(dna:Vec<u8>) -> Result<Gender,Error<T>>{
		let mut result = Gender::Female;
		if dna.len() % 2 == 0 {
			result = Gender::Male
		}
		Ok(result)
	}
}
