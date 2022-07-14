#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;
//pub use frame_system::Pallet;
pub use frame_support::inherent::Vec;
pub use frame_system::Error;


use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;

#[frame_support::pallet]
pub mod pallet {
	pub use super::*;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[derive(TypeInfo, Default, Encode, Decode, Clone)]
	#[scale_info(skip_type_params(T))]
	pub struct Kitty<T:Config> {
		dna: Vec<u8>,
		owner: T::AccountId,
		price: u32,
		gender: Gender,
	}

	pub type Id = u32;

	#[derive(TypeInfo, Encode, Decode, Clone)]
	pub enum Gender {
		Male,
		Female
	}
	impl Default for Gender {
		fn default() -> Self {
			Gender::Male
		}
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn kitty_id)]
	pub type KittyId<T> = StorageValue<_, Id, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn number)]
	pub type Number<T:Config> = StorageMap<_, Blake2_128Concat, T::AccountId, u32, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn kitties)]
	pub(super) type Kitties<T:Config> = StorageMap<_, Blake2_128Concat, Id , Kitty<T>, OptionQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]

		PutNumber(u32, T::AccountId),

		RemoveNumber(T::AccountId),

		CreateKitty(Vec<u8>, T::AccountId, u32, u32),

		TransferKitty(T::AccountId, u32),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,

		//KeyExist
		NotOwner
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn put_number(origin: OriginFor<T>, number: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let who = ensure_signed(origin)?;

			// Update storage.
			<Number<T>>::insert(who.clone(), number);

			// Emit an event.
			Self::deposit_event(Event::PutNumber(number, who));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn remove_number(origin: OriginFor<T>) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let who = ensure_signed(origin)?;

			// Update storage.
			<Number<T>>::remove(who.clone());

			// Emit an event.
			Self::deposit_event(Event::RemoveNumber(who));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		//function create kitty
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn create_kitty(origin: OriginFor<T>, dna: Vec<u8>) -> DispatchResult {
			let who = ensure_signed(origin)?;

			//ensure!(age > 20, Error::<T>::TooYoung);
			
			//let dna = Self::generate_dna()?;
			let gender = Self::generate_gender(dna.clone())?;
			let  kitty =  Kitty {
				dna: dna.clone(),
				owner: who.clone(),
				price: 0,
				gender: gender.clone()
			};

			//let current_id = Self::KittyId();
			//let current_id = KittyId::<T>::get();
			let mut current_id = <KittyId<T>>::get();

			//Kitties::<T>::insert(current_id, kitty);
			<Kitties<T>>::insert(current_id, kitty);
			current_id += 1;
			KittyId::<T>::put(current_id);

			// Emit an event.
			Self::deposit_event(Event::CreateKitty(dna, who, 0, current_id));
			Ok(())
		}
		//transfer kitty
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn transfer_kitty(origin: OriginFor<T>, kitty_id: u32, to: T::AccountId) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(<Kitties<T>>::contains_key(kitty_id), Error::<T>::NoneValue);
			
			let mut kitty_info = <Kitties<T>>::get(&kitty_id).unwrap();

			ensure!(kitty_info.owner == who, Error::<T>::NotOwner);
			kitty_info.owner = to.clone();
			// match kitty_info {
			// 	Some(x) => kitty_info.owner = to,
			// 	None => panic!(),
			// }

			//<Kitties<T>>::mutate(&kitty_id, &mut kitty_info);
			<Kitties<T>>::insert(&kitty_id, &mut kitty_info);

			// Emit an event.
			Self::deposit_event(Event::TransferKitty(to, kitty_id));
			Ok(())
		}
	}


//Helper
impl<T> Pallet<T> {
	fn generate_gender(dna: Vec<u8>) -> Result<Gender,Error<T>> {
		let mut res = Gender::Male;

		if dna.len() % 2 == 0 {
			res = Gender::Female;
		}

		Ok(res)
	}

	//
	// fn generate_dna() -> Result<Vec<u8>, Error<T>> {
	// 	let mut randLength = rand::thread_rng();

	// 	let rand_string: String = thread_rng()
    //     .sample_iter(&Alphanumeric)
    //     .take(randLength.gen_range(0..20))
    //     .map(char::from)
    //     .collect();

	// 	Ok(rand_string)
	// }
}


}
