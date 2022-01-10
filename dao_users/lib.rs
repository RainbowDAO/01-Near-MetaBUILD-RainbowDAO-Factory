#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;
use ink_lang as ink;
#[ink::contract]
mod dao_users {
    use alloc::string::String;
    use ink_prelude::vec::Vec;
    use dao_setting::DaoSetting;
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
        addr : AccountId,
        expire_time:u128,
        role:u64
    }


    #[ink(storage)]
    pub struct DaoUsers {
       user:StorageHashMap<AccountId,User>,
       // user_referer:StorageHashMap<AccountId,AccountId>,
       // length:u128,
       setting_addr:AccountId
    }

    impl DaoUsers {
        #[ink(constructor)]
        pub fn new(setting_addr:AccountId) -> Self {
            Self {
                user:StorageHashMap::new(),
                setting_addr
            }
        }
        #[ink(message)]
        pub fn join(&mut self) ->bool {
            let mut setting_instance: DaoSetting = ink_env::call::FromAccountId::from_account_id(setting_addr);
            let condition =  setting_instance.get_conditions();
            if condition == 2 {
                let fee_limit = setting_instance.get_fee_setting();
                let mut erc20_instance: Erc20 = ink_env::call::FromAccountId::from_account_id(fee_limit.token);
                assert_eq!(erc20_instance.balance_of(self.env().caller()) >= fee_limit.fee_limit, true);
                erc20_instance.transfer_from(Self::env().caller(),AccountId::default(),fee_limit.fee_limit); //todo 修改打入地址
                self.user.insert(Self::env().caller(),User{addr:Self::env().caller(),expire_time:0,role:0});//todo 修改时间
            } else if condition == 4 {
                let mut erc20_instance: Erc20 = ink_env::call::FromAccountId::from_account_id(fee_limit.token);
                let other_limit = setting_instance.get_other_setting();
                if other_limit.use_token {
                    assert_eq!(erc20_instance.balance_of(self.env().caller()) >= other_limit.token_balance_limit, true);
                }
                if other_limit.use_nft {

                }
                self.user.insert(Self::env().caller(),User{addr:Self::env().caller(),expire_time:0,role:0});//todo 修改时间
            }else if condition == 6 {

            }else{
                self.user.insert(Self::env().caller(),User{addr:Self::env().caller(),expire_time:0,role:0});//todo 修改时间
            }
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
