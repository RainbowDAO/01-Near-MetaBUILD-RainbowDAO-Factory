#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;
use ink_lang as ink;

#[ink::contract]
mod erc20_factory {
    use erc20::Erc20;
    use income_category::IncomeCategory;
    use route_manage::RouteManage;
    use alloc::string::String;
    use ink_prelude::vec::Vec;
    const CONTRACT_INIT_BALANCE: u128 = 1000 * 1_000_000_000_000;

    #[ink(storage)]
    pub struct Erc20Factory {
        route_addr:AccountId
    }

    impl Erc20Factory {
        #[ink(constructor)]
        pub fn new(route_addr:AccountId) -> Self {
            Self {
                route_addr
            }
        }
        #[ink(message)]
        pub fn new_erc20(
            &mut self,
            erc20_code_hash:Hash,
            version:u8,
            initial_supply: Balance,
            name:String,
            symbol:String,
            decimals:u8,
            owner:AccountId
        ) -> AccountId {
            let salt = version.to_le_bytes();
            let instance_params = Erc20::new(initial_supply,name,symbol,decimals,owner)
                .endowment(CONTRACT_INIT_BALANCE)
                .code_hash(erc20_code_hash)
                .salt_bytes(salt)
                .params();
            let init_result = ink_env::instantiate_contract(&instance_params);
            let contract_addr = init_result.expect("failed at instantiating the `Erc20` contract");
            let income_category_addr =  self.get_contract_addr(String::from("income_category"));
            if income_category_addr != AccountId::default()  {
                self.send_income_fee(income_category_addr);
            }
            contract_addr
        }




        fn send_income_fee(&mut self,income_category_addr:AccountId) -> bool {
            let mut income_instance: IncomeCategory = ink_env::call::FromAccountId::from_account_id(income_category_addr);
            let category =  income_instance.get_category(String::from("erc20"));
            if category.is_used {
                self.get_fee_from_user(category.token,category.fee,income_category_addr);
            }
            true
        }

        fn get_fee_from_user(&mut self,token_account:AccountId,fee:u128,to_account:AccountId) -> bool {
            let mut erc20_instance: Erc20 = ink_env::call::FromAccountId::from_account_id(token_account);
            erc20_instance.transfer_from(Self::env().caller(),to_account,fee);
            true
        }

        #[ink(message)]
        pub fn get_contract_addr(&self,target_name:String) ->AccountId {
            let route_instance: RouteManage = ink_env::call::FromAccountId::from_account_id(self.route_addr);
            return route_instance.query_route_by_name(target_name);
        }
    }
}
