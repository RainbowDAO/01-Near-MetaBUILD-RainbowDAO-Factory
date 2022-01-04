#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;
use ink_lang as ink;

#[ink::contract]
mod users_manage {
    use alloc::string::String;
    use ink_prelude::vec::Vec;
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
    pub struct User {
        id:u128,
        nickname:String,
        profile:String,
        code:u128,
        address:AccountId,
        referer:AccountId,
        childs:Vec<AccountId>
    }


    #[ink(storage)]
    pub struct UsersManage {
        user_info:StorageHashMap<AccountId,User>,
        // user_referer:StorageHashMap<AccountId,AccountId>,
        code_user:StorageHashMap<u128, AccountId>,
        length:u128
    }

    impl UsersManage {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                user_info:StorageHashMap::new(),
                code_user:StorageHashMap::new(),
                length:0,
            }
        }
        #[ink(message)]
        pub fn join(&mut self,invitation_code:u128,name:String,user_profile:String) -> bool {
            assert_eq!(self.length + 1 > self.length, true);
            let caller = Self::env().caller();
            assert_eq!(self.exists_user(caller),false);
            let code =  self.length + 1;
            self.code_user.insert(code,caller);
            let referer = if invitation_code == 0 { AccountId::default()} else { self.get_user_by_code(invitation_code) };
            let nickname = if name.is_empty() { String::default()} else {name };
            let profile = if user_profile.is_empty() { String::default()} else {user_profile };
            self.user_info.insert(
                caller,
                User{
                    id:self.length + 1,
                    nickname,
                    profile,
                    code,
                    address:caller,
                    referer,
                    childs:Vec::new()
                }
            );
            self.length += 1;
            if referer != AccountId::default() {
                self.insert_user_child(referer,caller);
            }
            true
        }
        #[ink(message)]
        pub fn get_user_referer(&self,user:AccountId) -> AccountId {
            let user_info : User =  self.user_info.get(&user).unwrap().clone();
            return user_info.referer;
        }
        #[ink(message)]
        pub fn exists_user(&self,user:AccountId) -> bool {
            let user_info = User{
                id:0,
                nickname:String::from(""),
                profile:String::from(""),
                code:0,
                address:AccountId::default(),
                referer:AccountId::default(),
                childs:Vec::new()
            };
            let exists_user =  self.user_info.get(&user).unwrap_or(&user_info);
            return exists_user.id !=0 ;
        }

        #[ink(message)]
        pub fn get_user_by_code(&self,invitation_code:u128) -> AccountId {
            self.code_user.get(&invitation_code).copied().unwrap_or(AccountId::default())
        }
        #[ink(message)]
        pub fn list_user(&self) -> Vec<User> {
            let mut user_vec = Vec::new();
            let mut iter = self.user_info.values();
            let mut user = iter.next();
            while user.is_some() {
                user_vec.push(user.unwrap().clone());
                user = iter.next();
            }
            user_vec
        }

        fn insert_user_child(&mut self,user:AccountId,child:AccountId) -> bool {
            let mut user_info = self.user_info.get_mut(&user).unwrap().clone();
            user_info.childs.push(child);
            true
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
        fn join_works() {
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                    .expect("Cannot get accounts");
            let mut users_manage = UsersManage::new();
            users_manage.join(0,String::from("test"),String::from("test"));
            assert!(users_manage.get_user_by_code(1) != AccountId::default());
            assert!(users_manage.exists_user(accounts.alice) == true);
            assert!(users_manage.get_user_referer(accounts.alice) == AccountId::default());
        }
    }
}
