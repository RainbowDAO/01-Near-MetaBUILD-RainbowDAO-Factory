#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod erc20_factory {

    #[ink(storage)]
    pub struct Erc20Factory {

    }

    impl Erc20Factory {
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self {
                value: init_value
            }
        }
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
            let erc20Factory = Erc20Factory::default();
            assert_eq!(erc20Factory.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut erc20Factory = Erc20Factory::new(false);
            assert_eq!(erc20Factory.get(), false);
            erc20Factory.flip();
            assert_eq!(erc20Factory.get(), true);
        }
    }
}
