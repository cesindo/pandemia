<template>
  <div>
    <div class="ui grid right floated">
      <div class="ten wide column">
        <button class="ui text icon button right floated" @click="addUser">
          <i class="fa-plus icon"></i> Tambah
        </button>
      </div>
    </div>
    <AnsTable
      :key="tableUsers"
      data-source-url="/admin/v1/list"
      :columns="['ID', 'Name', 'Email', 'Phone', 'Active', 'Register']"
      :searchable="true"
      :withActionButton="true"
      :mapItemFunc="userListAllMapper"
      :showDetailFunc="showDetail"
    />

    <DialogModal
      modalName="AddUserModal"
      caption="Tambah Pengguna"
      :withCloseButton="true"
      @onApprove="onApprove"
      @opened="onOpened"
      :buttonsText="{reject: 'Cancel', approve: 'Ok'}"
    >
      <template v-slot:content>
        <h2 class="ui header">Tambah Pengguna</h2>

        <div style="text-align: left;">
          <div class="ui form">
            <div class="field">
              <label>Name:</label>
              <input ref="nameInput" type="text" name="NameInput" id="NameInput" />
            </div>
            <div class="field">
              <label>Email:</label>
              <input ref="emailInput" type="text" name="EmailInput" id="EmailInput" />
            </div>
            <div class="field">
              <label>Phone:</label>
              <input ref="phoneInput" type="text" name="PhoneInput" id="PhoneInput" />
            </div>
            <div class="field">
              <label>Password:</label>
              <input ref="passInput" type="password" name="PassInput" id="PassInput" />
            </div>
            <div class="field">
              <label>Konfirmasi Password:</label>
              <input ref="confPassInput" type="password" name="ConfPassInput" id="ConfPassInput" />
            </div>

          </div>
        </div>
      </template>
    </DialogModal>
  </div>
</template>

<script>
import AnsTable from "@/components/AnsTable.vue";
import DialogModal from "@/components/modal/DialogModal.vue";
// import ConfirmDialog from "@/components/modal/ConfirmDialog.vue";

export default {
  name: "Users",
  components: {
    AnsTable,
    DialogModal,
    // ConfirmDialog
  },
  data() {
    return {
      tableUsers: "-0"
    };
  },
  methods: {
    addUser() {
      this.$modal.show("AddUserModal");
    },
    onApprove() {
      var name = this.$refs['nameInput'].value,
        email = this.$refs['emailInput'].value,
        phone = this.$refs['phoneInput'].value,
        password =this.$refs['passInput'].value,
        confPassword = this.$refs['confPassInput'].value;

      this.$pandemia
        .api()
        .publicApi.post("/admin/v1/add", {
          name: name,
          email: email,
          phone_num: phone,
          password: password,
          confirm_password: confPassword
        })
        .then(resp => {
          if (resp.data.code == 0) {
            this.$modal.hide("AddUserModal");

            this.showSuccess("User berhasil ditambahkan");
            this.refreshTable();
          } else {
            this.showError("Gagal menambahkan user. " + resp.data.description);
          }
        });
    },
    onOpened() {},
        showDetail(item) {
      this.$router.push("/dashboard/users/" + item.id);
    },
    txItemMap(item) {
      return item;
    },
    userListAllMapper(item) {
      return {
        "id": item["id"],
        "name": item["name"],
        "email": item["email"],
        "phone": item["phone_num"],
        "active": item["active"],
        "register_time": item["register_time"],
      };
    },
    userListAllMapper2(item) {
      return {
        id: item["id"],
        name: item["full_name"],
        email: item["email"]
      };
    },
    refreshTable() {
      this.tableUsers = "A-" + new Date().getTime();
    }
  }
};
</script>