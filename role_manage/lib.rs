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
        role_privileges:StorageHashMap<String,Vec<String>>,
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

        fn only_core(&self,sender:AccountId) {
            assert_eq!(self.owner, sender);
        }

        #[ink(message)]
        pub fn add_role(&mut self, name: String) -> bool {
            self.only_core(Self::env().caller());
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
        pub fn role_insert_privilege(&mut self ,name:String,privilege:String) -> bool {
            self.only_core(Self::env().caller());
            let role_privilege_list = self.role_privileges.entry(name.clone()).or_insert(Vec::new());
            role_privilege_list.push(privilege);
            true
        }

        #[ink(message)]
        pub fn list_role_privileges(&self,name:String) -> Vec<String> {
           let v =  self.role_privileges.get(&name).unwrap().clone();
            v
        }

        #[ink(message)]
        pub fn add_user_role(&mut self,user:AccountId,role:String) -> bool {
            self.only_core(Self::env().caller());
            let user_role_list = self.user_role.entry(user.clone()).or_insert(Vec::new());
            user_role_list.push(role);
            true
        }
        #[ink(message)]
        pub fn check_user_role(&self,user:AccountId,role:String) -> bool {
            let list =  self.get_user_roles(user);
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
        #[ink(message)]
        pub fn check_user_privilege(&self,user:AccountId,privilege:String) -> bool {
            let list =  self.get_user_privilege(user);
            for i in  list{
                if i == privilege {
                    return true
                }
            }
            false
        }
        #[ink(message)]
        pub fn get_user_privilege(&self,user:AccountId) -> Vec<String> {
            let mut privilege_vec = Vec::new();
            // role vec
            let list =  self.user_role.get(&user).unwrap().clone();
            for i in list {
               let mut privileges =  self.role_privileges.get(&i).unwrap().clone();
                privilege_vec.append(&mut privileges);
            }
            privilege_vec
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
        fn add_role_works() {
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                    .expect("Cannot get accounts");
            let mut role_manage = RoleManage::new();
            role_manage.add_role(String::from("test"));
            assert!(role_manage.query_role_by_index(0)== String::from("test"));

        }
        #[ink::test]
        fn add_user_role_works() {
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                    .expect("Cannot get accounts");
            let mut role_manage = RoleManage::new();
            role_manage.add_user_role(accounts.alice,String::from("test"));
            assert!(role_manage.check_user_role(accounts.alice,String::from("test"))== true);

        }
    }
}
