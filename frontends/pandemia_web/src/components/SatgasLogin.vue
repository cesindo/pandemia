<template>
  <div class="home">
    <div class="login">
      <div class="ui stackable center aligned grid">
        <div class="seven wide column center aligned">
          <div class="ui raised very padded container segment">
            <center>
              <img alt="Pandemia logo" src="../assets/logo.png" style="width: 200px;" />

              <h1>Login Satgas COVID-19</h1>
            </center>
            <div class="ui divider"></div>
            <form class="ui form" method="POST" @submit="doLogin($event)">
              <div class="field">
                <label>TOKEN:</label>
                <input
                  :disabled="isLoading"
                  type="text"
                  name="token"
                  placeholder="Kode Token"
                  ref="inputToken"
                  v-uppercase
                />
                <p style="margin-top: 10px;">Kode token bisa didapatkan dari Aplikasi Pandemia</p>
              </div>
              <center>
                <button
                  :class="isLoading ? 'ui loading large green button' : 'ui icon large green button' "
                  type="submit"
                  :disabled="isLoading"
                >
                  <i class="icon fa-key"></i>
                  Masuk
                </button>
              </center>
            </form>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import ErrorCode from "@/pandemia/ErrorCode.js";

export default {
  name: "Login",
  props: {
    title: String,
    desc: String
  },
  data() {
    return {
      token: this.token,
      isLoading: false
    };
  },
  methods: {
    doLogin: function(event) {
      var self = this;
      if (event) event.preventDefault();
      this.isLoading = true;
      this.$pandemia
        .api()
        .publicApi.post("/auth/v1/satgas/authorize", {
          token: this.$refs.inputToken.value
        })
        .then(resp => {
          this.isLoading = false;
          if (resp.data.code == 0) {
            this.$pandemia.authorize(resp.data.result);
            this.$pandemia.getMeInfo().then(self._handleGetMeInfo);
          } else if (resp.data.code == 3000) {
            showLoginError();
          } else if (resp.data.code == ErrorCode.DatabaseRecordNotFoundError) {
            this.showWarning("Token tidak valid");
          } else {
            showLoginError(resp.data.description);
          }
        })
        .catch(_e => {
          this.isLoading = false;
          showLoginError();
        });
      function showLoginError(desc) {
        self.$notify({
          group: "auth",
          title: "Login",
          type: "warn",
          text: desc ? desc : "No telp atau password salah."
        });
      }
    },
    _handleGetMeInfo(_resp) {
      this.$router.push("/dashboard");
      // location.reload();
    }
  }
};
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped lang="less">
h3 {
  margin: 40px 0 0;
}
ul {
  list-style-type: none;
  padding: 0;
}
li {
  display: inline-block;
  margin: 0 10px;
}
a {
  color: #42b983;
}
</style>

