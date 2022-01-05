#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;
use ink_lang as ink;
pub use self::dao_base::DaoBase;

#[ink::contract]
mod dao_base {

    use alloc::string::String;
    #[derive(scale::Encode, scale::Decode, Clone, SpreadLayout, PackedLayout)]
    #[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    )]

    pub struct DisplayDaoBaseInfo {
        owner: AccountId,
        name: String,
        logo: String,
        desc: String,
    }

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
        pub fn get_name(&self) -> String{
            self.name.clone()
        }
        #[ink(message)]
        pub fn set_logo(&mut self, logo: String) {
            self.logo = String::from(logo);
        }
        #[ink(message)]
        pub fn get_logo(&self) -> String{
            self.logo.clone()
        }
        #[ink(message)]
        pub fn set_desc(&mut self, desc: String) {
            self.desc = String::from(desc);
        }
        #[ink(message)]
        pub fn get_desc(&self) ->String{
            self.desc.clone()
        }

        #[ink(message)]
        pub fn get_base_info(&self) ->DisplayDaoBaseInfo{
            DisplayDaoBaseInfo{
                owner: self.owner,
                name: self.name.clone(),
                logo: self.logo.clone(),
                desc: self.desc.clone(),
            }
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
