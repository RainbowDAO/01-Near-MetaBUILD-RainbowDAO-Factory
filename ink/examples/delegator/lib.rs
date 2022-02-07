#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod delegator {
    use accumulator::AccumulatorRef;
    use ink_storage::{
        traits::{
            PackedLayout,
            SpreadLayout,
        },
    };

    #[derive(Debug, scale::Encode, scale::Decode, Clone, SpreadLayout, PackedLayout)]
    #[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    )]
    pub struct AddrInstance {
        pub base: Option<AccumulatorRef>,
    }
   

    #[derive(
        Debug, Copy, Clone, PartialEq, Eq, scale::Encode, scale::Decode, SpreadLayout, PackedLayout, Default
        )]
        #[cfg_attr(
        feature = "std",
        derive(::scale_info::TypeInfo, ::ink_storage::traits::StorageLayout)
        )]
        pub struct Addr {
            pub base_addr: Option<AccountId>,
        }

    #[ink(storage)]
    pub struct Delegator {
        pub init: bool,
        pub components: AddrInstance,
        pub component_addrs: Addr,
    }

    impl Delegator {
        /// Instantiate a `delegator` contract with the given sub-contract codes.
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                init: false,
                components: DAOComponents {
                    base: None,
                   
                },
                component_addrs: DAOComponentAddrs {
                    base_addr: None,
                },
            }
        }

        /// Returns the `accumulator` value.
        #[ink(message)]
        pub fn get(&self) -> Option<AccountId> {
            self.component_addrs.base_addr
        }


        #[ink(message)]
        pub fn init_base(&mut self, base_code_hash: Hash,
            init_value: i32,version: u32) -> bool {
            let total_balance = Self::env().balance();
            // instance base
            let salt = version.to_le_bytes();
            let instance_params = AccumulatorRef::new(init_value)
                .endowment(total_balance / 2)
                .code_hash(base_code_hash)
                .salt_bytes(salt)
                .params();
            let init_result = ink_env::instantiate_contract(&instance_params);
            let contract_addr = init_result.expect("failed at instantiating the `Base` contract");
            let mut contract_instance: AccumulatorRef = ink_env::call::FromAccountId::from_account_id(contract_addr);

            self.components.base = Some(contract_instance);
            self.component_addrs.base_addr = Some(contract_addr);

            true
}
    }
}
