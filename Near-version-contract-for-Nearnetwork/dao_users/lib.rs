use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen,AccountId};
use near_sdk::collections::LookupMap;
/// store a user info
/// addr:the address of user
/// expire_time : the expire of user
/// role : the role of user
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct User {
    addr : AccountId,
    expire_time:u128,
    role:u64
}
/// store a group info
/// id:the id of group
/// name:the name of group
/// join_directly:Join directly
/// is_open:Open or not
/// users:HashMap of user's address of bool
/// manager:the manager of group
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Group {
    id:u128,
    name:String,
    join_directly:bool,
    is_open:bool,
    users:LookupMap<AccountId,bool>,
    manager:AccountId
}

///All users in Dao are stored here
/// user:hashmap of user's address and userinfo
/// setting_addr:the address of setting
/// group:hashmap of group'id and group info
/// user_group:hashmap of user address , group id and bool
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct DaoUsers {
     user:LookupMap<AccountId,User>,
   // user_referer:StorageHashMap<AccountId,AccountId>,
   // length:u128,
    setting_addr:AccountId,
    group:LookupMap<u128,Group>,
    user_group:LookupMap<(AccountId,u128),bool>,
    group_index:u128
}

impl DaoUsers {
    pub fn new(setting_addr:AccountId) -> Self {
        Self {
            user:LookupMap::new(b"r".to_vec()),
            setting_addr,
            group:LookupMap::new(b"r".to_vec()),
            user_group:LookupMap::new(b"r".to_vec()),
            group_index:0
        }
    }
    /// add a group
    /// group:thr struct of group
    pub fn add_group(
        &mut self,
        name:String,
        join_directly:bool,
        is_open:bool,
    ) -> bool {
        let index = self.group_index.clone() + 1;
        let mut user = LookupMap::new(b"r".to_vec());
        user.insert(&env::signer_account_id(),&true);
        let group = Group{
            id:index,
            name,
            join_directly,
            is_open,
            users:user,
            manager:env::signer_account_id()
        };
        self.group_index += 1;
        self.group.insert(&index,&group);
        true
    }
    /// join the dao
    pub fn join(&mut self) ->bool {
        self.user.insert(&env::signer_account_id(),&User{addr:env::signer_account_id(),expire_time:0,role:0});
        true
    }
    /// Check whether the user has joined
    pub fn verify_user(&mut self,index:u128,user:AccountId) -> bool {
        let mut group =  self.group.get(&index).unwrap();
        assert_eq!(group.id > 0, true);
        group.users.insert(&user,&true);
        true
    }
    /// join a group
    /// index:the id of group
    pub fn join_group(&mut self,index:u128) -> bool {
        let  mut group =  self.group.get(&index).unwrap();
        let caller = env::signer_account_id();
        assert_eq!(group.id > 0, true);
        // let mut user_group = self.user_group.get_mut(&(caller,index)).unwrap();
        if group.join_directly == false {
            group.users.insert(&caller,&false);
        }else{
            group.users.insert(&caller,&true);
        }
        self.user_group.insert(&(caller,index),&true);
        true
    }
    /// show all user of dao
    pub fn list_user(self) -> LookupMap<AccountId,User> {
        self.user
    }
    /// show all group of dao
    pub fn list_group(self) -> LookupMap<u128,Group> {
        self.group
    }
    /// close a group
    /// id:the id of group
    pub fn close_group(&mut self,id:u128) -> bool {
        let mut group =  self.group.get(&id).unwrap();
        group.is_open = false;
        true
    }
}
