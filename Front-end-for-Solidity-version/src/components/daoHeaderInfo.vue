<template>
  <div class="dao-finance-header-box">
    <div class="nodata" v-show="!daoInfo.name">
      Please choose a DAO
      <div class="rainbow-button" @click="$router.push({name:'createDao'})">
        Create DAO
      </div>
    </div>
    <div class="dao-finance-header" v-show="daoInfo.name">
      <div class="dao-logo">
        <img :src="daoInfo.logo" alt="logo">
      </div>
      <div class="dao-info">
        <div class="dao-name">
          {{ daoInfo.name }}
        </div>
        <div class="dao-info-detail">
          <!--        <div class="dao-link">-->
          <!--          https://Rainbowdao.io/-->
          <!--        </div>-->
          <div class="dao-member">
            {{ daoInfo.membersLength }} Members
          </div>
        </div>
        <div class="dao-intro">
          <span>Introduction:</span>
          {{ daoInfo.des }}
        </div>
      </div>
        <div class="rainbow-button create-proposal" @click="$router.push({name:'createProposal',params:{address:daoInfo.manage,vault:daoInfo.vault}})">
          Create Proposal
        </div>
      <div class="rainbow-button create" @click="$router.push({name:'createDao'})">
        Create DAO
      </div>
      <div class="rainbow-button" @click="joinDao">
        JOIN
      </div>
    </div>
  </div>
</template>

<script>
export default {
  name: "daoHeaderInfo",
  props:["daoInfo"],
  methods:{
    joinDao(){
      this.$emit("joinDao")
    }
  }
}
</script>

<style lang="scss" scoped>
.dao-finance-header-box{
  box-shadow: 0px 6px 20px 0px rgba(0,0,0,0.05);
  background: #ffffff;
  border-radius: 20px;

}
.nodata{

}
.dao-finance-header {
  position: relative;

  display: flex;
  align-items: center;
  z-index: 1;
  width: 1200px;
  height: 180px;
  padding: 40px;
  margin: 0px auto 20px;
  .dao-logo{
    width: 60px;
    height: 60px;
    img{
      width: 100%;
    }
  }
  .dao-info{
    .dao-name{
      font-size: 24px;
      font-weight: 700;
      color: #333333;
      line-height: 28px;
    }
    .dao-info-detail{
      display: flex;
      margin: 10px 0;
      div{
        height: 30px;
        background: #f4dcf2;
        border: 1px solid rgba(255,31,132,0.30);
        border-radius: 5px;
        padding: 0 20px;
        line-height: 30px;
        color: #f96aaf;
      }
      .dao-member{
        margin-left: 10px;
      }
    }
    .dao-intro{
      span{
        color: #999;
      }
    }
  }

  .rainbow-button{
    position: absolute;
    right: 40px;
    width: 120px;
    height: 50px;

  }
  .create{
    right: 180px;
  }
  .create-proposal{
    right: 320px;
  }
}
</style>
