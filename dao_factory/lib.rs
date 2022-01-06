#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;
use ink_lang as ink;

#[ink::contract]
mod dao_factory {

    use alloc::string::String;
    use ink_prelude::vec::Vec;
    use ink_prelude::collections::BTreeMap;
    use ink_storage::{
        traits::{
            PackedLayout,
            SpreadLayout,
        },
        collections::HashMap as StorageHashMap,
    };
    use template_manager::TemplateManager;
    use template_manager::DAOTemplate;
    use dao_manager::DAOManager;

    const TEMPLATE_INIT_BALANCE: u128 = 1000 * 1000 * 1_000_000_000_000;
    const DAO_INIT_BALANCE: u128 = 1000 * 1000 * 1_000_000_000_000;

    /// Indicates whether a transaction is already confirmed or needs further confirmations.
    #[derive(scale::Encode, scale::Decode, Clone, SpreadLayout, PackedLayout)]
    #[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    )]

    #[derive(Debug)]
    pub struct DAOInstance {
        id: u64,
        owner: AccountId,
        size: u64,
        name: String,
        logo: String,
        desc: String,
        dao_manager: DAOManager,
        dao_manager_addr: AccountId,
    }

    #[ink(storage)]
    pub struct DaoFactory {
        owner: AccountId,
        template_addr: Option<AccountId>,
        template: Option<TemplateManager>,
        instance_index:u64,
        instance_map: StorageHashMap<u64, DAOInstance>,
        instance_map_by_owner: StorageHashMap<AccountId, Vec<u64>>,
        route_addr:AccountId
    }



    #[ink(event)]
    pub struct InstanceDAO {
        #[ink(topic)]
        index: u64,
        #[ink(topic)]
        owner: Option<AccountId>,
        #[ink(topic)]
        dao_addr: AccountId,
    }

    impl DaoFactory {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(route_addr:AccountId) -> Self {
            Self {
                owner: Self::env().caller(),
                template_addr: None,
                template: None,
                instance_index:0,
                instance_map: StorageHashMap::new(),
                instance_map_by_owner: StorageHashMap::new(),
                route_addr
            }
        }

        #[ink(message)]
        pub fn  init_factory (&mut self, template_code_hash: Hash, version:u8) -> bool
        {
            // instance template_manager
            let salt = version.to_le_bytes();
            let instance_params = TemplateManager::new(self.owner)
                .endowment(TEMPLATE_INIT_BALANCE)
                .code_hash(template_code_hash)
                .salt_bytes(&salt)
                .params();
            let init_result = ink_env::instantiate_contract(&instance_params);
            let contract_addr = init_result.expect("failed at instantiating the `TemplateManager` contract");
            let contract_instance = ink_env::call::FromAccountId::from_account_id(contract_addr);
            self.template = Some(contract_instance);
            self.template_addr = Some(contract_addr);
            true
        }

        pub fn init_dao_by_template(&mut self, index: u64, controller: AccountId,controller_type:u32,category:String) -> bool {
            assert_eq!(self.instance_index + 1 > self.instance_index, true);
            // let total_balance = Self::env().balance();
            // assert_eq!(total_balance >= 20, true);

            // instance dao_manager
            let template = self.query_template_by_index(index);
            let dao_manager_code_hash = template.dao_manager_code_hash;
            let salt = self.instance_index.to_le_bytes();
            let dao_instance_params = DAOManager::new(self.env().caller(),controller, self.instance_index,controller_type)
                .endowment(DAO_INIT_BALANCE)
                .code_hash(dao_manager_code_hash)
                .salt_bytes(salt)
                .params();
            let dao_init_result = ink_env::instantiate_contract(&dao_instance_params);
            let dao_addr = dao_init_result.expect("failed at instantiating the `DAO Instance` contract");
            let mut dao_instance: DAOManager = ink_env::call::FromAccountId::from_account_id(dao_addr);
            dao_instance.set_template(template);
            self.env().emit_event(InstanceDAO {
                index: self.instance_index,
                owner: Some(controller),
                dao_addr: dao_addr,
            });

            let id_list = self.instance_map_by_owner.entry(controller.clone()).or_insert(Vec::new());
            id_list.push(self.instance_index);
            self.instance_map.insert(self.instance_index, DAOInstance {
                id: self.instance_index,
                owner: controller,
                size: 0,
                name: String::from(""),
                logo: String::from(""),
                desc: String::from(""),
                dao_manager: dao_instance,
                dao_manager_addr: dao_addr,
            });
            self.instance_index += 1;
            true
        }

        #[ink(message)]
        pub fn query_template_by_index(&self, index: u64) -> DAOTemplate {
            self.template.as_ref().unwrap().query_template_by_index(index)
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
    //         let daoFactory = DaoFactory::default();
    //         assert_eq!(daoFactory.get(), false);
    //     }
    //
    //     /// We test a simple use case of our contract.
    //     #[ink::test]
    //     fn it_works() {
    //         let mut daoFactory = DaoFactory::new(false);
    //         assert_eq!(daoFactory.get(), false);
    //         daoFactory.flip();
    //         assert_eq!(daoFactory.get(), true);
    //     }
    // }
}
