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
              <td class="value">{{d.full_name}}</td>
            </tr>
            <tr>
              <td data-label="Email">Email:</td>
              <td class="value">{{d.email != null && d.email.startsWith("gen__") ? "-" : d.email }}</td>
            </tr>
            <tr>
              <td data-label="Phone">Phone:</td>
              <td class="value">{{d.phone_num}}</td>
            </tr>
            <tr>
              <td data-label="Active">Active:</td>
              <td class="value">{{d.active ? "YES" : "NO"}}</td>
            </tr>
            <tr>
              <td data-label="Active">Blocked:</td>
              <td class="value">{{d.blocked ? "YES" : "NO"}}</td>
            </tr>
            <tr>
              <td data-label="Active">Roles:</td>
              <td class="value">{{d.roles}}</td>
            </tr>
            <tr>
              <td data-label="Active">Lokasi:</td>
              <td class="value">{{d.village}}</td>
            </tr>
            <tr v-if="$session.get('user_id') == 1">
              <td  data-label="Active">Metadata:</td>
              <td class="value">{{d.meta}}</td>
            </tr>

          </tbody>
        </table>
      </div>
    </div>

    <div style="height: 10px;"></div>

    <!-- <div>
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
    </div> -->

    <ConfirmDialog
      modalName="Block"
      caption="Konfirmasi"
      approveText="Blokir"
      :withCloseButton="true"
      @onApprove="doBlock(d)"
    >
      <p>Yakin untuk memblokir akun ini?</p>
    </ConfirmDialog>

    <button v-if="!d.blocked" class="ui text icon button left floated" @click="blockSatgas">
      <i class="fa-ban icon"></i> Block
    </button>

    <ConfirmDialog
      modalName="Unblock"
      caption="Konfirmasi"
      approveText="Blokir"
      :withCloseButton="true"
      @onApprove="doUnblock(d)"
    >
      <p>Yakin untuk membuka akun ini?</p>
    </ConfirmDialog>

    <button v-if="d.blocked" class="ui text icon button left floated" @click="unblockSatgas">
      <i class="fa-ban icon"></i> Unblock
    </button>

    <ConfirmDialog v-if="$session.get('user_id') == 1"
      modalName="Delete"
      caption="Konfirmasi"
      approveText="Hapus"
      :withCloseButton="true"
      @onApprove="doDelete(d)"
    >
      <p>Yakin untuk menghapus akun ini?
      Operasi ini tidak bisa dikembalikan!</p>
    </ConfirmDialog>

    <button v-if="$session.get('user_id') == 1" class="ui text icon button left floated" @click="deleteSatgas">
      <i class="fa-trash icon"></i> Hapus
    </button>
  </div>
</template>

<script>
import DialogModal from "@/components/modal/DialogModal.vue";
import ConfirmDialog from "@/components/modal/ConfirmDialog.vue";

export default {
  name: "UserDetail",
  components: {
    DialogModal,
    ConfirmDialog
  },
  props: {
    userId: String,
    baseApiUrl: String
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
      .publicApi.get(`${this.baseApiUrl}?id=${this.userId}`)
      .then(resp => {
        console.log(resp);
        this.d = resp.data.result;
      });
  },
  methods: {
    deleteSatgas(){
      this.$modal.show("Delete");
    },
    doDelete(d){
      this.$pandemia
        .api()
        .publicApi.post(
          `/user/v1/satgas/delete`,
          {
            "id": d.id
          }
        )
        .then(resp => {
          if (resp.data.code == 0) {
            this.showSuccess(`Satgas dengan nama ${d.full_name} telah dihapus`);
            this.$modal.hide("Delete");
          }else{
            this.showError(resp.data.description);
          }
        }).catch(err => this.showError("Gagal menghubungi server, periksa kembali koneksi Anda"));
    },
    blockSatgas(){
      this.$modal.show("Block");
    },
    doBlock(d){
      this.$pandemia
        .api()
        .publicApi.post(
          `/user/v1/satgas/block`,
          {
            "id": d.id
          }
        )
        .then(resp => {
          if (resp.data.code == 0) {
            this.showSuccess(`Satgas dengan nama ${d.full_name} telah diblokir`);
            this.$modal.hide("Block");
          }else{
            this.showError(resp.data.description);
          }
        }).catch(err => this.showError("Gagal menghubungi server, periksa kembali koneksi Anda"));
    },
    unblockSatgas(){
      this.$modal.show("Unblock");
    },
    doUnblock(d){
      this.$pandemia
        .api()
        .publicApi.post(
          `/user/v1/satgas/unblock`,
          {
            "id": d.id
          }
        )
        .then(resp => {
          if (resp.data.code == 0) {
            this.showSuccess(`Satgas dengan nama ${d.full_name} telah dibuka`);
            this.$modal.hide("Unblock");
          }else{
            this.showError(resp.data.description);
          }
        }).catch(err => this.showError("Gagal menghubungi server, periksa kembali koneksi Anda"));
    },
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
        .publicApi.post("/user/v1/update_password", {
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

