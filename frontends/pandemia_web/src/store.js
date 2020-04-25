import Vue from 'vue'
import Vuex from 'vuex'

Vue.use(Vuex)

const user = {
  namespaced: true,
  state: {
    currentUser: null
  },
  mutations: {
    SET_CURRENT_USER (state, user) {
      state.currentUser = user
    },
    REMOVE_CURRENT_USER (state) {
      state.currentUser = null
    }
  },
  actions: {
    setCurrentUserByAPI({commit}) {
      this._vm.$pandemia.getMeInfo()
      .then(resp => {
        if (!(resp.status != 200 || (resp.data.status == "error" && resp.data.code != 0))) {
          commit('SET_CURRENT_USER', resp.data)
        } else {
          if (this._vm.$session.get('token')) {
            this._vm.$pandemia.unauthorize()
            window.location.reload()
          }
        }
      })
      .catch(_err => {
        // console.log(err)
      })
    },
    removeCurrentUser({commit}) {
      commit('REMOVE_CURRENT_USER')
    },
    setCurrentUser({commit}, user) {
      commit('SET_CURRENT_USER', user)
    },
  }
}

export default new Vuex.Store({
  modules: {
    user
  },
  state: {

  },
  mutations: {

  },
  actions: {

  }
})

