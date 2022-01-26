#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;
use ink_lang as ink;

pub use self::dao_manager::DAOManager;

#[allow(unused_imports)]
#[allow(dead_code)]
#[ink::contract]
mod dao_manager {
    use alloc::string::String;
    use template_manager::DAOTemplate;
    use dao_base::DaoBase;
    use dao_users::DaoUsers;
    use dao_setting::DaoSetting;
    use dao_proposal::DaoProposal;
    use erc20::Erc20;
    use dao_vault::VaultManager;
    use ink_prelude::vec::Vec;
    use ink_prelude::collections::BTreeMap;
    use ink_storage::{
        collections::HashMap as StorageHashMap,
        traits::{PackedLayout, SpreadLayout},
    };

    const CONTRACT_INIT_BALANCE: u128 = 1000 * 1_000_000_000_000;


    /// DAO component instances
    /// base:the instance of base
    /// erc20:the instance of erc20
    /// dao_users:the instance of dao_users
    /// dao_setting:the instance of dao_setting
    /// vault:the instance of vault
    #[derive(Debug, scale::Encode, scale::Decode, Clone, SpreadLayout, PackedLayout)]
    #[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    )]
    pub struct DAOComponents {
        pub base: Option<DaoBase>,
        pub erc20:Option<Erc20>,
        pub dao_users:Option<DaoUsers>,
        pub dao_setting:Option<DaoSetting>,
        pub vault: Option<VaultManager>,
        pub proposal: Option<DaoProposal>,
        //    github: Option<Github>,
    }

