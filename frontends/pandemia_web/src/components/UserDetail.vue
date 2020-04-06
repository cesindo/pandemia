<template>
  <div id="UserDetail">
    <div class="ui grid">
      <div class="six wide column">
        <table class="ui celled table">
          <tbody>
            <tr>
              <td data-label="ID">ID:</td>
              <td class="value">{{d.id}}</td>
            </tr>
            <tr>
              <td data-label="Name">Name:</td>
              <td class="value">{{d.name}}</td>
            </tr>
            <tr>
              <td data-label="Email">Email:</td>
              <td class="value">{{d.email}}</td>
            </tr>
            <tr>
              <td data-label="Phone">Phone:</td>
              <td class="value">{{d.phone_num}}</td>
            </tr>
            <tr>
              <td data-label="Active">Active:</td>
              <td class="value">{{d.active ? "YES" : "NO"}}</td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>

    <div style="height: 10px;"></div>

    <div>
      <DialogModal
        modalName="EditPasswordModal"
        caption="Rubah Kata Kunci"
        :withCloseButton="true"
        @onApprove="onEditPasswordOk"
        :buttonsText="{reject: 'Cancel', approve: 'Ok'}"
      >
        <template v-slot:content>
          <h2 class="ui header">Rubah Kata Kunci</h2>

          <div style="text-align: left;">
            <div class="ui form">
              <div class="field">
                <label>Kata kunci baru:</label>
                <input ref="passInput" type="password" name="NewPassInput" id="NewPassInput" />
              </div>
              <div class="field">
                <label>Konfirmasi kata kunci:</label>
                <input
                  ref="confPassInput"
                  type="password"
                  name="ConfNewPassInput"
                  id="ConfNewPassInput"
                />
              </div>
            </div>
          </div>
        </template>
      </DialogModal>
      <button class="ui text icon button left floated" @click="changePassword">
        <i class="fa-key icon"></i> Rubah kata kunci
      </button>
    </div>
  </div>
</template>

<script>
import DialogModal from "@/components/modal/DialogModal.vue";

export default {
  name: "UserDetail",
  components: {
    DialogModal
  },
  props: {
    userId: String
  },
  data() {
    return {
      d: {}
    };
  },
  created() {
    if (!this.userId) return;
    this.$pandemia
      .api()
      .publicApi.get(`/admin/v1/detail?id=${this.userId}`)
      .then(resp => {
        console.log(resp);
        this.d = resp.data.result;
      });
  },
  methods: {
    changePassword() {
      this.$modal.show("EditPasswordModal");
    },
    onEditPasswordOk() {
      var newPass = this.$refs["passInput"].value,
      confNewPass = this.$refs["confPassInput"].value;

      if (newPass != confNewPass) {
        this.showError(
          "Kata kunci konfirmasi tidak sama, mohon periksa kembali"
        );
        return;
      }
      this.$pandemia
        .api()
        .publicApi.post("/admin/v1/update_password", {
          id: parseInt(this.userId),
          password: newPass,
          password_confm: confNewPass
        })
        .then(resp => {
          if (resp.data.code == 0) {
            this.showSuccess("Update success");
            this.$modal.hide("EditPasswordModal");
          } else {
            this.showError("Cannot update password " + resp.data.description);
          }
        });
    }
  }
};
</script>


<style lang="less" scoped>
.value {
  font-weight: bold;
}
tr td:first-child {
  text-align: right !important;
}
</style>

