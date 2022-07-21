#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::pallet_prelude::*;
use frame_support::traits::Currency;
use frame_support::traits::UnixTime;
use frame_system::pallet_prelude::*;
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

type BalanceOf<T> = <<T as Config>::KittyCurrency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

#[frame_support::pallet]
pub mod pallet {
	pub use super::*;

	#[derive(TypeInfo, Encode, Decode, Debug, Clone)]
	pub enum Gender {
		Male,
		Female,
	}

	#[derive(TypeInfo, Default, Encode, Decode)]
	#[scale_info(skip_type_params(T))]
	pub struct Kitty<T: Config> {
		dna: Vec<u8>,
		price: BalanceOf<T>,
		gender: Gender,
		owner: T::AccountId,
		created_date: u64,
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
		type KittyCurrency: Currency<Self::AccountId>;
		type Timestamp: UnixTime;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub (super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn quantity)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
	pub type Kitties<T> = StorageValue<_, u8, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn kitty_detail)]
	// Key :Id, Value: Student
	pub(super) type KittyDetail<T: Config> =
	StorageMap<_, Blake2_128Concat, Vec<u8>, Kitty<T>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn ownership)]
	// Key :Id, Value: Student
	pub(super) type OwnerDetail<T: Config> =
	StorageMap<_, Blake2_128Concat, T::AccountId, Vec<Vec<u8>>, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub (super) fn deposit_event)]
	pub enum Event<T: Config> {
		CreatedKitty(Vec<u8>, T::AccountId, u64),
		TransferKitty(T::AccountId, T::AccountId, Vec<u8>),
	}

	#[pallet::error]
	pub enum Error<T> {
		PriceTooLow,
		AlreadyExisted,
		NoneExisted,
		WrongReceiver,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn create_kitty(origin: OriginFor<T>, dna: Vec<u8>, price: u32) -> DispatchResult {
			let who = ensure_signed(origin)?;
			log::info!("total balance:{:?}", T::KittyCurrency::total_balance(&who));
			ensure!(price > 0, Error::<T>::PriceTooLow);
			ensure!(!KittyDetail::<T>::contains_key(&dna), Error::<T>::AlreadyExisted);
			let gender = Self::kitty_gender(dna.clone())?;
			let timestamp = T::Timestamp::now();
			let kitty = Kitty { dna: dna.clone(), price: price.into(), gender: gender, owner: who.clone(), created_date: timestamp.as_secs() };

			let mut current_number = Self::quantity();
			<KittyDetail<T>>::insert(&dna, kitty);

			current_number += 1;

			Kitties::<T>::put(current_number);

			// use Value Query
			OwnerDetail::<T>::mutate(&who, |list_kitty| list_kitty.push(dna.clone()));

			Self::deposit_event(Event::CreatedKitty(dna, who, timestamp.as_secs()));
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn transfer_kitty(
			origin: OriginFor<T>,
			dna: Vec<u8>,
			to: T::AccountId,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(to != who, "Cannot transfer to owner");
			ensure!(KittyDetail::<T>::contains_key(&dna), Error::<T>::NoneExisted);
			let sender_kitties = OwnerDetail::<T>::get(&who);
			let mut index: i8 = -1;
			for i in 0..sender_kitties.len() {
				if sender_kitties[i] == dna {
					index = i.try_into().unwrap();
				}
			}
			ensure!(index != -1, Error::<T>::NoneExisted);
			// remove dna of kitty from the old owner's list
			OwnerDetail::<T>::mutate(&who, |list_kitty| {
				list_kitty.swap_remove(index.try_into().unwrap())
			});

			// insert dna of new kitty to the new owner's list
			OwnerDetail::<T>::mutate(&to, |list_kitty| list_kitty.push(dna.clone()));
			Self::deposit_event(Event::TransferKitty(who, to, dna));
			Ok(())
		}
	}
}

//helper function
impl<T: Config> Pallet<T> {
	fn kitty_gender(dna: Vec<u8>) -> Result<Gender, Error<T>> {
		let mut result = Gender::Female;
		if dna.len() % 2 == 0 {
			result = Gender::Male
		}
		Ok(result)
	}
}
