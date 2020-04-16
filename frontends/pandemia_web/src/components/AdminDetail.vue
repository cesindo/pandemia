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
            <tr>
              <td data-label="Active">Accesses:</td>
              <td class="value">{{d.accesses}}</td>
            </tr>
            <tr>
              <td data-label="Active">Meta:</td>
              <td class="value">{{d.meta}}</td>
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

      <DialogModal
        modalName="EditAccesses"
        caption="Edit Accesses"
        :withCloseButton="true"
        @onApprove="onEditAccessApproved"
        @opened="onEditAccessOpened"
        :buttonsText="{reject: 'Cancel', approve: 'Ok'}"
      >
        <template v-slot:content>
          <h2 class="ui header">Edit Accesses</h2>

          <div style="text-align: left;">
            <div class="ui form">
              <div class="field">
                <label>Accesses:</label>
                <textarea
                  ref="accessInput"
                  name="AccessInput"
                  id="AccessInput"
                  cols="30"
                  rows="3"
                  v-model="accesses"
                ></textarea>
                <small>Input separated by comma. eg: users,report_notes</small>
              </div>
            </div>
          </div>
        </template>
      </DialogModal>
      <button class="ui text icon button left floated" @click="editAccesses">
        <i class="fa-map-signs icon"></i> Edit akses
      </button>

      <DialogModal
        modalName="EditMeta"
        caption="Edit Meta"
        :withCloseButton="true"
        @onApprove="onEditMetaApproved"
        @opened="onEditMetaOpened"
        :buttonsText="{reject: 'Cancel', approve: 'Ok'}"
      >
        <template v-slot:content>
          <h2 class="ui header">Edit Meta</h2>

          <div style="text-align: left;">
            <div class="ui form">
              <div class="field">
                <label>Meta:</label>
                <textarea
                  ref="metaInput"
                  name="MetaInput"
                  id="MetaInput"
                  cols="30"
                  rows="7"
                  v-model="metax"
                ></textarea>
                <small>Input separated by newline.</small>
              </div>
            </div>
          </div>
        </template>
      </DialogModal>
      <button class="ui text icon button left floated" @click="editMeta">
        <i class="fa-align-right icon"></i> Edit Meta
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
    userId: String,
    baseApiUrl: String
  },
  data() {
    return {
      d: {},
      accesses: "",
      metax: ""
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
        this.accesses = this.d.accesses.join(", ");
        this.metax = this.d.meta.join('\n');
      });
  },
  methods: {
    editMeta() {
      this.$modal.show("EditMeta");
    },
    onEditMetaApproved() {
      var meta = this.$refs["metaInput"].value
        .split("\n")
        .map(a => a.trim());
      this.$pandemia
        .api()
        .publicApi.post(`/admin/v1/update_meta`, {
          id: this.d.id,
          meta: meta
        })
        .then(resp => {
          if (resp.data.code == 0) {
            this.showSuccess("Meta data berhasil diupdate");
            var d2 = this.d;
            d2.meta = meta;
            this.d = d2;
            this.meta = meta.join(", ");
            this.$modal.hide("EditMeta");
          }
        });
    },
    onEditMetaOpened() {
      this.$refs["metaInput"].focus();
    },
    editAccesses() {
      this.$modal.show("EditAccesses");
    },
    onEditAccessApproved() {
      var accesses = this.$refs["accessInput"].value
        .split(",")
        .map(a => a.trim());
      this.$pandemia
        .api()
        .publicApi.post(`/admin/v1/update_accesses`, {
          id: this.d.id,
          accesses: accesses
        })
        .then(resp => {
          if (resp.data.code == 0) {
            this.showSuccess("Akses berhasil diupdate");
            var d2 = this.d;
            d2.accesses = accesses;
            this.d = d2;
            this.accesses = accesses.join(", ");
            this.$modal.hide("EditAccesses");
          }
        });
    },
    onEditAccessOpened() {
      this.$refs["accessInput"].focus();
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

