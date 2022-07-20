#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;
//pub use frame_system::Pallet;
pub use frame_support::inherent::Vec;
pub use frame_system::Error;

use frame_support::pallet_prelude::*;
use frame_support::sp_runtime::SaturatedConversion;
use frame_support::storage::bounded_vec::BoundedVec;
use frame_support::traits::Randomness;
use frame_support::traits::Time;
use frame_system::pallet_prelude::*;
pub use sp_core::H256;
use frame_support::traits::Get;

#[frame_support::pallet]
pub mod pallet {

	pub use super::*;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type Timestamp: Time;

		type RandomnessHash: Randomness<H256, u32>;

		// #[pallet::constant]
		type KittyLimit: Get<u32>;
	}

	#[derive(TypeInfo, Default, Encode, Decode, Clone)]
	#[scale_info(skip_type_params(T))]
	pub struct Kitty<T: Config> {
		dna: Dna,
		owner: T::AccountId,
		price: u32,
		gender: Gender,
		created_date: u64,
	}

	pub type Id = u32;

	//pub type Dna = Vec<u32>;
	pub type Dna = H256;

	#[derive(TypeInfo, Encode, Decode, Clone)]
	pub enum Gender {
		Male,
		Female,
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
	pub type Number<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, u32, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn kitties)]
	pub(super) type Kitties<T: Config> = StorageMap<_, Blake2_128Concat, Id, Kitty<T>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn kitties_owner)]
	pub(super) type KittiesOwner<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, Vec<Dna>, OptionQuery>;

	// #[pallet::storage]
	// #[pallet::getter(fn kitties_owner_with_limit)]
	// pub(super) type KittiesOwnerWithLimit<T: Config> =
	// 	StorageMap<_, Blake2_128Concat, T::AccountId, BoundedVec<Dna, u32>, OptionQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		PutNumber(u32, T::AccountId),

		RemoveNumber(T::AccountId),

		CreateKitty(T::AccountId, u32, u32),

		TransferKitty(Dna, T::AccountId, u32),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,

		//KeyExist
		NotOwner,

		//Create kitty error
		CreateFailed,

		//Limit created kitty, exceed limit error
		TooMuchKitties,

		ExceedKittyLimit,
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
		//pub fn create_kitty(origin: OriginFor<T>, dna: Vec<u8>) -> DispatchResult {
		pub fn create_kitty(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;

			//ensure!(age > 20, Error::<T>::TooYoung);
			let mut kitties_of_owner = <KittiesOwner<T>>::get(who.clone());
			// ensure!(
			// 	kitties_of_owner.len() <= T::KittyLimit::get() as usize,
			// 	Error::<T>::TooMuchKitties
			// );

			//Get the currentId
			//let current_id = Self::KittyId();
			//let current_id = KittyId::<T>::get();
			let mut current_id = <KittyId<T>>::get();
			ensure!(current_id <= T::KittyLimit::get(), Error::<T>::ExceedKittyLimit);
			//Random dna
			let (mut dna, blockNumber) = Self::generate_dna(&current_id).unwrap();

			//Random gender by length of dna
			let gender = Self::generate_gender(dna.clone())?;

			//Get current time
			let current_time = T::Timestamp::now().saturated_into::<u64>();
			//Declare kitty object
			let kitty = Kitty {
				dna: dna.clone(),
				owner: who.clone(),
				price: 0,
				gender: gender.clone(),
				created_date: current_time,
			};

			//Insert kitty to storage as key is id - value is kitty
			//Kitties::<T>::insert(current_id, kitty);
			<Kitties<T>>::insert(current_id, kitty);

			//Increase id every time create kitty
			current_id += 1;
			KittyId::<T>::put(current_id);

			//Add kitty's dna to owner map as key: AccountId - value: dna

			// let mut kitties_of_owner = match kitties_of_owner[..] {
			// 	[..] => Vec::<Vec<u8>>::new(),
			// 	_ => <KittiesOwner<T>>::get(who.clone()).unwrap(),
			// };

			let mut kitties_of_owner = match kitties_of_owner {
				None => Vec::<Dna>::new(),
				Some(_) => <KittiesOwner<T>>::get(who.clone()).unwrap(),
			};
			kitties_of_owner.push(dna.clone());
			<KittiesOwner<T>>::insert(who.clone(), kitties_of_owner);

			// Emit an event.
			Self::deposit_event(Event::CreateKitty(who, 0, current_id));
			Ok(())
		}
		//transfer kitty

		// #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		// pub fn transfer_kitty(
		// 	origin: OriginFor<T>,
		// 	kitty_id: u32,
		// 	to: T::AccountId,
		// ) -> DispatchResult {
		// 	let who = ensure_signed(origin)?;

		// 	//ensure!(!<Kitties<T>>::contains_key(kitty_id), Error::<T>::NoneValue);
		// 	//debug::RuntimeLogger::init();
		// 	//Get kitty info as Key: From
		// 	let mut kitty_info: Kitty<T> = <Kitties<T>>::get(&kitty_id).ok_or(Error::<T>::NoneValue)?;
		// 	//debug::info!("kitty info by: {:?}", kitty_info);
		// 	ensure!(kitty_info.owner == who, Error::<T>::NotOwner);

		// 	//Change owner to TO
		// 	kitty_info.owner = to.clone();
		// 	let kitty_dna = kitty_info.clone().dna;
		// 	// match kitty_info {
		// 	// 	Some(x) => kitty_info.owner = to,
		// 	// 	None => panic!(),
		// 	// }
		// 	//<Kitties<T>>::mutate(&kitty_id, &mut kitty_info);
		// 	<Kitties<T>>::insert(&kitty_id, &mut kitty_info);

		// 	//Remove dna from FROM
		// 	let kitties_of_owner = <KittiesOwner<T>>::get(who.clone());
		// 	let mut kitties_of_owner = match kitties_of_owner {
		// 		None => Vec::<Dna>::new(),
		// 		Some(_) => <KittiesOwner<T>>::get(who.clone()).unwrap(),
		// 	};
		// 	//debug::info!("kitty dna index: {:?}", kitty_dna);
		// 	//let index = kitties_of_owner.iter().position(|x| String::from_utf8((**x).to_vec()).expect("Found invalid UTF-8") == String::from_utf8(kitty_dna.clone()).expect("Found invalid UTF-8"));
		// 	//debug::info!("remove index: {:?}", index);
		// 	//Self::deposit_event(Event::TransferKitty(to, index.unwrap()));
		// 	let index = kitties_of_owner.iter().position(|x| *x == kitty_dna);
		// 	//let index = kitties_of_owner.iter().position(| &x| *x.to_string() == kitty_dna.to_string());
		// 	kitties_of_owner.remove(index.unwrap());
		// 	// kitties_of_owner.retain(| &x| x != "abc1");
		// 	// <KittiesOwner<T>>::mutate(who.clone(), &mut kitties_of_owner);

		// 	//Add kitty's dna to owner map as key: AccountId - value: dna
		// 	let kitties_of_owner = <KittiesOwner<T>>::get(to.clone());
		// 	let mut kitties_of_owner = match kitties_of_owner {
		// 		None => Vec::<Dna>::new(),
		// 		Some(_) => <KittiesOwner<T>>::get(who.clone()).unwrap(),
		// 	};

		// 	kitties_of_owner.push(kitty_dna.clone());
		// 	//<KittiesOwner<T>>::append(to.clone(), kitty_dna.clone());
		// 	<KittiesOwner<T>>::insert(to.clone(), kitties_of_owner);

		// 	// Emit an event.
		// 	Self::deposit_event(Event::TransferKitty(to, kitty_id));
		// 	Ok(())
		// }
	}

	//Helper
	impl<T: Config> Pallet<T> {
		fn generate_gender(dna: H256) -> Result<Gender, Error<T>> {
			let mut res = Gender::Male;

			// if dna.len() % 2 == 0 {
			// 	res = Gender::Female;
			// }
			res = Gender::Female;

			Ok(res)
		}

		//
		fn generate_dna(nonce: &u32) -> Result<(H256, u32), Error<T>> {
			let random_string = T::RandomnessHash::random(&nonce.clone().encode());

			Ok(random_string)
		}
	}
}