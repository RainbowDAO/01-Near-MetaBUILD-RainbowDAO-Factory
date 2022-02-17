pragma solidity ^0.6.0;

contract RouteManage{
    address public core;
    
    constructor() public {
        core = msg.sender;
    }
    
     modifier onlyCore(){
        require(msg.sender == core, "only core");
        _;
    }
    
    mapping(string => address) public rainbowContracts;
     
    function getContract(string memory name) public view returns(address){
        return rainbowContracts[name];
    }

    function registerContract(string memory contractName, address contractAddress) onlyCore public {
        require(rainbowContracts[contractName] == address(0), "contract is exist");
        rainbowContracts[contractName] = contractAddress;
    }

    function updateContract(string memory contractName,address newAddress) onlyCore public {
        rainbowContracts[contractName] = newAddress;
    }
    
}
