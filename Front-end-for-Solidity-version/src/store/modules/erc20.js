import getContract from '@/abi/index'

const state = {
    token: null,
    daoAddress: null
}
const mutations = {
    SET_DAOADDRESS(state, address) {
        state.daoAddress = address
    }
}

function judgeToken(rootState,address) {
    if (!state.token) state.token = getContract.getContractByContract("erc20",address, rootState.app.web3)
}

const actions = {
    decimals({rootState}, address) {
        if(!address){
            return
        }
        judgeToken(rootState,address)
        return new Promise((resolve,reject) => {
            state.token.methods.decimals().call({
                from: rootState.app.account,
            }).then(res => {
                resolve(res)
            }).catch(err=>{
                reject(err)
            })
        })
    },

}
export default {
    namespaced: true,
    state,
    mutations,
    actions
}

