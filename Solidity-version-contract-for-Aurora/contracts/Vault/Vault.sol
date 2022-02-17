pragma solidity ^0.6.0;
pragma experimental ABIEncoderV2;
import "../interface/IERC20.sol";
import "../interface/IAuthority.sol";
import {Set} from "../lib/TokenSet.sol";

contract Vault{
    using Set for Set.Address;
    address public creator;
    address public manage;
    address public factory;
    IAuthority auth;
    
    struct Transfer{
        uint transferId;
        uint direction; //1: in ,2: out
        string tokenName;
        address from;
        address to;
        uint amount;
        uint time;
    }
    
    uint public index;

    Transfer[] public transferArray;
    Set.Address tokenArray;
    
    constructor(address _creator,address _factory,address _manage, address _auth) public {
        creator = _creator;
        factory = _factory;
        manage = _manage;
        auth = IAuthority(_auth);
    }
    
    modifier onlyCreator(){
        require(msg.sender == creator, "only owner");
        _;
    }
    
    event Deposit(address addr,address token, uint amount);
    event Withdraw(address token,address to, uint amount);
    
    function deposit(address token, uint amount) public returns(bool){
        require(msg.sender != address(0));
        require(tokenIfExisted(token),"Not Allowed");
        //auth or member
        //require()
        IERC20(token).transferFrom(msg.sender,address(this),amount);
        
        emit Deposit(msg.sender,token,amount);
    }
    
    function withdraw(address token,address to, uint amount) public returns(bool){
        require(tokenIfExisted(token),"Not Allowed");
        // require(msg.sender == daoManage,"only daoManage");
        // require(msg.sender != address(0));
        //auth or member
        //require()
        
        IERC20(token).transfer(to,amount);
        
        emit Withdraw(token,to,amount);
    }
    
    function getTransferArrayLength() public view returns(uint){
        return transferArray.length;
    }

    
    function getTokenAddr(uint _index) public view returns(address){
        return tokenArray.at(_index);
    }
    
    function getTokenNumber() public view returns(uint){
        return tokenArray.length();
    }
    
    function tokenIfExisted(address _addr) public view returns(bool){
        return tokenArray.contains(_addr);
    }
    
    function removeToken(address _addr) public {
        require(msg.sender == creator || auth.hasAuthority(msg.sender,"Vault","removeToken"));
        tokenArray.remove(_addr);
    }
    
    function addToken(address _addr) public{
        require(msg.sender == creator || msg.sender == factory,"None");
        tokenArray.add(_addr);
    }
    
    function getERC20Balance(address token) public view returns(uint){
        return IERC20(token).balanceOf(address(this));
    }
    
    function getTransferInfo(uint index) public view returns(Transfer memory){
        return transferArray[index];
    } 
    
    
    
}
