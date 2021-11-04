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
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(owner:AccountId,dao_id:u64) -> Self {
            Self {
                owner,
                template:None,
                active:false,
                dao_id
            }
        }

        #[ink(message)]
        pub fn set_template(&mut self, template: DAOTemplate) -> bool {
            assert_eq!(self.active, false);
            self.template = Some(template);
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
