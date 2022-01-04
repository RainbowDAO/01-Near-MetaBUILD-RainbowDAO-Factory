#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;
use ink_lang as ink;
pub use self::dao_base::DaoBase;

#[ink::contract]
mod dao_base {

    use alloc::string::String;
    #[ink(storage)]
    pub struct DaoBase {
        owner: AccountId,
        name: String,
        logo: String,
        desc: String,
    }

    impl DaoBase {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                owner: Self::env().caller(),
                name:String::default(),
                logo:String::default(),
                desc:String::default(),
            }
        }
        #[ink(message)]
        pub fn init_base(&mut self, name: String, logo: String, desc: String) {
            self.set_name(name);
            self.set_logo(logo);
            self.set_desc(desc);
        }
        #[ink(message)]
        pub fn set_name(&mut self, name: String) {
            self.name = String::from(name);
        }
        #[ink(message)]
        pub fn set_logo(&mut self, logo: String) {
            self.logo = String::from(logo);
        }
        #[ink(message)]
        pub fn set_desc(&mut self, desc: String) {
            self.desc = String::from(desc);
        }




    }


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
    //         let dao_base = DaoBase::default();
    //         assert_eq!(dao_base.get(), false);
    //     }
    //
    //     /// We test a simple use case of our contract.
    //     #[ink::test]
    //     fn it_works() {
    //         let mut dao_base = DaoBase::new(false);
    //         assert_eq!(dao_base.get(), false);
    //         dao_base.flip();
    //         assert_eq!(dao_base.get(), true);
    //     }
    // }
}
