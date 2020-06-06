#![cfg_attr(not(feature = "std"), no_std)]

/// A FRAME pallet template with necessary imports

/// Feel free to remove or edit this file as needed.
/// If you change the name of this file, make sure to update its references in runtime/src/lib.rs
/// If you remove this file, you can remove those references

/// For more guidance on Substrate FRAME, see the example pallet
/// https://github.com/paritytech/substrate/blob/master/frame/example/src/lib.rs

use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch, StorageMap};
use frame_system::{self as system, ensure_signed};
use frame_support::codec::{Encode, Decode};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// The pallet's configuration trait.
pub trait Trait: system::Trait {
	// Add other types and constants required to configure this pallet.

	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

#[derive(Encode, Decode, Clone, Default, Debug)]
pub struct Deposit<AccountId> {
	remitter : AccountId,
	expires : u32,
	value : u32,
}

// This pallet's storage items.
decl_storage! {
	// It is important to update your storage name so that your pallet's
	// storage items are isolated from other pallets.
	// ---------------------------------vvvvvvvvvvvvvv
	trait Store for Module<T: Trait> as RemittanceModule {
		Fees get(fn fees): map hasher(blake2_128_concat) T::AccountId => u32;
		DepositFee get(fn deposit_fee): u32; 
		Deposits get(fn deposits): map hasher(blake2_128_concat) T::Hash => Deposit<T::AccountId>;
	}
}

// The pallet's events
decl_event!(
	pub enum Event<T> 
		where 
		<T as system::Trait>::AccountId,
		<T as system::Trait>::Hash, {
		Deposit(AccountId, Hash, u32, u32, u32),
		Transfer(Hash, AccountId, u64),
		Withdraw(AccountId, u32),
		DepositFee(AccountId, u32, u32),
	}
);

// The pallet's errors
decl_error! {
	pub enum Error for Module<T: Trait> {
		/// Value was None
		NoneValue,
		/// Value reached maximum and cannot be incremented further
		StorageOverflow,
	}
}

// The pallet's dispatchable functions.
decl_module! {
	/// The module declaration.
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Initializing errors
		// this includes information about your errors in the node's metadata.
		// it is needed only if you are using errors in your pallet
		type Error = Error<T>;

		// Initializing events
		// this is needed only if you are using events in your pallet
		fn deposit_event() = default;

		#[weight = 10_000]
		fn set_deposit_fee(origin, fee: u32) -> dispatch::DispatchResult {
			if fee == 0 {
				Error()
			}
			DepositFee::put(fee);
			Ok(())
		}
	}
}
