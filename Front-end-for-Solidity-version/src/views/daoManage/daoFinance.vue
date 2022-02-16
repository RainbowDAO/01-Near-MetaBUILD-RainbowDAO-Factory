<template>
<div class="daoFinance">
  <div class="token-list">
    <div class="item" v-for="(item,index) in tokenList" :key="index">
      <div class="name">
        Token Address:{{item.token}}
      </div>
      <div class="balance">
        Balance:{{item.balance}}
      </div>
    </div>
  </div>
</div>
</template>

<script>
export default {
name: "daoFinance",
  props:["tokenList"]
}
</script>

<style lang="scss" scoped>
.daoFinance{
  padding: 30px;
  .token-list{
    .item{
      display: flex;
      padding: 20px;
      .balance{
        padding: 0 20px;

      }
    }
  }
}
</style>
