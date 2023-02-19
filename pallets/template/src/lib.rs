#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use sp_std::prelude::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	#[derive(Encode, Decode, Clone, PartialEq, Default, TypeInfo)]
	pub struct VoterInfo {
		/// Username, stored as an array of bytes.
		pub name: Vec<u8>,
		pub surname: Vec<u8>,
		pub email: Vec<u8>,
		pub dob: Vec<u8>,
		pub phone: i64,
		/// Number id of the user.
		pub national_id: i64,
		/// The "About Me" section of the user.
		pub address: Vec<u8>,
		pub vote: Vec<u8>,
	}
    /// Mapping of account ids to VoterInfo.
	#[pallet::storage]
	#[pallet::getter(fn info)]
	pub type AccountIdToVoterInfo<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, VoterInfo, ValueQuery>;


	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
	  /// Indicates a user has been registered.
		VoterCreated { user: T::AccountId },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		UserInfoTooLong,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		// Dispatchable calls go here!
		// Register a new user and change the state of the chain.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn register_voter(
			origin: OriginFor<T>,
		    name: Vec<u8>,
		    surname: Vec<u8>,
		    email: Vec<u8>,
		    dob: Vec<u8>,
		    phone: i64,
		    national_id: i64,
		    address: Vec<u8>,
		    vote: Vec<u8>,
		) -> DispatchResult {
			// Gets the caller or signer of the function.
			let sender = ensure_signed(origin)?;
			// Define a new user in accordance to VoterInfo.
			let new_user = VoterInfo { name, surname, email, dob, phone, national_id, address, vote };
			// Change the state of our storage mapping by adding user info to our sender AccountId.
			<AccountIdToVoterInfo<T>>::insert(&sender, new_user);
			// Emit an event indicating the user is now created and registered.
			Self::deposit_event(Event::<T>::VoterCreated { user: sender });
			Ok(())
		}
	}

}