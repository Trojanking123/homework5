#![cfg_attr(not(feature = "std"), no_std)]

/// a module for proof of existence	
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
	use frame_support::dispatch::DispatchResultWithPostInfo;

	use frame_system::pallet_prelude::*;
	use sp_std::vec::Vec;
	

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	//#[pallet::without_storage_info]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn proofs)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/v3/runtime/storage#declaring-storage-items	
	pub type Proofs<T: Config> = StorageMap<
		_, 
		Blake2_128Concat,
		Vec<u8>,
		(T::AccountId, T::BlockNumber)
	>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	//#[pallet::metadata(T::AccountId="AccountId")]
	pub enum Event<T: Config> {
		ClaimCreated(T::AccountId , Vec<u8>	),
		ClaimRevoked(T::AccountId , Vec<u8>	),
		ClaimOwnerChanged(T::AccountId , T::AccountId, Vec<u8>	),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		ProofAlreadyExist,
		ClaimNotExist,
		NotClaimOwner,

	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10)]
		pub fn create_claim(origin: OriginFor<T>, claim: Vec<u8>) -> DispatchResultWithPostInfo {

			let sender = ensure_signed(origin)?;

			ensure!( !Proofs::<T>::contains_key(&claim), Error::<T>::ProofAlreadyExist);
			Proofs::<T>::insert( &claim, (sender.clone(), frame_system::Pallet::<T>::block_number()) );
			Self::deposit_event(Event::ClaimCreated(sender, claim));
			Ok(().into())
			
		}


		#[pallet::weight(10)]
		pub fn revoke_claim(origin: OriginFor<T>, claim: Vec<u8>) -> DispatchResultWithPostInfo {

			let sender = ensure_signed(origin)?;
			let (owner, _) =  Proofs::<T>::get(&claim).ok_or(Error::<T>::ClaimNotExist)?;
			ensure!(sender == owner, Error::<T>::NotClaimOwner);

			Proofs::<T>::remove(&claim);


			Self::deposit_event(Event::ClaimRevoked(sender, claim));
			Ok(().into())
			
		}

		#[pallet::weight(10)]
		pub fn change_claim(origin: OriginFor<T>, claim: Vec<u8>, new: T::AccountId) -> DispatchResultWithPostInfo {

			let sender = ensure_signed(origin)?;
			let (owner, _) =  Proofs::<T>::get(&claim).ok_or(Error::<T>::ClaimNotExist)?;
			ensure!(sender == owner, Error::<T>::NotClaimOwner);

			Proofs::<T>::mutate( &claim, |res | { 
				if let  Some(ref mut s) = res {
					s.0 = new.clone();
				}
				
			}) ;


			Self::deposit_event(Event::ClaimOwnerChanged(sender, new, claim	));
			Ok(().into())
			
		}

	}
}
