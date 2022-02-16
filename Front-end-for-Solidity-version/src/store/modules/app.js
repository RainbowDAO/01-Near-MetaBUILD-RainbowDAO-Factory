const state = {
    web3:null,
    account:'',
    isConnected:false,
    chainId: 666,
    ethBalance:0,
    isCollapse:true
}
const mutations = {
    SET_ACCOUNT(state, account){
        state.account = account
    },
    SET_ISCONNECT(state, isConnect){
        state.isConnected = isConnect
    },
    SET_Web3(state, web3){
        state.web3= web3
    },
    SET_Balance(state, amount){
        state.ethBalance= amount
    },
    SET_SIDEBAR(state, isCollapse){
        state.isCollapse= isCollapse
    },
}
const actions = {
    toggleSideBar({commit},isCollapse){
        commit("SET_SIDEBAR",isCollapse)
    }
}
export default {
    namespaced: true,
    state,
    mutations,
    actions
}
