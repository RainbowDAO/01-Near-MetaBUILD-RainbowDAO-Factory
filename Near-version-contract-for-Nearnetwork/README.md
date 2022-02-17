# 01-Near-MetaBUILD-RainbowDAO-Factory
## Contract introduction
- dao_base: This contract is used to set and read the basic information of Dao.
- dao_category:This contract is used to control the type of Dao. Various types can be set here, such as alliance, mother child and independence.
- dao_factory:This is the factory contract of Dao, which is used to generate all kinds of Dao
- dao_manager:At this time, the contract of Dao is only used to manage all kinds of things of Dao. And initialize various other peripheral contracts. It can be said that this is the core of Dao.
- dao_proposal:This is the governance contract of Dao, which is used to vote and manage the size of Dao transactions.
- dao_setting:This is the basic setting contract of Dao, which controls the joining restrictions of various Dao.
- dao_users:It controls the departments and members of the whole Dao.
- dao_vault:This is the vault of Dao, which controls the token transfer and record of Dao.
- template_manager: The template of Dao is controlled here, which can be selected when generating Dao.



## Installing

Please make sure that you have these prerequisites installed on your computer:

```bash
rustup target add wasm32-unknown-unknown
```

- Install npm and node using a package manager such as nvm. Sometimes there are issues using Ledger due to how macOS handles node packages related to USB devices.
- Ensure you have installed Node version 12 or above.
- Install near-cli globally by running:

```bash
npm install -g near-cli
```

## Testing

First of all you need to clone the repository, run:

```bash
git clone https://github.com/RainbowDAO/01-Near-MetaBUILD-RainbowDAO-Factory.git
cd 01-Near-MetaBUILD-RainbowDAO-Factory
```

Then, You can enter any folder and enter the following command.

```bash
cargo  test
```

## Building

To build the WASM of your contract and metadata, You can enter any folder and enter the following command.
```bash
cargo build --target wasm32-unknown-unknown --release
```

