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
        role_privileges:StorageHashMap<u64,Vec<u64>>,
        user_role:StorageHashMap<AccountId,Vec<u64>>,
    }

    impl RoleManage {
        #[ink(constructor)]
        pub fn new() -> Self {
            let from = Self::env().caller();
            let instance = Self {
                owner:from,
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
        pub fn role_insert_privilege(&mut self ,index:u64,privilege_index:u64) -> bool {

            if   !self.role_privileges.contains_key(&index) {
                let mut privilege_vec = Vec::new();
                privilege_vec.push(privilege_index);
                self.role_privileges.insert(index, privilege_vec);
            } else {
                let  mut  v =self.role_privileges.get_mut(&index).unwrap().clone();
                v.push(privilege_index);
            }



            // match self.role_privileges.get_mut(&index) {
            //     Some(roles_by_index) => {
            //         roles_by_index.push(privilege_index);
            //     },
            //     None => {
            //         self.role_privileges.insert(index, vec![privilege_index]);
            //     }
            // }
            true
        }

        #[ink(message)]
        pub fn list_role_privileges(&self,index:u64) -> Vec<u64> {
            // let v = self.role_privileges.get(&index).unwrap().clone();
            // let mut privilege_vec:Vec<u64> = Vec::new();
            //
            // for i in &v {
            //     privilege_vec.push(i)
            // }
            // self.role_privileges.get(&index).values().cloned().collect();
           let v:Vec<u64> =  self.role_privileges.get(&index).unwrap().clone();
            // privilege_vec
            v
        }

        #[ink(message)]
        pub fn add_user_role(&self,user:AccountId,role:u64) -> bool {
            if   !self.user_role.contains_key(&user) {
                let mut role_vec = Vec::new();
                role_vec.push(role);
                self.user_role.insert(user, privilege_vec);
            } else {
                let  mut  v =self.user_role.get_mut(&user).unwrap().clone();
                v.push(role);
            }
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
