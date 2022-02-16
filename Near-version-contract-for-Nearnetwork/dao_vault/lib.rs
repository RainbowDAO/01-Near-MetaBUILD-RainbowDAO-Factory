use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen,AccountId,Gas};
use near_sdk::collections::LookupMap;
use near_sdk::serde_json::{json};
const SINGLE_CALL_GAS: Gas = Gas(200000000000000);
/// store a transfer record
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
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
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct TokenInfo {
    erc20: AccountId,
    symbol: String,
    name: String,
    balance: u64,
}
/// Fund management of Dao
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct VaultManager {
    tokens: LookupMap<AccountId, AccountId>,
    visible_tokens: LookupMap<AccountId, AccountId>,
    transfer_history:LookupMap<u64,Transfer>,
    vault_contract_address:AccountId,
}

impl VaultManager {
    pub fn new() -> Self {
        let vault_contract_address = env::current_account_id();
        Self {
            tokens: LookupMap::new(b"r".to_vec()),
            visible_tokens: LookupMap::new(b"r".to_vec()),
            transfer_history:LookupMap::new(b"r".to_vec()),
            vault_contract_address: vault_contract_address,
        }
    }
    /// get the erc20 instance
    pub fn get_erc20_by_address(&self, address:AccountId) -> AccountId {
        self.tokens.get(&address).unwrap()
    }
    /// add a vault token
    pub fn add_vault_token(&mut self,erc_20_address:AccountId) -> bool  {
        let _caller = env::signer_account_id();
        // let is_permission = auth.has_permission(caller,String::from("vault"),String::from("add_vault_token"));
        let is_permission = true;
        if is_permission == false {
            return false;
        }
        match self.tokens.insert(
             &erc_20_address,&self.vault_contract_address
        ) {

            Some(_) => { false},
            None => {
                self.visible_tokens.insert(&erc_20_address,&self.vault_contract_address);
                true
            }
        }
    }
    /// remove a token
    pub fn remove_vault_token(&mut self,erc_20_address: AccountId) -> bool  {
        let _caller = env::signer_account_id();
        //let is_permission = auth.has_permission(caller,String::from("vault"),String::from("remove_vault_token"));
        let is_permission = true;
        if is_permission == false {
            return false;
        }
        self.visible_tokens.remove(&erc_20_address);
        true
    }
    /// show all token
    pub fn get_token_list(self) ->  LookupMap<AccountId, AccountId> {
        self.visible_tokens
    }
    /// get balance of by token
    pub fn get_balance_of(&self,erc_20_address: AccountId) -> u128 {
        if self.tokens.contains_key(&erc_20_address) {
            env::promise_create(
                erc_20_address,
                "balance_of",
                json!({ "owner": self.vault_contract_address }).to_string().as_bytes(),
                0,
                SINGLE_CALL_GAS,
            );
            0
        } else{
            0
        }
    }
    /// deposit a token
    pub fn deposit(&mut self, erc_20_address:AccountId, from_address:AccountId,value:u128) -> bool {
        let to_address = self.vault_contract_address.clone();
        if self.tokens.contains_key(&erc_20_address) {
            let transfer_result = true;
            env::promise_create(
                erc_20_address,
                "transfer_from",
                json!({ "from": from_address,"to":to_address,"value":value }).to_string().as_bytes(),
                0,
                SINGLE_CALL_GAS,
            );
            if transfer_result == false {
                return false;
            }
            let transfer_id:u64 = 0;
            let transfer_time: u64 = env::block_timestamp();
            self.transfer_history.insert(
                &transfer_id,
                 &Transfer{
                     transfer_direction:2,// 1: out 2: in
                     transfer_id:transfer_id,
                     from_address:from_address,
                     to_address:to_address,
                     value,
                     transfer_time});
            true
        } else{
            false
        }
    }
    /// withdraw a token
    pub fn withdraw(&mut self,erc_20_address:AccountId,to_address:AccountId,value:u128) -> bool {
        let from_address = self.vault_contract_address.clone();
        if self.visible_tokens.contains_key(&erc_20_address) {
            let _caller = env::signer_account_id();
            let is_permission = true;
            if is_permission == false {
                return false;
            }
            let transfer_result  = true;
            env::promise_create(
                erc_20_address,
                "transfer_from",
                json!({"to":to_address,"value":value }).to_string().as_bytes(),
                0,
                SINGLE_CALL_GAS,
            );
            if transfer_result == false {
                return false;
            }
            let transfer_id:u64 = 0;
            let transfer_time: u64 = env::block_timestamp();
            self.transfer_history.insert(
                &transfer_id,
                 &Transfer{
                     transfer_direction:1,// 1: out 2: in
                     transfer_id:transfer_id,
                     from_address:from_address,
                     to_address:to_address,
                     value:value,
                     transfer_time:transfer_time});
            true
        } else{
            false
        }
    }
    /// get the history of transfer
    pub fn get_transfer_history(self) -> LookupMap<u64,Transfer> {
        self.transfer_history
    }
}
