#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod template_management {

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct TemplateManagement {
        /// Stores a single `bool` value on the storage.
        value: bool,
    }

    impl TemplateManagement {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { value: init_value }
        }



        // pub fn add_template() -> bool {
        //
        // }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
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
    //         let templateManagement = TemplateManagement::default();
    //         assert_eq!(templateManagement.get(), false);
    //     }
    //
    //     /// We test a simple use case of our contract.
    //     #[ink::test]
    //     fn it_works() {
    //         let mut templateManagement = TemplateManagement::new(false);
    //         assert_eq!(templateManagement.get(), false);
    //         templateManagement.flip();
    //         assert_eq!(templateManagement.get(), true);
    //     }
    // }
}
