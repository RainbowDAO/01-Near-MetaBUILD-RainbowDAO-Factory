import Vue from 'vue'
import App from './App.vue'
import store from './store'
import router from './router'
import "./style/index.scss"
import daoHeaderInfo from "./components/daoHeaderInfo";
Vue.config.productionTip = false
import 'animate.css';
Vue.component('daoHeaderInfo', daoHeaderInfo)
new Vue({
  router,
  store,
  render: h => h(App)
}).$mount('#app')
