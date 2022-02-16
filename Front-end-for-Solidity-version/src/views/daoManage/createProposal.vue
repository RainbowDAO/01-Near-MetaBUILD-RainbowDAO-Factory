<template>
  <div class="createProposal">
    <div class="rainbow-panel">
      <h2 class="title">
        Create Proposal
      </h2>
      <div class="content">
        <div class="proposal-form">
          <div class="item">
            <div class="item-title">
              Name
            </div>
            <input type="text" v-model="form.title">
          </div>

          <div class="item">
            <div class="item-title">
              voteDays
            </div>
            <input type="number" v-model="form.voteDays">
          </div>
          <div class="item">
            <div class="item-title">
              token
            </div>
            <input type="text" v-model="form.token">
          </div>
          <div class="item">
            <div class="item-title">
              to
            </div>
            <input type="text" v-model="form.to">
          </div>
          <div class="item">
            <div class="item-title">
              amount
            </div>
            <input type="number" v-model="form.amount">
          </div>
          <div class="item">
            <div class="item-title">
              minAmount
            </div>
            <input type="number" v-model="form.minAmount">
          </div>

          <div class="des">
            <h4 class="item-title">
              Description
            </h4>
            <textarea v-model="form.description" rows="10" cols="60"></textarea>
          </div>
          <div class="btn-box">
            <div class="sub-btn" @click="createProposal">
              Create
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import {mapGetters} from "vuex";

export default {
  name: "createProposal",
  data() {
    return {
      address: "",
      form: {
        title: "", description: "", endTime: "", voteTime: "", minAmount: "", token: "", to: "", amount: ""
      },
      vault:"",
      decimal:10
    }
  },
  computed: {
    ...mapGetters(['account', 'isConnected'])
  },
  created() {
    if (!this.$route.params.address) {
      this.$router.push({name: "daoManage"})
    }
    this.address = this.$route.params.address
    this.vault = this.$route.params.vault
    this.$store.dispatch("vault/getTokenAddr",{address:this.vault,index:0}).then(res=>{
      this.token = res
      this.$store.dispatch("erc20/decimals",res).then(decimal=>{
        this.decimal = decimal
      })
    })
  },
  methods: {
    createProposal() {
      this.form.amount = this.form.amount * (10**this.decimal)
      this.form.voteTime = parseInt(this.form.voteDays) * 86400
      this.form.endTime = (new Date()).getTime() + this.form.voteTime
      this.$store.dispatch("daoManage/newProposal", {
        address: this.address,
        ...this.form
      }).then(() => {
        this.$router.push({name: "mineDao"})
      })
    }
  }
}
</script>

<style lang="scss" scoped>
.createProposal {
  background: #fff;
  padding: 30px;

  .title {
    font-weight: bold;
    text-align: center;
  }

  .proposal-form {
    display: flex;
    flex-wrap: wrap;

    .item {
      width: 33.3%;
      text-align: center;
      padding: 10px 0;

      .item-title {
        line-height: 30px;
        font-weight: bold;
        font-size: 16px;
        color: #999;
      }

      input {
        padding: 0 10px;
        width: 260px;
        height: 32px;
        background: #ffffff;
        border: 1px solid #eaeaea;
        border-radius: 10px;
      }
    }

    .des {
      .item-title {
        line-height: 30px;
      }

      margin: 10px auto;
    }

    .btn-box {
      width: 100%;
      display: flex;
      justify-content: center;

      .sub-btn {
        width: 120px;
        text-align: center;
        line-height: 40px;
        font-size: 18px;
        font-weight: bold;
        height: 40px;
        cursor: pointer;
        background: linear-gradient(90deg, #12c2e9 0%, #c471ed 64%, #f64f59 100%);
        border: 1px solid #eaeaea;
        border-radius: 10px;
        color: white;
      }
    }

  }
}
</style>
