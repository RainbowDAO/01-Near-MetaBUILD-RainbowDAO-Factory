#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;
use ink_lang as ink;

#[ink::contract]
mod multisig_factory {
    use multisig::Multisig;
    use alloc::string::String;
    use ink_prelude::vec::Vec;
    use ink_prelude::collections::BTreeMap;
    use ink_storage::{collections::HashMap as StorageHashMap, };
    const CONTRACT_INIT_BALANCE: u128 = 1000 * 1_000_000_000_000;

    #[ink(storage)]
    pub struct MultisigFactory {
        /// Stores a single `bool` value on the storage.
        multisign:StorageHashMap<u64,AccountId>,
        index:u64
    }

    impl MultisigFactory {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                multisign:StorageHashMap::new(),
                index:0
            }
        }
        #[ink(message)]
        pub fn new_multisig(
            &mut self,
            multisig_hash:Hash,
            owners: Vec<AccountId>,
            min_sign_count: i32,
            version:u8
        ) -> AccountId {
            let salt = version.to_le_bytes();
            let instance_params = Multisig::new(owners,min_sign_count)
                .endowment(CONTRACT_INIT_BALANCE)
                .code_hash(multisig_hash)
                .salt_bytes(salt)
                .params();
            let init_result = ink_env::instantiate_contract(&instance_params);
            let contract_addr = init_result.expect("failed at instantiating the `multisig` contract");
            assert_eq!(self.index + 1 > self.index, true);
            self.multisign.insert(self.index, contract_addr);
            self.index += 1;
            contract_addr
        }
    }
}
