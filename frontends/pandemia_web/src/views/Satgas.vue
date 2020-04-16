<template>
  <div>
    <div id="Main">
      <AnsTable
        :key="tableUsers"
        data-source-url="/user/v1/satgas/search"
        :columns="['ID', 'Nama', 'Email', 'Telp', 'Waktu Daftar', 'Posisi', 'Aktif', 'Diblokir', 'Desa']"
        :searchable="true"
        :withActionButton="true"
        :mapItemFunc="itemMapper"
        :showDetailFunc="showDetail"
      />

      <DialogModal
        modalName="EditValueModal"
        caption="Edit Value"
        :withCloseButton="true"
        @beforeOpen="beforeOpenDialog"
        @onApprove="approveDialog"
        @opened="editValueDialogOpened"
        :buttonsText="{reject: 'Cancel', approve: 'Ok'}"
      >
        <template v-slot:content>
          <h2 class="ui header">Edit jumlah {{editedCatName}} di {{editedItem['loc']}}</h2>

          <div>
            <p>
              Jumlah saat ini:
              <br />
              <strong>{{editedItem[editedCat]}}</strong>
            </p>

            <p>Jumlah baru:</p>
            <div class="ui input">
              <input ref="newValue" type="text" name="NewValue" id="NewValue" />
            </div>
          </div>
        </template>
      </DialogModal>

      <DialogModal
        modalName="AddRecordModal"
        caption="Tambah Rekod"
        :withCloseButton="true"
        @onApprove="doAddRecord"
        @opened="onAddRecordOpened"
        :buttonsText="{reject: 'Cancel', approve: 'Ok'}"
      >
        <template v-slot:content>
          <h2 class="ui header">Tambah rekod baru</h2>

          <div style="text-align: left;">
            <div class="ui form">
              <div class="field">
                <label>Nama Lokasi:</label>
                <input ref="addRecLocInput" type="text" name="Loc" id="Loc" />
              </div>
              <div class="field">
                <label>Jenis:</label>
                <select ref="addRecLocKind" class="ui fluid dropdown" name="Kind" id="Kind">
                  <option value="4">Kota/Kabupaten</option>
                  <option value="3">Provinsi</option>
                  <option value="2">Negara</option>
                  <option value="1">Benua</option>
                  <option value="0">Global</option>
                  <option value="5">Unknown</option>
                </select>
              </div>
              <div class="field">
                <label>Area (scope):</label>
                <input ref="addRecLocScopeInput" type="text" name="LocScope" id="LocScope" />
                <small>bisa nama negara di mana lokasi ini berada, contoh: Indonesia</small>
              </div>
              <div class="ui grid">
                <div class="three wide column">
                  <div class="field">
                    <label>ODP:</label>
                    <input ref="addRecOdp" type="text" name="Odp" id="Odp" />
                  </div>
                </div>
                <div class="three wide column">
                  <div class="field">
                    <label>PDP:</label>
                    <input ref="addRecPdp" type="text" name="Pdp" id="Pdp" />
                  </div>
                </div>
                <div class="three wide column">
                  <div class="field">
                    <label>Positive:</label>
                    <input ref="addRecPositive" type="text" name="Positive" id="Positive" />
                  </div>
                </div>
                <div class="three wide column">
                  <div class="field">
                    <label>Sembuh:</label>
                    <input ref="addRecTotalRecovered" type="text" name="Recovered" id="Recovered" />
                  </div>
                </div>
                <div class="three wide column">
                  <div class="field">
                    <label>Meninggal:</label>
                    <input ref="addRecTotalDeaths" type="text" name="Deaths" id="Deaths" />
                  </div>
                </div>
              </div>
            </div>
          </div>
        </template>
      </DialogModal>

      <ConfirmDialog
        modalName="Commit"
        caption="Commit data"
        approveText="Lanjut"
        :withCloseButton="true"
        @onApprove="doCommit"
      >
        <p>Yakin untuk melakukan commit? Semua perubahan akan di-simpan ke server, pastikan koneksi internet Anda lancar.</p>
      </ConfirmDialog>

      <ConfirmDialog
        modalName="Delete"
        caption="Confirmation"
        approveText="Hapus"
        :withCloseButton="true"
        @onApprove="doDelete"
      >
        <p>Yakin untuk menghapus record {{toDelete['id']}} "{{toDelete['loc']}}"?</p>
      </ConfirmDialog>
    </div>
  </div>
</template>


<script>
import AnsTable from "@/components/AnsTable.vue";
import DialogModal from "@/components/modal/DialogModal.vue";
import ConfirmDialog from "@/components/modal/ConfirmDialog.vue";
import { obMapToArrayValues } from "@/utils/utils";

