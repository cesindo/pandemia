import Vue from 'vue'
import VueSession from 'vue-session';
import VueSidebarMenu from "vue-sidebar-menu";
import 'vue-sidebar-menu/dist/vue-sidebar-menu.css'
import Notifications from 'vue-notification'

import App from './App.vue'
import router from './router'
import store from './store'
import Pandemia from './plugins/pandemia';
import notifMixin from "./mixins/notifMixin";

// ----- Vuejs Dialog Stuff -------------
import VuejsDialog from "vuejs-dialog"
// import VuejsDialogMixin from "vuejs-dialog/vuejs-dialog-mixin.min.js"

// include the default style
import 'vuejs-dialog/dist/vuejs-dialog.min.css'

// Tell Vue to install the plugin.
Vue.use(VuejsDialog)
// ------- end of Vuejs Dialog Stuff ---------

import VTooltip from 'v-tooltip'
Vue.use(VTooltip)

import vmodal from 'vue-js-modal'
Vue.use(vmodal, { dynamic: true, injectModalsContainer: true })

import './registerServiceWorker'

Vue.config.productionTip = false

// Configure your base api endpoint for production here:
Vue.config.prodApiEndpoint = "http://pandemia.cesindo.top";

// Run mode ini menerima nilai:
// * `prod` - Apabila ingin menggunakan API dari server production.
// * `dev` - Apabila ingin menggunakan API dari server local atau docker (untuk development).
// * `mock` - Apabila ingin menggunakan API dari server mocking Apiary (untuk development).

if (!process.env.VUE_APP_RUN_MODE) {
  throw new Error('cannot find .env file or "VUE_APP_RUN_MODE" not set in .env file')
}
Vue.config.runMode = process.env.VUE_APP_RUN_MODE;

Vue.use(VueSession)
Vue.use(Notifications)
// Vue.use(Notif)
Vue.use(Pandemia)
Vue.use(VueSidebarMenu)


// Add utils option in components
Vue.mixin({
  beforeCreate() {
    // mixin utils dan notif ke semua components
    const utils = Object.assign({}, this.$options.utils, notifMixin);
    if (utils) {
      const keys = Object.keys(utils)
      for (let i = 0; i < keys.length; i++) {
        this[keys[i]] = utils[keys[i]]
      }
    }
  }
})

new Vue({
  router,
  store,
  render: h => h(App)
}).$mount('#app')

