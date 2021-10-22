# RainbowDao ink!  
## Version 01
### 彩虹DAO治理
- 治理 rainbowGovnance

### 多签管理
- 多签管理合约 multiSignManage
### RainbowCore
- 角色管理 roleManage
- 权限管理 privilegeManage
- 路由管理 routeManage
- 核心     core
### 用户管理
- 用户合约（包含推荐关系） user
### 收入管理
- 收入类别管理 incomeCategory
- 收入比例管理 incomeProportion
### 代币工厂  
- erc20Factory  erc20
### DAO工厂  
- 工厂合约 daoFactory
- 模板管理 templateManagement
- DAO类别管理 daoCategory
- DAO基本信息管理 daoManage
- DAO治理(提案) daoGovnance
- DAO投票 daoVote
- DAO成员管理 daoUsers
- DAO金库管理 daoVault

### Dao类型的逻辑
创建Dao的时候可以选择创建哪种类型的Dao。类型共有3种，独立Dao,联盟Dao，母子DAO。创建部门的时候，需要选择隶属于哪个Dao。联盟Dao创建成功之后，需要独立Dao进行申请，申请成功之后加入联盟Dao。母子DAO，母子DAO的子DAO由母DAO创建，母DAO有控制子DAO的权限，子DAO的子DAO也可由母DAO管理，以此类推。子DAO对自己直接的子DAO有管理权限，不能跨级管理。例：母DAO A创建了子DAO B,子DAO B创建了C，C创建了D。A有对B,C,D管理的权限。B有C的管理权限，但B管不了D
