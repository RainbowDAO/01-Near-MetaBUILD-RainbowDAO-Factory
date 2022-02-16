import Vue from 'vue'
import VueRouter from 'vue-router'


Vue.use(VueRouter)

const routes = [
    {
        path: "/404",
        name: "notFound",
        component: () => import('@/views/errorPage/404'),
    },
    {
        path: '/',
        redirect: "/daoManage"
    },
    {
        path: '/createDao',
        name:"createDao",
        component: () => import('@/views/createDao'),
    },
    {
        path: '/daoManage',
        name:"daoManage",
        component: () => import('@/views/daoManage'),
    },
    {
        path: '/createProposal',
        name:"createProposal",
        component: () => import('@/views/daoManage/createProposal'),
    },
    {
        path: '/proposalDetail',
        name:"proposalDetail",
        component: () => import('@/views/daoManage/proposalDetail'),
    },
    {
        path: '/mineDao',
        name:"mineDao",
        component: () => import('@/views/mineDao'),
    },{
        path: "*",
        redirect: "/404"
    }
]
const router = new VueRouter({
    routes
})

export default router
