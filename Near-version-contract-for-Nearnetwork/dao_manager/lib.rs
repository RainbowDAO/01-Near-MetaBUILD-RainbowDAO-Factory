use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen,AccountId,Promise};
use near_sdk::collections::LookupMap;


const CONTRACT_INIT_BALANCE: u128 = 1000 * 1_000_000_000_000;

///the base information
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct BaseParam {
    owner: AccountId,
    name: String,
    logo: String,
    desc: String,
}
/// DAO component instance addresses
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct DAOInitParams {
    base: BaseParam,
    erc20:ERC20Param,
}


/// DAO component instance addresses
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct DAOComponentAddrs {
    // base module contract's address
    pub base_addr: Option<AccountId>,
    pub erc20_addr: Option<AccountId>,
    pub dao_users_addr: Option<AccountId>,
    pub dao_setting_addr: Option<AccountId>,
    // pub vault: Option<VaultManager>,
    // vote module contract's address
    pub vault_addr: Option<AccountId>,
    pub proposal_addr: Option<AccountId>,
}
/// the erc20 param
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct ERC20Param {
    owner: AccountId,
    name: String,
    symbol: String,
    total_supply: u128,
    decimals: u8,
}
/// the union dao setting
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Union {
    open: bool,
    join_limit: u128,
    daos: LookupMap<AccountId,bool>,
    manager:AccountId
}
/// Store important information of Dao
/// creator:the creator of dao
/// owner:the owner of dao
/// active:the creator of dao
/// dao_id:the dao_id of dao
/// controller_type:the controller_type of dao
/// components:the components of dao
/// component_addrs:the component_addrs of dao
/// category:the category of dao
/// union:store the union info
/// childs_dao:HashMap dao'id and child dao's
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct DAOManager {
    pub creator:AccountId,
    pub owner: AccountId,
    pub active: bool,
    pub dao_id:u64,
    pub controller_type:u32,
    pub component_addrs: DAOComponentAddrs,
    pub category:String,
    pub union:Union,
    pub childs_dao:LookupMap<AccountId,Vec<AccountId>>
}

impl DAOManager {
    /// Create a new dao
    pub fn new(creator:AccountId,owner:AccountId,dao_id:u64,controller_type:u32,category:String) -> Self {
        Self {
            creator,
            owner,
            active:false,
            dao_id,
            controller_type,
            childs_dao:LookupMap::new(b"r".to_vec()),
            component_addrs:DAOComponentAddrs{
                base_addr:None,
                erc20_addr:None,
                dao_users_addr:None,
                dao_setting_addr:None,
                vault_addr:None,
                proposal_addr:None
            },
            category,
            union:Union{
                open: false,
                join_limit: 0,
                daos: LookupMap::new(b"r".to_vec()),
                manager: env::current_account_id()
            }
        }
    }
    /// get the category of dao
    pub fn get_dao_category(&mut self) -> String {
        self.category.clone()
    }

    /// change the dao status
    pub fn operate_join(&mut self,operate:bool) -> bool {
        self.check_dao_category(String::from("union"));
        self.union.open = operate;
        true
    }
    /// get the component_addrs
    pub fn get_component_addrs(self) -> DAOComponentAddrs{
        self.component_addrs
    }
    /// get the child daos

    pub fn get_childs_daos(&self,address:AccountId) -> Vec<AccountId>{
        self.childs_dao.get(&address).unwrap().clone()
    }
    /// set the limit of join dao

    pub fn set_join_limit(&mut self,limit:u128) -> bool {
        self.check_dao_category(String::from("union"));
        self.union.join_limit = limit;
        true
    }
    /// join the dao

