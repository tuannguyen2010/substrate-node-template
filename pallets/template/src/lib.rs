#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

//use sp_runtime::traits::{ BlakeTwo256, Hash, SaturatedConversion};

// use sp_core::H256;
// use sp_core::H512;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

//type Value = u32;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;


	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	// UTXO Development
	//

	// #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
	// #[derive(Hash, Debug)]
	// pub struct TransactionInput {
	// 	pub output_tx: H256, // reference to unspent output
	// 	pub sign_script: H512 // signature that proof the owner of tx
	// }

	// #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
	// #[derive(Hash, Debug)]
	// pub struct TransactionOutput {
	// 	pub value: Value, // value of transaction
	// 	pub public_key: H256 // key of owner of tx
	// }

	// #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
	// #[derive(Hash, Debug)]
	// pub struct Transaction {
	// 	input_txs: Vec<TransactionInput>,
	// 	output_txs: Vec<TransactionOutput>
	// }

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
	pub type Something<T> = StorageValue<_, u32>;

	#[pallet::storage]
	#[pallet::getter(fn number)]
	pub type Number<T:Config> = StorageMap<_, Blake2_128Concat, T::AccountId, u32, ValueQuery>;

	// #[pallet::storage]
	// #[pallet::getter(fn utxo)]
	// pub type Utxo<T:Config> = StorageMap<_, Blake2_128Concat, TransactionOutput, TransactionOutput, ValueQuery>;
	
	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(u32, T::AccountId),

		PutNumber(u32, T::AccountId),

		RemoveNumber(T::AccountId),

		// AddTransactionSuccess(Transaction)
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let who = ensure_signed(origin)?;

			// Update storage.
			<Something<T>>::put(something);

			// Emit an event.
			Self::deposit_event(Event::SomethingStored(something, who));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

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


		// #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		// pub fn spend_utxo(origin: OriginFor<T>, transaction: &Transaction) -> DispatchResult {
		// 	let who = ensure_signed(origin)?;

		// 	Self::update_storage(transaction);
		// 	Self::deposit_event(Event::AddTransactionSuccess(transaction));
		// 	Ok(())
		// }

		// pub fn update_storage(transaction: &Transaction) -> DispatchResult {
		// 	//1. Remove utxo input
		// 	for input in &transaction.input_txs {
		// 		<Utxo>::remove(input.output_tx);
		// 	}

		// 	//2. Insert output
		// 	let mut index = 0;
		// 	for output in &transaction.output_txs {
		// 		//Create key by hash output itself,
		// 		let hash = Blake2_128Concat::hash_of(&transaction.encode(), index);
		// 		index = index + 1;
		// 		<Utxo>::insert(hash, output);
		// 	}

		// 	Ok(())
		// }

		/// An example dispatchable that may throw a custom error.
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			match <Something<T>>::get() {
				// Return an error if the value has not been set.
				None => return Err(Error::<T>::NoneValue.into()),
				Some(old) => {
					// Increment the value read from storage; will error in the event of overflow.
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					<Something<T>>::put(new);
					Ok(())
				},
			}
		}
	}
}
