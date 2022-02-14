use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};
use near_sdk::collections::LookupMap;

near_sdk::setup_alloc!();
pub type AccountId = String;
    /// the contract store the category of dao
    /// owner:the manager of the contract
    /// category_map:HashMap of the index and name
    #[near_bindgen]
    #[derive(BorshDeserialize, BorshSerialize)]
    pub struct DaoCategory {
        owner: AccountId,
        category_map:LookupMap<u64,String>,
        index:u64
    }
    impl DaoCategory {
        pub fn new() -> Self {
            Self {
                owner: env::signer_account_id(),
                category_map:LookupMap::new(b"r".to_vec()),
                index:0
            }
        }
        /// add a new category
        /// name:the name of category
        pub fn add_category(&mut self,name:String) ->  bool {
            assert_eq!(self.index + 1 > self.index, true);
            self.category_map.insert(&self.index, &name);
            self.index += 1;
            true
        }
        /// show all category
        // pub fn list_category(&self) -> Vec<String> {
        //     let mut category_vec = Vec::new();
        //     let mut iter = self.category_map.values();
        //     let mut category = iter.next();
        //     while category.is_some() {
        //         category_vec.push(category.unwrap().clone());
        //         category = iter.next();
        //     }
        //     category_vec
        // }

        /// show all category
        pub fn list_category(self) -> LookupMap<u64,String> {
            self.category_map
        }

        /// Get a category by id
        pub fn query_category_by_index(&self, index: u64) -> String {
            self.category_map.get(&index).unwrap().clone()
        }
    }

