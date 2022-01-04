#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;
use ink_lang as ink;

pub use self::authority_management::{
    AuthorityManagement,
};
#[ink::contract]
mod authority_management {
    use alloc::string::String;
    use ink_prelude::vec::Vec;
    use ink_prelude::collections::BTreeMap;
    use ink_storage::{collections::HashMap as StorageHashMap, };


    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct AuthorityManagement {
        owner:AccountId,
        index:u64,
        privilege_map:StorageHashMap<u64,String>,

    }

    impl AuthorityManagement {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new() -> Self {
            let instance = Self {
                owner:Self::env().caller(),
                index: 0,
                privilege_map : StorageHashMap::new(),
            };
            instance
        }

        fn only_core(&self,sender:AccountId) {
            assert_eq!(self.owner, sender);
        }

        #[ink(message)]
        pub fn add_privilege(&mut self, name: String) -> bool {
            self.only_core(Self::env().caller());
            assert_eq!(self.index + 1 > self.index, true);
            self.privilege_map.insert(self.index, name);
            self.index += 1;
            true
        }

        #[ink(message)]
        pub fn list_privileges(&self) -> Vec<String> {
            let mut privilege_vec = Vec::new();
            let mut iter = self.privilege_map.values();
            let mut privilege = iter.next();
            while privilege.is_some() {
                privilege_vec.push(privilege.unwrap().clone());
                privilege = iter.next();
            }
            privilege_vec
        }

        #[ink(message)]
        pub fn query_privilege_by_index(&self, index: u64) -> String {
            self.privilege_map.get(&index).unwrap().clone()
        }

    }


    #[cfg(test)]
    mod tests {


        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;

        /// You need to get the hash from  RouteManage,authority_management and RoleManage contract
        #[ink::test]
        fn init_works() {
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                    .expect("Cannot get accounts");
            let mut authority_management = AuthorityManagement::new();
            authority_management.add_privilege(String::from("test"));
            assert!(authority_management.query_privilege_by_index(0)== String::from("test"));
        }
    }
}
