
pragma solidity ^0.6.0;

library StructTypes {
  struct cityNode{
        uint64 creationTime; //创建时间
        address nodeOwner; //所有者（管理员）
        string name; //名称
        uint nodeId; //节点id
        uint cityId; //当前节点所属城市
        bool isEffect; //是否激活
        bool firstEffect; //是否激活过
        address managerAddress; //节点管理合约的地址
        address cityNodePublicFundAddress ; //节点公共基金合约地址
        uint blockNumber; // 节点的当前区块高度  这个是可以变得
        address[] MsManager;//多签的数组 有12名
        uint currentCampaignManagerId; //当前管理员竞选ID
        uint currentCampaignMsManagerId; //当前多签竞选ID
        // uint64 msManagerExpireTime;  //多签到期时间
        // uint64 voteContinueTime; //多签在投票期内没到15个人 要延续时间
        // uint currentMsManagerIndex; //当前多签序列 一般为11(从0开始) 若发生弹劾 需往后顺延
    }
    
    struct info{
        uint id;
    }
    
    struct user {
        uint id; // 用户ID
        address addr;// 推荐人地址
        string nickname;
        uint vipLevel;
        address userAddress; //用户地址
        //        uint tickets; //用户的票数
        uint64 joinTime; //加入时间
        uint256 nodeId; //节点ID
        uint cityId;
        uint role;//1 管理员  2 多签  3普通成员
        uint expireTime; //管理or多签到期时间
        uint depositAmount ; //抵押RBT的数量
    }


    // struct user{
    //     address userAddress; //用户地址
    //     //        uint tickets; //用户的票数
    //     uint64 joinTime; //加入时间
    //     uint256 nodeId; //节点ID
    //     uint cityId;
    //     uint role;//1 管理员  2 多签  3普通成员
    //     uint expireTime; //管理or多签到期时间
    //     uint depositAmount ; //抵押RBT的数量
    // }


     struct campaignMsManager{
        address campaigner; //竞选者地址
        uint tickets; //选票数
    }


     struct governance {
        uint id;
        address owner;
        uint64 creationTime;
        address addr;
        uint prevId; //上级治理的id
        uint currentCampaignManagerId;
        address managerAddress;
        address publicFundAddress;
        uint64 expireTime;
        uint64 startTakeOfficeTime;
        address[] MsManager;
    }
}
