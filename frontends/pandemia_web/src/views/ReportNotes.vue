<template>
  <div>
    <div id="Main">
      
      <AnsTable
        :key="tableReportNotes"
        data-source-url="/pandemia/v1/report_note/search"
        add-params="state=all"
        :columns="['ID', 'Waktu', 'Catatan', 'Pelapor', 'Lokasi', 'Status', 'Action']"
        :searchable="true"
        :withActionButton="false"
        :showDetailFunc="showDetail"
        :limit="100"
      >
        <!-- <template v-slot:bar>
          <button v-if="isDirty" class="ui text icon green button right floated" @click="commit">
            <i class="fa-angle-double-up icon"></i> Commit
          </button>
          <button class="ui text icon button right floated" @click="addVillage">
            <i class="fa-plus icon"></i> Tambah
          </button>
        </template> -->

        <template v-slot:tdmap="self">
          <td>{{self.item['id']}}</td>
          <td>{{self.item['ts']}}</td>
          <td>{{self.item['notes']}}</td>
          <td>{{self.item['creator_name']}}</td>
          <td>{{self.item['location']}}</td>
          <td>{{self.item['status'].join(", ") }}</td>
          <td style="width: 120px;">
            <button v-if="self.item['status'] != 'approved'" class="ui icon button" title="Approve" @click="approve(self.item)">
              <i class="check icon"></i>
            </button>
            <button class="ui icon button" title="Hapus" @click="confirmDelete(self.item)">
              <i class="trash icon"></i>
            </button>
          </td>
        </template>
      </AnsTable>

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
<!-- 
      <DialogModal
        modalName="AddReportModal"
        caption="Tulis Laporan"
        :withCloseButton="true"
        @onApprove="doAddVillage"
        @opened="onAddVillageOpened"
        :buttonsText="{reject: 'Cancel', approve: 'Ok'}"
      >
        <template v-slot:content>
          <h2 class="ui header">Laporan</h2>

          <div style="text-align: left;">
            <div class="ui form">
              <div class="field">
                <label>Nama desa:</label>
                <input ref="addNameInput" type="text" name="Name" id="Name" autofocus />
              </div>
              <div class="field">
                <label>Kecamatan:</label>
                <input ref="addSubDistrictInput" type="text" name="SubDistrict" id="SubDistrict" />
              </div>
              <div class="field">
                <label>Kota/Kabupaten:</label>
                <input ref="addCityInput" type="text" name="City" id="City" />
              </div>
              <div class="field">
                <label>Provinsi:</label>
                <input ref="addProvinceInput" type="text" name="Province" id="Province" />
              </div>
              <div class="ui grid">
                <div class="six wide column">
                  <div class="field">
                    <label>Latitude:</label>
                    <input ref="addLatInput" type="text" name="Lat" id="Lat" />
                  </div>
                </div>
                <div class="six wide column">
                  <div class="field">
                    <label>Longitude:</label>
                    <input ref="addLonInput" type="text" name="Lon" id="Lon" />
                  </div>
                </div>
              </div>
            </div>
          </div>
        </template>
      </DialogModal> -->

      <ConfirmDialog
        modalName="Delete"
        caption="Konfirmasi"
        approveText="Hapus"
        :withCloseButton="true"
        @onApprove="doDelete"
      >
        <p>Yakin untuk menghapus data id {{toProcess['id']}} ?</p>
      </ConfirmDialog>

      <ConfirmDialog
        modalName="Approve"
        caption="Konfirmasi"
        approveText="Approve"
        :withCloseButton="true"
        @onApprove="doApprove"
      >
        <p>Yakin untuk meng-approve laporan ini? :</p>
        <strong>"{{toProcess['notes']}}"</strong>

        <p>dari <strong>{{toProcess['creator_name']}}</strong> - <strong>{{toProcess['location']}}</strong></p>
      </ConfirmDialog>

    </div>
  </div>
</template>


<script>
import AnsTable from "@/components/AnsTable.vue";
import DialogModal from "@/components/modal/DialogModal.vue";
import ConfirmDialog from "@/components/modal/ConfirmDialog.vue";

export default {
  name: "Satgas",
  components: {
    AnsTable,
    DialogModal,
    ConfirmDialog,
  },
  data() {
    return {
      editedItem: { loc: "" },
      editedCatName: "",
      editedCat: "",
      commitLogs: {},
      isDirty: false,
      tableReportNotes: "-0",
      toProcess: { id: 0, loc: "" }
    };
  },
  methods: {
    approve(item){
      this.toProcess = item;
      this.$modal.show('Approve');
    },
    doApprove(){
      
      this.$pandemia
        .api()
        .publicApi.post("/pandemia/v1/report_note/update_state", {
          'id': this.toProcess["id"],
          'state': ['approved']
        })
        .then(resp => {
          // console.log(resp);
          if (resp.data.code == 0) {
            this.refreshTable();
            this.$modal.hide("Approve");
            this.showSuccess("Laporan telah berhasil di-aprove");
          } else {
            this.showError(
              "Gagal meng-approve laporan, hubungi sistem administrator"
            );
          }
        });
    },
    onAddVillageOpened() {
      this.$refs["addRecLocInput"].focus();
    },
    showDetail(item) {
      this.$router.push("/dashboard/villages/" + item.id);
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
    confirmDelete(item) {
      this.toProcess = item;
      this.$modal.show("Delete");
    },
    doDelete() {
      this.$modal.hide("Delete");
      this.$pandemia
        .api()
        .publicApi.post("/pandemia/v1/report_note/delete", {
          id: this.toProcess["id"]
        })
        .then(resp => {
          // console.log(resp);
          if (resp.data.code == 0) {
            this.refreshTable();
            this.showSuccess("Data telah berhasil dihapus");
          } else {
            this.showError(
              "Gagal menghapus data, hubungi sistem administrator"
            );
          }
        });
    },
    refreshTable() {
      this.tableReportNotes = "A-" + new Date().getTime();
    }
  }
};
</script>

<style lang="less">
td.dirty {
  color: orange;
}
</style>