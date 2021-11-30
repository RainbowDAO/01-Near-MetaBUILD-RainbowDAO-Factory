#![cfg_attr(not(feature = "std"), no_std)]
pub use self::income_category::{
    IncomeCategory
};
use ink_lang as ink;

#[ink::contract]
mod income_category {

    /// Indicates whether a transaction is already confirmed or needs further confirmations.
    #[derive(scale::Encode, scale::Decode, Clone, SpreadLayout, PackedLayout)]
    #[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    )]

    #[derive(Debug)]
    pub struct IncomeInfo {
        is_used:bool,
        fee: u128,
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
        #[ink(selector = "0xDEADBEEF")]
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
        }


        fn only_owner(&self,sender:AccountId) {
            assert_eq!(self.owner, sender);
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let incomeCategory = IncomeCategory::default();
            assert_eq!(incomeCategory.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut incomeCategory = IncomeCategory::new(false);
            assert_eq!(incomeCategory.get(), false);
            incomeCategory.flip();
            assert_eq!(incomeCategory.get(), true);
        }
    }
}
