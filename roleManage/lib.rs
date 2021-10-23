#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;
use ink_lang as ink;

#[ink::contract]
mod roleManage {
    use alloc::string::String;
    use ink_prelude::vec::Vec;
    use ink_prelude::collections::BTreeMap;
    use ink_storage::{collections::HashMap as StorageHashMap, };

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct RoleManage {
        /// Stores a single `bool` value on the storage.
        index:u64,
        role_map:StorageHashMap<u64,String>,
        role_privileges:StorageHashMap<u64,Vec<u64>>,
    }

    impl RoleManage {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new() -> Self {

            let instance = Self {
                index: 0,
                role_map : StorageHashMap::new(),
                role_privileges: StorageHashMap::new()
            };
            instance
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        // #[ink(constructor)]
        // pub fn default() -> Self {
        //     Self::new(Default::default())
        // }

        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        // #[ink(message)]
        // pub fn flip(&mut self) {
        //     self.value = !self.value;
        // }

        /// Simply returns the current value of our `bool`.
        // #[ink(message)]
        // pub fn get(&self) -> bool {
        //     self.value
        // }


        #[ink(message)]
        pub fn add_role(&mut self, name: String) -> bool {
            assert_eq!(self.index + 1 > self.index, true);
            self.role_map.insert(self.index, name);
            self.index += 1;
            true
        }

        #[ink(message)]
        pub fn list_roles(&self) -> Vec<String> {
            let mut role_vec = Vec::new();
            let mut iter = self.role_map.values();
            let mut role = iter.next();
            while role.is_some() {
                role_vec.push(role.unwrap().clone());
                role = iter.next();
            }
            role_vec
        }

        #[ink(message)]
        pub fn query_role_by_index(&self, index: u64) -> String {
            self.role_map.get(&index).unwrap().clone()
        }


        #[ink(message)]
        pub fn role_insert_privilege(&self ,index:u64,privilege_index:u64) -> bool {
           // let mut privilege_arr =  self.role_privileges.get(&index).unwrap().clone();
            if   !self.role_privileges.contains_key(&index) {
                self.role_privileges.insert(index, vec![privilege_index]);
            } else {
                let  v =self.role_privileges.get_mut(&index).unwrap().clone();
                v.push(privilege_index);

            }

            true
        }

    }


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
    //         let roleManage = RoleManage::default();
    //         assert_eq!(roleManage.get(), false);
    //     }
    //
    //     /// We test a simple use case of our contract.
    //     #[ink::test]
    //     fn it_works() {
    //         let mut roleManage = RoleManage::new(false);
    //         assert_eq!(roleManage.get(), false);
    //         roleManage.flip();
    //         assert_eq!(roleManage.get(), true);
    //     }
    // }
}
