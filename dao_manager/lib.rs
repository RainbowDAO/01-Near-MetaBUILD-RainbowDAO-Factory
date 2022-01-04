#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;
use ink_lang as ink;

pub use self::dao_manager::DAOManager;

#[ink::contract]
mod dao_manager {
    use alloc::string::String;
    use template_manager::DAOTemplate;
    use dao_base::DaoBase;
    use ink_prelude::vec::Vec;
    use ink_prelude::collections::BTreeMap;
    use ink_storage::{
        // collections::HashMap as StorageHashMap,
        traits::{PackedLayout, SpreadLayout},
    };

    const CONTRACT_INIT_BALANCE: u128 = 100 * 1000 * 1_000_000_000_000;


    /// DAO component instances
    #[derive(Debug, scale::Encode, scale::Decode, Clone, SpreadLayout, PackedLayout)]
    #[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    )]
    pub struct DAOComponents {
        pub base: Option<DaoBase>,
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
        base: BaseParam
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
        pub base_addr: Option<AccountId>
    }



    #[ink(storage)]
    pub struct DAOManager {
        owner: AccountId,
        template: Option<DAOTemplate>,
        active: bool,
        dao_id:u64,
        components: DAOComponents,
        component_addrs: DAOComponentAddrs,
    }

    impl DAOManager {
        /// Create a new dao
        #[ink(constructor)]
        pub fn new(owner:AccountId,dao_id:u64) -> Self {
            Self {
                owner,
                template:None,
                active:false,
                dao_id,
                components:DAOComponents {
                    base: None
                },
                component_addrs:DAOComponentAddrs{
                    base_addr:None
                }
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
            assert_eq!(owner == self.owner, true);
            let components_hash_map = self.template.as_ref().unwrap().components.clone();
            let base_code_hash = components_hash_map.get("BASE");
            self._init_base(base_code_hash, params.base, &salt);



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
