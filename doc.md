##Contract introduction
- Kernel: controls the most basic modules, route_manage, authority_management, and role_manage of Rainbow Dao.
At the same time, it is the administrator of route_manage contract, authority_management contract and role_manage contract. 
The above three contracts can only be called through the kernel. 
Later, the control of the kernel will be transferred to community governance, and this contract can be invoked only through governance voting.
- role_manage:It controls the role of the whole rainbow protocol. It can give roles to various controls, modules and addresses, and let different modules drive different powers.
- authority_management:Set various permissions and bind them to roles. Multiple permissions can be added to a role.
- route_manage:Used to store the name and address of each contract bound. When a contract needs to call other contracts, you can get the address here. When a contract changes its address, you only need to update the contract address in route_manage, and other contracts do not need to upgrade the code.
- users_manage:It is used to store user information using rainbow protocol.
- multisig:It is used to generate multi sign addresses, which can be used as Dao administrators. You can set the number of multiple signatures and the number of multiple signatures you need to agree to perform the operation. 
- govnance_dao:It is the governance basis of the whole rainbow agreement, where you can initiate proposals to sort out the whole rainbow agreement.
- income_category:The classification of the whole rainbow agreement revenue is recorded here. When the switch of a classification is turned on, it means that he can charge.
- erc20Factory: It is a contract to generate tokens. The creator of Dao can easily generate erc20 tokens by passing in the basic information of tokens

