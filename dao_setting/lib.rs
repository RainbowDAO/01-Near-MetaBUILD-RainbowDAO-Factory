#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;
use ink_lang as ink;
pub use self::dao_setting::{
    DaoSetting
};
#[ink::contract]
mod dao_setting {
    use alloc::string::String;
    use ink_prelude::vec::Vec;
    use ink_prelude::collections::BTreeMap;
    use ink_storage::{
        collections::HashMap as StorageHashMap,
        traits::{PackedLayout, SpreadLayout},
    };

    #[derive(
    Debug, Clone, PartialEq, Eq, scale::Encode, scale::Decode, SpreadLayout, PackedLayout, Default
    )]
    #[cfg_attr(
    feature = "std",
    derive(::scale_info::TypeInfo, ::ink_storage::traits::StorageLayout)
    )]
    pub struct FeeConditions {
        pub  time_limit:u128,
        pub  fee_limit:u128,
        pub  token:AccountId
    }
    #[derive(
    Debug, Clone, PartialEq, Eq, scale::Encode, scale::Decode, SpreadLayout, PackedLayout, Default
    )]
    #[cfg_attr(
    feature = "std",
    derive(::scale_info::TypeInfo, ::ink_storage::traits::StorageLayout)
    )]
    pub struct OtherConditions {
        pub use_token:bool,
        pub use_nft:bool,
        pub token:AccountId,
        pub token_balance_limit:u128,
        pub nft:AccountId,
        pub nft_balance_limit:u128,
        pub nft_time_limit:u128
    }
    #[ink(storage)]
    pub struct DaoSetting {
        creator:AccountId,
        owner:AccountId,
        fee_limit:FeeConditions,
        other_limit:OtherConditions,
        conditions : u64,
    }

    impl DaoSetting {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(creator:AccountId) -> Self {
            Self {
                creator,
                owner:Self::env().caller(),
                fee_limit:FeeConditions{
                    time_limit:0,
                    fee_limit:0,
                    token:AccountId::default()
                },
                other_limit:OtherConditions{
                    use_token:false,
                    use_nft:false,
                    token:AccountId::default(),
                    token_balance_limit:0,
                    nft:AccountId::default(),
                    nft_balance_limit:0,
                    nft_time_limit:0
                },
                conditions:0,
            }
        }

        #[ink(message)]
        pub fn get_conditions(&self) -> u64 {
            self.conditions
        }
        #[ink(message)]
        pub fn get_fee_setting(&self) -> FeeConditions { self.fee_limit.clone() }
        #[ink(message)]
        pub fn get_other_setting(&self) -> OtherConditions {
            self.other_limit.clone()
        }
        #[ink(message)]
        pub fn set_join_limit(&mut self,conditions:u64,other_conditions:OtherConditions,fee_conditions:FeeConditions) -> bool {
            let owner = self.env().caller();
            assert_eq!(owner == self.creator, true);
            if conditions == 2 {
                self.fee_limit = fee_conditions;
            }else if conditions == 4 {
                self.other_limit = other_conditions;
            } else if conditions == 6 {
                self.fee_limit = fee_conditions;
                self.other_limit = other_conditions;
            }
            self.conditions = conditions;

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
    //         let dao_setting = DaoSetting::default();
    //         assert_eq!(dao_setting.get(), false);
    //     }
    //
    //     /// We test a simple use case of our contract.
    //     #[ink::test]
    //     fn it_works() {
    //         let mut dao_setting = DaoSetting::new(false);
    //         assert_eq!(dao_setting.get(), false);
    //         dao_setting.flip();
    //         assert_eq!(dao_setting.get(), true);
    //     }
    // }
}
