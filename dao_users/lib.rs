#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;
use ink_env::Environment;

#[ink::chain_extension]
pub trait FetchRandom {
    type ErrorCode = RandomReadErr;

    /// Note: this gives the operation a corresponding `func_id` (1101 in this case),
    /// and the chain-side chain extension will get the `func_id` to do further operations.
    #[ink(extension = 1101, returns_result = false)]
    fn fetch_random() -> [u8; 32];
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum RandomReadErr {
    FailGetRandomSource,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum CustomEnvironment {}


impl ink_env::chain_extension::FromStatusCode for RandomReadErr {
    fn from_status_code(status_code: u32) -> Result<(), Self> {
        match status_code {
            0 => Ok(()),
            1 => Err(Self::FailGetRandomSource),
            _ => panic!("encountered unknown status code"),
        }
    }
}

impl Environment for CustomEnvironment {
    const MAX_EVENT_TOPICS: usize =
        <ink_env::DefaultEnvironment as Environment>::MAX_EVENT_TOPICS;

    type AccountId = <ink_env::DefaultEnvironment as Environment>::AccountId;
    type Balance = <ink_env::DefaultEnvironment as Environment>::Balance;
    type Hash = <ink_env::DefaultEnvironment as Environment>::Hash;
    type BlockNumber = <ink_env::DefaultEnvironment as Environment>::BlockNumber;
    type Timestamp = <ink_env::DefaultEnvironment as Environment>::Timestamp;
    type RentFraction = <ink_env::DefaultEnvironment as Environment>::RentFraction;

    type ChainExtension = FetchRandom;
}
extern crate alloc;
#[ink::contract(env = crate::CustomEnvironment)]
mod dao_users {
    // use rand::{distributions::Alphanumeric, Rng};
    use super::RandomReadErr;
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
        code:[u8; 32],
        address:AccountId,
        referer:AccountId,
        childs:Vec<AccountId>
    }


    #[ink(storage)]
    pub struct DaoUsers {
       user_info:StorageHashMap<AccountId,User>,
       // user_referer:StorageHashMap<AccountId,AccountId>,
       code_user:StorageHashMap<[u8; 32],AccountId>,
       length:u128
    }

    impl DaoUsers {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                user_info:StorageHashMap::new(),
                code_user:StorageHashMap::new(),
                length:0,
            }
        }
        #[ink(message)]
        pub fn join(&mut self,invitation_code:[u8; 32],name:String,user_profile:String) -> Result<(), RandomReadErr> {
            assert_eq!(self.length + 1 > self.length, true);
            let caller = self.env().caller();
            //let user = self.user_info.get(&caller).unwrap().clone();
            assert_eq!(self.exists_user(caller),false);
            // let code = self.create_code();
            let code =  self.env().extension().fetch_random()?;

            self.code_user.insert(code,caller);
            let referer = if invitation_code.is_empty() { AccountId::default()} else { self.get_user_by_code(invitation_code) };
            let nickname = if name.is_empty() { String::default()} else {name };
            let profile = if user_profile.is_empty() { String::default()} else {user_profile };
            self.user_info.insert(caller, User{id:self.length + 1,nickname,profile,code,address:caller,referer,childs:Vec::new()});
            self.length += 1;
            if referer != AccountId::default() {
                self.insert_user_child(referer,caller);
            }
            Ok(())
        }
        #[ink(message)]
        pub fn get_user_referer(&self,user:AccountId) -> AccountId {
           let user_info : User =  self.user_info.get(&user).unwrap().clone();
            return user_info.referer;
        }
        #[ink(message)]
        pub fn exists_user(&self,user:AccountId) -> bool {
            let user_info = self.user_info.get(&user).unwrap().clone();
            return user_info.id != 0 ;
        }

        #[ink(message)]
        pub fn get_user_by_code(&self,invitation_code:[u8; 32]) -> AccountId {
            self.code_user.get(&invitation_code).unwrap().clone()
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
        #[ink(message)]
        pub fn insert_user_child(&mut self,user:AccountId,child:AccountId) -> bool {
            let mut user_info = self.user_info.get_mut(&user).unwrap().clone();
            user_info.childs.push(child);
            true
        }
        #[ink(message)]
        pub fn set_nickname(&mut self,nickname:String) -> bool {
            let caller = self.env().caller();
            let mut user_info : User =  self.user_info.get_mut(&caller).unwrap().clone();
            user_info.nickname = nickname;
            true
        }


        // fn create_code(&self) -> String {
        //     let s: String = rand::thread_rng()
        //         .sample_iter(&Alphanumeric)
        //         .take(7)
        //         .map(char::from)
        //         .collect();
        //     return s
        // }
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
