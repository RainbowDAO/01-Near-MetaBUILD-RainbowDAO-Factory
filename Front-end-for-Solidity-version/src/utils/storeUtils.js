require('dotenv').config();
const fs = require('fs');

const buildDir = "./src/abi/data/";

const writeDir = "./src/store/modules/abisMethods/"
if (!fs.existsSync(buildDir)) {
    throw new Error('ABI buildDir not exists');
}
if (!fs.existsSync(writeDir)) {
    throw new Error('ABI abisDirMap not exists');
}
const files = fs.readdirSync(buildDir);

console.log("contract number：", files.length)
//create index.js export all contract
let indexStr = ``, exportStr = `export default { `;
//deal all contract
for (let i = files.length - 1; i >= 0; i--) {
    // get contract name 、 abi
    let contract = JSON.parse(fs.readFileSync(buildDir + files[i]), 'utf8');
    let name = contract.contractName;
    let abi = contract.abi;

    let tempFileStr = `
		import getContract from "@/utils/abiUtil";

		function judgeToken(rootState) {
			if (!state.token) state.token = getContract.getContractByName('${name}', rootState.app.web3)
		}
		const state = {};
		const mutations = {};
	`
    let actions = ` const actions = { \n`;
    if(!contract.networks[666]){continue}
    console.log("create" + contract.contractName + ".js")

    // deal index.js
    indexStr += `import ${name} from "@/store/modules/abisMethods/${name}";`
    exportStr += `${name},`
    // end deal index.js

    // deal Methods
    for (let j = 0; j < abi.length; j++) {
        let functionObj = abi[j];

        //call Methods
        if(functionObj.type == "function" &&functionObj.stateMutability == "view"){
            // deal Methods
            let params = functionObj.inputs;
            let tempParamStr = ``
            for (let k = 0; k < params.length; k++) {
                tempParamStr += params[k].name + ","
            }
            //   deal Methods end

            tempParamStr = tempParamStr.substr(0,tempParamStr.length-1)
            actions += `${functionObj.name} ({rootState}${tempParamStr.length>0?',':''} ${tempParamStr}){
					judgeToken(rootState)
					return new Promise((resolve,reject) => {
						state.token.methods.${functionObj.name}(${tempParamStr}).call().then(res => {
						resolve(res)
					}).catch(err=>{
                        reject(JSON.parse(err.message.substr(24,err.message.length)).message)
                    })
				})
			},\n`

        }
        //end call Methods
        //send Methods
        if(functionObj.type == "function" &&functionObj.stateMutability == "nonpayable"){
            // deal Methods
            let params = functionObj.inputs;
            let tempParamStr = ``
            for (let k = 0; k < params.length; k++) {
                tempParamStr += params[k].name + ","
            }
            // deal Methods end

            tempParamStr = tempParamStr.substr(0,tempParamStr.length-1)
            actions += `${functionObj.name} ({rootState}${tempParamStr.length>0?',':''} ${tempParamStr}){
                        judgeToken(rootState)
                        return new Promise((resolve,reject) => {
                            state.token.methods.${functionObj.name}(${tempParamStr}).estimateGas({
                            from: rootState.app.account,
                        }).then(gas => {
                            state.token.methods.${functionObj.name}(${tempParamStr}).send({
                                from: rootState.app.account,
                                gas: parseInt(gas * 1.2)
                            }).then(res => {
                             
                                 resolve(res)
                            })
                        }).catch(err=>{
                            reject(JSON.parse(err.message.substr(24,err.message.length)).message)
                        })
				    })
			},\n`

        }
        //end send Methods
        // send with value Methods
        if(functionObj.type == "function" &&functionObj.stateMutability == "payable"){
            // deal params
            let params = functionObj.inputs;
            let tempParamStr = ``
            for (let k = 0; k < params.length; k++) {
                tempParamStr += params[k].name + ","
            }
            // deal params end

            tempParamStr = tempParamStr.substr(0,tempParamStr.length-1)
            actions += `${functionObj.name} ({rootState}, value ${tempParamStr.length>0?',':''} ${tempParamStr}){
                        judgeToken(rootState)
                        return new Promise((resolve,reject) => {
                            state.token.methods.${functionObj.name}(${tempParamStr}).estimateGas({
                            from: rootState.app.account,
                        }).then(gas => {
                            state.token.methods.${functionObj.name}(${tempParamStr}).send({
                                from: rootState.app.account,
                                gas: parseInt(gas * 1.2),
                                value: rootState.app.web3.utils.toWei(value)
                            }).then(res => {
                   
                              resolve(res)
                            })
                        }).catch(err=>{
                            reject(JSON.parse(err.message.substr(24,err.message.length)).message)
                        })
				    })
			},\n`

        }
        //end send with value Methods
    }
    actions += `}`
    tempFileStr += actions
    // Methods end

    tempFileStr += `
			export default {
			namespaced: true,
			mutations,
			state,
			actions
		}
	`
    fs.writeFileSync(`${writeDir}${name}.js`, tempFileStr, 'utf8');
}
//end deal all contract
exportStr += `}`
indexStr += exportStr
fs.writeFileSync(`${writeDir}index.js`, indexStr, 'utf8');
console.log("you get all vue store")
