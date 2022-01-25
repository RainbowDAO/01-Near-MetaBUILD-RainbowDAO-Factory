#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;
use ink_lang as ink;

#[allow(unused_imports)]
#[ink::contract]
mod dao_category {

    use alloc::string::String;
    use ink_prelude::vec::Vec;
    use ink_prelude::collections::BTreeMap;
    use ink_storage::{collections::HashMap as StorageHashMap, };

    /// the contract store the category of dao
    /// owner:the manager of the contract
    /// category_map:HashMap of the index and name
    #[ink(storage)]
    pub struct DaoCategory {
        owner: AccountId,
        category_map:StorageHashMap<u64,String>,
        index:u64
    }

    impl DaoCategory {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                owner: Self::env().caller(),
                category_map:StorageHashMap::new(),
                index:0
            }
        }
        /// add a new category
        /// name:the name of category
        #[ink(message)]
        pub fn add_category(&mut self,name:String) ->  bool {
            assert_eq!(self.index + 1 > self.index, true);
            self.category_map.insert(self.index, name);
            self.index += 1;
            true
        }
        /// show all category
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

        /// Get a category by id
        #[ink(message)]
        pub fn query_category_by_index(&self, index: u64) -> String {
            self.category_map.get(&index).unwrap().clone()
        }
    }


    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;

        #[ink::test]
        fn it_works() {
            let mut dao_category = DaoCategory::new();
            dao_category.add_category(String::from("test"));
            assert!(dao_category.query_category_by_index(0)== String::from("test"));
        }
    }
}
