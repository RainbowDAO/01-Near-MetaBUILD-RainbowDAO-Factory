use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};

near_sdk::setup_alloc!();
pub type AccountId = String;

    /// Construct a structure to display the basic information of Dao as a whole
    #[derive(Default, BorshDeserialize, BorshSerialize)]
    pub struct DisplayDaoBaseInfo {
        owner: AccountId,
        name: String,
        logo: String,
        desc: String,
    }
    /// Store basic information about Dao
    /// owner:the contract's manager
    /// name:the name of dao
    /// logo:the logo of dao
    /// desc:the desc of dao
    #[near_bindgen]
    #[derive(Default, BorshDeserialize, BorshSerialize)]
    pub struct DaoBase {
        owner: AccountId,
        name: String,
        logo: String,
        desc: String,
    }
    // #[near_bindgen]
    impl DaoBase {
        pub fn new() -> Self {
            Self {
                owner:env::signer_account_id(),
                name:String::default(),
                logo:String::default(),
                desc:String::default(),
            }
        }
        /// init the dao base
        /// name:the name of dao
        /// logo:the logo of dao
        /// desc:the desc of dao

        pub fn init_base(&mut self, name: String, logo: String, desc: String) {
            self.set_name(name);
            self.set_logo(logo);
            self.set_desc(desc);
        }
        /// set the dao's name

        pub fn set_name(&mut self, name: String) {
            self.name = String::from(name);
        }
        /// get the dao's name

        pub fn get_name(&self) -> String{
            self.name.clone()
        }
        /// set the dao's logo

        pub fn set_logo(&mut self, logo: String) {
            self.logo = String::from(logo);
        }
        /// get the dao's logo

        pub fn get_logo(&self) -> String{
            self.logo.clone()
        }
        /// set the dao's desc

        pub fn set_desc(&mut self, desc: String) {
            self.desc = String::from(desc);
        }
        /// get the dao's desc

        pub fn get_desc(&self) ->String{
            self.desc.clone()
        }
        /// get the base

        pub fn get_base_info(&self) ->DisplayDaoBaseInfo{
            DisplayDaoBaseInfo{
                owner: self.owner.clone(),
                name: self.name.clone(),
                logo: self.logo.clone(),
                desc: self.desc.clone(),
            }
        }
    }

    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;
        #[test]
        fn init_works() {
            let mut dao_base = DaoBase::new();
            dao_base.init_base(String::from("test"),String::from("test"),String::from("test"));
            assert!(dao_base.get_name()== String::from("test"));
            assert!(dao_base.get_logo()== String::from("test"));
            assert!(dao_base.get_desc()== String::from("test"));
        }
    }