    pub fn join_union_dao(&mut self,dao:AccountId) -> bool {
        self.union.daos.insert(&dao,&true);
        true
    }
    /// leave a dao
    pub fn leave_union_dao(&mut self,dao:AccountId) -> bool {
        self.union.daos.insert(&dao,&false);
        true
    }
    /// show all union dao
    pub fn list_union_dao(self) -> LookupMap<AccountId,bool> {
        self.union.daos
    }
    /// create a child dao
    pub fn create_child_dao(&mut self,dao_id:AccountId,child_dao:AccountId) -> bool {
        self.check_dao_category(String::from("mother"));
        let mut list = self.childs_dao.get(&dao_id).unwrap();
        list.push(child_dao);
        true
    }
    /// Initialize Dao and generate various
    /// params:Generate basic contract information
    /// version:Random number for generating contract
    pub fn  init_by_params(
        &mut self,
        base_code_hash:Vec<u8>,
        erc20_code_hash:Vec<u8>,
        user_code_hash:Vec<u8>,
        setting_code_hash:Vec<u8>,
        vault_code_hash:Vec<u8>,
        proposal_code_hash:Vec<u8>,
    ) -> bool {
        assert!(self.active == false, "not enough unit to instance contract");
        self._init_setting(setting_code_hash);
        self._init_base(base_code_hash);
        self._init_erc20(erc20_code_hash);
        self._init_user(user_code_hash);
        self._init_vault(vault_code_hash);
        self._init_proposal(proposal_code_hash);
        self.active = true;
        true
    }
    fn _init_proposal(&mut self, proposal_code_hash: Vec<u8>) -> bool {
        let subaccount_id = AccountId::new_unchecked(
            format!("{}.{}", env::current_account_id(), env::current_account_id())
        );
        Promise::new(subaccount_id.clone())
            .create_account()
            .add_full_access_key(env::signer_account_pk())
            .transfer(CONTRACT_INIT_BALANCE)
            .deploy_contract(proposal_code_hash);
        self.component_addrs.proposal_addr = Some(subaccount_id);
        true
    }
    /// init vault
    fn _init_vault(&mut self, vault_code_hash: Vec<u8>) -> bool {
        let subaccount_id = AccountId::new_unchecked(
            format!("{}.{}", env::current_account_id(), env::current_account_id())
        );
        Promise::new(subaccount_id.clone())
            .create_account()
            .add_full_access_key(env::signer_account_pk())
            .transfer(CONTRACT_INIT_BALANCE)
            .deploy_contract(vault_code_hash);
        self.component_addrs.vault_addr = Some(subaccount_id);
        true
    }

    /// init setting
    fn _init_setting(&mut self, setting_code_hash: Vec<u8>) -> bool {

        let subaccount_id = AccountId::new_unchecked(
            format!("{}.{}", env::current_account_id(), env::current_account_id())
        );

        Promise::new(subaccount_id.clone())
            .create_account()
            .add_full_access_key(env::signer_account_pk())
            .transfer(CONTRACT_INIT_BALANCE)
            .deploy_contract(setting_code_hash);

        self.component_addrs.dao_setting_addr = Some(subaccount_id);
        true
    }

    /// init base
    fn _init_base(&mut self, base_code_hash: Vec<u8>) -> bool {
        let subaccount_id = AccountId::new_unchecked(
            format!("{}.{}", env::current_account_id(), env::current_account_id())
        );

        Promise::new(subaccount_id.clone())
            .create_account()
            .add_full_access_key(env::signer_account_pk())
            .transfer(CONTRACT_INIT_BALANCE)
            .deploy_contract(base_code_hash);

        self.component_addrs.base_addr = Some(subaccount_id);

        true
    }
    /// init user
    fn _init_user(&mut self,user_code_hash:Vec<u8>) -> bool {
        let subaccount_id = AccountId::new_unchecked(
            format!("{}.{}", env::current_account_id(), env::current_account_id())
        );

        Promise::new(subaccount_id.clone())
            .create_account()
            .add_full_access_key(env::signer_account_pk())
            .transfer(CONTRACT_INIT_BALANCE)
            .deploy_contract(user_code_hash);
        self.component_addrs.dao_users_addr = Some(subaccount_id);
        true
    }
    fn check_dao_category(&self,category:String) {
        assert!(self.category == category);
    }
    /// init erc20
    fn _init_erc20(&mut self, erc20_code_hash: Vec<u8>) -> bool {

        let subaccount_id = AccountId::new_unchecked(
            format!("{}.{}", env::current_account_id(), env::current_account_id())
        );

        Promise::new(subaccount_id.clone())
            .create_account()
            .add_full_access_key(env::signer_account_pk())
            .transfer(CONTRACT_INIT_BALANCE)
            .deploy_contract(erc20_code_hash);
        self.component_addrs.erc20_addr = Some(subaccount_id);
        true
    }
}
