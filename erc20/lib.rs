#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;
use ink_lang as ink;
pub use self::erc20::{
    Erc20
};
#[ink::contract]
mod erc20 {
    use alloc::string::String;
    use ink_storage::{
        collections::HashMap as StorageHashMap,
        lazy::Lazy,
        traits::{
            PackedLayout,
            SpreadLayout,
        }
    };

     /// It records how many tickets there are in a block
    #[derive(scale::Encode, scale::Decode, Clone, SpreadLayout, PackedLayout)]
    #[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    )]

    #[derive(Debug)]
    pub struct Checkpoint {
        from_block:u32,
        votes:u128
    }
    /// A  ERC-20 contract.
    #[ink(storage)]
    pub struct Erc20 {
        /// Total token supply.
        total_supply: Lazy<Balance>,
        /// Mapping from owner to number of owned token.
        balances: StorageHashMap<AccountId, Balance>,
        /// Mapping of the token amount which an account is allowed to withdraw
        /// from another account.
        allowances: StorageHashMap<(AccountId, AccountId), Balance>,
        /// The name of token
        name: String,
        /// The symbol of token
        symbol: String,
        /// The decimals of token
        decimals: u8,
        /// The manager of token
        owner: AccountId,
        /// The points of user
        num_check_points:StorageHashMap<AccountId,u32>,
        /// The number of votes at a point is recorded
        check_points:StorageHashMap<(AccountId, u32), Checkpoint>,
        /// Delegation information of ticket
        delegates:StorageHashMap<AccountId,AccountId>,
    }

    #[derive(scale::Encode, scale::Decode, Clone)]
    #[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    )]
    pub struct TokenInfo {
        name: String,
        symbol: String,
        total_supply: u128,
        decimals: u8,
        owner: AccountId,
    }




