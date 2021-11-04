#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

pub use self::dao_manager::DAOManager;

#[ink::contract]
mod dao_manager {

    use template_manager::DAOTemplate;
    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct DAOManager {
        owner: AccountId,
        template: Option<DAOTemplate>,
        active: bool,
        dao_id:u64
    }

    impl DAOManager {
        /// Create a new dao
        #[ink(constructor)]
        pub fn new(owner:AccountId,dao_id:u64) -> Self {
            Self {
                owner,
                template:None,
                active:false,
                dao_id
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
        // #[ink(message)]
        // pub fn  init_by_params(&mut self, params: DAOInitParams, salt: Vec<u8>) -> bool {}

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
