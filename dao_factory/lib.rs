#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;
use ink_lang as ink;
#[allow(unused_imports)]
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

    const TEMPLATE_INIT_BALANCE: u128 = 1000 * 1_000_000_000_000;
    const DAO_INIT_BALANCE: u128 = 1000 * 1_000_000_000_000;

    ///Initialization information of Dao
    /// owner:the manager of the dao
    /// size:the size of the dao
    /// name:the name of the dao
    /// logo:the logo of the dao
    /// desc:the introduce of the dao
    /// dao_manager:the object of the dao
    /// dao_manager_addr:the address of the dao
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
    ///This is the factory contract used to generate Dao
    ///owner:the manager of the contract
    ///template_addr:the template's address of the dao
    ///template:the template's object of the dao
    ///instance_index:the instance index of  all dao
    ///instance_map:HashMap of index and DAOInstance
    ///instance_map_by_owner:HashMap of user and address
    #[ink(storage)]
    pub struct DaoFactory {
        owner: AccountId,
        template_addr: Option<AccountId>,
        template: Option<TemplateManager>,
        instance_index:u64,
        instance_map: StorageHashMap<u64, DAOInstance>,
        instance_map_by_owner: StorageHashMap<AccountId, Vec<u64>>,
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
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                owner: Self::env().caller(),
                template_addr: None,
                template: None,
                instance_index:0,
                instance_map: StorageHashMap::new(),
                instance_map_by_owner: StorageHashMap::new(),
            }
        }

        ///Initialize factory contract
        ///template_code_hash:the hash of the template contract
        ///version:The random number used to generate the contract
        #[ink(message)]
        pub fn  init_factory (&mut self, template_code_hash: Hash, version:u128) -> bool
        {
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
        ///Generate a Dao through a template
        ///index:the index of the template
        ///controller:the manager of the dao
        ///controller_type:the manager's category of the dao
        ///category:the category of the dao
        #[ink(message)]
        pub fn init_dao_by_template(
            &mut self,
            dao_manager_code_hash:Hash,
            controller: AccountId,
            controller_type:u32,
            category:String
        ) -> bool {
            assert_eq!(self.instance_index + 1 > self.instance_index, true);
            let salt = self.instance_index.to_le_bytes();
            let dao_instance_params = DAOManager::new(self.env().caller(),controller, self.instance_index,controller_type,category)
                .endowment(DAO_INIT_BALANCE)
                .code_hash(dao_manager_code_hash)
                .salt_bytes(salt)
                .params();
            let dao_init_result = ink_env::instantiate_contract(&dao_instance_params);
            let dao_addr = dao_init_result.expect("failed at instantiating the `DAO Instance` contract");
            let  dao_instance: DAOManager = ink_env::call::FromAccountId::from_account_id(dao_addr);
            // dao_instance.set_template(template);
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
        ///Find templates through index
        ///index:the index of template
        #[ink(message)]
        pub fn query_template_by_index(&self, index: u64) -> DAOTemplate {
            self.template.as_ref().unwrap().query_template_by_index(index)
        }

        /// get dao by id
        #[ink(message)]
        pub fn get_dao_by_index(&self,id:u64) -> DAOInstance {
            self.instance_map.get(&id).unwrap().clone()
        }
        /// show list of user create
        #[ink(message)]
        pub fn get_daos_by_owner(&self) -> Vec<u64> {
            let user = self.env().caller();
            let list = self.instance_map_by_owner.get(&user).unwrap().clone();
            list
        }

        /// show all daos
        #[ink(message)]
        pub fn list_dao(&self) -> Vec<DAOInstance> {
            let mut dao_vec = Vec::new();
            let mut iter = self.instance_map.values();
            let mut dao = iter.next();
            while dao.is_some() {
                dao_vec.push(dao.unwrap().clone());
                dao = iter.next();
            }
            dao_vec
        }
        /// create a record after user join
        #[ink(message)]
        pub fn joined_dao(&mut self,index:u64) -> bool {
            let user = self.env().caller();
            let  id_list = self.instance_map_by_owner.entry(user.clone()).or_insert(Vec::new());
            id_list.push(index);
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
            let mut dao_factory = DaoFactory::new();
            assert!(dao_factory.joined_dao() == true);
        }
    }
}
