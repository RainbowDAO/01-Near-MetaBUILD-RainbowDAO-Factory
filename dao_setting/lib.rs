#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;
use ink_lang as ink;

#[ink::contract]
mod dao_setting {
    use alloc::string::String;
    use ink_prelude::vec::Vec;
    use erc20::Erc20;
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
        time_limit:u128,
        fee_limit:u128,
        token:AccountId
    }
    #[derive(
    Debug, Clone, PartialEq, Eq, scale::Encode, scale::Decode, SpreadLayout, PackedLayout, Default
    )]
    #[cfg_attr(
    feature = "std",
    derive(::scale_info::TypeInfo, ::ink_storage::traits::StorageLayout)
    )]
    pub struct OtherConditions {
        use_token:bool,
        use_nft:bool,
        token:AccountId,
        token_balance_limit:u128,
        nft:AccountId,
        nft_balance_limit:u128,
        nft_time_limit:u128
    }
    #[ink(storage)]
    pub struct DaoSetting {
        creator:AccountId,
        owner:AccountId,
        fee_limit:Option<FeeConditions>,
        other_limit:Option<OtherConditions>,
        conditions : u64,
    }

    impl DaoSetting {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(creator:AccountId) -> Self {
            Self {
                creator,
                owner:Self::env().caller(),
                fee_limit:None,
                other_limit:None,
                conditions:0,
            }
        }

        #[ink(message)]
        pub fn get_conditions(&self) -> u64 {
            self.conditions
        }
        #[ink(message)]
        pub fn get_fee_setting(&self) -> &FeeConditions {
            self.fee_limit.as_ref().unwrap()
        }
        #[ink(message)]
        pub fn get_other_setting(&self) -> &OtherConditions {
            self.other_limit.as_ref().unwrap()
        }
        #[ink(message)]
        pub fn set_join_limit(&mut self,conditions:u64,other_conditions:OtherConditions,fee_conditions:FeeConditions) -> bool {
            let owner = self.env().caller();
            assert_eq!(owner == self.creator, true);
            if conditions == 2 {
                self.fee_limit = Some(fee_conditions);
            }else if conditions == 4 {
                self.other_limit = Some(other_conditions);
            } else if conditions == 6 {
                self.fee_limit = Some(fee_conditions);
                self.other_limit = Some(other_conditions);
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