export default {
  name: "Satgas",
  components: {
    AnsTable,
    DialogModal,
    ConfirmDialog
  },
  data() {
    return {
      editedItem: { loc: "" },
      editedCatName: "",
      editedCat: "",
      commitLogs: {},
      isDirty: false,
      tableRecords: "-0",
      toDelete: { id: 0, loc: "" }
    };
  },
  computed: {
    loggedIn() {
      return this.$session.get("user_id") != null;
    }
  },
  methods: {
    itemMapper(item) {
      return {
        id: item["id"],
        full_name: item["full_name"],
        email: item["email"].startsWith("gen__") ? "-" : item["email"],
        telp: item["phone_num"],
        register_time: item["register_time"],
        roles: item["roles"].join(", "),
        active: item["active"] ? "Ya" : "Tidak",
        blocked: item["blocked"] ? "Ya" : "Tidak",
        village: item["village"]
      };
    },
    addRecord() {
      this.$modal.show("AddRecordModal");
    },
    doAddRecord() {
      var loc = this.$refs["addRecLocInput"].value,
        locKind = parseInt(this.$refs["addRecLocKind"].value),
        locScope = this.$refs["addRecLocScopeInput"].value,
        // Odp = parseInt(this.$refs['addRecOdp'].value),
        // Pdp = parseInt(this.$refs['addRecPdp'].value),
        totalCases = parseInt(this.$refs["addRecPositive"].value) || 0,
        totalDeaths = parseInt(this.$refs["addRecTotalDeaths"].value) || 0,
        totalRecovered =
          parseInt(this.$refs["addRecTotalRecovered"].value) || 0;
      this.$pandemia
        .api()
        .publicApi.post("/pandemia/v1/add_record", {
          loc: loc,
          loc_scope: locScope,
          loc_kind: locKind,
          total_cases: totalCases,
          total_deaths: totalDeaths,
          total_recovered: totalRecovered,
          active_cases: 0,
          critical_cases: 0
        })
        .then(resp => {
          if (resp.data.code == 0) {
            this.$modal.hide("AddRecordModal");

            this.showSuccess("Rekod berhasil ditambahkan");
            this.refreshTable();
          } else {
            var suggest = "";
            if (resp.data.description.indexOf("Invalid") > -1) {
              suggest = "Mohon periksa kembali data masukan Anda";
            }
            this.showError("Gagal menambahkan rekod. " + suggest);
          }
        });
    },
    onAddRecordOpened() {
      this.$refs["addRecLocInput"].focus();
    },
    showDetail(item) {
      this.$router.push("/dashboard/satgas/" + item.id);
    },
    editValue(self, catName, cat) {
      this.editedItem = self.item;
      this.editedCatName = catName;
      this.editedCat = cat;
      this.$modal.show("EditValueModal", { item: self.item });
    },
    beforeOpenDialog(_) {
      // console.log(this.editedItem);
      // console.log(this.$refs.editValueDialogPlaceholder);
      // this.$refs.editValueDialogPlaceholder.html(`<template v-slot:content>
      //   <h2 class="ui header">Edit jumlah positive di ${event.params.item["loc"]}</h2>
      //   <p>
      //     nilai lama:
      //     <br />
      //     <strong>{{item['total_cases']}}</strong>
      //   </p>
      //   <p>nilai baru:</p>
      //   <div class="ui input">
      //     <input
      //       :ref="'newValueInput' + item['id']"
      //       type="text"
      //       :id="'NewValueTotalCases-' + item['id']"
      //       autofocus="autofocus"
      //     />
      //   </div>
      // </template>`);
    },
    editValueDialogOpened(_) {
      this.$refs["newValue"].focus();
    },
    approveDialog() {
      var commitLog = this.commitLogs[this.editedItem["id"]];

      if (commitLog == undefined) {
        commitLog = Object.assign({}, this.editedItem);
        this.isDirty = true;
      }

      commitLog[this.editedCat] = parseInt(this.$refs.newValue.value);

      this.$set(this.commitLogs, this.editedItem["id"], commitLog);

      this.isDirty = true;

      this.$modal.hide("EditValueModal");
    },
    commit() {
      this.$modal.show("Commit");
    },
    doCommit() {
      this.$modal.hide("Commit");
      var normCommitLogs = obMapToArrayValues(this.commitLogs);
      this.$pandemia
        .api()
        .publicApi.post("/pandemia/v1/update_records", {
          records: normCommitLogs
        })
        .then(resp => {
          // console.log(resp);
          if (resp.data.code == 0) {
            this.isDirty = false;
            this.commitLogs = {};
            this.refreshTable();
            this.showSuccess("Data telah sukses diunggah ke server pusat");
          } else {
            this.showError(resp.data.description);
          }
        });
    },
    confirmDelete(item) {
      this.toDelete = item;
      this.$modal.show("Delete");
    },
    doDelete() {
      this.$modal.hide("Delete");
      this.$pandemia
        .api()
        .publicApi.post("/pandemia/v1/delete_record", {
          id: this.toDelete["id"]
        })
        .then(resp => {
          // console.log(resp);
          if (resp.data.code == 0) {
            this.refreshTable();
            this.showSuccess("Record telah berhasil dihapus");
          } else {
            this.showError(
              "Gagal menghapus record, hubungi sistem administrator"
            );
          }
        });
    },
    refreshTable() {
      this.tableRecords = "A-" + new Date().getTime();
    },
    onMenuClick(_event, item) {
      console.log(_event);
      if (item.title == "Logout") {
        this.$dialog
          .confirm("Yakin untuk logout?")
          .then(_dialog => {
            this.$pandemia.unauthorize();
            window.reload();
          })
          .catch(() => {});
      }
    }
  }
};
</script>

<style lang="less">
td.dirty {
  color: orange;
}
</style>