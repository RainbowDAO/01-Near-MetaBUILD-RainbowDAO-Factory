use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen,AccountId,Gas,Promise};
use near_sdk::collections::LookupMap;
use near_sdk::serde_json::{json};

    // use template_manager::TemplateManager;
    // use template_manager::DAOTemplate;
    // use dao_manager::DAOManager;

    const TEMPLATE_INIT_BALANCE: u128 = 1000 * 1_000_000_000_000;
    const DAO_INIT_BALANCE: u128 = 1000 * 1_000_000_000_000;
    const SINGLE_CALL_GAS: Gas = Gas(200000000000000);

    ///Initialization information of Dao
    /// owner:the manager of the dao
    /// size:the size of the dao
    /// name:the name of the dao
    /// logo:the logo of the dao
    /// desc:the introduce of the dao
    /// dao_manager:the object of the dao
    /// dao_manager_addr:the address of the dao
    #[near_bindgen]
    #[derive(BorshDeserialize, BorshSerialize)]
    pub struct DAOInstance {
        id: u64,
        owner: AccountId,
        size: u64,
        name: String,
        logo: String,
        desc: String,
        dao_manager_addr: AccountId,
    }
    ///This is the factory contract used to generate Dao
    ///owner:the manager of the contract
    ///template_addr:the template's address of the dao
    ///template:the template's object of the dao
    ///instance_index:the instance index of  all dao
    ///instance_map:HashMap of index and DAOInstance
    ///instance_map_by_owner:HashMap of user and address
    #[near_bindgen]
    #[derive(BorshDeserialize, BorshSerialize)]
    pub struct DaoFactory {
        owner: AccountId,
        template_addr: AccountId,
        instance_index:u64,
        instance_map: LookupMap<u64, DAOInstance>,
        instance_map_by_owner: LookupMap<AccountId, Vec<u64>>,
    }

    impl DaoFactory {
        pub fn new() -> Self {
            Self {
                owner:  env::signer_account_id(),
                template_addr:env::signer_account_id(),
                instance_index:0,
                instance_map: LookupMap::new(b"r".to_vec()),
                instance_map_by_owner: LookupMap::new(b"r".to_vec()),
            }
        }

        ///Initialize factory contract
        ///template_code_hash:the hash of the template contract
        ///version:The random number used to generate the contract
        pub fn  init_factory (&mut self, prefix: AccountId, template_code: Vec<u8>) -> bool
        {
            let subaccount_id = AccountId::new_unchecked(
                format!("{}.{}", prefix, env::current_account_id())
            );
            Promise::new(subaccount_id.clone())
                .create_account()
                .add_full_access_key(env::signer_account_pk())
                .transfer(TEMPLATE_INIT_BALANCE)
                .deploy_contract(template_code);

            self.template_addr = subaccount_id;
            true
        }
        ///Generate a Dao through a template
        ///index:the index of the template
        ///controller:the manager of the dao
        ///controller_type:the manager's category of the dao
        ///category:the category of the dao
        pub fn init_dao_by_template(
            &mut self,
            prefix: AccountId,
            dao_manager_code: Vec<u8>,
            controller: AccountId,
        ) -> bool {
            assert_eq!(self.instance_index + 1 > self.instance_index, true);
            let subaccount_id = AccountId::new_unchecked(
                format!("{}.{}", prefix, env::current_account_id())
            );
            Promise::new(subaccount_id.clone())
                .create_account()
                .add_full_access_key(env::signer_account_pk())
                .transfer(DAO_INIT_BALANCE)
                .deploy_contract(dao_manager_code);
            let mut id_list = self.instance_map_by_owner.get(&controller).unwrap();
            id_list.push(self.instance_index);
            self.instance_map.insert(&self.instance_index, &DAOInstance {
                id: self.instance_index,
                owner: controller,
                size: 0,
                name: String::from(""),
                logo: String::from(""),
                desc: String::from(""),
                dao_manager_addr: subaccount_id,
            });
            self.instance_index += 1;
            true
        }
        ///Find templates through index
        ///index:the index of template

        pub fn query_template_by_index(&self, index: u64)  {
            env::promise_create(
                self.template_addr.clone(),
                "query_template_by_index",
                json!({ "index": index }).to_string().as_bytes(),
                0,
                SINGLE_CALL_GAS,
            );
        }

        /// get dao by id
        pub fn get_dao_by_index(&self,id:u64) -> DAOInstance {
            self.instance_map.get(&id).unwrap()
        }
        /// show list of user create
        pub fn get_daos_by_owner(&self) -> Vec<u64> {
            let user =   env::signer_account_id();
            let list = self.instance_map_by_owner.get(&user).unwrap().clone();
            list
        }

        /// show all daos
        pub fn list_dao(self) ->  LookupMap<u64, DAOInstance> {
            self.instance_map
        }
        /// create a record after user join
        pub fn joined_dao(&mut self,index:u64) -> bool {
            let user =  env::signer_account_id();
            let mut id_list = self.instance_map_by_owner.get(&user).unwrap();
            id_list.push(index);
            true
        }
    }

    // #[cfg(test)]
    // mod tests {
    //     /// Imports all the definitions from the outer scope so we can use them here.
    //     use super::*;
    //
    //     /// Imports `ink_lang` so we can use `#[ink::test]`.
    //     use ink_lang as ink;
    //     #[ink::test]
    //     fn it_works() {
    //         let mut dao_factory = DaoFactory::new();
    //         assert!(dao_factory.joined_dao() == true);
    //     }
    // }