    /// Event emitted when a token transfer occurs.
    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        value: Balance,
    }

    /// Event emitted when an approval occurs that `spender` is allowed to withdraw
    /// up to the amount of `value` tokens from `owner`.
    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        value: Balance,
    }

    #[ink(event)]
    pub struct DelegateVotesChanged {
        #[ink(topic)]
        delegate: AccountId,
        #[ink(topic)]
        old_votes: u128,
        new_votes: u128,
    }
    #[ink(event)]
    pub struct DelegateChanged {
        delegator:AccountId,
        current_delegate:AccountId,
        delegatee:AccountId
    }

    /// The ERC-20 error types.
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        /// Returned if not enough balance to fulfill a request is available.
        InsufficientBalance,
        /// Returned if not enough allowance to fulfill a request is available.
        InsufficientAllowance,
    }

    /// The ERC-20 result type.
    pub type Result<T> = core::result::Result<T, Error>;

    impl Erc20 {
        /// Creates a new ERC-20 contract with the specified initial supply.
        #[ink(constructor)]
        pub fn new(initial_supply: Balance,name:String,symbol:String,decimals:u8,owner:AccountId) -> Self {
            let mut balances = StorageHashMap::new();
            balances.insert(owner, initial_supply);
            let mut instance = Self {
                total_supply: Lazy::new(initial_supply),
                balances,
                allowances: StorageHashMap::new(),
                name,
                symbol,
                decimals,
                owner,
                check_points:StorageHashMap::new(),
                num_check_points:StorageHashMap::new(),
                delegates:StorageHashMap::new(),
            };

            Self::env().emit_event(Transfer {
                from: None,
                to: Some(owner),
                value: initial_supply,
            });
            instance.move_delegates(AccountId::default(), owner, initial_supply);
            instance
        }
        /// Displays the details of the token
        #[ink(message)]
        pub fn query_info(&self) -> TokenInfo {
            TokenInfo {
                name: self.name.clone(),
                symbol: self.symbol.clone(),
                total_supply: *self.total_supply,
                decimals: self.decimals,
                owner: self.owner
            }
        }

        /// Returns the account balance for the specified `owner`.
        ///
        /// Returns `0` if the account is non-existent.
        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> Balance {
            self.balances.get(&owner).copied().unwrap_or(0)
        }

        /// Returns the amount which `spender` is still allowed to withdraw from `owner`.
        ///
        /// Returns `0` if no allowance has been set `0`.
        #[ink(message)]
        pub fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
            self.allowances.get(&(owner, spender)).copied().unwrap_or(0)
        }

        /// Transfers `value` amount of tokens from the caller's account to account `to`.
        ///
        /// On success a `Transfer` event is emitted.
        ///
        /// # Errors
        ///
        /// Returns `InsufficientBalance` error if there are not enough tokens on
        /// the caller's account balance.
        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<()> {
            let from = self.env().caller();
            self.transfer_from_to(from, to, value)
        }

        /// Allows `spender` to withdraw from the caller's account multiple times, up to
        /// the `value` amount.
        ///
        /// If this function is called again it overwrites the current allowance with `value`.
        ///
        /// An `Approval` event is emitted.
        #[ink(message)]
        pub fn approve(&mut self, spender: AccountId, value: Balance) -> Result<()> {
            let owner = self.env().caller();
            self.allowances.insert((owner, spender), value);
            self.env().emit_event(Approval {
                owner,
                spender,
                value,
            });
            Ok(())
        }

        /// Transfers `value` tokens on the behalf of `from` to the account `to`.
        ///
        /// This can be used to allow a contract to transfer tokens on ones behalf and/or
        /// to charge fees in sub-currencies, for example.
        ///
        /// On success a `Transfer` event is emitted.
        ///
        /// # Errors
        ///
        /// Returns `InsufficientAllowance` error if there are not enough tokens allowed
        /// for the caller to withdraw from `from`.
        ///
        /// Returns `InsufficientBalance` error if there are not enough tokens on
        /// the account balance of `from`.
        #[ink(message)]
        pub fn transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            value: Balance,
        ) -> Result<()> {
            let caller = self.env().caller();
            let allowance = self.allowance(from, caller);
            if allowance < value {
                return Err(Error::InsufficientAllowance)
            }
            self.transfer_from_to(from, to, value)?;
            self.allowances.insert((from, caller), allowance - value);
            Ok(())
        }

        /// Transfers `value` amount of tokens from the caller's account to account `to`.
        ///
        /// On success a `Transfer` event is emitted.
        ///
        /// # Errors
        ///
        /// Returns `InsufficientBalance` error if there are not enough tokens on
        /// the caller's account balance.
        fn transfer_from_to(
            &mut self,
            from: AccountId,
            to: AccountId,
            value: Balance,
        ) -> Result<()> {
            let from_balance = self.balance_of(from);
            if from_balance < value {
                return Err(Error::InsufficientBalance)
            }
            self.balances.insert(from, from_balance - value);
            let to_balance = self.balance_of(to);
            self.balances.insert(to, to_balance + value);

            self.move_delegates(from, to, value);

            self.env().emit_event(Transfer {
                from: Some(from),
                to: Some(to),
                value,
            });
            Ok(())
        }

        // fn _mint_token(
        //     &mut self,
        //     to: AccountId,
        //     amount: Balance,
        // ) -> bool {
        //     let total_supply = self.total_supply();
        //     assert_eq!(total_supply + amount >= total_supply, true);
        //     let to_balance = self.balance_of_or_zero(&to);
        //     assert_eq!(to_balance + amount >= to_balance, true);
        //     self.total_supply += amount;
        //     self.balances.insert(to, to_balance + amount);
        //     self.env().emit_event(Transfer {
        //         from: None,
        //         to: Some(to),
        //         value: amount,
        //     });
        //     true
        // }

        /// Get current votes
        /// # Fields
        /// user:the address of user
        #[ink(message)]
        pub fn get_current_votes(&self,user:AccountId) -> u128 {
            let default_checkpoint = Checkpoint{from_block:0, votes:0};
            let n_checkpoints = self.num_check_points.get(&user).unwrap_or(&0).clone();
            return if n_checkpoints > 0 {  let check_point:Checkpoint = self.check_points.get(&(user,n_checkpoints - 1)).unwrap_or(&default_checkpoint).clone();check_point.votes}  else { 0 } ;
        }
        /// Get the number of votes for a block
        /// # Fields
        /// account:the address of user
        /// block_number : the block number
        #[ink(message)]
        pub fn get_prior_votes(&self,account:AccountId,block_number:u32) -> u128 {
            assert!(block_number <  self.env().block_number());
            let default_checkpoint = Checkpoint{from_block:0, votes:0};
            let n_checkpoints = self.num_check_points.get(&account).unwrap_or(&0).clone();
            if n_checkpoints == 0 {
                return 0;
            }
            let check_point:Checkpoint = self.check_points.get(&(account,n_checkpoints - 1)).unwrap_or(&default_checkpoint).clone();
            if check_point.from_block <= block_number {
                return check_point.votes;
            }
            // Next check implicit zero balance
            let check_point_zero:Checkpoint = self.check_points.get(&(account,0)).unwrap_or(&default_checkpoint).clone();
            if check_point_zero.from_block > block_number {
                return 0;
            }
            let mut lower:u32 = 0;
            let mut upper:u32 = n_checkpoints - 1;
            while upper > lower {
                let center:u32 = upper - (upper - lower) / 2; // ceil, avoiding overflow
                let  cp:Checkpoint = self.check_points.get(&(account,center)).unwrap_or(&default_checkpoint).clone();
                if cp.from_block == block_number {
                    return cp.votes;
                } else if cp.from_block < block_number {
                    lower = center;
                } else {
                    upper = center - 1;
                }
            }
            let outer_cp:Checkpoint = self.check_points.get(&(account,lower)).unwrap_or(&default_checkpoint).clone();
            return outer_cp.votes;
        }
        /// Delegate votes to others
        /// # Fields
        /// delegatee:the address of others
        #[ink(message)]
        pub fn delegate(&mut self,delegatee:AccountId) -> bool {
            let delegator = self.env().caller();
            let current_delegate =  self.delegates.get(&delegator).copied().unwrap_or(AccountId::default());
            let delegator_balance = self.balance_of(delegator);
            self.delegates.insert(delegator,delegatee);
            Self::env().emit_event(DelegateChanged {
                delegator,
                current_delegate,
                delegatee
            });
            self.move_delegates(current_delegate, delegatee, delegator_balance);

            true
        }
        /// Get user's delegation information
       /// # Fields
       /// delegator:the address of user
        #[ink(message)]
        pub fn get_user_delegates(&self,delegator:AccountId) -> AccountId {
            self.delegates.get(&delegator).unwrap_or(&AccountId::default()).clone()
        }
        /// Get user's points
        /// # Fields
        /// user:the address of user
        #[ink(message)]
        pub fn get_user_num_check_points(&self,user:AccountId) -> u32 {
            self.num_check_points.get(&user).unwrap_or(&0).clone()
        }
        /// Get user's check_points
        /// # Fields
        /// account:the address of user
        /// checkpoint:the point of user
        #[ink(message)]
        pub fn get_user_check_points(&self,account:AccountId,checkpoint:u32) -> Checkpoint {
            let default_checkpoint = Checkpoint{from_block:0, votes:0};
            self.check_points.get(&(account,checkpoint)).unwrap_or(&default_checkpoint).clone()
        }
        fn move_delegates(&mut self,src_rep:AccountId,dst_rep:AccountId,amount:u128) -> bool {
            let default_checkpoint = Checkpoint{from_block:0, votes:0};
            if src_rep != dst_rep && amount > 0 {
                if src_rep != AccountId::default() {
                    let src_rep_num =  self.num_check_points.get(&src_rep).unwrap_or(&0).clone();
                    let src_rep_old = if src_rep_num > 0 {
                        let check_point_src:Checkpoint = self.check_points.get(&(src_rep,src_rep_num - 1)).unwrap_or(&default_checkpoint).clone();
                        check_point_src.votes
                    }  else {
                        0
                    } ;
                    let src_rep_new =  src_rep_old - amount;
                    self.write_check_point(src_rep,src_rep_num,src_rep_old,src_rep_new);
                }
                if dst_rep != AccountId::default() {
                    let dst_rep_num = self.num_check_points.get(&dst_rep).copied().unwrap_or(0);
                    let dst_rep_old = if dst_rep_num > 0 {
                        let check_point_dst:Checkpoint = self.check_points.get(&(dst_rep,dst_rep_num - 1)).unwrap_or(&default_checkpoint).clone();
                        check_point_dst.votes
                    }  else {
                        0
                    } ;
                    let dsp_rep_new =  dst_rep_old + amount;
                    self.write_check_point(dst_rep,dst_rep_num,dst_rep_old,dsp_rep_new);
                }
            }
            true
        }

        fn write_check_point(&mut self,delegatee:AccountId,n_checkpoints:u32,old_votes:u128,new_votes:u128) -> bool {
            let block_number = self.env().block_number();
            let mut check_point = Checkpoint{from_block:0, votes:0};
            if n_checkpoints > 0 {
                 check_point = self.check_points.get(&(delegatee,n_checkpoints - 1)).unwrap().clone();
            }
            if  n_checkpoints>0 && check_point.from_block == block_number  {
                self.check_points.insert((delegatee, n_checkpoints - 1), Checkpoint{from_block:check_point.from_block,votes:new_votes});
            } else {
                self.check_points.insert((delegatee, n_checkpoints),Checkpoint{from_block:block_number,votes:new_votes});
                self.num_check_points.insert(delegatee,n_checkpoints + 1 );
            }
            Self::env().emit_event(DelegateVotesChanged {
                delegate: delegatee,
                old_votes,
                new_votes,
            });
            true
        }

    }

    /// Unit tests.
    #[cfg(not(feature = "ink-experimental-engine"))]
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        type Event = <Erc20 as ::ink_lang::BaseEvent>::Type;

        use ink_lang as ink;

        fn assert_transfer_event(
            event: &ink_env::test::EmittedEvent,
            expected_from: Option<AccountId>,
            expected_to: Option<AccountId>,
            expected_value: Balance,
        ) {
            let decoded_event = <Event as scale::Decode>::decode(&mut &event.data[..])
                .expect("encountered invalid contract event data buffer");
            if let Event::Transfer(Transfer { from, to, value }) = decoded_event {
                assert_eq!(from, expected_from, "encountered invalid Transfer.from");
                assert_eq!(to, expected_to, "encountered invalid Transfer.to");
                assert_eq!(value, expected_value, "encountered invalid Trasfer.value");
            } else {
                panic!("encountered unexpected event kind: expected a Transfer event")
            }
            let expected_topics = vec![
                encoded_into_hash(&PrefixedValue {
                    value: b"Erc20::Transfer",
                    prefix: b"",
                }),
                encoded_into_hash(&PrefixedValue {
                    prefix: b"Erc20::Transfer::from",
                    value: &expected_from,
                }),
                encoded_into_hash(&PrefixedValue {
                    prefix: b"Erc20::Transfer::to",
                    value: &expected_to,
                }),
                encoded_into_hash(&PrefixedValue {
                    prefix: b"Erc20::Transfer::value",
                    value: &expected_value,
                }),
            ];
            for (n, (actual_topic, expected_topic)) in
                event.topics.iter().zip(expected_topics).enumerate()
            {
                let topic = actual_topic
                    .decode::<Hash>()
                    .expect("encountered invalid topic encoding");
                assert_eq!(topic, expected_topic, "encountered invalid topic at {}", n);
            }
        }

        /// The default constructor does its job.
        #[ink::test]
        fn new_works() {
            // Constructor works.
            let _erc20 = Erc20::new(100,String::from("test"),String::from("test"),8,AccountId::from([0x01; 32]),AccountId::from([0x01; 32]),AccountId::from([0x01; 32]));

            // Transfer event triggered during initial construction.
            let emitted_events = ink_env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(1, emitted_events.len());

            assert_transfer_event(
                &emitted_events[0],
                None,
                Some(AccountId::from([0x01; 32])),
                100,
            );
        }

        /// The total supply was applied.
        #[ink::test]
        fn total_supply_works() {
            // Constructor works.
            let erc20 = Erc20::new(100,String::from("test"),String::from("test"),8,AccountId::from([0x01; 32]),AccountId::from([0x01; 32]),AccountId::from([0x01; 32]));
            // Transfer event triggered during initial construction.
            let emitted_events = ink_env::test::recorded_events().collect::<Vec<_>>();
            assert_transfer_event(
                &emitted_events[0],
                None,
                Some(AccountId::from([0x01; 32])),
                100,
            );
            // Get the token total supply.
            assert_eq!(erc20.total_supply(), 100);
        }

        /// Get the actual balance of an account.
        #[ink::test]
        fn balance_of_works() {
            // Constructor works
            let erc20 = Erc20::new(
                100,
                String::from("test"),
                String::from("test"),
                8,
                AccountId::from([0x01; 32]),
                AccountId::from([0x01; 32]),
                AccountId::from([0x01; 32])
            );
            // Transfer event triggered during initial construction
            let emitted_events = ink_env::test::recorded_events().collect::<Vec<_>>();
            assert_transfer_event(
                &emitted_events[0],
                None,
                Some(AccountId::from([0x01; 32])),
                100,
            );
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                    .expect("Cannot get accounts");
            // Alice owns all the tokens on deployment
            assert_eq!(erc20.balance_of(accounts.alice), 100);
            // Bob does not owns tokens
            assert_eq!(erc20.balance_of(accounts.bob), 0);
        }

        #[ink::test]
        fn transfer_works() {
            // Constructor works.
            let mut erc20 = Erc20::new(100,String::from("test"),String::from("test"),8,AccountId::from([0x01; 32]),AccountId::from([0x01; 32]),AccountId::from([0x01; 32]));
            // Transfer event triggered during initial construction.
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                    .expect("Cannot get accounts");

            assert_eq!(erc20.balance_of(accounts.bob), 0);
            // Alice transfers 10 tokens to Bob.
            assert_eq!(erc20.transfer(accounts.bob, 10), Ok(()));
            // Bob owns 10 tokens.
            assert_eq!(erc20.balance_of(accounts.bob), 10);

            let emitted_events = ink_env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(emitted_events.len(), 2);
            // Check first transfer event related to ERC-20 instantiation.
            assert_transfer_event(
                &emitted_events[0],
                None,
                Some(AccountId::from([0x01; 32])),
                100,
            );
            // Check the second transfer event relating to the actual trasfer.
            assert_transfer_event(
                &emitted_events[1],
                Some(AccountId::from([0x01; 32])),
                Some(AccountId::from([0x02; 32])),
                10,
            );
        }

        #[ink::test]
        fn invalid_transfer_should_fail() {
            // Constructor works.
            let mut erc20 = Erc20::new(100,String::from("test"),String::from("test"),8,AccountId::from([0x01; 32]),AccountId::from([0x01; 32]),AccountId::from([0x01; 32]));
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                    .expect("Cannot get accounts");

            assert_eq!(erc20.balance_of(accounts.bob), 0);
            // Get contract address.
            let callee = ink_env::account_id::<ink_env::DefaultEnvironment>()
                .unwrap_or_else(|_| [0x0; 32].into());
            // Create call
            let mut data =
                ink_env::test::CallData::new(ink_env::call::Selector::new([0x00; 4])); // balance_of
            data.push_arg(&accounts.bob);
            // Push the new execution context to set Bob as caller
            ink_env::test::push_execution_context::<ink_env::DefaultEnvironment>(
                accounts.bob,
                callee,
                1000000,
                1000000,
                data,
            );

            // Bob fails to transfers 10 tokens to Eve.
            assert_eq!(
                erc20.transfer(accounts.eve, 10),
                Err(Error::InsufficientBalance)
            );
            // Alice owns all the tokens.
            assert_eq!(erc20.balance_of(accounts.alice), 100);
            assert_eq!(erc20.balance_of(accounts.bob), 0);
            assert_eq!(erc20.balance_of(accounts.eve), 0);

            // Transfer event triggered during initial construction.
            let emitted_events = ink_env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(emitted_events.len(), 1);
            assert_transfer_event(
                &emitted_events[0],
                None,
                Some(AccountId::from([0x01; 32])),
                100,
            );
        }

        #[ink::test]
        fn transfer_from_works() {
            // Constructor works.
            let mut erc20 = Erc20::new(100,String::from("test"),String::from("test"),8,AccountId::from([0x01; 32]),AccountId::from([0x01; 32]),AccountId::from([0x01; 32]));
            // Transfer event triggered during initial construction.
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                    .expect("Cannot get accounts");

            // Bob fails to transfer tokens owned by Alice.
            assert_eq!(
                erc20.transfer_from(accounts.alice, accounts.eve, 10),
                Err(Error::InsufficientAllowance)
            );
            // Alice approves Bob for token transfers on her behalf.
            assert_eq!(erc20.approve(accounts.bob, 10), Ok(()));

            // The approve event takes place.
            assert_eq!(ink_env::test::recorded_events().count(), 2);

            // Get contract address.
            let callee = ink_env::account_id::<ink_env::DefaultEnvironment>()
                .unwrap_or_else(|_| [0x0; 32].into());
            // Create call.
            let mut data =
                ink_env::test::CallData::new(ink_env::call::Selector::new([0x00; 4])); // balance_of
            data.push_arg(&accounts.bob);
            // Push the new execution context to set Bob as caller.
            ink_env::test::push_execution_context::<ink_env::DefaultEnvironment>(
                accounts.bob,
                callee,
                1000000,
                1000000,
                data,
            );

            // Bob transfers tokens from Alice to Eve.
            assert_eq!(
                erc20.transfer_from(accounts.alice, accounts.eve, 10),
                Ok(())
            );
            // Eve owns tokens.
            assert_eq!(erc20.balance_of(accounts.eve), 10);

            // Check all transfer events that happened during the previous calls:
            let emitted_events = ink_env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(emitted_events.len(), 3);
            assert_transfer_event(
                &emitted_events[0],
                None,
                Some(AccountId::from([0x01; 32])),
                100,
            );
            // The second event `emitted_events[1]` is an Approve event that we skip checking.
            assert_transfer_event(
                &emitted_events[2],
                Some(AccountId::from([0x01; 32])),
                Some(AccountId::from([0x05; 32])),
                10,
            );
        }

        #[ink::test]
        fn allowance_must_not_change_on_failed_transfer() {
            let mut erc20 = Erc20::new(100,String::from("test"),String::from("test"),8,AccountId::from([0x01; 32]),AccountId::from([0x01; 32]),AccountId::from([0x01; 32]));
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                    .expect("Cannot get accounts");

            // Alice approves Bob for token transfers on her behalf.
            let alice_balance = erc20.balance_of(accounts.alice);
            let initial_allowance = alice_balance + 2;
            assert_eq!(erc20.approve(accounts.bob, initial_allowance), Ok(()));

            // Get contract address.
            let callee = ink_env::account_id::<ink_env::DefaultEnvironment>()
                .unwrap_or_else(|_| [0x0; 32].into());
            // Create call.
            let mut data =
                ink_env::test::CallData::new(ink_env::call::Selector::new([0x00; 4])); // balance_of
            data.push_arg(&accounts.bob);
            // Push the new execution context to set Bob as caller.
            ink_env::test::push_execution_context::<ink_env::DefaultEnvironment>(
                accounts.bob,
                callee,
                1000000,
                1000000,
                data,
            );

            // Bob tries to transfer tokens from Alice to Eve.
            let emitted_events_before = ink_env::test::recorded_events();
            assert_eq!(
                erc20.transfer_from(accounts.alice, accounts.eve, alice_balance + 1),
                Err(Error::InsufficientBalance)
            );
            // Allowance must have stayed the same
            assert_eq!(
                erc20.allowance(accounts.alice, accounts.bob),
                initial_allowance
            );
            // No more events must have been emitted
            let emitted_events_after = ink_env::test::recorded_events();
            assert_eq!(emitted_events_before.count(), emitted_events_after.count());
        }
    }

    /// For calculating the event topic hash.
    struct PrefixedValue<'a, 'b, T> {
        pub prefix: &'a [u8],
        pub value: &'b T,
    }

    impl<X> scale::Encode for PrefixedValue<'_, '_, X>
    where
        X: scale::Encode,
    {
        #[inline]
        fn size_hint(&self) -> usize {
            self.prefix.size_hint() + self.value.size_hint()
        }

        #[inline]
        fn encode_to<T: scale::Output + ?Sized>(&self, dest: &mut T) {
            self.prefix.encode_to(dest);
            self.value.encode_to(dest);
        }
    }

    #[cfg(feature = "ink-experimental-engine")]
    #[cfg(test)]
    mod tests_experimental_engine {
        use super::*;

        use ink_env::Clear;
        use ink_lang as ink;

        type Event = <Erc20 as ::ink_lang::BaseEvent>::Type;

        fn assert_transfer_event(
            event: &ink_env::test::EmittedEvent,
            expected_from: Option<AccountId>,
            expected_to: Option<AccountId>,
            expected_value: Balance,
        ) {
            let decoded_event = <Event as scale::Decode>::decode(&mut &event.data[..])
                .expect("encountered invalid contract event data buffer");
            if let Event::Transfer(Transfer { from, to, value }) = decoded_event {
                assert_eq!(from, expected_from, "encountered invalid Transfer.from");
                assert_eq!(to, expected_to, "encountered invalid Transfer.to");
                assert_eq!(value, expected_value, "encountered invalid Trasfer.value");
            } else {
                panic!("encountered unexpected event kind: expected a Transfer event")
            }
            let expected_topics = vec![
                encoded_into_hash(&PrefixedValue {
                    value: b"Erc20::Transfer",
                    prefix: b"",
                }),
                encoded_into_hash(&PrefixedValue {
                    prefix: b"Erc20::Transfer::from",
                    value: &expected_from,
                }),
                encoded_into_hash(&PrefixedValue {
                    prefix: b"Erc20::Transfer::to",
                    value: &expected_to,
                }),
                encoded_into_hash(&PrefixedValue {
                    prefix: b"Erc20::Transfer::value",
                    value: &expected_value,
                }),
            ];

            let topics = event.topics.clone();
            for (n, (actual_topic, expected_topic)) in
                topics.iter().zip(expected_topics).enumerate()
            {
                let mut topic_hash = Hash::clear();
                let len = actual_topic.len();
                topic_hash.as_mut()[0..len].copy_from_slice(&actual_topic[0..len]);

                assert_eq!(
                    topic_hash, expected_topic,
                    "encountered invalid topic at {}",
                    n
                );
            }
        }

        /// The default constructor does its job.
        #[ink::test]
        fn new_works() {
            // Constructor works.
            let _erc20 = Erc20::new(100);

            // Transfer event triggered during initial construction.
            let emitted_events = ink_env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(1, emitted_events.len());

            assert_transfer_event(
                &emitted_events[0],
                None,
                Some(AccountId::from([0x01; 32])),
                100,
            );
        }

        /// The total supply was applied.
        #[ink::test]
        fn total_supply_works() {
            // Constructor works.
            let erc20 = Erc20::new(100);
            // Transfer event triggered during initial construction.
            let emitted_events = ink_env::test::recorded_events().collect::<Vec<_>>();
            assert_transfer_event(
                &emitted_events[0],
                None,
                Some(AccountId::from([0x01; 32])),
                100,
            );
            // Get the token total supply.
            assert_eq!(erc20.total_supply(), 100);
        }

        /// Get the actual balance of an account.
        #[ink::test]
        fn balance_of_works() {
            // Constructor works
            let erc20 = Erc20::new(100);
            // Transfer event triggered during initial construction
            let emitted_events = ink_env::test::recorded_events().collect::<Vec<_>>();
            assert_transfer_event(
                &emitted_events[0],
                None,
                Some(AccountId::from([0x01; 32])),
                100,
            );
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();
            // Alice owns all the tokens on deployment
            assert_eq!(erc20.balance_of(accounts.alice), 100);
            // Bob does not owns tokens
            assert_eq!(erc20.balance_of(accounts.bob), 0);
        }

        #[ink::test]
        fn transfer_works() {
            // Constructor works.
            let mut erc20 = Erc20::new(100);
            // Transfer event triggered during initial construction.
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();

            assert_eq!(erc20.balance_of(accounts.bob), 0);
            // Alice transfers 10 tokens to Bob.
            assert_eq!(erc20.transfer(accounts.bob, 10), Ok(()));
            // Bob owns 10 tokens.
            assert_eq!(erc20.balance_of(accounts.bob), 10);

            let emitted_events = ink_env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(emitted_events.len(), 2);
            // Check first transfer event related to ERC-20 instantiation.
            assert_transfer_event(
                &emitted_events[0],
                None,
                Some(AccountId::from([0x01; 32])),
                100,
            );
            // Check the second transfer event relating to the actual trasfer.
            assert_transfer_event(
                &emitted_events[1],
                Some(AccountId::from([0x01; 32])),
                Some(AccountId::from([0x02; 32])),
                10,
            );
        }

        #[ink::test]
        fn invalid_transfer_should_fail() {
            // Constructor works.
            let mut erc20 = Erc20::new(100);
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();

            assert_eq!(erc20.balance_of(accounts.bob), 0);

            // Set the contract as callee and Bob as caller.
            let contract = ink_env::account_id::<ink_env::DefaultEnvironment>()?;
            ink_env::test::set_callee::<ink_env::DefaultEnvironment>(contract);
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(accounts.bob);

            // Bob fails to transfers 10 tokens to Eve.
            assert_eq!(
                erc20.transfer(accounts.eve, 10),
                Err(Error::InsufficientBalance)
            );
            // Alice owns all the tokens.
            assert_eq!(erc20.balance_of(accounts.alice), 100);
            assert_eq!(erc20.balance_of(accounts.bob), 0);
            assert_eq!(erc20.balance_of(accounts.eve), 0);

            // Transfer event triggered during initial construction.
            let emitted_events = ink_env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(emitted_events.len(), 1);
            assert_transfer_event(
                &emitted_events[0],
                None,
                Some(AccountId::from([0x01; 32])),
                100,
            );
        }

        #[ink::test]
        fn transfer_from_works() {
            // Constructor works.
            let mut erc20 = Erc20::new(100);
            // Transfer event triggered during initial construction.
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();

            // Bob fails to transfer tokens owned by Alice.
            assert_eq!(
                erc20.transfer_from(accounts.alice, accounts.eve, 10),
                Err(Error::InsufficientAllowance)
            );
            // Alice approves Bob for token transfers on her behalf.
            assert_eq!(erc20.approve(accounts.bob, 10), Ok(()));

            // The approve event takes place.
            assert_eq!(ink_env::test::recorded_events().count(), 2);

            // Set the contract as callee and Bob as caller.
            let contract = ink_env::account_id::<ink_env::DefaultEnvironment>()?;
            ink_env::test::set_callee::<ink_env::DefaultEnvironment>(contract);
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(accounts.bob);

            // Bob transfers tokens from Alice to Eve.
            assert_eq!(
                erc20.transfer_from(accounts.alice, accounts.eve, 10),
                Ok(())
            );
            // Eve owns tokens.
            assert_eq!(erc20.balance_of(accounts.eve), 10);

            // Check all transfer events that happened during the previous calls:
            let emitted_events = ink_env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(emitted_events.len(), 3);
            assert_transfer_event(
                &emitted_events[0],
                None,
                Some(AccountId::from([0x01; 32])),
                100,
            );
            // The second event `emitted_events[1]` is an Approve event that we skip checking.
            assert_transfer_event(
                &emitted_events[2],
                Some(AccountId::from([0x01; 32])),
                Some(AccountId::from([0x05; 32])),
                10,
            );
        }

        #[ink::test]
        fn allowance_must_not_change_on_failed_transfer() {
            let mut erc20 = Erc20::new(100);
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();

            // Alice approves Bob for token transfers on her behalf.
            let alice_balance = erc20.balance_of(accounts.alice);
            let initial_allowance = alice_balance + 2;
            assert_eq!(erc20.approve(accounts.bob, initial_allowance), Ok(()));

            // Get contract address.
            let callee = ink_env::account_id::<ink_env::DefaultEnvironment>()?;
            ink_env::test::set_callee::<ink_env::DefaultEnvironment>(callee);
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(accounts.bob);

            // Bob tries to transfer tokens from Alice to Eve.
            let emitted_events_before =
                ink_env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(
                erc20.transfer_from(accounts.alice, accounts.eve, alice_balance + 1),
                Err(Error::InsufficientBalance)
            );
            // Allowance must have stayed the same
            assert_eq!(
                erc20.allowance(accounts.alice, accounts.bob),
                initial_allowance
            );
            // No more events must have been emitted
            let emitted_events_after =
                ink_env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(emitted_events_before.len(), emitted_events_after.len());
        }
    }

    #[cfg(test)]
    fn encoded_into_hash<T>(entity: &T) -> Hash
    where
        T: scale::Encode,
    {
        use ink_env::{
            hash::{
                Blake2x256,
                CryptoHash,
                HashOutput,
            },
            Clear,
        };
        let mut result = Hash::clear();
        let len_result = result.as_ref().len();
        let encoded = entity.encode();
        let len_encoded = encoded.len();
        if len_encoded <= len_result {
            result.as_mut()[..len_encoded].copy_from_slice(&encoded);
            return result
        }
        let mut hash_output = <<Blake2x256 as HashOutput>::Type as Default>::default();
        <Blake2x256 as CryptoHash>::hash(&encoded, &mut hash_output);
        let copy_len = core::cmp::min(hash_output.len(), len_result);
        result.as_mut()[0..copy_len].copy_from_slice(&hash_output[0..copy_len]);
        result
    }
}
