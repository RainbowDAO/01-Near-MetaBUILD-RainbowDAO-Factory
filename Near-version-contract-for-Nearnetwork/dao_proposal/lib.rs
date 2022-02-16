use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen,AccountId,Gas};
use near_sdk::collections::LookupMap;
use near_sdk::serde_json::{json};
const SINGLE_CALL_GAS: Gas = Gas(200000000000000);
/// The Voting details of a person
/// has_voted:Whether to vote
/// support:Is it supported
/// votes:Number of votes cast
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Receipt {
    has_voted: bool,
    support: bool,
    votes: u128,
}

 /// Details of the proposal
 /// proposal_id:proposal's id
 /// title:proposal's title
 /// desc:proposal's content
 /// start_block:proposal's start block
 /// end_block:proposal's end block
 /// for_votes:Number of support votes
 /// against_votes:Number of against votes
 /// canceled:it is cancel
 /// executed:it is executed
 /// receipts:Voting details
 /// transaction:Proposal implementation details
 #[near_bindgen]
 #[derive(BorshDeserialize, BorshSerialize)]
pub struct Proposal {
    proposal_id: u64,
    title: String,
    desc: String,
    start_block: u64,
    end_block: u64,
    for_votes: u128,
    against_votes: u128,
    owner: AccountId,
    canceled: bool,
    executed: bool,
    receipts: LookupMap<AccountId, Receipt>,
    transaction: Transaction,
    category:u32,
    publicity_votes:u128,
    publicity_delay:u64
}

///Restrictions on initiating proposals
///fee_open:Open charge limit
///fee_number:Charge quantity
///fee_token:Charging token
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Limit {
    fee_open:bool,
    fee_number:u128,
    fee_token:AccountId
}
/// Voting validity settings
/// category:the category of the settings
/// vote_scale:Voting rate setting
/// entrust_scale:Entrust rate setting
/// support_scale:Support rate setting
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct VoteEffective {
    category:u32,
    vote_scale:u128,
    entrust_scale:u128,
    support_scale:u128
}


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Transaction {
    /// The `AccountId` of the contract that is called in this transaction.
    callee: AccountId,
    /// The selector bytes that identifies the function of the callee that should be called.
    selector: [u8; 4],
    /// The SCALE encoded parameters that are passed to the called function.
    input: Vec<u8>,
    /// The amount of chain balance that is transferred to the callee.
    transferred_value: u64,
    /// Gas limit for the execution of the call.
    gas_limit: u64,
}

#[derive(BorshDeserialize, BorshSerialize)]
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

/// This is a proposal in Dao
/// creator:the creator of the contract
/// owner:the owner of the contract
/// proposals:HashMap of the proposal'id and proposal
/// voting_delay:Voting buffer
/// voting_period:Voting time
/// proposal_length:Total number of proposals
/// erc20_addr:the addr of erc20
/// limit:the limit of create proposal
/// vote_effective:the effective of vote
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct DaoProposal {
    creator:AccountId,
    owner: AccountId,
    proposals: LookupMap<u64, Proposal>,
    voting_delay: u32,
    voting_period: u32,
    proposal_length: u64,
    erc20_addr: AccountId,
    limit:Limit,
    vote_effective:VoteEffective
}

impl DaoProposal {
    pub fn new(creator:AccountId, erc20_addr: AccountId) -> Self {
        Self {
            creator,
            owner: env::signer_account_id(),
            proposals: LookupMap::new(b"r".to_vec()),
            voting_delay: 1,
            voting_period: 259200, //3 days
            proposal_length: 0,
            erc20_addr,
            limit:Limit{
                fee_open:false,
                fee_number:1,
                fee_token:env::signer_account_id()
            },
            vote_effective:VoteEffective{
                category:0,
                vote_scale:0,
                entrust_scale:0,
                support_scale:0
            }
        }
    }

    /// Set requirements for initiating proposals
    pub fn set_permission(&mut self,limit:Limit) -> bool {
        assert!(env::signer_account_id() != self.creator);
        self.limit = limit;

        true
    }
    /// Set the conditions for successful proposal
    pub fn set_vote_effective(&mut self,vote_effective:VoteEffective) -> bool {
        assert!(env::signer_account_id() != self.creator);
        self.vote_effective = vote_effective;
        true
    }


