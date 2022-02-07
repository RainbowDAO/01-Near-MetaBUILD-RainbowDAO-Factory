#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;
use ink_lang as ink;
pub use self::dao_base::DaoBase;

#[allow(unused_imports)]
#[ink::contract]
mod dao_base {
    use alloc::string::String;
    use ink_storage::{
        traits::{
            PackedLayout,
            SpreadLayout,
        },
    };
    /// Construct a structure to display the basic information of Dao as a whole
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
    /// Store basic information about Dao
    /// owner:the contract's manager
    /// name:the name of dao
    /// logo:the logo of dao
    /// desc:the desc of dao
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
        /// init the dao base
        /// name:the name of dao
        /// logo:the logo of dao
        /// desc:the desc of dao
        #[ink(message)]
        pub fn init_base(&mut self, name: String, logo: String, desc: String) {
            self.set_name(name);
            self.set_logo(logo);
            self.set_desc(desc);
        }
        /// set the dao's name
        #[ink(message)]
        pub fn set_name(&mut self, name: String) {
            self.name = String::from(name);
        }
        /// get the dao's name
        #[ink(message)]
        pub fn get_name(&self) -> String{
            self.name.clone()
        }
        /// set the dao's logo
        #[ink(message)]
        pub fn set_logo(&mut self, logo: String) {
            self.logo = String::from(logo);
        }
        /// get the dao's logo
        #[ink(message)]
        pub fn get_logo(&self) -> String{
            self.logo.clone()
        }
        /// set the dao's desc
        #[ink(message)]
        pub fn set_desc(&mut self, desc: String) {
            self.desc = String::from(desc);
        }
        /// get the dao's desc
        #[ink(message)]
        pub fn get_desc(&self) ->String{
            self.desc.clone()
        }
        /// get the base
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

    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;
        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;
        #[ink::test]
        fn init_works() {
            let mut dao_base = DaoBase::new();
            dao_base.init_base(String::from("test"),String::from("test"),String::from("test"));
            assert!(dao_base.get_name()== String::from("test"));
            assert!(dao_base.get_logo()== String::from("test"));
            assert!(dao_base.get_desc()== String::from("test"));
        }
    }
}
