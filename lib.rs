#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod KVstore {
    use ink_storage::traits::SpreadAllocate;

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct KVstore {
        // Store a mapping from AccountIds to a u32
        map: ink_storage::Mapping<AccountId, u64>,
    }

    impl KVstore {
        #[ink(constructor)]
        pub fn new(count: u64) -> Self {
            // This call is required in order to correctly initialize the
            // `Mapping`s of our contract.
            ink_lang::utils::initialize_contract(|contract: &mut Self| {
                let caller = Self::env().caller();
                contract.map.insert(caller, &count);
            })
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            // Even though we're not explicitly initializing the `Mapping`,
            // we still need to call this
            ink_lang::utils::initialize_contract(|_| {})
        }

        // Grab the number at the caller's AccountID, if it exists
        #[ink(message)]
        pub fn get(&self) -> u64{
            let caller = Self::env().caller();
            self.map.get(caller).unwrap_or_default()
        }
    }
    #[cfg(test)]
    mod tests {
       use super::*; 
       use ink_lang as ink; 

        #[ink::test]
        fn new_works() {
            let count: u64 = 5;
            let flipper = KVstore::new(count);
            assert_eq!(flipper.get(), 5);
            
        }
    }
}
