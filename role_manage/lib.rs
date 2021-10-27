#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;
pub use self::role_manage::{
    RoleManage,
    // RoleManageRef,
};
use ink_lang as ink;

#[ink::contract]
mod role_manage {
    use alloc::string::String;
    use ink_prelude::vec::Vec;
    use ink_prelude::collections::BTreeMap;
    use ink_storage::{collections::HashMap as StorageHashMap, };



    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct RoleManage {
        owner:AccountId,
        index:u64,
        role_map:StorageHashMap<u64,String>,
        role_privileges:StorageHashMap<u64,Vec<String>>,
        user_role:StorageHashMap<AccountId,Vec<String>>,
    }

    impl RoleManage {
        #[ink(constructor)]
        pub fn new() -> Self {
            let from = Self::env().caller();
            let instance = Self {
                owner:from,
                index: 0,
                role_map : StorageHashMap::new(),
                role_privileges: StorageHashMap::new(),
                user_role: StorageHashMap::new()
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
        pub fn role_insert_privilege(&mut self ,index:u64,privilege:String) -> bool {
            let role_privilege_list = self.role_privileges.entry(index.clone()).or_insert(Vec::new());
            role_privilege_list.push(privilege);
            true
        }

        #[ink(message)]
        pub fn list_role_privileges(&self,index:u64) -> Vec<String> {
           let v =  self.role_privileges.get(&index).unwrap().clone();
            v
        }

        #[ink(message)]
        pub fn add_user_role(&mut self,user:AccountId,role:String) -> bool {
            let user_role_list = self.user_role.entry(user.clone()).or_insert(Vec::new());
            user_role_list.push(role);
            true
        }
        #[ink(message)]
        pub fn check_user_role(&self,user:AccountId,role:String) -> bool {
            let list =  self.user_role.get(&user).unwrap().clone();
            for i in  list{
                if i == role {
                    return true
                }
            }
            false
        }
        #[ink(message)]
        pub fn get_user_roles(&self,user:AccountId) -> Vec<String> {
           let list =  self.user_role.get(&user).unwrap().clone();
            list
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
