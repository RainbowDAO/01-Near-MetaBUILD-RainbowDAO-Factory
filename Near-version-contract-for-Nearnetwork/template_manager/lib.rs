#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
use ink_lang as ink;
pub use self::template_manager::TemplateManager;
pub use self::template_manager::DAOTemplate;

#[allow(unused_imports)]
#[ink::contract]
mod template_manager {
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

    /// store the template info
    /// id:the id of template
    /// owner:the owner of the template
    /// name:the name of the template
    /// dao_manager_code_hash:the hash of the dao manager
    /// components:the  of the template
    #[derive(Debug, scale::Encode, scale::Decode, Clone, SpreadLayout, PackedLayout)]
    #[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    )]
    pub struct DAOTemplate {
        // template's id
        pub id: u64,
        // template's owner
        pub owner: AccountId,
        // template's name
        pub name: String,
        // template's dao manager
        pub dao_manager_code_hash: Hash,
        // components code hash
        // like { "ERC20": 0xqw...122, "ORG": 0xqw...123 }
        pub components: BTreeMap<String, Hash>,
    }

    /// This controls the template type of Dao
    /// owner:the manager of the contract
    /// template_index:the length of template
    /// template_map:hashmap of index and DAOTemplate
    #[ink(storage)]
    pub struct TemplateManager {
        owner: AccountId,
        template_index: u64,
        template_map: StorageHashMap<u64, DAOTemplate>,
    }

    #[ink(event)]
    pub struct AddTemplate {
        #[ink(topic)]
        index: u64,
        #[ink(topic)]
        owner: Option<AccountId>,
    }

    impl TemplateManager {
        #[ink(constructor)]
        pub fn new(controller: AccountId) -> Self {
            let instance = Self {
                owner: controller,
                template_index: 0,
                template_map: StorageHashMap::new(),
            };
            instance
        }

        /// add a template
        /// name:the name of template
        /// dao_manager_code_hash:the hash of ao_manager
        /// components:the components of ao_manager
        #[ink(message)]
        pub fn add_template(
            &mut self,
            name: String,
            dao_manager_code_hash: Hash,
            base_hash:Hash,
            erc20_hash:Hash,
            user_hash:Hash,
            setting_hash:Hash,
            vault_hash:Hash,
        ) -> bool {
            assert_eq!(self.template_index + 1 > self.template_index, true);
            let mut components = BTreeMap::new();
            components.insert(String::from("BASE"),base_hash);
            components.insert(String::from("ERC20"),erc20_hash);
            components.insert(String::from("USER"),user_hash);
            components.insert(String::from("SETTING"),setting_hash);
            components.insert(String::from("VAULT"),vault_hash);
            let from = self.env().caller();
            self.template_map.insert(self.template_index, DAOTemplate {
                id: self.template_index,
                owner: from,
                name,
                dao_manager_code_hash,
                components,
            });
            self.env().emit_event(AddTemplate {
                index: self.template_index,
                owner: Some(from),
            });
            self.template_index += 1;
            true
        }

        /// show all template
        #[ink(message)]
        pub fn list_templates(&self) -> Vec<DAOTemplate> {
            let mut temp_vec = Vec::new();
            let mut iter = self.template_map.values();
            let mut temp = iter.next();
            while temp.is_some() {
                temp_vec.push(temp.unwrap().clone());
                temp = iter.next();
            }
            temp_vec
        }
        /// get a template by index
        #[ink(message)]
        pub fn query_template_by_index(&self, index: u64) -> DAOTemplate {
            self.template_map.get(&index).unwrap().clone()
        }
    }
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;
        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;
        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut dao_users = DaoUsers::new(AccountId::from([0x01; 32]));
            assert!(dao_users.add_group(Group {id:0,
                name:String::from("test"),
                join_directly:true,
                is_open:true,
                users:BTreeMap::new(),
                manager:AccountId::from([0x01; 32])
            }) == true);
        }
    }
}
