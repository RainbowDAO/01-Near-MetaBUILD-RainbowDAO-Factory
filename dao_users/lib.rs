#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;
use ink_lang as ink;
pub use self::dao_users::{
    DaoUsers
};
#[ink::contract]
mod dao_users {
    use alloc::string::String;
    use ink_prelude::vec::Vec;
    use dao_setting::DaoSetting;
    use erc20::Erc20;
    use ink_prelude::collections::BTreeMap;
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
    /// Indicates whether a transaction is already confirmed or needs further confirmations.
    #[derive(scale::Encode, scale::Decode, Clone, SpreadLayout, PackedLayout)]
    #[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    )]

    #[derive(Debug)]
    pub struct Group {
        id:u128,
        name:String,
        join_directly:bool,
        is_open:bool,
        users:BTreeMap<AccountId,bool>,
        manager:AccountId
    }

    #[ink(storage)]
    pub struct DaoUsers {
         user:StorageHashMap<AccountId,User>,
       // user_referer:StorageHashMap<AccountId,AccountId>,
       // length:u128,
        setting_addr:AccountId,
        group:StorageHashMap<u128,Group>,
        user_group:StorageHashMap<(AccountId,u128),bool>,
        group_index:u128
    }

    impl DaoUsers {
        #[ink(constructor)]
        pub fn new(setting_addr:AccountId) -> Self {
            Self {
                user:StorageHashMap::new(),
                setting_addr,
                group:StorageHashMap::new(),
                user_group:StorageHashMap::new(),
                group_index:0
            }
        }
        #[ink(message)]
        pub fn add_group(&mut self,group:Group) -> bool {
            let index = self.group_index.clone() + 1;
            self.group_index += 1;
            self.group.insert(index,group);
            true
        }
        #[ink(message)]
        pub fn join(&mut self) ->bool {
            let mut setting_instance: DaoSetting = ink_env::call::FromAccountId::from_account_id(self.setting_addr);
            let condition =  setting_instance.get_conditions();
            let fee_limit = setting_instance.get_fee_setting();
            if condition == 2 {
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
            true
        }
        #[ink(message)]
        pub fn verify_user(&mut self,index:u128,user:AccountId) -> bool {
            let mut group =  self.group.get_mut(&index).unwrap();
            assert_eq!(group.id > 0, true);
            group.users.insert(user,true);
            true
        }

        #[ink(message)]
        pub fn join_group(&mut self,index:u128) -> bool {
            let mut group =  self.group.get_mut(&index).unwrap();
            let caller = Self::env().caller();
            assert_eq!(group.id > 0, true);
            let mut user_group = self.user_group.get_mut(&(caller,index)).unwrap();
            if group.join_directly == false {
                group.users.insert(caller,false);
            }else{
                group.users.insert(caller,true);
            }
            self.user_group.insert((caller,index),true);
            true
        }
        #[ink(message)]
        pub fn list_user(&self) -> Vec<User> {
            let mut user_vec = Vec::new();
            let mut iter = self.user.values();
            let mut user = iter.next();
            while user.is_some() {
                user_vec.push(user.unwrap().clone());
                user = iter.next();
            }
            user_vec
        }
        #[ink(message)]
        pub fn list_group(&self) -> Vec<Group> {
            let mut group_vec = Vec::new();
            let mut iter = self.group.values();
            let mut group = iter.next();
            while group.is_some() {
                group_vec.push(group.unwrap().clone());
                group = iter.next();
            }
            group_vec
        }
        #[ink(message)]
        pub fn close_group(&mut self,id:u128) -> bool {
            let mut group =  self.group.get_mut(&id).unwrap();
            group.is_open = false;
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