    /// Create a new proposal
    /// #Fields
    /// title:proposal's title
    /// desc:proposal's content
    /// category:proposal's category
    /// start_block:proposal's start_block
    /// end_block:proposal's end_block
    /// publicity_delay:Date of publication of the proposal
    /// transaction:proposal's transaction
    pub fn propose(
        &mut self,
        title: String,
        desc: String,
        category:u32,
        start_block:u64,
        end_block:u64,
        transaction: Transaction,
        publicity_delay:u64,
    ) -> bool {
        assert!(start_block > env::block_height());
        assert!(end_block > start_block);
        let limit = &self.limit;
        if limit.fee_open {
            env::promise_create(
                limit.fee_token.clone(),
                "transfer_from",
                json!({ "from": env::signer_account_id(),"to":env::signer_account_id(),"value":limit.fee_number }).to_string().as_bytes(),
                0,
                SINGLE_CALL_GAS,
            );
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
            owner: env::signer_account_id(),
            canceled: false,
            executed: false,
            receipts: LookupMap::new(b"r".to_vec()),
            transaction,
            publicity_votes:0,
            publicity_delay
        };
        self.proposals.insert(&proposal_id, &proposal_info);
        true
    }
     /// Show state of proposal
     /// proposal_id:proposal's id
    pub fn state(self, proposal_id: u64) -> ProposalState {
        let proposal: Proposal = self.proposals.get(&proposal_id).unwrap();
        let block_number = env::block_height();
        let effective:VoteEffective = self.vote_effective;
        let mut failed = false;
        let all_vote = proposal.for_votes + proposal.against_votes;
        if effective.category == 1 {
            if all_vote /  100 <= effective.vote_scale {
                failed = true;
            }
        }else if effective.category == 3 {
            if proposal.for_votes / all_vote * 100 <= effective.support_scale {
                failed = true;
            }
        }
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
    /// Set a proposal to cancel
    /// proposal_id:proposal's id
    pub fn cancel(&self, proposal_id: u64) -> bool {
        let mut proposal: Proposal = self.proposals.get(&proposal_id).unwrap();
        assert!(proposal.owner == env::signer_account_id());
        proposal.canceled = true;
        true
    }
    /// Vote for the publicity period
    /// proposal_id:proposal's id
    pub fn public_vote(&mut self, proposal_id: u64) -> bool {
        let block_number = env::block_height();
        let caller = env::signer_account_id();
        let mut proposal: Proposal = self.proposals.get(&proposal_id).unwrap();
        assert!(proposal.end_block < block_number);
        assert!(proposal.end_block + proposal.publicity_delay > block_number);
        env::promise_create(
            self.erc20_addr.clone(),
            "get_current_votes",
            json!({ "user": caller }).to_string().as_bytes(),
            0,
            SINGLE_CALL_GAS,
        );
        proposal.publicity_votes = 10;
        true
    }
    /// Vote on a proposal
    /// proposal_id:proposal's id
    /// support:Is it supported
    pub fn cast_vote(&mut self, proposal_id: u64, support: bool) -> bool {
        let caller = env::signer_account_id();
        let mut proposal: Proposal = self.proposals.get(&proposal_id).unwrap();
        let mut receipts = proposal.receipts.get(&caller).unwrap();
        assert!(receipts.has_voted == false);
        env::promise_create(
            self.erc20_addr.clone(),
            "get_current_votes",
            json!({ "user": caller }).to_string().as_bytes(),
            0,
            SINGLE_CALL_GAS,
        );
        if support {
            proposal.for_votes += 10;
        } else {
            proposal.against_votes += 10;
        }
        receipts.has_voted = true;
        receipts.support = support;
        receipts.votes = 10;
        true
    }
    /// Show all proposals
    pub fn list_proposals(self) -> LookupMap<u64, Proposal> {
        self.proposals
    }
    /// Show a proposal by id
    pub fn get_proposal_by_id(&self, proposal_id: u64) -> Proposal {
        let proposal: Proposal = self.proposals.get(&proposal_id).unwrap();
        proposal
    }
}
