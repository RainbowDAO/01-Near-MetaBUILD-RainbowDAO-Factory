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
    if (!state.token) state.token = getContract.getContractByContract("daoManage",address, rootState.app.web3)
}

const actions = {
    vote({rootState}, {address,id,account,support}) {
        if(!address){
            return
        }
        judgeToken(rootState,address)
        return new Promise((resolve,reject) => {
            state.token.methods.vote(parseInt(id),account,parseInt(support)).send({
                from: rootState.app.account,
            }).then(res => {
                resolve(res)
            }).catch(err=>{
                reject(err)
            })
        })
    },
    getProposalLength({rootState},address) {
        if(!address){
            return
        }
        judgeToken(rootState,address)
        return new Promise((resolve,reject) => {
            state.token.methods.getProposalLength().call({
                from: rootState.app.account,
            }).then(res => {
                resolve(res)
            }).catch(err=>{
                reject(err)
            })
        })
    },
    getProposalInfo({rootState},{address,index}) {
        if(!address){
            return
        }
        judgeToken(rootState,address)
        return new Promise((resolve,reject) => {
            state.token.methods.getProposalInfo(index).call({
                from: rootState.app.account,
            }).then(res => {
                resolve(res)
            }).catch(err=>{
                reject(err)
            })
        })
    },
    proposalArray({rootState},{address,index}) {
        if(!address){
            return
        }
        judgeToken(rootState,address)
        return new Promise((resolve,reject) => {
            state.token.methods.proposalArray(index).call({
                from: rootState.app.account,
            }).then(res => {
                resolve(res)
            }).catch(err=>{
                reject(err)
            })
        })
    },
    newProposal({rootState},{address, title,description,endTime,voteTime,minAmount, token, to,   amount }) {
        if(!address){
            return
        }
        judgeToken(rootState,address)
        console.log(address, title,description,endTime,voteTime,minAmount, token, to,   amount)
        return new Promise((resolve,reject) => {
            state.token.methods.newProposal( title,description,endTime,voteTime,minAmount, token, to,   amount).send({
                from: rootState.app.account,
            }).then(res => {
                resolve(res)
            }).catch(err=>{
                reject(err)
            })
        })
    },
    getMemberByIndex({rootState},{address,index}) {
        if(!address){
            return
        }
        judgeToken(rootState,address)
        return new Promise((resolve,reject) => {
            state.token.methods.getMemberByIndex(index).call({
                from: rootState.app.account,
            }).then(res => {
                resolve(res)
            }).catch(err=>{
                reject(err)
            })
        })
    },
    getMemberLength({rootState},address) {
        if(!address){
            return
        }
        judgeToken(rootState,address)
        return new Promise((resolve,reject) => {
            state.token.methods.getMemberLength().call({
                from: rootState.app.account,
            }).then(res => {
                resolve(res)
            }).catch(err=>{
                reject(err)
            })
        })
    },
    applyJoin({rootState}, {address,manage}) {
        if(!address){
            return
        }
        judgeToken(rootState,address)
        return new Promise((resolve,reject) => {
            state.token.methods.applyJoin(manage).send({
                from: rootState.app.account,
            }).then(res => {
                resolve(res)
            }).catch(err=>{
                reject(err)
            })
        })
    },
    getApplyingLength({rootState},address) {
        if(!address){
            return
        }
        judgeToken(rootState,address)
        return new Promise((resolve,reject) => {
            state.token.methods.getApplyingLength().call({
                from: rootState.app.account,
            }).then(res => {
                resolve(res)
            }).catch(err=>{
                reject(err)
            })
        })
    },
    getApplyingByIndex({rootState}, {address,index}) {
        if(!address){
            return
        }
        judgeToken(rootState,address)
        return new Promise((resolve,reject) => {
            state.token.methods.getApplyingByIndex(index).call({
                from: rootState.app.account,
            }).then(res => {
                resolve(res)
            }).catch(err=>{
                reject(err)
            })
        })
    },
    approveApply({rootState}, {address,addr}) {
        if(!address){
            return
        }
        judgeToken(rootState,address)
        return new Promise((resolve,reject) => {
            state.token.methods.approveApply(addr).send({
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