    ///the base information
    #[derive(
    Debug, Clone, PartialEq, Eq, scale::Encode, scale::Decode, SpreadLayout, PackedLayout, Default
    )]
    #[cfg_attr(
    feature = "std",
    derive(::scale_info::TypeInfo, ::ink_storage::traits::StorageLayout)
    )]
    pub struct BaseParam {
        owner: AccountId,
        name: String,
        logo: String,
        desc: String,
    }
    /// DAO component instance addresses
    #[derive(
    Debug, Clone, PartialEq, Eq, scale::Encode, scale::Decode, SpreadLayout, PackedLayout, Default
    )]
    #[cfg_attr(
    feature = "std",
    derive(::scale_info::TypeInfo, ::ink_storage::traits::StorageLayout)
    )]
    pub struct DAOInitParams {
        base: BaseParam,
        erc20:ERC20Param,
    }


    /// DAO component instance addresses
    #[derive(
    Debug, Copy, Clone, PartialEq, Eq, scale::Encode, scale::Decode, SpreadLayout, PackedLayout, Default
    )]
    #[cfg_attr(
    feature = "std",
    derive(::scale_info::TypeInfo, ::ink_storage::traits::StorageLayout)
    )]
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
    #[derive(
    Debug, Clone, PartialEq, Eq, scale::Encode, scale::Decode, SpreadLayout, PackedLayout, Default
    )]
    #[cfg_attr(
    feature = "std",
    derive(::scale_info::TypeInfo, ::ink_storage::traits::StorageLayout)
    )]
    pub struct ERC20Param {
        owner: AccountId,
        name: String,
        symbol: String,
        total_supply: u128,
        decimals: u8,
    }
    /// the union dao setting
    #[derive(
    Debug, Clone, PartialEq, Eq, scale::Encode, scale::Decode, SpreadLayout, PackedLayout, Default
    )]
    #[cfg_attr(
    feature = "std",
    derive(::scale_info::TypeInfo, ::ink_storage::traits::StorageLayout)
    )]
    pub struct Union {
        open: bool,
        join_limit: u128,
        daos: BTreeMap<AccountId,bool>,
        manager:AccountId
    }
    /// Store important information of Dao
    /// creator:the creator of dao
    /// owner:the owner of dao
    /// template:the template of dao
    /// active:the creator of dao
    /// dao_id:the dao_id of dao
    /// controller_type:the controller_type of dao
    /// components:the components of dao
    /// component_addrs:the component_addrs of dao
    /// category:the category of dao
    /// union:store the union info
    /// childs_dao:HashMap dao'id and child dao's
    #[ink(storage)]
    pub struct DAOManager {
        pub creator:AccountId,
        pub owner: AccountId,
        pub template: Option<DAOTemplate>,
        pub active: bool,
        pub dao_id:u64,
        pub controller_type:u32,
        pub components: DAOComponents,
        pub component_addrs: DAOComponentAddrs,
        pub category:String,
        pub union:Union,
        pub childs_dao:StorageHashMap<AccountId,Vec<AccountId>>
    }

    impl DAOManager {
        /// Create a new dao
        #[ink(constructor)]
        pub fn new(creator:AccountId,owner:AccountId,dao_id:u64,controller_type:u32,category:String) -> Self {
            Self {
                creator,
                owner,
                template:None,
                active:false,
                dao_id,
                controller_type,
                childs_dao:StorageHashMap::new(),
                components:DAOComponents {
                    base: None,
                    erc20:None,
                    dao_users:None,
                    dao_setting:None,
                    vault:None,
                    proposal:None
                },
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
                    daos: BTreeMap::new(),
                    manager:AccountId::default()
                }
            }
        }

        /// change the dao status
        #[ink(message)]
        pub fn operate_join(&mut self,operate:bool) -> bool {
            self.check_dao_category(String::from("union"));
            self.union.open = operate;
            true
        }
        /// get the component_addrs
        #[ink(message)]
        pub fn get_component_addrs(&self) -> DAOComponentAddrs{
            self.component_addrs
        }
        /// get the child daos
        #[ink(message)]
        pub fn get_childs_daos(&self,address:AccountId) -> Vec<AccountId>{
            self.childs_dao.get(&address).unwrap().clone()
        }
        /// set the limit of join dao
        #[ink(message)]
        pub fn set_join_limit(&mut self,limit:u128) -> bool {
            self.check_dao_category(String::from("union"));
            self.union.join_limit = limit;
            true
        }
        /// join the dao
        #[ink(message)]
        pub fn join_union_dao(&mut self,dao:AccountId) -> bool {
            self.union.daos.insert(dao,true);
            true
        }
        /// leave a dao
        #[ink(message)]
        pub fn leave_union_dao(&mut self,dao:AccountId) -> bool {
            self.union.daos.insert(dao,false);
            true
        }
        /// show all union dao
        #[ink(message)]
        pub fn list_union_dao(&mut self) -> Vec<AccountId> {
            let mut dao_vec = Vec::new();
            let mut iter = self.union.daos.values();
            let mut key_iter = self.union.daos.keys();
            let mut category = iter.next();
            let mut name = key_iter.next();
            while category.is_some() {
                if category.unwrap().clone() == true {
                    dao_vec.push(name.unwrap().clone());
                }
                // route_vec.push(route.unwrap().clone());
                category = iter.next();
                name = key_iter.next();
            }
            dao_vec
        }
        /// create a child dao
        #[ink(message)]
        pub fn create_child_dao(&mut self,dao_id:AccountId,child_dao:AccountId) -> bool {
            self.check_dao_category(String::from("mother"));
            let list = self.childs_dao.entry(dao_id.clone()).or_insert(Vec::new());
            list.push(child_dao);
            true
        }
        /// Set the dao use which template
        #[ink(message)]
        pub fn set_template(&mut self, template: DAOTemplate) -> bool {
            assert_eq!(self.active, false);
            self.template = Some(template);
            true
        }

        /// Initialize Dao and generate various
        /// params:Generate basic contract information
        /// version:Random number for generating contract
        #[ink(message)]
        pub fn  init_by_params(
            &mut self,
            params: DAOInitParams,
            version: u128,
            base_code_hash:Hash,
            erc20_code_hash:Hash,
            user_code_hash:Hash,
            setting_code_hash:Hash,
            vault_code_hash:Hash,
            proposal_code_hash:Hash,
        ) -> bool {
            assert!(self.active == false, "not enough unit to instance contract");
            self._init_setting(setting_code_hash,version);
            self._init_base(base_code_hash, params.base, version);
            self._init_erc20(erc20_code_hash, params.erc20, version);
            self._init_user(user_code_hash, version);
            self._init_vault(vault_code_hash, version);
            self._init_proposal(proposal_code_hash, version);
            self.active = true;
            true
        }
        fn _init_proposal(&mut self, proposal_code_hash: Hash, version: u128) -> bool {
            let salt = version.to_le_bytes();
            let total_balance = Self::env().balance();
            assert!(total_balance > CONTRACT_INIT_BALANCE, "not enough unit to instance contract");
            // instance org
            // let salt = version.to_le_bytes();
            let proposal_instance_params = DaoProposal::new(self.env().caller(),self.component_addrs.erc20_addr.unwrap())
                .endowment(CONTRACT_INIT_BALANCE)
                .code_hash(proposal_code_hash)
                .salt_bytes(salt)
                .params();
            let proposal_init_result = ink_env::instantiate_contract(&proposal_instance_params);
            let proposal_addr = proposal_init_result.expect("failed at instantiating the `vault` contract");
            let proposal_instance: DaoProposal = ink_env::call::FromAccountId::from_account_id(proposal_addr);
            self.components.proposal = Some(proposal_instance);
            self.component_addrs.proposal_addr = Some(proposal_addr);
            true
        }
        /// init vault
        fn _init_vault(&mut self, vault_code_hash: Hash, version: u128) -> bool {
            let salt = version.to_le_bytes();
            let total_balance = Self::env().balance();
            assert!(total_balance > CONTRACT_INIT_BALANCE, "not enough unit to instance contract");
            // instance org
            // let salt = version.to_le_bytes();
            let vault_instance_params = VaultManager::new()
                .endowment(CONTRACT_INIT_BALANCE)
                .code_hash(vault_code_hash)
                .salt_bytes(salt)
                .params();
            let vault_init_result = ink_env::instantiate_contract(&vault_instance_params);
            let vault_addr = vault_init_result.expect("failed at instantiating the `vault` contract");
            let vault_instance: VaultManager = ink_env::call::FromAccountId::from_account_id(vault_addr);
            self.components.vault = Some(vault_instance);
            self.component_addrs.vault_addr = Some(vault_addr);
            true
        }

        /// init setting
        fn _init_setting(&mut self, setting_code_hash: Hash, version: u128) -> bool {
            let salt = version.to_le_bytes();
            let total_balance = Self::env().balance();
            assert!(total_balance > CONTRACT_INIT_BALANCE, "not enough unit to instance contract");
            let setting_instance_params = DaoSetting::new(self.env().caller())
                .endowment(CONTRACT_INIT_BALANCE)
                .code_hash(setting_code_hash)
                .salt_bytes(salt)
                .params();
            let setting_init_result = ink_env::instantiate_contract(&setting_instance_params);
            let setting_addr = setting_init_result.expect("failed at instantiating the `setting` contract");
            let  setting_instance: DaoSetting = ink_env::call::FromAccountId::from_account_id(setting_addr);
            self.components.dao_setting = Some(setting_instance);
            self.component_addrs.dao_setting_addr = Some(setting_addr);
            true
        }

        /// init base
        fn _init_base(&mut self, base_code_hash: Hash,
                      param: BaseParam, version: u128) -> bool {
            let salt = version.to_le_bytes();
            let total_balance = Self::env().balance();
            assert!(total_balance > CONTRACT_INIT_BALANCE, "not enough unit to instance contract");
            // instance base
            // let salt = version.to_le_bytes();
            let instance_params = DaoBase::new()
                .endowment(CONTRACT_INIT_BALANCE)
                .code_hash(base_code_hash)
                .salt_bytes(salt)
                .params();
            let init_result = ink_env::instantiate_contract(&instance_params);
            let contract_addr = init_result.expect("failed at instantiating the `Base` contract");
            let mut contract_instance: DaoBase = ink_env::call::FromAccountId::from_account_id(contract_addr);
            contract_instance.init_base(param.name, param.logo, param.desc);

            self.components.base = Some(contract_instance);
            self.component_addrs.base_addr = Some(contract_addr);

            true
        }
        /// init user
        fn _init_user(&mut self,user_code_hash:Hash,version: u128) -> bool {
            let salt = version.to_le_bytes();
            let total_balance = Self::env().balance();
            assert!(total_balance > CONTRACT_INIT_BALANCE, "not enough unit to instance contract");
            // let vault_addr = self.component_addrs.vault_addr.unwrap();
            let user_instance_params = DaoUsers::new(self.component_addrs.dao_setting_addr.unwrap())
                .endowment(CONTRACT_INIT_BALANCE)
                .code_hash(user_code_hash)
                .salt_bytes(salt)
                .params();
            let user_init_result = ink_env::instantiate_contract(&user_instance_params);
            let user_addr = user_init_result.expect("failed at instantiating the `user` contract");
            let  user_instance: DaoUsers = ink_env::call::FromAccountId::from_account_id(user_addr);
            self.components.dao_users = Some(user_instance);
            self.component_addrs.dao_users_addr = Some(user_addr);
            true
        }
        fn check_dao_category(&self,category:String) {
            assert!(self.category == category);
        }
        /// init erc20
        fn _init_erc20(&mut self, erc20_code_hash: Hash,
                       param: ERC20Param, version: u128) -> bool {
            let salt = version.to_le_bytes();

            let total_balance = Self::env().balance();
            assert!(total_balance > CONTRACT_INIT_BALANCE, "not enough unit to instance contract");
            // let vault_addr = self.component_addrs.vault_addr.unwrap();
            let erc20_instance_params = Erc20::new(param.total_supply,param.name,
                                                   param.symbol, param.decimals, param.owner)
                .endowment(CONTRACT_INIT_BALANCE)
                .code_hash(erc20_code_hash)
                .salt_bytes(salt)
                .params();
            let erc20_init_result = ink_env::instantiate_contract(&erc20_instance_params);
            let erc20_addr = erc20_init_result.expect("failed at instantiating the `Erc20` contract");
            let  erc20_instance: Erc20 = ink_env::call::FromAccountId::from_account_id(erc20_addr);
            self.components.erc20 = Some(erc20_instance);
            self.component_addrs.erc20_addr = Some(erc20_addr);
            true
        }
    }

    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;
        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;
        #[ink::test]
        fn it_works() {
            let mut dao_manage = DAOManager::new(AccountId::from([0x01; 32]),AccountId::from([0x01; 32]),1,1,String::from("union"));
            assert!(dao_manage.join_union_dao(AccountId::from([0x01; 32])) == true);
            assert!(dao_manage.leave_union_dao(AccountId::from([0x01; 32])) == true);
        }
    }
}
