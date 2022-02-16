import daoFactory from "./data/daoFactory.json"
import daoManage from "./data/daoManage.json"
import vault from "./data/vault.json"
import erc20 from "./data/erc20.json"
const CONTRACTS = {
    daoFactory:{address:"0xb65daA181D320Eae638C8e71087e7e7C5c00db1b",abi:daoFactory},
    daoManage:{abi:daoManage},
    vault:{abi:vault},
    erc20:{abi:erc20},
};

function getContractByName(name, web3) {
    return new web3.eth.Contract(CONTRACTS[name].abi, CONTRACTS[name].address, {});
}

function getContractByContract(name, address, web3) {
    return new web3.eth.Contract(CONTRACTS[name].abi, address, {});
}

function getContractAddress(name) {
    return CONTRACTS[name].address;
}

export default {
    CONTRACTS,
    getContractByName,
    getContractByContract,
    getContractAddress
};
