<template>
  <div class="daoManage">
    <daoHeaderInfo @joinDao="joinDao" :daoInfo="curDao"></daoHeaderInfo>
    <div class="dao-manage-content">
      <div class="dao-manage-content-box">
        <div class="box-nav">
          <div class="item" :class="{'active':activeNavIndex==0}" @click="activeNavIndex=0">
            HOME
          </div>
          <div class="item" :class="{'active':activeNavIndex==1}" @click="activeNavIndex=1">
            PROPOSAL
          </div>
          <div class="item" :class="{'active':activeNavIndex==2}" @click="activeNavIndex=2">
            FINANCIAL
          </div>
          <div class="item" :class="{'active':activeNavIndex==3}" @click="activeNavIndex=3">
            MEMBER
          </div>
          <div class="item" :class="{'active':activeNavIndex==4}" @click="activeNavIndex=4">
            APPLY LIST
          </div>
        </div>
        <div class="home">

        </div>
        <daoHome @chooseDao="chooseDao" :dao-list="daoList" v-show="activeNavIndex==0"></daoHome>
        <proposalList :address="curDao.manage" :vault="curDao.vault" :proposal-list="proposalArr" v-show="activeNavIndex==1"></proposalList>
        <daoFinance :token-list="tokenList" v-show="activeNavIndex==2"></daoFinance>
        <daoMember :members-list="membersList" v-show="activeNavIndex==3"></daoMember>
        <applyList @getData="getApplyList" :cur-dao="curDao" :list="applyArr" v-show="activeNavIndex==4"></applyList>
      </div>
    </div>

  </div>
</template>

<script>
import daoMember from "./daoManage/daoMember";
import proposalList from "./daoManage/proposalList";
import daoHome from "./daoManage/daoHome";
import applyList from "./daoManage/applyList";
import {mapGetters} from "vuex";
import daoFinance from "./daoManage/daoFinance"

export default {
  name: "daoManage",
  components: {
    daoMember,
    daoHome,
    applyList,
    proposalList,
    daoFinance
  },
  data() {
    return {
      curDao: {},
      membersLength: 0,
      activeNavIndex: 0,
      daoList: [],
      tokenList: [],
      applyArr: [],
      membersList: [],
      proposalArr: []
    }
  },

  computed: {
    ...mapGetters([
      'isConnected',
      'account'
    ]),
  },
  watch: {
    membersLength() {
    },
    isConnected() {
      this.getData()
    }
  },
  created() {
    if (this.$route.params) {
      this.curDao = this.$route.params
    }
    this.getData()
  },
  methods: {
    getApplyList() {
      this.$store.dispatch("daoManage/getApplyingLength", this.curDao.manage).then(length => {
        for (let i = 0; i < length; i++) {
          this.$store.dispatch("daoManage/getApplyingByIndex", {address: this.curDao.manage, index: i}).then(res => {
            this.applyArr.push(res)
          })
        }
      })
    },
    joinDao() {
      this.$store.dispatch("daoManage/applyJoin", {
        address: this.curDao.manage,
        manage: this.curDao.manage,
      }).then(() => {
        this.getData()
        this.getApplyList()
      })
    },
    chooseDao(item) {
      this.curDao = item
      this.getMembers()
      this.getVaultData()
      this.getApplyList()
      this.getProposalList()
    },
    getMembers() {
      this.membersList = []
      this.$store.dispatch("daoManage/getMemberLength", this.curDao.manage).then(memLen => {
        for (let i = 0; i < memLen; i++) {
          this.$store.dispatch("daoManage/getMemberByIndex", {address: this.curDao.manage, index: i}).then(res => {
            this.membersList.push(res)
          })

        }
      })

    },
    getVaultData() {
      this.$store.dispatch("vault/getTokenNumber", this.curDao.vault).then(length => {
        console.log(length)
        this.tokenList = []
        for (let i = 0; i < length; i++) {
          this.$store.dispatch("vault/getTokenAddr", {address: this.curDao.vault, index: i}).then(token => {
            this.$store.dispatch("vault/getERC20Balance", {address: this.curDao.manage, token: token}).then(balance => {
              this.tokenList.push({
                token,
                balance
              })
            })
          })
        }
      })
    },
    getProposalList(){
      this.$store.dispatch("daoManage/getProposalLength", this.curDao.manage).then(length => {
        this.proposalArr = []
        for (let i = 0; i < length; i++) {
          this.$store.dispatch("daoManage/proposalArray", {address: this.curDao.manage, index: i}).then(res => {
            this.proposalArr.push(res)
          })
        }
      })
    },
    getData() {
      if(!this.isConnected){
        return
      }
      if (this.curDao.manage) {
        this.getApplyList()
        this.getMembers()
        this.getVaultData()
        this.getProposalList()
      }

      this.$store.dispatch("daoFactory/getArrayLength").then(length => {
        this.daoList = []
        for (let i = 0; i < length; i++) {
          this.$store.dispatch("daoFactory/getDaoInfo", i).then(res => {
            this.$store.dispatch("daoManage/getMemberLength", res.manage).then(memLen => {
              this.daoList.push({
                ...res,
                membersLength: memLen
              })
            })
          })
        }
      })

    }
  }
}
</script>

<style lang="scss" scoped>
.daoManage {

  .dao-manage-content {

    .dao-manage-content-box {
      box-shadow: 0px 6px 20px 0px rgba(0, 0, 0, 0.05);

      border-radius: 20px;
      width: 1200px;
      margin: 10px auto;
      background: #fff;
      padding-bottom: 20px;

      .box-nav {
        display: flex;
        padding: 0 20px;
        border-bottom: 1px solid #eee;

        .item {
          user-select: none;
          width: 150px;
          height: 60px;
          line-height: 60px;
          text-align: center;
          cursor: pointer;

          &.active {
            color: #F96AAF;
            border-bottom: 1px solid #F96AAF;
          }
        }
      }
    }
  }

}
</style>
