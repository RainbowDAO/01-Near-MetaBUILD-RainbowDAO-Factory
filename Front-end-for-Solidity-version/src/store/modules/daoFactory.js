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

function judgeToken(rootState) {
    if (!state.token) state.token = getContract.getContractByName("daoFactory", rootState.app.web3)
}

const actions = {
    creatDaoWithNewToken({rootState}, {
        name,
        logo,
        des,
        tokenName,
        symbol,
        decimals,
        totalSupply,
        support
    }) {
        judgeToken(rootState)
        return new Promise((resolve,reject) => {
            state.token.methods.creatDaoWithNewToken(name,
                logo,
                des,
                tokenName,
                symbol,
                decimals,
                totalSupply,
                support).send({
                from: rootState.app.account,
            }).then(res => {
                resolve(res)
            }).catch(err=>{
                reject(err)
            })
        })
    },
    getOwnedDaos({rootState}) {
        judgeToken(rootState)
        return new Promise((resolve,reject) => {
            state.token.methods.getOwnedDaos().call({
                from: rootState.app.account,
            }).then(res => {
                resolve(res)
            }).catch(err=>{
                reject(err)
            })
        })
    },
    getArrayLength({rootState}) {
        judgeToken(rootState)
        return new Promise((resolve,reject) => {
            state.token.methods.getArrayLength().call({
                from: rootState.app.account,
            }).then(res => {
                resolve(res)
            }).catch(err=>{
                reject(err)
            })
        })
    },
    getDaoInfo({rootState},index) {
        judgeToken(rootState)
        return new Promise((resolve,reject) => {
            state.token.methods.getDaoInfo(index).call({
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

