//! # XVM pallet 
//!
//! ## Overview
//!
//!
//! ## Interface
//!
//! ### Dispatchable Function
//!
//!
//! ### Other
//!
//!

#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Encode, Decode};
use frame_support::pallet;
pub use pallet::*;

/// Abstract execution engine.
pub trait AbstractVM {
    /// This constant use for mathing VMs in XVM call.
    const char[4] name;
}

/// The engine that support synchronous smart contract execution.
/// For example, EVM.
pub trait<T, A, R, E> SyncVM : AbstractVM
where
    A: Decode,
    R, E: Encode,
{
    /// Make a call to VM contract and return result or error. 
    fn xvm_call(from: T, to: T, args: A) -> Result<R, E>;
}

/// The engine that support asynchronous smart contract execution.
/// For example, XCVM.
pub trait<A, M> AsyncVM : AbstractVM 
where
    T: Decode,
    M: Encode,
{
    /// Send a message.
    fn xvm_send(from: A, to: A, message: T);

    /// Query for incoming messages.
    fn xvm_query(inbox: A) -> Vec<M>;
}

#[impl_trait_for_tuples::impl_for_tuples(30)]
impl SyncVM for Tuple {
    for_tuples!( #(
    )* );
}

#[pallet]
pub mod pallet {

    use crate::weights::WeightInfo;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    #[pallet::pallet]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type SyncVM: SyncVM;
        type AsyncVM: AsyncVM;
        type WeightInfo: WeightInfo;
    }

    #[pallet::error]
    pub enum Error<T> {
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(crate) fn deposit_event)]
    pub enum Event<T: Config> {
    }


    #[pallet::call]
    impl<T: Config> Pallet<T> {
    }
}
