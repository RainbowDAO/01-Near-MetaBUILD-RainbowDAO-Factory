<template>
  <div class="mine-dao">
    <div class="dao-list">
      <div class="item" @click="chooseDao(item)" v-for="(item,index) in daoList" :key="index">
        <div class="logo">
          <img :src="item.logo" alt="">
        </div>
        <div class="dao-info">
          <div class="name">
            {{ item.name }}
          </div>
          <div class="members">
            Number of DAOs: {{ item.membersLength }}
          </div>
        </div>
      </div>
    </div>
    <div class="nodata" v-show="daoList.length==0">
      No Data
    </div>
  </div>
</template>

<script>
import {mapGetters} from "vuex";

export default {
  name: "mineDao",
  data() {
    return {
      daoList: []
    }
  },
  watch: {
    isConnected() {
      this.getData()
    }
  },
  created() {
    this.getData()
  },
  computed:{
    ...mapGetters([
      'isConnected',
      'account'
    ]),
  },
  methods: {
    chooseDao(item) {
      this.$router.push({name:"daoManage",params:{
          ...item,
          isOwner:true
        }})
    },
    getData() {
      console.log(this.isConnected)
      if (this.isConnected) {
        this.$store.dispatch("daoFactory/getOwnedDaos").then(myDaos=>{
          if(myDaos.length>0){
            for (let i=0;i<myDaos.length;i++){
              this.$store.dispatch("daoFactory/getDaoInfo", myDaos[i]).then(res => {
                console.log(res)
                this.$store.dispatch("daoManage/getMemberLength", res.manage).then(memLen => {
                  this.membersLength = memLen
                  this.daoList.push({
                    ...res,
                    membersLength: memLen
                  })
                })
              })
            }
          }
        })

      }
    }
  }
}
</script>

<style lang="scss" scoped>
.mine-dao{
  padding: 20px;
  background: #fff;
}
.dao-list {
  display: flex;
  flex-wrap: wrap;

  .item {
    margin: 20px;
    display: flex;
    width: 252px;
    height: 80px;
    background: #f5f5f5;
    border-radius: 10px;
    padding: 10px;
    align-items: center;
    cursor: pointer;

    .dao-info {
      padding-left: 10px;

      .name {
        font-weight: bold;
      }

      .members {
        color: #666;
      }
    }

    .logo {
      width: 50px;
      height: 50px;

      img {
        width: 100%;
      }
    }
  }
}
</style>
