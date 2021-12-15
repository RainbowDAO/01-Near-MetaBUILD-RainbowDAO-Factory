#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;
pub use self::multisig::{
    Multisig,
};
use ink_lang as ink;

#[ink::contract]
mod multisig {
    use alloc::string::String;
    use ink_prelude::vec::Vec;
    use ink_prelude::collections::BTreeMap;
    use ink_storage::{
        collections::{
            HashMap as StorageHashMap,
            Vec as StorageVec,
        },
        traits::{
            PackedLayout,
            SpreadLayout,
        },
    };

    #[derive(scale::Encode, scale::Decode, Clone, SpreadLayout, PackedLayout)]
    #[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    )]
    #[derive(Debug)]
    pub struct Transaction {
        status: bool,
        to: AccountId,
        amount: u64,
        signature_count: i32,
        signatures: BTreeMap<AccountId, i32>,
    }


    #[ink(storage)]
    pub struct Multisig {
        owner: AccountId,
        transaction_idx: u64,
        manager: StorageHashMap<AccountId, i32>,
        transactions: StorageHashMap<u64, Transaction>,
        info: StorageHashMap<u64, AccountId>,
        min_sign_count: i32,
    }

    // #[ink(event)]
    // pub struct RequirementChange {
    //     new_requirement: u32,
    // }

    impl Multisig {
        //账户数组、 执行交易的最小多签数
        #[ink(constructor)]
        pub fn new(owners: Vec<AccountId>,min_sign_count: i32,) -> Self {
            let mut map: StorageHashMap<AccountId, i32> = StorageHashMap::new();
            for addr in &owners{
                    map.insert(*addr,1);
                }

            Self {
                owner: Self::env().caller(),
                transaction_idx: 0,
                manager: map,
                transactions: StorageHashMap::new(),
                info: StorageHashMap::new(),
                min_sign_count,
            }
        }
        ///创建交易
        #[ink(message)]
        pub fn creat_transfer(&mut self,to: AccountId ,amount: u64) -> bool {
            self.ensure_caller_is_manager();
            let from = self.env().caller();
            assert_eq!(self.env().balance() >= amount.into(), true);
            self.transactions.insert(self.transaction_idx,
                Transaction{
                    status: false,
                    to,
                    amount,
                    signature_count: 0,
                    signatures: BTreeMap::new(),
                }
            );
            self.transaction_idx += 1;

            true
        }
        ///执行签名
        #[ink(message)]
        pub fn sign_transaction(&mut self, transaction_id: u64) -> bool {
            self.ensure_caller_is_manager();
            let from = self.env().caller();
            let mut t = self.transactions.get_mut(&transaction_id).unwrap();
            assert!(t.status == false, "out!");
            //判断是否已签名
            let if_sign = t.signatures.get(&from);
            assert!(if_sign == None, "out!");

            t.signatures.insert(from, 1);
            t.signature_count += 1;
            let addr = t.to;
            let num = t.amount;
            if t.signature_count >= self.min_sign_count {
                t.status = true;
                self.env().transfer(addr, num.into());
            }

            true
        }

        ///获取交易信息
        #[ink(message)]
        pub fn get_transaction(&self,trans_id: u64) -> Transaction {
            self.transactions.get(&trans_id).unwrap().clone()
        }
        ///添加管理员
        #[ink(message)]
        pub fn add_manage(&mut self,addr: AccountId) -> bool {
            self.ensure_caller_is_owner();
            self.manager.insert(addr, 1);
            true
        }
        ///移除管理员
        #[ink(message)]
        pub fn remove_manage(&mut self,addr: AccountId) -> bool {
            self.ensure_caller_is_owner();
            self.manager.insert(addr, 0);
            true
        }
        ///获取管理员列表
        #[ink(message)]
        pub fn get_manage_list(&self) -> Vec<AccountId> {
            let mut manager_list = Vec::new();
            let mut iter = self.manager.keys();
            let mut role = iter.next();
            while role.is_some() {
                manager_list.push(role.unwrap().clone());
                role = iter.next();
            }
            manager_list
        }
        ///确保调用者是拥有者
        fn ensure_caller_is_owner(&self) -> bool{
            self.owner == self.env().caller()
        }
        ///确保调用者是管理员
        fn ensure_caller_is_manager(&self) -> bool {
            let caller = self.env().caller();
            self.manager.get(&caller) == Some(&1) || self.owner == caller
        }

    }
}
