#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

use ink_lang as ink;
pub use self::kernel::{
    Kernel
};
#[ink::contract]
mod kernel {
    use alloc::string::String;
    use role_manage::RoleManage;
    use route_manage::RouteManage;
    use authority_management::AuthorityManagement;
    const DAO_INIT_BALANCE: u128 = 1000 * 1_000_000_000_000;

    #[ink(storage)]
    pub struct Kernel {
        owner:AccountId,
        role_manage: Option<RoleManage>,
        role_manage_addr: AccountId,
        route_manage: Option<RouteManage>,
        route_manage_addr: AccountId,
        authority_management:Option<AuthorityManagement>,
        authority_management_addr:AccountId,
        init : bool
    }

    impl Kernel {
        #[ink(constructor)]
        pub fn new() -> Self {
            let instance = Self {
                owner:Self::env().caller(),
                role_manage : None,
                role_manage_addr : AccountId::default(),
                route_manage : None,
                route_manage_addr : AccountId::default(),
                authority_management : None,
                authority_management_addr : AccountId::default(),
                init:false
            };
            instance
        }
        #[ink(message)]
        pub fn add_role(&mut self, name: String) {
            // self.role_manage.add_role(name);
            self.role_manage.as_mut().unwrap().add_role(name);

        }
        #[ink(message)]
        pub fn role_insert_privilege(&mut self, name:String,privilege:String) {
            // self.role_manage.role_insert_privilege(name,privilege);
            self.role_manage.as_mut().unwrap().role_insert_privilege(name,privilege);
        }
        #[ink(message)]
        pub fn add_privilege(&mut self, name: String) {
            // self.authority_management.add_privilege(name);
            self.authority_management.as_mut().unwrap().add_privilege(name);
        }
        #[ink(message)]
        pub fn add_route(&mut self, name: String,value: AccountId) {
            // self.route_manage.add_route(name,value);
            self.route_manage.as_mut().unwrap().add_route(name,value);
        }
        #[ink(message)]
        pub fn change_route(&mut self, name: String,value: AccountId) {
            // self.route_manage.add_route(name,value);
            self.route_manage.as_mut().unwrap().change_route(name,value);
        }

        #[ink(message)]
        pub fn get_role_addr(&self) -> AccountId {
            self.role_manage_addr
        }
        #[ink(message)]
        pub fn get_auth_addr(&self) -> AccountId {
            self.authority_management_addr
        }
        #[ink(message)]
        pub fn get_route_addr(&self) -> AccountId {
            self.route_manage_addr
        }
        #[ink(message)]
        pub fn init(
            &mut self,
            version: u32,
            role_code_hash: Hash,
            privilege_code_hash: Hash,
            route_code_hash: Hash
        ) -> bool {
            assert_eq!(self.init, false);
            let salt = version.to_le_bytes();
            let role_manage = RoleManage::new()
                .endowment(DAO_INIT_BALANCE)
                .code_hash(role_code_hash)
                .salt_bytes(salt)
                .params();
            let init_role_result = ink_env::instantiate_contract(&role_manage);
            let role_manage_addr = init_role_result.expect("failed at instantiating the `roleManager` contract");
            let role_contract_instance = ink_env::call::FromAccountId::from_account_id(role_manage_addr);
            self.role_manage = Some(role_contract_instance);
            self.role_manage_addr = role_manage_addr;

            let authority_management = AuthorityManagement::new()
                .endowment(DAO_INIT_BALANCE)
                .code_hash(privilege_code_hash)
                .salt_bytes(salt)
                .params();
            let init_authority_result = ink_env::instantiate_contract(&authority_management);
            let authority_management_addr = init_authority_result.expect(
                "failed at instantiating the `TemplateManager` contract"
            );
            let authority_contract_instance = ink_env::call::FromAccountId::from_account_id(authority_management_addr);
            self.authority_management = Some(authority_contract_instance);
            self.authority_management_addr = authority_management_addr;

            let route_manage = RouteManage::new()
                .endowment(DAO_INIT_BALANCE)
                .code_hash(route_code_hash)
                .salt_bytes(salt)
                .params();
            let init_route_result = ink_env::instantiate_contract(&route_manage);
            let route_manage_addr = init_route_result.expect(
                "failed at instantiating the `TemplateManager` contract"
            );
            let route_contract_instance = ink_env::call::FromAccountId::from_account_id(role_manage_addr);
            self.route_manage = Some(route_contract_instance);
            self.route_manage_addr = route_manage_addr;
            self.init = true;
            true
        }
    }

    #[cfg(test)]
    mod tests {


        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;

       /// You need to get the hash from  RouteManage,authority_management and RoleManage contract
        #[ink::test]
        fn init_works() {
            // let kernel = Kernel::new();
            // kernel.init(1,authority_contract_instance,authority_management_hash,route_manage_hash);
            // assert!(kernel.get_role_addr() != AccountId::default());
        }
    }
}
