# Documentation on how to run the project

## Contract introduction
- DaoFactory: Users can create their own DAO through factory contracts,displays a list of dao information,It's a collection of all the contracts.
- Authority: Used to manage permissions between accounts or contracts, which can be changed by proposal.
- DaoBaseInfo: It is used to save the basic information and types of dao.
- DaoManage: The dao's main logical management contract can manage funds or change contract parameters or contract permissions by requesting proposals.
- DaoMembers: Management contract for DAO members.
- ERC20Factory: This contract allows you to create your own ERC20 tokens.
- RouteManage: Routing contracts, record the addresses of all contracts.
- Vault: Token management contracts manage the expenditure and income of various tokens.
- Vote: The ability to initiate proposals for voting is the primary way to manage dao functionality.
## Dependencies

- Linux or Mac
- node ≥ 14

## Network & Contract
- Network Name: Aurora Testnet
- RPC URL: https://testnet.aurora.dev

### contract address

#### DaoFactory
0xb65daA181D320Eae638C8e71087e7e7C5c00db1b
#### Authority
0xF0b4F15495AE189A1F9e1210861aD47f18aD188f
#### DaoBaseInfo
0xb53E020F21bC17479443d7552F5681538d9De61c
#### DaoManage
0xDEf7188Bd888e230E7258410EF8a4e6Bee8AaaFF
#### DaoMembers
0xe1240c60f37ae1CdeC036591C9814c1Fce8d986c
#### ERC20Factory
0xa12F0BCc4E25607685F1B27e884fEa008Eb4bD0d
#### RouteManage
0x90E7F2c414a066C0995dccDADCd095d3e057cdE4
#### Vault
0x466Dc7d36085e981A7Ec5cb773e9b159f88CFF23
#### Vote
0xf9A3f8eb156B49Ae50ebe0A9e45A46aa22766795

## Installing

```bash
git clone https://github.com/RainbowDAO/04-Hackathon-DAO-factory.git
npm install
```

**Note**: Only the Metamask wallet is available for this demo


## Deploy Contracts
Open a local node or complete the information in the ```truffle-config.js``` and add the mnemonic to the ```secret```

```bash
npm install -g @truffle/hdwallet-provider
truffle compile && truffle migrate
```
You will deploy contracts
- DaoFactory

## Create a DAO
The creatDao in the DaoMain contract creates a DAO of its own, defining names, logos, and symbols

```creatDaoWithNewToken()``` or ```creatDao()```

