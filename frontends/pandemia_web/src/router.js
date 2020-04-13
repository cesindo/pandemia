import Vue from 'vue'
import Router from 'vue-router'
import Home from './views/Home.vue'
import Dashboard from './views/Dashboard.vue'
import SatgasLogin from './views/Satgas.vue'
import NotFound from './views/NotFound.vue'

Vue.use(Router)

const defaultTitle = "Pandemia"
const titleDesc = ` | ${defaultTitle}`

let router = new Router({
  mode: 'history',
  base: process.env.BASE_URL,
  routes: [
    {
      path: '/',
      name: 'home',
      component: Home,
      meta: {
        title: 'Home' + titleDesc,
      },
    },
    {
      path: '/dashboard',
      name: 'Dashboard',
      component: Dashboard,
      meta: {
        title: 'Dashboard' + titleDesc,
      },
    },
    {
      path: '/dashboard/users',
      name: 'User',
      component: Dashboard,
      meta: {
        title: 'Dashboard' + titleDesc,
      },
    },
    {
      path: '/dashboard/users/:id',
      name: 'User Detail',
      component: Dashboard,
      meta: {
        title: 'Dashboard' + titleDesc,
      },
    },
    {
      path: '/dashboard/records',
      name: 'Records',
      component: Dashboard,
      meta: {
        title: 'Dashboard' + titleDesc,
      },
    },
    {
      path: '/dashboard/satgas',
      name: 'Satgas COVID-19',
      component: Dashboard,
      meta: {
        title: 'Dashboard' + titleDesc,
      },
    },
    {
      path: '/dashboard/journal',
      name: 'Journal',
      component: Dashboard,
      meta: {
        title: 'Dashboard' + titleDesc,
      },
    },
    {
      path: '/satgas',
      name: 'Satgas',
      component: SatgasLogin,
      meta: {
        title: 'Login Satgas COVID-19'
      },
    },
    {
      path: '/dashboard/villages',
      name: 'Data Desa',
      component: Dashboard,
      meta: {
        title: 'Desa'
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
  scrollBehavior () {
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

