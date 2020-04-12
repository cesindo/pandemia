import { ApiClient, Crypto } from "../../../../libs/pandemia-client-js";


export default class Pandemia {
  static install(Vue) {

    var api;

    const env = process.env

    if (Vue.config.runMode == "prod") {
      api = new ApiClient(env.VUE_APP_PROD_PUBLIC_URL,
        env.VUE_APP_PROD_PRIVATE_URL);
    } else if (Vue.config.runMode == "dev") {
      api = new ApiClient(env.VUE_APP_DEV_PUBLIC_URL,
        env.VUE_APP_DEV_PRIVATE_URL);
    } else if (Vue.config.runMode == "mock") {
      api = new ApiClient(env.VUE_APP_MOCK_URL,
        env.VUE_APP_MOCK_URL);
    } else {
      throw "Unknown mode: " + Vue.config.runMode
    }

    updateSession();

    // var $session = Vue.prototype.$session;
    function session() {
      return Vue.prototype.$session;
    }

    function updateSession() {
      var token = session().get("token");
      api.publicApi.defaults.headers["X-Access-Token"] = token;
      api.privateApi.defaults.headers["X-Access-Token"] = token;
    }

    Vue.prototype.$pandemia = {
      crypto() {
        return Crypto;
      },
      login(email, phone, password) {

        var emailOrPhone = email ? email : phone;
        var data = {
          "email": emailOrPhone,
          "phone": phone,
          "password": password
        }

        return api.publicApi.post("/auth/v1/authorize", data)
          .then((resp) => {
            if (resp.data.code == 0) {
              session().set("token", resp.data.result.token);
              updateSession(resp.data.result.token);
              this.loadUserKey();
            }
            return resp;
          });
      },
      unauthorize() {
        console.log("unauthorize");
        session().remove("token");
        updateSession();
        return api.publicApi.post("/auth/v1/unauthorize", {});
      },
      adminLogin(email, phone, password) {

        var emailOrPhone = email ? email : phone;
        var data = {
          "email": emailOrPhone,
          "password": password
        }

        return api.publicApi.post("/auth/v1/admin/authorize", data)
          .then((resp) => {
            if (resp.data.code == 0) {
              var access_token = resp.data.result.access_token,
                user = resp.data.result.user;
              session().set("token", access_token.token);
              session().set("user_id", user.id);
              session().set("user_name", user.name);
              session().set("user_email", user.email);
              updateSession(resp.data.result.token);
              this.loadUserKey();
            }
            return resp;
          });
      },
      adminUnauthorize() {
        console.log("unauthorize");
        session().remove("token");
        updateSession();
        return api.publicApi.post("/auth/v1/admin/unauthorize", {});
      },
      isLoggedIn(cb) {
        this.getAdminMeInfo().then((resp) => {
          if (resp.status != 200 || (resp.data.status == "error" && resp.data.code != 0)) {
            cb(false)
          } else {
            cb(true)
          }
        }).catch((_e) => cb(false))
      },
      getMeInfo() {
        return api.publicApi.get("/user/v1/me/info");
      },
      getAdminMeInfo() {
        return api.publicApi.get("/admin/v1/me/info");
      },

      // Fetch current user key-pair.
      loadUserKey() {
        return api.publicApi.get("/auth/v1/get_key")
          .then((resp) => {
            console.log("user key loaded.");
            session().set("pk", resp.data.result.pub_key);
            session().set("sk", resp.data.result.secret_key);
          }).catch(_e => {
            // this.$notify({
            //   group: "default",
            //   type: "error",
            //   title: "Error",
            //   text: "Cannot load keys"
            // });
          });
      },

      // Mendapatkan current user key pair dari local session.
      getKeys() {
        var pk = session().get("pk");
        var sk = session().get("sk");
        return {
          pubKey: Buffer.from(pk, 'hex'),
          secretKey: Buffer.from(sk, 'hex'),
        }
      },
      // creditUserBalance(userId, amount) {
      //   var credit = new protos.user.Credit({
      //     user: userId,
      //     amount: parseFloat(amount),
      //     timestamp: this.now(),
      //     seed: this.generateSeed()
      //   });

      //   var buffer = protos.user.Credit.encode(credit).finish();
      //   let keys = this.getKeys();
      //   var signature = Crypto.sign(buffer, keys.pubKey, keys.secretKey);

      //   var data = {
      //     body: protos.user.Credit.toObject(credit),
      //     signature: signature
      //   };
      //   return api.privateApi.post("/user/v1/credit", data);
      // },
      generateSeed() {
        return Math.floor(Math.random() * 1000000000);
      },
      now() {
        return new Date().getTime();
      },
      api() {
        return api;
      }
    }
  }
  static version = ""
}



