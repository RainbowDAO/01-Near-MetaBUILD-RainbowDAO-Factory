<template>
  <div class="apply-list-box">
    <div class="apply-list">
      <div class="item" v-for="(item, index) in list" :key="index">
        <div class="address">
          {{ item }}
        </div>
        <div class="agree rainbow-button" @click="agree(item)">
          Agree
        </div>
      </div>
    </div>
    <div class="nodata" v-show="list.length==0">
      No Data
    </div>
  </div>
</template>

<script>
export default {
  name: "applyList",
  props: ["list","curDao"],
  methods:{
    agree(address){
      console.log(this.curDao)
      this.$store.dispatch("daoManage/approveApply",{address:this.curDao.manage,addr:address}).then(()=>{
        this.$emit("getData")
      })
    }
  }
}
</script>

<style lang="scss" scoped>
.apply-list {
  .item {
    display: flex;
    padding-left: 20px;
    margin: 10px;
    align-items: center;
    .address {

    }

    .agree {
      margin: 0 10px;
      width:100px;
      height: 30px;
    }
  }
}
</style>
