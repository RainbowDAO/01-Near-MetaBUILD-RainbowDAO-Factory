<template>
  <div class="rainbow-nav" >
    <!-- navbar-nav -->
    <div class="nav-box">
      <img class="logo" src="../assets/imgs/RainbowDAOLogo.png" @click="$router.push('/')"/>
      <div class="tip" style="color: #fa4949">
        Test version
      </div>
      <ul class="navbar-nav">
        <li :class="{'active':$route.name=='daoManage'}" @click="$router.push({name:'daoManage'})">
          <a class="w-font" >Home</a>
          <a class="re-active" >Home</a>
        </li>
        <li :class="{'active':$route.name=='mineDao'}" @click="$router.push({name:'mineDao'})">
          <a class="w-font" >MineDao</a>
          <a class="re-active" >MineDao</a>
        </li>
        <li>
          <ConnectWallet class="connect"></ConnectWallet>
        </li>
      </ul><!-- /.navbar-nav -->
    </div>
  </div>
</template>

<script>
import ConnectWallet from "./ConnectWallet";
export default {
  name: "daoNav",
  props: ['isShowMore'],
  components:{
    ConnectWallet
  },
  data() {
    return {

    }
  },
  created(){
  },
  methods: {
    openNew(pathStr) {
      // let path = this.$router.resolve({path: pathStr, query: {}})
      // window.open(path.href)
      this.$router.push({path:pathStr})
    }
  }
}
</script>

<style lang="scss" scoped>

.rainbow-nav {
  background: url("../assets/daoImgs/header_bg.png");
  height: 230px;
  background-size: 100% 100%;
  color: white;
  position: relative;

  .nav-box {
    display: flex;
    margin: 0 auto;
    width: 100%;
    min-width: 1180px;
    padding: 0 60px;
    align-items: center;
    justify-content: space-between;
    background: rgba(#fff, 0.1);
    border-bottom: 1px solid rgba(255, 255, 255, 0.1);

    .logo {
      cursor: pointer;
      width: 120px;
      height: 16px;
    }

    .language {
      cursor: pointer;
      width: 80px;
      height: 30px;
      margin-top: 12px;
      text-align: center;
      line-height: 30px;
      background: rgba(255, 255, 255, 0.10);
      border: 1px solid rgba(255, 255, 255, 0.20);
      border-radius: 16px;
    }

  }

  .navbar-nav {
    display: flex;
    height: 50px;
    align-items: center;
    position: relative;
    z-index: 10;

    li {
      height: 100%;
      line-height: 50px;
      position: relative;
      overflow: hidden;
      display: flex;
      align-items: center;
      font-weight: bold;
      &:hover {
        .re-active {
          top: 0;
        }
        .w-font{
          top: -40px;
        }
      }

    }

    .masking {
      position: fixed;
      top: 0;
      left: 0;
      width: 100vw;
      height: 100vh;
      z-index: 1;
    }

    .more-btn {
      position: relative;
      cursor: pointer;
      overflow: visible!important;
      .down {
        float: right;
        margin: 3px 0 0 3px;
      }
    }

    .w-font{
      position: relative;
      top: 0;
      left: 0;
      transition: 0.2s;
    }
    .re-active{
      position: absolute;
      top: 40px;

      left: 0;
      transition: 0.2s;
      background-image: linear-gradient(90deg, #12c2e9 0%, #f3daff 64%, #fda0a0 100%);
      -webkit-background-clip: text;
      -webkit-text-fill-color: transparent;
    }
    .active {
      &:hover{
        .w-font{
          top: 0;
        }
        .re-active{
          top: 40px;
        }
      }
      .w-font{
        position: relative;
        background-image: linear-gradient(90deg, #12c2e9 0%, #bfffe9 64%, #fdd0d0 100%);
        -webkit-background-clip: text;
        -webkit-text-fill-color: transparent;
      }
      &:after {
        content: '';
        position: absolute;
        width: 100%;
        height: 1px;
        bottom: 0;
        left: 0;
        background: linear-gradient(90deg, #12c2e9 0%, #c471ed 64%, #f64f59 100%);
      }
    }

    li {
      margin: 0 10px;
    }
  }
}
</style>
