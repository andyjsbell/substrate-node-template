#![cfg_attr(not(feature = "std"), no_std)]

/// A FRAME pallet template with necessary imports

/// Feel free to remove or edit this file as needed.
/// If you change the name of this file, make sure to update its references in runtime/src/lib.rs
/// If you remove this file, you can remove those references

/// For more guidance on Substrate FRAME, see the example pallet
/// https://github.com/paritytech/substrate/blob/master/frame/example/src/lib.rs

use frame_support::{
	decl_module, decl_storage, decl_event, decl_error, dispatch, StorageMap, ensure,
	traits::{Currency, ExistenceRequirement::AllowDeath},
	sp_runtime::{traits::{AccountIdConversion, Zero}, ModuleId}
};
use frame_system::{self as system, ensure_signed};
use frame_support::codec::{Encode, Decode};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

const PALLET_ID: ModuleId = ModuleId(*b"Charity!");
/// The pallet's configuration trait.
pub trait Trait: system::Trait {
	// Add other types and constants required to configure this pallet.

	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
	type Currency: Currency<Self::AccountId>;
}

pub type BalanceOf<T> = <<T as Trait>::Currency as Currency<<T as frame_system::Trait>::AccountId>>::Balance;
pub type NegativeImbalanceOf<T> = <<T as Trait>::Currency as Currency<<T as frame_system::Trait>::AccountId>>::NegativeImbalance;
#[derive(Encode, Decode, Clone, Default, Debug)]
pub struct Deposit<AccountId, Balance> {
	remitter : AccountId,
	expires : u32,
	value : Balance
}

// This pallet's storage items.
decl_storage! {
	// It is important to update your storage name so that your pallet's
	// storage items are isolated from other pallets.
	// ---------------------------------vvvvvvvvvvvvvv
	trait Store for Module<T: Trait> as RemittanceModule {
		Fees get(fn fees): map hasher(blake2_128_concat) T::AccountId => u32;
		DepositFee get(fn deposit_fee): u32;
		Deposits get(fn deposits): map hasher(blake2_128_concat) T::Hash => Deposit<T::AccountId, BalanceOf<T>>;
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
		FeeDeposited(AccountId, u32, u32),
	}
);

// The pallet's errors
decl_error! {
	pub enum Error for Module<T: Trait> {
		/// Value was None
		NoneValue,
		/// Values are Equal
		EqualValue,
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

		#[weight = 0]
		fn set_deposit_fee(origin, fee: u32) -> dispatch::DispatchResult {
			let sender = ensure_signed(origin)?;
			ensure!(fee > 0, <Error<T>>::NoneValue);
			ensure!(fee != Self::deposit_fee(), <Error<T>>::EqualValue);

			Self::deposit_event(RawEvent::FeeDeposited(sender, 0, 0));

			DepositFee::put(fee);
			Ok(())
		}

		#[weight = 0]
		fn deposit(origin, puzzle: T::Hash, timeout: u32, amount: BalanceOf<T>) -> dispatch::DispatchResult {

			let sender = ensure_signed(origin)?;
			ensure!(timeout > 0, "Timeout needs to be bigger than 0");
			ensure!(!amount.is_zero(), "We need to send something");

			<Deposits<T>>::insert(puzzle, Deposit {
				remitter: sender.clone(),
				expires: timeout + 0,
				value: amount
			});

			T::Currency::transfer(&sender, &Self::account_id(), amount, AllowDeath)
				.map_err(|_| dispatch::DispatchError::Other("Can't make donation"))?;
			Ok(())
		}
	}
}

impl<T: Trait> Module<T> {
	/// The account ID that holds the Charity's funds
	pub fn account_id() -> T::AccountId {
		PALLET_ID.into_account()
	}

	/// The Charity's balance
	fn pot() -> BalanceOf<T> {
		T::Currency::free_balance(&Self::account_id())
	}
}
