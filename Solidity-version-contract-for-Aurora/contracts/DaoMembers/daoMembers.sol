pragma solidity ^0.6.0;
pragma experimental ABIEncoderV2;
import "../interface/IERC20.sol";
import {Set} from "../lib/TokenSet.sol";

contract DaoMembers{
    using Set for Set.Address;
    address public creator;
   
    uint public voteId;
    
    mapping(address => string) public memberName;
    mapping(address => bool) public moderators;
    mapping(uint => mapping(address => bool)) public voters;
    mapping(address => uint) public joinTime;

    Set.Address applying;
    Set.Address members;
    
    constructor(address _creator) public {
        creator =_creator;
    }
    
    modifier onlyManage(){
        require(msg.sender == creator || moderators[msg.sender], "No permisstion");
        _;
    }
    
    function applyJoin(address addr) public returns(bool){
        require(!members.contains(addr),"Is memeber");
        require(!applying.contains(addr),"Applying");
        applying.add(addr);
        return true;
    }
    
    function approveApply(address addr) public returns(bool){
        require(msg.sender == creator || auth.hasAuthority(msg.sender,"DaoManage","approveApply"));
        applying.remove(addr);
        members.add(addr);
        return true;
    }
    
    function vetoApply(address addr) public returns(bool){
        require(msg.sender == creator || auth.hasAuthority(msg.sender,"DaoManage","vetoApply"));
        applying.remove(addr);
        return true;
    }
    
    
    // function vote(uint _voteId,address _account, bool _support) public {
    //     require(_voteId <= voteId,"Not exsited");
    //     require(members.contains(_account),"Not member");
    //     require(proposalInfo[voteId].endTime >= block.timestamp && proposalInfo[voteId].executed == false, "expired");
    //     require(!voters[_voteId][_account]);
        
    //     proposalInfo[_voteId].supportAmount ++;
    //     voters[_voteId][_account] = true;
    // }
    
    function execute(uint _voteId) public {
        require(_voteId <= voteId,"Not exsited");
        require(!proposalInfo[_voteId].executed);
        require(proposalInfo[_voteId].supportAmount !=0);
        require(proposalInfo[_voteId].supportAmount >= proposalInfo[_voteId].minAmount);
        
        proposalInfo[_voteId].executed = true;
        
        //Trading methods should be moved to Treasury contracts
        IERC20(proposalInfo[_voteId].erc20).transferFrom(vault,proposalInfo[_voteId].to,proposalInfo[_voteId].amount);
        
    }
    
    function getProposalByVoteId(uint voteId) public view returns(Proposal memory){
        return proposalInfo[voteId];
    }
    
    
    
    function getApplyingLength() public view returns(uint){
        return applying.length();
    }
    
    function getApplyingByIndex(uint index) public view returns(address){
        return applying.at(index);
    }
    
    function getMemberLength() public view returns(uint){
        return members.length();
    }
    
    function getMemberByIndex(uint index) public view returns(address){
        return members.at(index);
    }
    

    
    
}


