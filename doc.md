##Contract introduction
- Kernel: controls the most basic modules, route_manage, authority_management, and role_manage of Rainbow Dao.
At the same time, it is the administrator of route_manage contract, authority_management contract and role_manage contract. 
The above three contracts can only be called through the kernel. 
Later, the control of the kernel will be transferred to community governance, and this contract can be invoked only through governance voting.
- role_manage:It controls the role of the whole rainbow protocol. It can give roles to various controls, modules and addresses, and let different modules drive different powers.
- authority_management:Set various permissions and bind them to roles. Multiple permissions can be added to a role.
- route_manage:Used to store the name and address of each contract bound. When a contract needs to call other contracts, you can get the address here. When a contract changes its address, you only need to update the contract address in route_manage, and other contracts do not need to upgrade the code.
- users_manage:It is used to store user information using rainbow protocol.


