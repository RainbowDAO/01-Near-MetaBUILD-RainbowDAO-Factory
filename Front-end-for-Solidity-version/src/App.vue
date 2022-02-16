<template>
  <div id="app">
    <daoNav></daoNav>
      <router-view class="content-box"/>
    <daoFooter></daoFooter>
  </div>
</template>

<script>

import daoFooter from "./components/daoFooter";
import daoNav from "./components/daoNav";

export default {
  name: 'App',
  components: {
    daoNav,
    daoFooter
  }
}
</script>

<style lang="scss">
#app {
  overflow: auto;
  background: url("./assets/daoImgs/background.png");
  background-size: 100% 100%;
  .content-box {
    width: 1200px;
    min-height: calc(100vh - 530px);
    border-radius: 20px;
    margin: -100px auto 30px;
    position: relative;
    z-index: 1;
  }
  .nodata{
    text-align: center;
    padding: 50px;
    font-weight: bold;
    font-size: 26px;
    .rainbow-button{
      margin: 10px auto;
      width: 200px;
      height: 50px;
    }
  }
  ::v-deep .rainbow-dialog-box{
    position: fixed;
    width: 100vw;
    height: 100vh;
    top: 0;
    z-index: 101;
    display: flex;
    justify-content: center;
    align-items: center;
    .mask{
      width: 100%;
      height: 100%;
      background: rgba(0,0,0,0.50);
    }
    .rainbow-dialog{
      position: absolute;
      z-index: 102;
      background: #fff;
      border-radius: 20px;
      padding: 40px;
    }
  }
}
.rainbow-dialog-box{
  position: fixed;
  width: 100vw;
  height: 100vh;
  top: 0;
  left: 0;
  z-index: 101;
  display: flex;
  justify-content: center;
  align-items: center;
  .mask{
    width: 100%;
    height: 100%;
    background: rgba(0,0,0,0.50);
  }
  .rainbow-dialog{
    position: absolute;
    z-index: 102;
    background: #fff;
    border-radius: 20px;
    padding: 40px;
  }
}
</style>
