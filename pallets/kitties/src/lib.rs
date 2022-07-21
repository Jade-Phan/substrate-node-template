#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::pallet_prelude::*;
use frame_support::storage::bounded_vec::BoundedVec;
use frame_support::traits::UnixTime;
use frame_support::traits::{ConstU8, Currency};
use frame_system::pallet_prelude::*;
pub use sp_std::vec::Vec;
const LIMIT_BOUNDED: u8 = 10;
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

type BalanceOf<T> =
	<<T as Config>::KittyCurrency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
type ListKitties = BoundedVec<Vec<u8>, ConstU8<LIMIT_BOUNDED>>;
#[frame_support::pallet]
pub mod pallet {
	pub use super::*;

	#[derive(Clone, Encode, Decode, PartialEq, Copy, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub enum Gender {
		Male,
		Female,
	}

	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
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
		StorageMap<_, Blake2_128Concat, T::AccountId, ListKitties, ValueQuery>;

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
		NotOwner,
		WrongReceiver,
		OwnerAlready,
		OutOfBound,
		IndexOutOfBounds,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn create_kitty(origin: OriginFor<T>, dna: Vec<u8>, price: u32) -> DispatchResult {
			let who = ensure_signed(origin)?;
			//log::info!("total balance:{:?}", T::KittyCurrency::total_balance(&who));
			ensure!(price > 0, Error::<T>::PriceTooLow);
			ensure!(!KittyDetail::<T>::contains_key(&dna), Error::<T>::AlreadyExisted);
			let gender = Self::kitty_gender(dna.clone())?;
			let timestamp = T::Timestamp::now();
			let kitty = Kitty {
				dna: dna.clone(),
				price: price.into(),
				gender,
				owner: who.clone(),
				created_date: timestamp.as_secs(),
			};

			let mut current_number = Self::quantity();
			<KittyDetail<T>>::insert(&dna, kitty);

			current_number += 1;

			Kitties::<T>::put(current_number);

			// use Value Query
			OwnerDetail::<T>::try_mutate(&who, |list_kitty| list_kitty.try_push(dna.clone()))
				.map_err(|_| Error::<T>::OutOfBound)?;

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
			ensure!(to != who, Error::<T>::OwnerAlready);
			ensure!(KittyDetail::<T>::contains_key(&dna), Error::<T>::NoneExisted);

			// remove dna of kitty from the old owner's list
			<OwnerDetail<T>>::try_mutate(&who, |owned| {
				if let Some(ind) = owned.iter().position(|&id| id == *dna) {
					owned.swap_remove(ind);
					return Ok(());
				}
				Err(())
			})
				.map_err(|_| Error::<T>::NotOwner)?;

			// insert dna of new kitty to the new owner's list
			OwnerDetail::<T>::try_mutate(&to, |list_kitty| list_kitty.try_push(dna.clone()))
				.map_err(|_| Error::<T>::OutOfBound)?;
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
