<template>
  <div class="home">
    <div class="login">
      <div class="ui center aligned grid">
        <div class="seven wide column left aligned">
          <div class="ui raised very padded container segment">
            <center>
              <img alt="Pandemia logo" src="../assets/logo.png" style="width: 200px;" />

              <h1>Login Satgas COVID-19</h1>

              <p>{{ desc }}</p>
            </center>
            <div class="ui divider"></div>
            <form class="ui form" method="POST" @submit="doLogin($event)">
              <div class="field">
                <label>No telp (HP):</label>
                <input type="text" name="telp" placeholder="No telp (HP), contoh: 08123123123" ref="inputTelp" />
              </div>
              <div class="field">
                <label>Password:</label>
                <input type="password" name="password" placeholder="Password" ref="inputPassword" />
              </div>
              <div class="field">
                <div class="ui checkbox">
                  <input type="checkbox" tabindex="0" class="hidden" />
                  <label>Remember me</label>
                </div>
              </div>
              <button class="ui button" type="submit">Masuk</button>
            </form>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  name: "Login",
  props: {
    title: String,
    desc: String
  },
  data() {
    return {
      token: this.token
    };
  },
  methods: {
    doLogin: function(event) {
      var self = this;
      if (event) event.preventDefault();
      this.$pandemia
        .login(
          this.$refs.inputTelp.value,
          null,
          this.$refs.inputPassword.value
        )
        .then(resp => {
          if (resp.data.code == 0) {
            this.$pandemia.getMeInfo().then(self._handleGetMeInfo);
          } else if (resp.data.code == 3000) {
            showLoginError();
          } else {
            showLoginError(resp.data.description);
          }
        })
        .catch(_e => {
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

