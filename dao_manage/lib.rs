#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod dao_manage {

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct DaoManage {
        /// Stores a single `bool` value on the storage.
        owner: AccountId,
    }

    impl DaoManage {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                owner: Self::env().caller()
            }
        }

        // pub fn new_dao(&self,)

    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let daoManage = DaoManage::default();
            assert_eq!(daoManage.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut daoManage = DaoManage::new(false);
            assert_eq!(daoManage.get(), false);
            daoManage.flip();
            assert_eq!(daoManage.get(), true);
        }
    }
}
