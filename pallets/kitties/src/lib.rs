#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::pallet_prelude::*;
use frame_support::storage::types::StorageMap;
use frame_support::traits::{Get, Randomness};
use frame_support::traits::Currency;
use frame_support::traits::UnixTime;
use frame_system::pallet_prelude::*;
pub use sp_std::vec::Vec;

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

type BalanceOf<T> =
<<T as Config>::KittyCurrency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::dispatch::fmt;

	pub use super::*;

	#[derive(Clone, Encode, Decode, PartialEq, Copy, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub enum Gender {
		Male,
		Female,
	}

	#[derive(Clone, Encode, Decode, PartialEq, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct Kitty<T: Config> {
		dna: Vec<u8>,
		price: BalanceOf<T>,
		gender: Gender,
		owner: T::AccountId,
		created_date: u64,
	}

	impl<T: Config> fmt::Debug for Kitty<T> {
		fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
			f.debug_struct("Kitty")
				.field("dna", &self.dna)
				.field("price", &self.price)
				.field("gender", &self.gender)
				.field("owner", &self.owner)
				.field("created_date", &self.created_date)
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
		type KittyCurrency: Currency<Self::AccountId>;
		type Timestamp: UnixTime;
		type Max: Get<u8>;
		type KittyRandomness: Randomness<Self::Hash, Self::BlockNumber>;
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

	#[pallet::storage]
	#[pallet::getter(fn nonce)]
	// Key :Id, Value: Student
	pub(super) type Nonce<T: Config> = StorageValue<_, u32, ValueQuery>;

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
		#[pallet::weight(46_367_000 + T::DbWeight::get().reads_writes(6, 4))]
		pub fn create_kitty(origin: OriginFor<T>, price: u32) -> DispatchResult {
			let who = ensure_signed(origin)?;
			//log::info!("total balance:{:?}", T::KittyCurrency::total_balance(&who));
			let nonce = Self::get_nonce();
			let (rand, _) = T::KittyRandomness::random(&nonce);
			let dna = rand.encode();
			let kitties = OwnerDetail::<T>::get(&who);
			ensure!(kitties.len() < T::Max::get().into(), Error::<T>::OutOfBound);
			ensure!(price > 0, Error::<T>::PriceTooLow);
			ensure!(!KittyDetail::<T>::contains_key(&dna), Error::<T>::AlreadyExisted);
			let gender = Self::kitty_gender(price)?;
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
			OwnerDetail::<T>::mutate(&who, |list_kitty| list_kitty.push(dna.clone()));

			Self::deposit_event(Event::CreatedKitty(dna, who, timestamp.as_secs()));
			Ok(())
		}

		#[pallet::weight(37_404_000 + T::DbWeight::get().reads_writes(3, 2))]
		pub fn transfer_kitty(
			origin: OriginFor<T>,
			dna: Vec<u8>,
			to: T::AccountId,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let kitties = OwnerDetail::<T>::get(&to);
			ensure!(kitties.len() < T::Max::get().into(), Error::<T>::OutOfBound);
			ensure!(to != who, Error::<T>::OwnerAlready);
			ensure!(KittyDetail::<T>::contains_key(&dna), Error::<T>::NoneExisted);

			// remove dna of kitty from the old owner's list
			<OwnerDetail<T>>::mutate(&who, |owned| {
				if let Some(ind) = owned.iter().position(|id| id == &dna) {
					owned.swap_remove(ind);
					return Ok(());
				}
				Err(())
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
	fn kitty_gender(price: u32) -> Result<Gender, Error<T>> {
		let mut result = Gender::Female;
		if price % 2 == 0 {
			result = Gender::Male
		}
		Ok(result)
	}

	fn get_nonce() -> Vec<u8> {
		let nonce = Nonce::<T>::get();
		Nonce::<T>::put(nonce.wrapping_add(1));
		nonce.encode()
	}
}
