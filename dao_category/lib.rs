#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;
use ink_lang as ink;

#[ink::contract]
mod dao_category {

    use alloc::string::String;
    use ink_prelude::vec::Vec;
    use ink_prelude::collections::BTreeMap;
    use ink_storage::{collections::HashMap as StorageHashMap, };

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct DaoCategory {
        /// Stores a single `bool` value on the storage.
        owner: AccountId,
        category_map:StorageHashMap<u64,String>,
        index:u64
    }

    impl DaoCategory {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                owner: Self::env().caller(),
                category_map:StorageHashMap::new(),
                index:0
            }
        }
        #[ink(message)]
        pub fn add_category(&mut self,name:String) ->  bool {
            assert_eq!(self.index + 1 > self.index, true);
            self.category_map.insert(self.index, name);
            self.index += 1;
            true
        }

        #[ink(message)]
        pub fn list_category(&self) -> Vec<String> {
            let mut category_vec = Vec::new();
            let mut iter = self.category_map.values();
            let mut category = iter.next();
            while category.is_some() {
                category_vec.push(category.unwrap().clone());
                category = iter.next();
            }
            category_vec
        }

        #[ink(message)]
        pub fn query_category_by_index(&self, index: u64) -> String {
            self.category_map.get(&index).unwrap().clone()
        }
    }

    // /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    // /// module and test functions are marked with a `#[test]` attribute.
    // /// The below code is technically just normal Rust code.
    // #[cfg(test)]
    // mod tests {
    //     /// Imports all the definitions from the outer scope so we can use them here.
    //     use super::*;
    //
    //     /// Imports `ink_lang` so we can use `#[ink::test]`.
    //     use ink_lang as ink;
    //
    //     /// We test if the default constructor does its job.
    //     #[ink::test]
    //     fn default_works() {
    //         let daoCategory = DaoCategory::default();
    //         assert_eq!(daoCategory.get(), false);
    //     }
    //
    //     /// We test a simple use case of our contract.
    //     #[ink::test]
    //     fn it_works() {
    //         let mut daoCategory = DaoCategory::new(false);
    //         assert_eq!(daoCategory.get(), false);
    //         daoCategory.flip();
    //         assert_eq!(daoCategory.get(), true);
    //     }
    // }
}
