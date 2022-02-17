pragma solidity ^0.6.0;

interface IAuthority {
    function addAct(string calldata _contractName, string calldata _func) external;
    function hasAuthority(address _account, string calldata _contractName,string calldata _func) external view returns(bool);
    function addAuthority(address _account,string calldata _contractName, string calldata _func) external;
}