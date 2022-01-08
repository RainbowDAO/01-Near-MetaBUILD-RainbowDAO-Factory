#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
use ink_lang as ink;

pub use self::dao_proposal::DaoProposal;

#[ink::contract]
mod dao_proposal {
    use alloc::string::String;
    use erc20::Erc20;
    use ink_storage::{
        collections::HashMap as StorageHashMap,
        traits::{PackedLayout, SpreadLayout},
    };
    

    #[ink(storage)]
    pub struct DaoProposal {
        manager_address:AccountId,
        proposal_id:u64,
        proposal_name:String,
        proposal_category:u32,///1,change contract prama ,2,change role 
        vote_time:u64,
        proposal_check_status:StorageHashMap<(String,u64), u32 >,
        erc20_address:AccountId,
        // proposal_status:u32,///1 voting 2,over 
        all_proposal:Porposal,
        a_proposals:StorageHashMap<u64,Porposal>,
        proposal_vote:StorageHashMap<String,u64>,
        voted_list:StorageHashMap<AccountId, u64>,
        pass_ratio:u32,
        vote:u32,
    }
    #[derive(Debug, scale::Encode, scale::Decode, Clone, SpreadLayout, PackedLayout)]
    #[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    )]
    pub struct Porposal{
        index:u64,
        name:String,
        category:u32,
        start_time:u64,
        time:u64,
        vote:u64,

    }
    // pub struct 
    impl DaoProposal {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(owner:AccountId, erc20_addr: AccountId) -> Self {
            Self { 
                manager_address:owner,
                proposal_id:0,
                proposal_name:String::from(""),
                proposal_category:0,
                pass_ratio:0,
                vote_time:0,
                vote:0,
                proposal_check_status:StorageHashMap::new(),
                erc20_address:erc20_addr,
                all_proposal:Porposal{
                    index:0,
                    name:String::from(""),
                    category:0,
                    start_time:0,
                    time:0,
                    vote:0,
                },
                a_proposals:StorageHashMap::new(),
                proposal_vote:StorageHashMap::new(),
                voted_list:StorageHashMap::new(),
                // proposal_status:0,
             }
        }



        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        #[ink(message)]
        pub fn creat_proposal(&mut self ,proposal_name:String,proposal_category:u32,vote_time:Timestamp) -> bool {
            let caller = self.env().caller();
            let balance_of_caller: Erc20 = ink_env::call::FromAccountId::from_account_id(self.erc20_address);
            let balance = balance_of_caller.balance_of(caller);
            let time = self.env().block_timestamp();

            if balance >= 0 {
                if vote_time -time <= 0{
                    return false;
                }
                self.proposal_id+=1;
                let p_status = 1;
                let a = Porposal{ 
                    index:self.proposal_id,
                    name:proposal_name.clone(),
                    category:proposal_category,
                    start_time:time,
                    time:vote_time,
                    vote:0,
                };
                self.a_proposals.insert(self.proposal_id,a.clone());

                self.all_proposal = a;
                self.proposal_check_status.insert((proposal_name ,self.proposal_id), p_status);
            }
            true
        }
        #[ink(message)]
        pub fn proposal_voting(&mut self ,proposal_name:String ,proposal_id:u64) ->bool{
            assert!(self._check_status(proposal_name.clone(), proposal_id) == 1);
            let caller = self.env().caller();
            assert!(self._check_voted(caller) != &proposal_id);
            let a = self._have_erc20(caller);
            let time = self.env().block_timestamp();
            // if  a > 0 {
            //     if self.all_proposal.start_time < time && time < self.all_proposal.time{
            //     self.all_proposal.vote += 1;
            //     self.voted_list.insert(caller, proposal_id);
            //     if self.all_proposal.vote == 20 {
            //         let p_status = 2;
            //         self.proposal_check_status.insert((proposal_name,proposal_id), p_status);
            //     }
            // }
            // }
            if a > 0 {
                let mut b = self.a_proposals.get_mut(&proposal_id).unwrap();
                if b.start_time < time && time <b.time{
                    b.vote +=1;
                    self.voted_list.insert(caller,proposal_id);
                    if b.vote ==20 {
                        let p_status = 2;
                        self.proposal_check_status.insert((proposal_name,proposal_id),p_status);
                    }
                }
            }
            true
        }
        
        fn _check_voted(&self, user_addr:AccountId) -> &u64{
            self.voted_list.get(&user_addr).unwrap_or(&0)
        }
        fn _have_erc20(&self, user_addr:AccountId) -> u64 {
            let balance_of_caller:Erc20 =ink_env::call::FromAccountId::from_account_id(self.erc20_address);
            let balance = balance_of_caller.balance_of(user_addr);
            balance
        }
        #[ink(message)]
        pub fn _check_status(&self, proposal_name:String , proposal_id:u64 ) ->u32{
            *self.proposal_check_status.get(&(proposal_name,proposal_id)).unwrap_or(&0)
        }
        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn cancle_proposal(&mut self, proposal_id:u64 ) -> bool {
            let caller = self.env().caller();
            assert_eq!(caller == self.manager_address, true);
            self.a_proposals.take(&proposal_id);
            true
        }
        fn _get_caller(&self) -> AccountId{
            return self.env().caller();
        }
        #[ink(message)]
        pub fn get_manager_addr(&self) ->AccountId{
            self.manager_address

        }
        #[ink(message)]
        pub fn set_new_manager_addr(&mut self , to:AccountId) -> bool{
            assert_eq!(self._get_caller() == self.manager_address ,true);
            self.manager_address = to;
            true
        }
        #[ink(message)]
        pub fn get_proposal_by_id(&self,proposal_id:u64) -> Porposal{
            self.a_proposals.get(&proposal_id).unwrap().clone()
        }
    }
}
