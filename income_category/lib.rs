#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;
pub use self::income_category::{
    IncomeCategory
};
use ink_lang as ink;

#[ink::contract]
mod income_category {
    use alloc::string::String;
    use ink_storage::{
        collections::HashMap as StorageHashMap,
        lazy::Lazy,
        traits::{
            PackedLayout,
            SpreadLayout,
        }
    };
    /// Indicates whether a transaction is already confirmed or needs further confirmations.
    #[derive(scale::Encode, scale::Decode, Clone, SpreadLayout, PackedLayout)]
    #[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    )]

    #[derive(Debug)]
    pub struct IncomeInfo {
       pub is_used:bool,
       pub fee: u128,
       pub token: AccountId
    }
    #[ink(storage)]
    pub struct IncomeCategory {
        owner:AccountId,
        category:StorageHashMap<String, IncomeInfo>,
    }

    impl IncomeCategory {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(owner:AccountId) -> Self {
            Self {
                owner,
                category: StorageHashMap::new()
            }
        }
        #[ink(message)]
        #[ink(selector = 0xDEADBEEF)]
        pub fn save_category(&mut self,name:String,income:IncomeInfo) -> bool {
            self.only_owner(Self::env().caller());
            self.category.insert(name,income);
            true
        }
        // #[ink(message)]
        // pub fn set_contract_fee(&mut self,new_owner:AccountId) -> bool {
        //     self.only_owner(Self::env().caller());
        //     self.owner = new_owner;
        // }
        #[ink(message)]

        pub fn get_category(&mut self,name:String) -> IncomeInfo {
           self.category.get(&name).unwrap().clone()
        }


        #[ink(message)]
        pub fn transfer_owner(&mut self,new_owner:AccountId) -> bool {
            self.only_owner(Self::env().caller());
            self.owner = new_owner;
            true
        }


        fn only_owner(&self,sender:AccountId) {
            assert_eq!(self.owner, sender);
        }
    }
}
