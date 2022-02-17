use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen,AccountId};
use near_sdk::collections::LookupMap;
/// store the template info
/// id:the id of template
/// owner:the owner of the template
/// name:the name of the template
/// dao_manager_code_hash:the hash of the dao manager
/// components:the  of the template
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct DAOTemplate {
    // template's id
    pub id: u64,
    // template's owner
    pub owner: AccountId,
    // template's name
    pub name: String,
    // template's dao manager
    pub dao_manager_code_hash: String,
    // components code hash
    // like { "ERC20": 0xqw...122, "ORG": 0xqw...123 }
    pub components: LookupMap<String, String>,
}

/// This controls the template type of Dao
/// owner:the manager of the contract
/// template_index:the length of template
/// template_map:hashmap of index and DAOTemplate
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct TemplateManager {
    owner: AccountId,
    template_index: u64,
    template_map: LookupMap<u64, DAOTemplate>,
}

impl TemplateManager {
    pub fn new(controller: AccountId) -> Self {
        let instance = Self {
            owner: controller,
            template_index: 0,
            template_map: LookupMap::new(b"r".to_vec()),
        };
        instance
    }
    /// add a template
    /// name:the name of template
    /// dao_manager_code_hash:the hash of ao_manager
    /// components:the components of ao_manager
    pub fn add_template(
        &mut self,
        name: String,
        dao_manager_code_hash: String,
        base_hash:String,
        erc20_hash:String,
        user_hash:String,
        setting_hash:String,
        vault_hash:String,
    ) -> bool {
        assert_eq!(self.template_index + 1 > self.template_index, true);
        let mut components = LookupMap::new(b"r".to_vec());
        components.insert(&String::from("BASE"),&base_hash);
        components.insert(&String::from("ERC20"),&erc20_hash);
        components.insert(&String::from("USER"),&user_hash);
        components.insert(&String::from("SETTING"),&setting_hash);
        components.insert(&String::from("VAULT"),&vault_hash);
        let from = env::signer_account_id();
        self.template_map.insert(&self.template_index, &DAOTemplate {
            id: self.template_index,
            owner: from,
            name,
            dao_manager_code_hash,
            components,
        });
        self.template_index += 1;
        true
    }
    /// show all template
    pub fn list_templates(self) ->LookupMap<u64, DAOTemplate> {
        self.template_map
    }
    /// get a template by index
    pub fn query_template_by_index(&self, index: u64) -> DAOTemplate {
        self.template_map.get(&index).unwrap()
    }
}

