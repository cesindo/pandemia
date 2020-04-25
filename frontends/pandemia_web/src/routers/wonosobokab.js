import Vue from 'vue'
import Router from 'vue-router'
import Analytic from '@/views/Analytic.vue'
import NotFound from '@/views/NotFound.vue'

Vue.use(Router)

const defaultTitle = "Pandemia"
const titleDesc = ` | ${defaultTitle}`

let router = new Router({
    mode: 'history',
    base: process.env.BASE_URL,
    routes: [
        {
            path: '/',
            name: 'Analitik Daerah Terdampak COVID-19 Wonosobo',
            component: Analytic,
            props: {
                province: 'jawa-tengah',
                city: 'wonosobo'
            },
            meta: {
                title: 'Analitik Daerah',
            },
        },
        {
            path: '/desa',
            name: 'Analitik Daerah Terdampak COVID-19 Wonosobo',
            component: Analytic,
            props: {
                province: 'jawa-tengah',
                city: 'wonosobo'
            },
            meta: {
                title: 'COVID19 Daerah Wonosobo data per Desa',
            },
        },
        {
            path: '*',
            name: '404',
            component: NotFound,
            meta: {
                title: 'Oops! Not Found' + titleDesc,
            }
        }
    ],
    scrollBehavior() {
        return { x: 0, y: 0 }
    }
})

router.beforeEach((to, _from, next) => {
    to.matched.forEach(record => {
        document.title = record.meta.title || defaultTitle
    });

    next()
})

export default router

