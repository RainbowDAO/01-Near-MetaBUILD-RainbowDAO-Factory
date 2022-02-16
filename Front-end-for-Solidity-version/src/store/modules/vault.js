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
    if (!state.token) state.token = getContract.getContractByContract("vault",address, rootState.app.web3)
}

const actions = {
    approve({rootState}, {address,token,account}) {
        if(!address){
            return
        }
        judgeToken(rootState,address)
        return new Promise((resolve,reject) => {
            state.token.methods.approve(token,account).send({
                from: rootState.app.account,
            }).then(res => {
                resolve(res)
            }).catch(err=>{
                reject(err)
            })
        })
    },
    getTokenNumber({rootState},address) {
        if(!address){
            return
        }
        judgeToken(rootState,address)

        return new Promise((resolve,reject) => {
            state.token.methods.getTokenNumber().call({
                from: rootState.app.account,
            }).then(res => {
                resolve(res)
            }).catch(err=>{
                reject(err)
            })
        })
    },
    getTokenAddr({rootState}, {address,index}) {
        if(!address){
            return
        }
        judgeToken(rootState,address)
        return new Promise((resolve,reject) => {
            state.token.methods.getTokenAddr(index).call({
                from: rootState.app.account,
            }).then(res => {
                resolve(res)
            }).catch(err=>{
                reject(err)
            })
        })
    },
    getERC20Balance({rootState},{address,token}) {
        judgeToken(rootState,address)
        return new Promise((resolve,reject) => {
            state.token.methods. getERC20Balance(token).call({
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

