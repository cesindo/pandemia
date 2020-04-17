import Vue from 'vue'
import Router from 'vue-router'
import Home from './views/Home.vue'
import Dashboard from './views/Dashboard.vue'
// import SatgasLogin from './views/Satgas.vue'
import Analytic from './views/Analytic.vue'
// import ReportNotes from './views/ReportNotes.vue'
import SatgasPanel from './views/SatgasPanel.vue'
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
      path: '/dashboard/admins',
      name: 'Administrators',
      component: Dashboard,
      meta: {
        title: 'Dashboard' + titleDesc,
      },
    },
    {
      path: '/dashboard/admins/:id',
      name: 'Administrator Detail',
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
      path: '/dashboard/village-data',
      name: 'Data Desa',
      component: Dashboard,
      meta: {
        title: 'Dashboard' + titleDesc,
      },
    },
    {
      path: '/dashboard/satgas',
      name: 'Data Satgas COVID-19',
      component: Dashboard,
      meta: {
        title: 'Dashboard' + titleDesc,
      },
    },
    {
      path: '/dashboard/satgas/:id',
      name: 'Satgas',
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
      path: '/dashboard/hospital',
      name: 'Hospital',
      component: Dashboard,
      meta: {
        title: 'Dashboard' + titleDesc,
      },
    },
    {
      path: '/dashboard/map',
      name: 'Map',
      component: Dashboard,
      meta: {
        title: 'Dashboard' + titleDesc,
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
      path: '/dashboard/data',
      name: 'Data Perorang',
      component: Dashboard,
      meta: {
        title: 'Data'
      },
    },
    {
      path: '/dashboard/reports',
      name: 'Laporan',
      component: Dashboard,
      meta: {
        title: 'Desa'
      },
    },
    {
      path: '/area/:province/:city',
      name: 'Analitik Daerah Terdampak COVID-19',
      component: Analytic,
      props: true,
      meta: {
        title: 'Analitik Daerah'
      },
    },
    {
      path: '/satgas',
      name: 'Satgas Panel',
      component: SatgasPanel,
      meta: {
        title: 'Desa'
      },
    },
    {
      path: '/satgas/data',
      name: 'Data ODP/PDP',
      component: SatgasPanel,
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

