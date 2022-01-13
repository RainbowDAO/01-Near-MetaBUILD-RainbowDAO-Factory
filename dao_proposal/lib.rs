#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

use ink_lang as ink;

#[ink::contract]
mod dao_proposal {
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
        category:u32,
        publicity_votes:u128,
        publicity_delay:u32
    }

    /// Indicates whether a transaction is already confirmed or needs further confirmations.
    #[derive(scale::Encode, scale::Decode, Clone, SpreadLayout, PackedLayout)]
    #[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    )]
    #[derive(Debug)]
    pub struct Limit {
        fee_open:bool,
        fee_number:u128,
        fee_token:AccountId
    }
    /// Indicates whether a transaction is already confirmed or needs further confirmations.
    #[derive(scale::Encode, scale::Decode, Clone, SpreadLayout, PackedLayout)]
    #[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    )]
    #[derive(Debug)]
    pub struct VoteEffective {
        category:u32,
        vote_scale:u32,
        entrust_scale:u32,
        support_scale:u32
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
        Publicity,
        Queued,
    }

    #[ink(storage)]
    pub struct DaoProposal {
        creator:AccountId,
        owner: AccountId,
        proposals: StorageHashMap<u64, Proposal>,
        voting_delay: u32,
        voting_period: u32,
        proposal_length: u64,
        route_addr: AccountId,
        erc20_addr: AccountId,
        limit:Option<Limit>,
        vote_effective:Option<VoteEffective>
    }

    impl DaoProposal {
        #[ink(constructor)]
        pub fn new(creator:AccountId,route_addr: AccountId, erc20_addr: AccountId) -> Self {
            Self {
                creator,
                owner: Self::env().caller(),
                proposals: StorageHashMap::new(),
                voting_delay: 1,
                voting_period: 259200, //3 days
                proposal_length: 0,
                route_addr,
                erc20_addr,
                limit:None,
                vote_effective:None
            }
        }

        /// Set requirements for initiating proposals
        #[ink(message)]
        pub fn set_permission(&mut self,limit:Limit) -> bool {
            assert!(self.env().caller() != self.creator);
            self.limit = limit;
        }
        /// Set the conditions for successful proposal
        #[ink(message)]
        pub fn set_vote_effective(&mut self,vote_effective:VoteEffective) -> bool {
            assert!(self.env().caller() != self.creator);
            self.vote_effective = vote_effective;
        }


        #[ink(message)]
        pub fn propose(
            &mut self,
            title: String,
            desc: String,
            category:u32,
            start_block:u32,
            end_block:u32,
            transaction: Transaction,
            publicity_delay:u32,
        ) -> bool {
            assert!(start_block > self.env().block_number());
            assert!(end_block > start_block);
            let limit = self.limit;
            if limit.fee_open {
                let erc20_instance: Erc20 = ink_env::call::FromAccountId::from_account_id(limit.fee_token);
                //todo change this account address to vault
                erc20_instance.transfer_from(Self::env().caller(),AccountId::default(),limit.fee_number);
            }

            let proposal_id = self.proposal_length.clone() + 1;
            self.proposal_length += 1;
            let proposal_info = Proposal {
                category,
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
                publicity_votes:0,
                publicity_delay
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
            let effective:VoteEffective = self.vote_effective;
            let mut failed = false;
            let erc20_instance: Erc20 = ink_env::call::FromAccountId::from_account_id(self.erc20_addr);
            let token_info = erc20_instance.query_info();
            let all_vote = proposal.for_votes + proposal.against_votes;
            if effective.category == 1 {
                if all_vote / token_info.total_supply * 100 <= effective.vote_scale {
                    failed = true;
                }
            }else if effective.category == 3 {
                if proposal.for_votes / all_vote * 100 <= effective.support_scale {
                    failed = true;
                }
            }

            let proposal: Proposal = self.proposals.get(&proposal_id).unwrap().clone();
            if proposal.canceled { return ProposalState::Canceled; }
            else if block_number <= proposal.start_block { return ProposalState::Pending; }
            else if block_number <= proposal.end_block { return ProposalState::Active; }
            else if failed { return ProposalState::Defeated; }
            else if proposal.executed { return ProposalState::Executed; }
            else if block_number > proposal.end_block { return ProposalState::Expired; }
            else if block_number < proposal.end_block + proposal.publicity_delay { return ProposalState::Publicity; }
            else if proposal.publicity_votes > proposal.for_votes{ return ProposalState::Defeated; }
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
        #[ink(message)]
        pub fn public_vote(&mut self, proposal_id: u64) -> bool {
            let block_number = self.env().block_number();
            let caller = Self::env().caller();
            let mut proposal: Proposal = self.proposals.get(&proposal_id).unwrap().clone();
            assert!(proposal.end_block < block_number);
            assert!(proposal.end_block + proposal.publicity_delay > block_number);
            let erc20_instance: Erc20 = ink_env::call::FromAccountId::from_account_id(self.rbd_addr);
            let votes = erc20_instance.get_current_votes(caller);
            proposal.publicity_votes = votes;
            true
        }
        #[ink(message)]
        pub fn cast_vote(&mut self, proposal_id: u64, support: bool) -> bool {
            let caller = Self::env().caller();
            assert!(self.state(proposal_id) == ProposalState::Active);
            let mut proposal: Proposal = self.proposals.get(&proposal_id).unwrap().clone();
            let mut receipts = proposal.receipts.get(&caller).unwrap().clone();
            assert!(receipts.has_voted == false);
            let erc20_instance: Erc20 = ink_env::call::FromAccountId::from_account_id(self.rbd_addr);
            let votes = erc20_instance.get_current_votes(caller);
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
            let mut govnance_dao = DaoProposal::new(
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
