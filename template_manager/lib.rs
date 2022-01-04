#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
use ink_lang as ink;
pub use self::template_manager::TemplateManager;
pub use self::template_manager::DAOTemplate;

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

    /// Indicates whether a transaction is already confirmed or needs further confirmations.
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

        #[ink(message)]
        pub fn add_template(&mut self, name: String, dao_manager_code_hash: Hash, components: BTreeMap<String, Hash>) -> bool {
            assert_eq!(self.template_index + 1 > self.template_index, true);
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

        #[ink(message)]
        pub fn query_template_by_index(&self, index: u64) -> DAOTemplate {
            self.template_map.get(&index).unwrap().clone()
        }
    }
}
