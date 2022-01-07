#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;
use ink_lang as ink;

pub use self::dao_manager::DAOManager;

#[ink::contract]
mod dao_manager {
    use alloc::string::String;
    use template_manager::DAOTemplate;
    use dao_base::DaoBase;
    use dao_users::DaoUsers;
    use dao_setting::DaoSetting;
    use erc20::Erc20;
    use ink_prelude::vec::Vec;
    use ink_prelude::collections::BTreeMap;
    use ink_storage::{
        collections::HashMap as StorageHashMap,
        traits::{PackedLayout, SpreadLayout},
    };

    const CONTRACT_INIT_BALANCE: u128 = 1000 * 1_000_000_000_000;


    /// DAO component instances
    #[derive(Debug, scale::Encode, scale::Decode, Clone, SpreadLayout, PackedLayout)]
    #[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    )]
    pub struct DAOComponents {
        pub base: Option<DaoBase>,
        pub erc20:Option<Erc20>,
        pub dao_users:Option<DaoUsers>,
        pub dao_setting:Option<DaoSetting>
        //    github: Option<Github>,
    }


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
    }
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
        total_supply: u64,
        decimals: u8,
    }



    #[ink(storage)]
    pub struct DAOManager {
        creator:AccountId,
        owner: AccountId,
        template: Option<DAOTemplate>,
        active: bool,
        dao_id:u64,
        controller_type:u32,
        components: DAOComponents,
        component_addrs: DAOComponentAddrs,
        category:String
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
                components:DAOComponents {
                    base: None,
                    erc20:None,
                    dao_users:None,
                    dao_setting:None,
                },
                component_addrs:DAOComponentAddrs{
                    base_addr:None,
                    erc20_addr:None,
                    dao_users_addr:None,
                    dao_setting_addr:None
                },
                category
            }
        }

        /// Set the dao use which template
        #[ink(message)]
        pub fn set_template(&mut self, template: DAOTemplate) -> bool {
            assert_eq!(self.active, false);
            self.template = Some(template);
            true
        }



        /// Initialize Dao and generate various
        #[ink(message)]
        pub fn  init_by_params(&mut self, params: DAOInitParams, salt: Vec<u8>) -> bool {
            assert_eq!(self.active, false);
            assert_eq!(self.template.is_some(), true);
            let owner = self.env().caller();
            assert_eq!(owner == self.creator, true);
            let components_hash_map = self.template.as_ref().unwrap().components.clone();
            let base_code_hash = components_hash_map.get("BASE");
            let erc20_code_hash = components_hash_map.get("ERC20");
            let user_code_hash = components_hash_map.get("USER");
            let setting_code_hash = components_hash_map.get("SETTING");
            self._init_base(base_code_hash, params.base, &salt);
            self._init_erc20(erc20_code_hash, params.erc20, &salt);
            self._init_user(user_code_hash, &salt);

            true
        }

        /// init setting
        fn _init_setting(&mut self, setting_code_hash: Option<&Hash>, salt: &Vec<u8>) -> bool {
            if setting_code_hash.is_none() {
                return true;
            }
            let setting_code_hash = setting_code_hash.unwrap().clone();
            let total_balance = Self::env().balance();
            assert!(total_balance > CONTRACT_INIT_BALANCE, "not enough unit to instance contract");
            let setting_instance_params = DaoSetting::new(self.env().caller())
                .endowment(CONTRACT_INIT_BALANCE)
                .code_hash(setting_code_hash)
                .salt_bytes(salt)
                .params();
            let setting_init_result = ink_env::instantiate_contract(&setting_instance_params);
            let setting_addr = setting_init_result.expect("failed at instantiating the `setting` contract");
            let mut setting_instance: DaoSetting = ink_env::call::FromAccountId::from_account_id(setting_addr);
            self.components.dao_setting = Some(setting_instance);
            self.component_addrs.dao_setting_addr = Some(setting_addr);
            true
        }

        /// init base
        fn _init_base(&mut self, base_code_hash: Option<&Hash>,
                      param: BaseParam, salt: &Vec<u8>) -> bool {
            if base_code_hash.is_none() {
                return true;
            }
            let base_code_hash = base_code_hash.unwrap().clone();
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
        fn _init_user(&mut self,user_code_hash:Option<&Hash>,salt: &Vec<u8>) -> bool {
            if user_code_hash.is_none() {
                return true;
            }
            let user_code_hash = user_code_hash.unwrap().clone();
            let total_balance = Self::env().balance();
            assert!(total_balance > CONTRACT_INIT_BALANCE, "not enough unit to instance contract");
            // let vault_addr = self.component_addrs.vault_addr.unwrap();
            let user_instance_params = DaoUsers::new(self.component_addrs.dao_setting_addr)
                .endowment(CONTRACT_INIT_BALANCE)
                .code_hash(user_code_hash)
                .salt_bytes(salt)
                .params();
            let user_init_result = ink_env::instantiate_contract(&user_instance_params);
            let user_addr = user_init_result.expect("failed at instantiating the `user` contract");
            let mut user_instance: DaoUsers = ink_env::call::FromAccountId::from_account_id(user_addr);
            self.components.dao_users = Some(user_instance);
            self.component_addrs.dao_users_addr = Some(user_addr);
            true
        }



        /// init erc20
        fn _init_erc20(&mut self, erc20_code_hash: Option<&Hash>,
                       param: ERC20Param, salt: &Vec<u8>) -> bool {
            if erc20_code_hash.is_none() {
                return true;
            }
            let erc20_code_hash = erc20_code_hash.unwrap().clone();
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
            let mut erc20_instance: Erc20 = ink_env::call::FromAccountId::from_account_id(erc20_addr);
            self.components.erc20 = Some(erc20_instance);
            self.component_addrs.erc20_addr = Some(erc20_addr);
            true
        }

    }

    // /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    // /// module and test functions are marked with a `#[test]` attribute.
    // /// The below code is technically just normal Rust code.
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
    //         let daoManager = DaoManage::default();
    //         assert_eq!(daoManage.get(), false);
    //     }
    //
    //     /// We test a simple use case of our contract.
    //     #[ink::test]
    //     fn it_works() {
    //         let mut daoManage = DaoManage::new(false);
    //         assert_eq!(daoManage.get(), false);
    //         daoManage.flip();
    //         assert_eq!(daoManage.get(), true);
    //     }
    // }
}
