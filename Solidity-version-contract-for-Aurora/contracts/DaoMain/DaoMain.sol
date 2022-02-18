pragma solidity ^0.6.0;
pragma experimental ABIEncoderV2;

contract DaoMain{
    
    address public creator;
   
    constructor(address _creator) public {
        creator =_creator;
    }
}


