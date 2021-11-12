#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;
use ink_lang as ink;

#[ink::contract]
mod rainbow_govnance {
    use alloc::string::String;
    use ink_prelude::vec::Vec;
    use ink_prelude::collections::BTreeMap;
    use ink_storage::{collections::HashMap as StorageHashMap, };



    /// Indicates whether a transaction is already confirmed or needs further confirmations.
    #[derive(scale::Encode, scale::Decode, Clone)]
    #[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    )]

    #[derive(Debug)]
    pub struct Proposal {
        // base module contract's address
        title: String,
        desc: String,
        start_block:u64,
        end_block:u64,
        for_votes:u64,
        against_votes:u64,
        owner:AccountId,
        proposal_id:u64,
        canceled:bool

    }

    #[ink(event)]
    pub struct ProposalCreated {
        #[ink(topic)]
        proposal_id: u64,
        #[ink(topic)]
        creator: AccountId,
        #[ink(topic)]
        title:String
    }
    /// The kind of access allows for a storage cell.
    #[derive(
    Debug, Copy, Clone, PartialEq, Eq, scale::Encode, scale::Decode, scale_info::TypeInfo,
    )]
    pub enum ProposalState {
        Canceled,
        Pending,
        Active,
        Defeated,
        Succeeded
    }
    #[ink(storage)]
    pub struct RainbowGovnance {
        owner: AccountId,
        proposals:StorageHashMap<u64, Proposal>,
        voting_delay:u64,
        voting_period:u64,
        proposal_length:u64
    }

    impl RainbowGovnance {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                owner: Self::env().caller(),
                proposals:StorageHashMap::new(),
                voting_delay:1,
                voting_period:259200, //3 days
                proposal_length:0
            }
        }

        #[ink(message)]
        pub fn propose(&mut self,title:String,desc:String) -> bool {
            let start_block = self.env().block_number() + self.voting_delay;
            let end_block = start_block + self.voting_period;
            let proposal_id = self.proposal_length.clone();
            self.proposal_length += 1;
            let proposal_info = Proposal{
                proposal_id,
                title,
                desc,
                start_block,
                end_block,
                for_votes:0,
                against_votes:0,
                owner:Self::env().caller(),
                canceled:false
            };
            self.proposals.insert(proposal_id, proposal_info);
            self.env().emit_event(ProposalCreated{
                proposal_id,
                creator: self.env().caller(),
                title
            });
            true
        }
        #[ink(message)]
        pub fn state(&self,index:u64) -> ProposalState {
            let block_number = self.env().block_number();
            let proposal:Proposal =  self.proposals.get(&index).unwrap().clone();
            if proposal.canceled {return ProposalState::Canceled }
            else if block_number <= proposal.start_block { return ProposalState::Pending}
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
    //         let rainbowGovnance = RainbowGovnance::default();
    //         assert_eq!(rainbowGovnance.get(), false);
    //     }
    //
    //     /// We test a simple use case of our contract.
    //     #[ink::test]
    //     fn it_works() {
    //         let mut rainbowGovnance = RainbowGovnance::new(false);
    //         assert_eq!(rainbowGovnance.get(), false);
    //         rainbowGovnance.flip();
    //         assert_eq!(rainbowGovnance.get(), true);
    //     }
    // }
}
