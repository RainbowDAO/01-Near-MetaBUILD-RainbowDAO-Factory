use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen,AccountId};
/// Fee limit for joining Dao
/// time_limit:How long is the total limit
/// fee_limit:the number of fee
/// token:the token of limit
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct FeeConditions {
    pub  time_limit:u128,
    pub  fee_limit:u128,
    pub  token:AccountId
}

/// Other limit for joining Dao
/// use_token:Whether to enable token restriction
/// use_nft:Whether to enable nft restriction
/// token:the token of limit
/// token_balance_limit:the balance of limit
/// nft:the nft address
/// nft_balance_limit:the balance of limit
/// nft_time_limit:Remaining time limit of NFT
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct OtherConditions {
    pub use_token:bool,
    pub use_nft:bool,
    pub token:AccountId,
    pub token_balance_limit:u128,
    pub nft:AccountId,
    pub nft_balance_limit:u128,
    pub nft_time_limit:u128
}
///creator:the creator's address
///owner:the manager's address
/// fee_limit:the fee limit info
/// other_limit:the other limit info
/// conditions:Specific restriction type
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct DaoSetting {
    creator:AccountId,
    owner:AccountId,
    fee_limit:FeeConditions,
    other_limit:OtherConditions,
    conditions : u64,
}

impl DaoSetting {
    pub fn new(creator:AccountId) -> Self {
        Self {
            creator,
            owner:env::signer_account_id(),
            fee_limit:FeeConditions{
                time_limit:0,
                fee_limit:0,
                token:env::signer_account_id()
            },
            other_limit:OtherConditions{
                use_token:false,
                use_nft:false,
                token:env::signer_account_id(),
                token_balance_limit:0,
                nft:env::signer_account_id(),
                nft_balance_limit:0,
                nft_time_limit:0
            },
            conditions:0,
        }
    }

    ///Get what restrictions to use
    pub fn get_conditions(&self) -> u64 {
        self.conditions
    }
    ///Get fee limit
    pub fn get_fee_setting(self) -> FeeConditions { self.fee_limit }
    ///Get other limit
    pub fn get_other_setting(self) -> OtherConditions {
        self.other_limit
    }
    ///set join limit
    pub fn set_join_limit(&mut self,conditions:u64,other_conditions:OtherConditions,fee_conditions:FeeConditions) -> bool {
        let owner = env::signer_account_id();
        assert_eq!(owner == self.creator, true);
        if conditions == 2 {
            self.fee_limit = fee_conditions;
        }else if conditions == 4 {
            self.other_limit = other_conditions;
        } else if conditions == 6 {
            self.fee_limit = fee_conditions;
            self.other_limit = other_conditions;
        }
        self.conditions = conditions;
        true
    }
}
