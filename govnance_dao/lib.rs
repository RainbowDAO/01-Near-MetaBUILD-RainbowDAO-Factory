#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

use ink_lang as ink;

#[ink::contract]
mod govnance_dao {
    use ink_env::call::{
        build_call,
        utils::ReturnType,
        ExecutionInput,
    };

    use route_manage::RouteManage;
    use erc20::Erc20;
    // use core::Core;
    use alloc::string::String;

    use ink_prelude::vec::Vec;
    use ink_prelude::collections::BTreeMap;
    use ink_storage::{
        traits::{
            PackedLayout,
            SpreadLayout,
        },
        collections::HashMap as StorageHashMap,
    };
    use scale::Output;

    /// A wrapper that allows us to encode a blob of bytes.
  ///
  /// We use this to pass the set of untyped (bytes) parameters to the `CallBuilder`.
    struct CallInput<'a>(&'a [u8]);

    impl<'a> scale::Encode for CallInput<'a> {
        fn encode_to<T: Output + ?Sized>(&self, dest: &mut T) {
            dest.write(self.0);
        }
    }

    /// Indicates whether a transaction is already confirmed or needs further confirmations.
    #[derive(scale::Encode, scale::Decode, Clone, SpreadLayout, PackedLayout)]
    #[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    )]
    #[derive(Debug)]
    pub struct Receipt {
        has_voted: bool,
        support: bool,
        votes: u128,
    }

    /// Indicates whether a transaction is already confirmed or needs further confirmations.
    #[derive(scale::Encode, scale::Decode, Clone, SpreadLayout, PackedLayout)]
    #[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    )]
    #[derive(Debug)]
    pub struct Proposal {
        proposal_id: u64,
        title: String,
        desc: String,
        start_block: u32,
        end_block: u32,
        for_votes: u128,
        against_votes: u128,
        owner: AccountId,
        canceled: bool,
        executed: bool,
        receipts: BTreeMap<AccountId, Receipt>,
        transaction: Transaction,
    }

    /// Indicates whether a transaction is already confirmed or needs further confirmations.
    #[derive(scale::Encode, scale::Decode, Clone, SpreadLayout, PackedLayout)]
    #[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    )]
    #[derive(Debug)]
    pub struct Transaction {
        /// The `AccountId` of the contract that is called in this transaction.
        callee: AccountId,
        /// The selector bytes that identifies the function of the callee that should be called.
        selector: [u8; 4],
        /// The SCALE encoded parameters that are passed to the called function.
        input: Vec<u8>,
        /// The amount of chain balance that is transferred to the callee.
        transferred_value: Balance,
        /// Gas limit for the execution of the call.
        gas_limit: u64,
    }


    #[ink(event)]
    pub struct ProposalCreated {
        #[ink(topic)]
        proposal_id: u64,
        #[ink(topic)]
        creator: AccountId,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum ProposalState {
        Canceled,
        Pending,
        Active,
        Defeated,
        Succeeded,
        Executed,
        Expired,
        Queued,
    }

    #[ink(storage)]
    pub struct GovnanceDao {
        owner: AccountId,
        proposals: StorageHashMap<u64, Proposal>,
        voting_delay: u32,
        voting_period: u32,
        proposal_length: u64,
        route_addr: AccountId,
        rbd_addr: AccountId,
    }

    impl GovnanceDao {
        #[ink(constructor)]
        pub fn new(route_addr: AccountId, rbd_addr: AccountId) -> Self {
            Self {
                owner: Self::env().caller(),
                proposals: StorageHashMap::new(),
                voting_delay: 1,
                voting_period: 259200, //3 days
                proposal_length: 0,
                route_addr,
                rbd_addr,
            }
        }

        #[ink(message)]
        pub fn propose(&mut self, title: String, desc: String, transaction: Transaction) -> bool {
            let start_block = self.env().block_number() + self.voting_delay;
            let end_block = start_block + self.voting_period;
            let proposal_id = self.proposal_length.clone() + 1;
            self.proposal_length += 1;
            let proposal_info = Proposal {
                proposal_id,
                title,
                desc,
                start_block,
                end_block,
                for_votes: 0,
                against_votes: 0,
                owner: Self::env().caller(),
                canceled: false,
                executed: false,
                receipts: BTreeMap::new(),
                transaction,
            };
            self.proposals.insert(proposal_id, proposal_info);
            self.env().emit_event(ProposalCreated {
                proposal_id,
                creator: self.env().caller(),
            });
            true
        }
        #[ink(message)]
        pub fn state(&self, proposal_id: u64) -> ProposalState {
            let block_number = self.env().block_number();
            let proposal: Proposal = self.proposals.get(&proposal_id).unwrap().clone();
            if proposal.canceled { return ProposalState::Canceled; }
            else if block_number <= proposal.start_block { return ProposalState::Pending; }
            else if block_number <= proposal.end_block { return ProposalState::Active; }
            else if proposal.for_votes <= proposal.against_votes { return ProposalState::Defeated; }
            else if proposal.executed { return ProposalState::Executed; }
            else if block_number > proposal.end_block { return ProposalState::Expired; }
            else { return ProposalState::Queued; }
        }
        #[ink(message)]
        pub fn cancel(&self, proposal_id: u64) -> bool {
            let mut proposal: Proposal = self.proposals.get(&proposal_id).unwrap().clone();
            assert!(self.state(proposal_id) != ProposalState::Executed);
            assert!(proposal.owner == Self::env().caller());
            proposal.canceled = true;
            true
        }
        #[ink(message)]
        pub fn exec(&mut self, proposal_id: u64) -> bool {
            let mut proposal: Proposal = self.proposals.get(&proposal_id).unwrap().clone();
            assert!(self.state(proposal_id) == ProposalState::Queued);
            let result = build_call::<<Self as ::ink_lang::ContractEnv>::Env>()
                .callee(proposal.transaction.callee)
                .gas_limit(proposal.transaction.gas_limit)
                .transferred_value(proposal.transaction.transferred_value)
                .exec_input(
                    ExecutionInput::new(
                        proposal.transaction.selector.into()).
                        push_arg(CallInput(&proposal.transaction.input)
                    ),
                )
                .returns::<()>()
                .fire()
                .unwrap();
            proposal.executed = true;
            true
        }
        // #[ink(message)]
        // pub fn get_contract_addr(&self,target_name:String) ->AccountId {
        //     let route_instance: RouteManage = ink_env::call::FromAccountId::from_account_id(self.route_addr);
        //     return route_instance.query_route_by_name(target_name);
        // }
        #[ink(message)]
        pub fn cast_vote(&mut self, proposal_id: u64, support: bool) -> bool {
            let caller = Self::env().caller();
            assert!(self.state(proposal_id) == ProposalState::Active);
            let mut proposal: Proposal = self.proposals.get(&proposal_id).unwrap().clone();
            let mut receipts = proposal.receipts.get(&caller).unwrap().clone();
            assert!(receipts.has_voted == false);
            let erc20_instance: Erc20 = ink_env::call::FromAccountId::from_account_id(self.rbd_addr);
            let votes = erc20_instance.get_prior_votes(caller, proposal.start_block);
            if support {
                proposal.for_votes += votes;
            } else {
                proposal.against_votes += votes;
            }
            receipts.has_voted = true;
            receipts.support = support;
            receipts.votes = votes;

            true
        }
        #[ink(message)]
        pub fn list_proposals(&self) -> Vec<Proposal> {
            let mut proposal_vec = Vec::new();
            let mut iter = self.proposals.values();
            let mut proposal = iter.next();
            while proposal.is_some() {
                proposal_vec.push(proposal.unwrap().clone());
                proposal = iter.next();
            }
            proposal_vec
        }
        #[ink(message)]
        pub fn get_proposal_by_id(&self, proposal_id: u64) -> Proposal {
            let proposal: Proposal = self.proposals.get(&proposal_id).unwrap().clone();
            proposal
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
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                    .expect("Cannot get accounts");
            let mut govnance_dao = GovnanceDao::new(
                AccountId::from([0x01; 32]),
                AccountId::from([0x01; 32])
            );
            let mut vec = Vec::new();
            vec.push(1);
            let select: [u8; 4] = [1, 2, 3, 4];
            govnance_dao.propose(String::from("test"), String::from("test"), Transaction {
                callee: accounts.alice,
                selector: select,
                input: vec,
                transferred_value: 0,
                gas_limit: 1000000 }
            );
            let proposal: Proposal = govnance_dao.get_proposal_by_id(1);
            assert!(proposal.title == String::from("test"));
        }
    }
}
