pragma solidity ^0.6.0;
pragma experimental ABIEncoderV2;
import "./interface/IERC20.sol";
import "./interface/IUniswapV2Router01.sol";
import "./interface/ILendingPool.sol";

contract MultiSign{
    mapping (address => bool) public managers;
    uint public MIN_SIGNATURES;
    uint public transactionIdx;
    address private ETH_ADDR = 0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE;
    ICreator creator;
    struct Transaction {
        bool status;
        address token;
        address from;
        address to;
        uint amount;
        bool goal;  //true: add manager ,false:remove manager 
        uint8 kind; //5:lending  4:addLiquidity 3: manage 2:signature 1:transaction
        uint newMinSignatures;
        uint8 signatureCount;
        
        address secToken;
        uint secAmount;
    }
    
    mapping (uint => mapping(address => bool)) public signatures;
    mapping (uint => Transaction) public transactions;
    uint[] public pendingTransactions;
    
    modifier isManager{
        require(managers[msg.sender] == true);
        _;
    }
    
    constructor(uint _minSignCount,address[] memory _managers,address _creator) public{
        creator = ICreator(_creator);
        MIN_SIGNATURES = _minSignCount;
        for(uint i = 0; i < _managers.length; i++){
            managers[_managers[i]] = true;
        }
    }
    
    event DepositFunds(address from, uint amount);
    event TransferFunds(address to, uint amount);
    event TransactionCreated(
        address from,
        uint transactionId
    );
    
    event SignatureChanged(
        uint signatureOld,
        uint signature,
        uint transactionId
    );
    
    event ManagerChanged(
        address account,
        bool goal,
        uint transactionId
    );

    function receive() public payable{
        emit DepositFunds(msg.sender, msg.value);
    }
    
    function withdraw(uint amount) isManager public{
        creatTransaction(ETH_ADDR, msg.sender, amount);
    }
    
    function creatTransaction(address token, address to, uint amount) isManager public{
        require(address(0) != msg.sender);
        if(token == ETH_ADDR){
            require(address(this).balance >= amount);
        }
        if(token != ETH_ADDR){
            require(IERC20(token).balanceOf(address(this)) >= amount);
        }
        
        uint transactionId = transactionIdx++;
        
        Transaction memory transaction;
        transaction.status = false;
        transaction.token = token;
        transaction.from = address(this);
        transaction.to = to;
        transaction.amount = amount;
        transaction.signatureCount = 0;
        transaction.kind = 1;
        transactions[transactionId] = transaction;
        pendingTransactions.push(transactionId);
        emit TransactionCreated(msg.sender,transactionId);
    }
    
    function signTransaction(uint transactionId) public isManager{
        Transaction storage transaction = transactions[transactionId];
        require(address(0) != msg.sender);
        require(true != transaction.status);
        require(signatures[transactionId][msg.sender]!=true);
        signatures[transactionId][msg.sender] = true;
        transaction.signatureCount++;
        
        //transaction
        if(transaction.kind == 1 && transaction.signatureCount >= MIN_SIGNATURES){
            if(transaction.token == ETH_ADDR){
                require(address(this).balance >= transaction.amount);
                payable(transaction.to).transfer(transaction.amount);
            }
            if(transaction.token != ETH_ADDR){
                require(IERC20(transaction.token).balanceOf(address(this)) >= transaction.amount);
                IERC20(transaction.token).transfer(transaction.to, transaction.amount);
            }
            
            transaction.status = true;
            emit TransferFunds(transaction.to, transaction.amount);
        }
        //signature
        if(transaction.kind == 2 && transaction.signatureCount >= MIN_SIGNATURES){
            uint older = MIN_SIGNATURES;
            MIN_SIGNATURES = transaction.newMinSignatures;
            transaction.status = true;
            emit SignatureChanged(older,MIN_SIGNATURES,transactionId);
        }
        
        //manage
        if(transaction.kind == 3 && transaction.signatureCount >= MIN_SIGNATURES){
            //add
            if(transaction.goal){
                addManager(transaction.to);
            }
            //remove
            if(!transaction.goal){
                removeManager(transaction.to);
            }
            transaction.status = true;
            emit ManagerChanged(transaction.to, transaction.goal,transactionId);
        }
        //addLiquidity
        if(transaction.kind == 4 && transaction.signatureCount >= MIN_SIGNATURES){
            _addLiquidity(transaction.token,transaction.secToken,transaction.amount,transaction.secAmount);
            transaction.status = true;
        }
        //lending
        if(transaction.kind == 5 && transaction.signatureCount >= MIN_SIGNATURES){
            _lending(transaction.token,transaction.amount);
            transaction.status = true;
        }
    }
    
    function getPendingTransactions() public view returns(uint[] memory){
        return pendingTransactions;
    }
    
    function getLength() public view returns(uint){
        return pendingTransactions.length;
    }
    
    function getPendingTransactionById(uint transactionId) public view returns(Transaction memory){
        return transactions[transactionId];
    }
    
    function changeSignature(uint num) isManager public{
        uint transactionId = transactionIdx++;
        Transaction memory transaction;
        transaction.status = false;
        transaction.from = address(this);
        transaction.signatureCount = 0;
        transaction.kind = 2;
        transaction.newMinSignatures = num;
        transactions[transactionId] = transaction;
        pendingTransactions.push(transactionId);
        emit TransactionCreated(msg.sender, transactionId);
    }
    
    
    function changeManage(address account,bool goal) isManager public{
        uint transactionId = transactionIdx++;
        Transaction memory transaction;
        transaction.status = false;
        transaction.from = address(this);
        transaction.to = account;
        transaction.goal = goal;
        transaction.kind = 3;
        transactions[transactionId] = transaction;
        pendingTransactions.push(transactionId);
        emit TransactionCreated(msg.sender, transactionId);
    }
    
    function creatLiquidity(address tokenA,address tokenB,uint amountA,uint amountB) isManager public returns(bool){
        require(IERC20(tokenA).balanceOf(address(this)) >= amountA,"not enongh");
        require(IERC20(tokenB).balanceOf(address(this)) >= amountB,"not enongh");
        
        uint transactionId = transactionIdx++;
        Transaction memory transaction;
        transaction.status = false;
        transaction.from = address(this);
        transaction.token = tokenA;
        transaction.amount = amountA;
        transaction.secToken = tokenB;
        transaction.secAmount = amountB;
        transaction.kind = 4;
        transactions[transactionId] = transaction;
        pendingTransactions.push(transactionId);
        emit TransactionCreated(msg.sender, transactionId);
        
    }

    function creatLending(address token,uint amount) isManager public returns(bool){
        require(IERC20(token).balanceOf(address(this)) >= amount,"not enongh");

        uint transactionId = transactionIdx++;
        Transaction memory transaction;
        transaction.status = false;
        transaction.from = address(this);
        transaction.token = token;
        transaction.amount = amount;
        transaction.kind = 5;
        transactions[transactionId] = transaction;
        pendingTransactions.push(transactionId);
        emit TransactionCreated(msg.sender, transactionId);
        
    }
    
    function _addLiquidity(address tokenA,address tokenB,uint amountA,uint amountB) internal returns(bool){
        address router = creator.router();
        uint deadline = block.timestamp + 100;
        uint qr;
        // if(tokenA == ETH_ADDR){
        //     IERC20(tokenB).approve(router,amountB);
        //     (,,qr) = IUniswapV2Router01(router).addLiquidityETH(tokenB,amountB,0,0,address(this),deadline);
        // }
        // if(tokenB == ETH_ADDR){
        //     IERC20(tokenA).approve(router,amountA);
        //     (,,qr) = IUniswapV2Router01(router).addLiquidityETH(tokenA,amountA,0,0,address(this),deadline);
        // }
        
        IERC20(tokenA).approve(router,amountA);
        IERC20(tokenB).approve(router,amountB);
        (,,qr) = IUniswapV2Router01(router).addLiquidity(tokenA,tokenB,amountA,amountB,0,0,address(this),deadline);
        return true;
    }
    
    
    // function addLiquidityETH(address token,uint amount,address router) payable public returns(bool){
    //     // address router = creator.router();
    //     uint deadline = block.timestamp + 60;
    //     IERC20(token).approve(router,amount);
    //     (,,uint qr) = IUniswapV2Router01(router).addLiquidityETH(token,amount,0,0,address(this),deadline);
    //     liquidity[token][ETH_ADDR] += qr;
    //     liquidity[ETH_ADDR][token] += qr;
    //     return true;
    // }

    function _lending(address token,uint amount) internal returns(bool){
        address lending = creator.lending();
        IERC20(token).approve(lending,amount);
        ILendingPool(lending).deposit(token,amount,address(this),0);
        return true;
    }
    
    function removeLiquidity(address tokenA,address tokenB) isManager public returns(bool){
        address factory = creator.factory();
        address router = creator.router();
        address pair = IFactory(factory).getPair(tokenA,tokenB);
        uint deadline = block.timestamp + 60;
        IUniswapV2Router01(router).removeLiquidity(tokenA,tokenB,IERC20(pair).balanceOf(address(this)),0,0,address(this),deadline);
    }

    function withdrawLending(address token,uint amount) isManager public returns(bool){
        address lending = creator.lending();
        ILendingPool(lending).withdraw(token,amount,address(this));
    }
    
    function getLiquidity(address pair) public view returns(uint){
        return IERC20(pair).balanceOf(address(this));
    }
    
    function getBalance() public view returns(uint){
        return address(this).balance;
    }
    
    function getERC20Balance(address token) public view returns(uint){
        return IERC20(token).balanceOf(address(this));
    }
    
    function addManager(address manager) internal {
        managers[manager] = true;
        creator.addMultiSign(manager,address(this));
    }
    
    function removeManager(address manager) internal {
        managers[manager] = false;
        creator.removeMultiSign(manager,address(this));
    }
    
}

interface ICreator{
    function removeMultiSign(address account, address addr) external;
    function addMultiSign(address account, address addr) external;
    function factory() external view returns(address);
    function router() external view returns(address);
    function lending() external view returns(address);
}

interface IFactory{
    function getPair(address tokenA, address tokenB) external view returns (address pair);
}
