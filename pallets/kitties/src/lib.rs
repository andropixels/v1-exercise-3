#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Encode,Decode};																																																						
use frame_support::{
	decl_module,decl_storage, decl_event,decl_error,StorageValue,StorageDoubleMap,
	traits::Randomness, RuntimeDebug, dispatch::DispatchResult,

};

use sp_io::hashing::blake2_128;
use frame_system::ensure_signed;

#[derive(Encode,Decode,Clone, RuntimeDebug, PartialEq,Eq )]
pub struct Kitty(pub [u8;16]);

pub trait Trait: frame_system::Trait{

type Event:  From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;

}

	

	decl_event! {

          pub enum Event<T> where 
           <T as  frame_system::Trait>::AccountId,
		   {


			KittyCreated(AccountId, u32, Kitty),

		   }
              
	}


decl_storage!{

 trait Store for Module<T:Trait> as Kitties{
//  stores all the kitty 
   pub Kitties get(fn Kitties): double_map hasher(blake2_128_concat) T::AccountId, hasher(blake2_128_concat) u32 => Option<Kitty>;	

   pub NextKittyId get(fn next_kitty_id): u32;	
 }


}

decl_error!{

	pub enum  Error  for Module<T: Trait> {

		KittiesIdOverflow,
	}
}


decl_module!{


	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event()=default;

         
		#[weight=1000]
		pub fn  create(origin){
			let sender =ensure_signed(origin)?;
			NextKittyId::try_mutate(|next_id| -> DispatchResult {
				let current_id = *next_id;
				*next_id = next_id.checked_add(1).ok_or(Error::<T>::KittiesIdOverflow)?;
			//generate random 128 bit value 
			let payload = (
				<pallet_randomness_collective_flip::Module<T> as Randomness<T::Hash>>::random_seed(),
				&sender,
				<frame_system::Module<T>>::extrinsic_index(),
			);
			let dna = payload.using_encoded(blake2_128);

			let kitty = Kitty(dna);
				Kitties::<T>::insert(&sender, current_id, &kitty);

			//emit event 	
			Self::deposit_event(RawEvent::KittyCreated(sender, current_id, kitty));

				Ok(())
			})?;

		}


	}
}

