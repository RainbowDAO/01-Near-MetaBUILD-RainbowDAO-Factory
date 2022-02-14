#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
use ink_lang as ink;
//use ink_prelude::vec::Vec;
pub use self::dao_vault::VaultManager;

#[allow(unused_imports)]
#[ink::contract]
mod dao_vault {
    use alloc::string::String;
    use ink_storage::{
        collections::HashMap as StorageHashMap,
        traits::{PackedLayout, SpreadLayout},
    };
    use erc20::Erc20;

    /// store a transfer record
    #[derive(
    Debug, Clone, PartialEq, Eq, scale::Encode, scale::Decode, SpreadLayout, PackedLayout,Default
    )]
    #[cfg_attr(
    feature = "std",
    derive(::scale_info::TypeInfo, ::ink_storage::traits::StorageLayout)
    )]
    pub struct Transfer {
        transfer_id:u64,
        transfer_direction:u64,// 1: out 2 : in
        // token_name: String,
        from_address:AccountId,
        to_address:AccountId,
        value: u128,
        transfer_time:u64,
    }

    /// Token info for query purpose.
    #[derive(
        Debug, Clone, PartialEq, Eq, scale::Encode, scale::Decode, SpreadLayout, PackedLayout,Default
        )]
        #[cfg_attr(
        feature = "std",
        derive(::scale_info::TypeInfo, ::ink_storage::traits::StorageLayout)
        )]
    pub struct TokenInfo {
        erc20: AccountId,
        symbol: String,
        name: String,
        balance: u64,
    }
    /// Fund management of Dao
    #[ink(storage)]
    pub struct VaultManager {
        tokens: StorageHashMap<AccountId, AccountId>,
        visible_tokens: StorageHashMap<AccountId, AccountId>,
        transfer_history:StorageHashMap<u64,Transfer>,
        vault_contract_address:AccountId,
    }

    /// Errors that can occur upon calling this contract.
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
    pub enum Error {
        InvalidTransferRecord,
    }


    #[ink(event)]
    pub struct AddVaultTokenEvent {
        #[ink(topic)]
        token_address: AccountId,

    }
    #[ink(event)]
    pub struct RemoveVaultTokenEvent {
        #[ink(topic)]
        token_address: AccountId,

    }
    #[ink(event)]
    pub struct GetTokenBalanceEvent {
        #[ink(topic)]
        token_address:AccountId,
        #[ink(topic)]
        balance:u128,
    }
    #[ink(event)]
    pub struct DepositTokenEvent {
        #[ink(topic)]
        from_address:AccountId,
        #[ink(topic)]
        value:u128,
    }
    #[ink(event)]
    pub struct WithdrawTokenEvent {
        #[ink(topic)]
        to_address:AccountId,
        #[ink(topic)]
        value:u128,
    }
    impl VaultManager {
        #[ink(constructor)]
        pub fn new() -> Self {
            let vault_contract_address = Self::env().account_id();
            Self {
                tokens: StorageHashMap::default(),
                visible_tokens: StorageHashMap::default(),
                transfer_history: StorageHashMap::default(),
                vault_contract_address: vault_contract_address,
            }
        }
        /// get the erc20 instance
        pub fn get_erc20_by_address(&self, address:AccountId) -> Erc20 {
            let  erc20_instance: Erc20 = ink_env::call::FromAccountId::from_account_id(address);
            erc20_instance
        }
        /// add a vault token
        #[ink(message)]
        pub fn add_vault_token(&mut self,erc_20_address:AccountId) -> bool  {
            let _caller = self.env().caller();
            // let is_permission = auth.has_permission(caller,String::from("vault"),String::from("add_vault_token"));
            let is_permission = true;
            if is_permission == false {
                return false;
            }
            match self.tokens.insert(
                 erc_20_address,self.vault_contract_address
            ) {

                Some(_) => { false},
                None => {
                    self.visible_tokens.insert(erc_20_address,self.vault_contract_address);
                    self.env().emit_event(AddVaultTokenEvent{ token_address:erc_20_address,});
                    true
                }
            }
        }
        /// remove a token
        #[ink(message)]
        pub fn remove_vault_token(&mut self,erc_20_address: AccountId) -> bool  {
            let _caller = self.env().caller();
            //let is_permission = auth.has_permission(caller,String::from("vault"),String::from("remove_vault_token"));
            let is_permission = true;
            if is_permission == false {
                return false;
            }

            match self.visible_tokens.take(&erc_20_address) {
                None => { false}
                Some(_) => {

                    self.env().emit_event(RemoveVaultTokenEvent{
                        token_address:erc_20_address,
                        });
                    true
                }
            }
        }
        /// show all token
        #[ink(message)]
        pub fn get_token_list(&self) -> ink_prelude::vec::Vec<AccountId> {
            self.visible_tokens.keys();
            let mut v:ink_prelude::vec::Vec<AccountId> = ink_prelude::vec::Vec::new();
            for key in self.visible_tokens.keys() {
                v.push(*key)
            }
            v
        }
        /// get balance of by token
        #[ink(message)]
        pub fn get_balance_of(&self,erc_20_address: AccountId) -> u128 {
            if self.tokens.contains_key(&erc_20_address) {
               // let mut erc_20 = self.get_erc20_by_address(*erc_20_address.unwrap());
                let  erc_20 = self.get_erc20_by_address(erc_20_address);
                //let token_name = (&erc_20).name();
                let balanceof = erc_20.balance_of(self.vault_contract_address);
                self.env().emit_event(GetTokenBalanceEvent{
                    token_address:erc_20_address,
                    balance:balanceof,}
                );
                balanceof
            } else{
                0
            }
        }
        /// deposit a token
        #[ink(message)]
        pub fn deposit(&mut self, erc_20_address:AccountId, from_address:AccountId,value:u128) -> bool {
            let to_address = self.vault_contract_address;
            if self.tokens.contains_key(&erc_20_address) {
                let mut erc_20 = self.get_erc20_by_address(erc_20_address);
                let transfer_result = erc_20.transfer_from(from_address,to_address, value);
                if transfer_result == false {
                    return false;
                }
                let transfer_id:u64 = (self.transfer_history.len()+1).into();
                let transfer_time: u64 = self.env().block_timestamp();
                self.transfer_history.insert(
                    transfer_id,
                     Transfer{
                         transfer_direction:2,// 1: out 2: in
                         // token_name:token_name.clone(),
                         transfer_id:transfer_id,
                         from_address:from_address,
                         to_address:to_address,
                         value,
                         transfer_time});
                self.env().emit_event(DepositTokenEvent{
                    // token_name: token_name.clone(),
                    from_address:from_address,
                    value:value});
                true
            } else{
                false
            }
        }
        /// withdraw a token
        #[ink(message)]
        pub fn withdraw(&mut self,erc_20_address:AccountId,to_address:AccountId,value:u128) -> bool {
            let from_address = self.vault_contract_address;
            if self.visible_tokens.contains_key(&erc_20_address) {
                let _caller = self.env().caller();
                let is_permission = true;
                if is_permission == false {
                    return false;
                }
                let mut erc_20 = self.get_erc20_by_address(erc_20_address);
                let transfer_result  = erc_20.transfer(to_address, value);
                if transfer_result == false {
                    return false;
                }
                let transfer_id:u64 = (self.transfer_history.len()+1).into();
                let transfer_time: u64 = self.env().block_timestamp();
                self.transfer_history.insert(
                    transfer_id,
                     Transfer{
                         transfer_direction:1,// 1: out 2: in
                         // token_name: token_name.clone(),
                         transfer_id:transfer_id,
                         from_address:from_address,
                         to_address:to_address,
                         value:value,
                         transfer_time:transfer_time});
                self.env().emit_event(WithdrawTokenEvent{
                    // token_name: token_name.clone(),
                    to_address:to_address,
                    value:value,});
                true
            } else{
                false
            }
        }
        /// get the history of transfer
        #[ink(message)]
        pub fn get_transfer_history(&self) -> ink_prelude::vec::Vec<Transfer> {
            let mut temp_vec = ink_prelude::vec::Vec::new();
            let mut iter = self.transfer_history.values();
            let mut temp = iter.next();
            while temp.is_some() {
                temp_vec.push(temp.unwrap().clone());
                temp = iter.next();
            }
            temp_vec.reverse();
            temp_vec
        }
    }
    /// Unit tests
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;
        // use ink_env::{
        //     call,
        //     test,
        // };
        use ink_lang as ink;

        #[ink::test]
        fn add_token_works() {
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                    .expect("Cannot get accounts");
            // Create a new contract instance.
            // FIXME: using alice instead of auth, please be caution!!
            let mut vault_manager = VaultManager::new();
            vault_manager.add_vault_token(accounts.bob);
            assert_eq!(vault_manager.tokens.len(), 1);
        }


        #[ink::test]
        fn remove_token_works() {
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                    .expect("Cannot get accounts");
            // Create a new contract instance.
            // FIXME: using alice instead of auth, please be caution!!
            let mut vault_manager = VaultManager::new();
            vault_manager.add_vault_token(accounts.bob);
            vault_manager.remove_vault_token(accounts.bob);
            assert_eq!(vault_manager.tokens.len(), 1);
            assert_eq!(vault_manager.visible_tokens.len(), 0);
        }


        #[ink::test]
        fn get_token_list_works() {
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                    .expect("Cannot get accounts");
            // Create a new contract instance.
            // FIXME: using alice instead of auth, please be caution!!
            let mut vault_manager = VaultManager::new();
            vault_manager.add_vault_token(accounts.bob);
            vault_manager.add_vault_token(accounts.alice);
            assert_eq!(vault_manager.get_token_list().len(), 2);
        }
    }
}
