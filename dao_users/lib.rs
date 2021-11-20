#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;
use ink_lang as ink;

#[ink::contract]
mod dao_users {
    // use rand::{distributions::Alphanumeric, Rng};
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
        address:AccountId,
        referer:AccountId,
        childs:Vec<AccountId>
    }


    #[ink(storage)]
    pub struct DaoUsers {
       user_info:StorageHashMap<AccountId,User>,
       // user_referer:StorageHashMap<AccountId,AccountId>,
       code_user:StorageHashMap<String,AccountId>,
        length:u128
    }

    impl DaoUsers {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                user_info:StorageHashMap::new(),
                code_user:StorageHashMap::new(),
                length:0
            }
        }
        #[ink(message)]
        pub fn join(&mut self,invitation_code:String,nickname:String) -> bool {
            assert_eq!(self.length + 1 > self.length, true);
            let caller = self.env().caller();
            let user = self.user_info.get(&caller).unwrap().clone();
            assert!(user.id != 0);
            let code = self.create_code();

            self.code_user.insert(code,caller);
            let referer = if invitation_code.is_empty() { AccountId::default()} else { self.code_user.get(&invitation_code).unwrap().clone() };
            self.user_info.insert(caller,User{id:self.length + 1,nickname,address:caller,referer,childs:Vec::new()});
            self.length += 1;
            true
        }
        #[ink(message)]
        pub fn get_user_referer(&self,user:AccountId) -> AccountId {
           let user_info : User =  self.user_info.get(&user).unwrap().clone();
            return user_info.referer;
        }

        fn create_code(&self) -> String {
            let s: String = rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(7)
                .map(char::from)
                .collect();
            return s
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
    //         let daoUsers = DaoUsers::default();
    //         assert_eq!(daoUsers.get(), false);
    //     }
    //
    //     /// We test a simple use case of our contract.
    //     #[ink::test]
    //     fn it_works() {
    //         let mut daoUsers = DaoUsers::new(false);
    //         assert_eq!(daoUsers.get(), false);
    //         daoUsers.flip();
    //         assert_eq!(daoUsers.get(), true);
    //     }
    // }
}
