<template>
  <div>
    <SatgasLogin v-if="$session.get('user_id') == null" />

    <div id="Main">
      <AnsTable
        key="tableDistricts"
        data-source-url="/district/v1/search"
        :columns="['ID', 'Nama', 'City', 'Province', 'Action']"
        :searchable="true"
        :withActionButton="false"
        :mapItemFunc="mapItemFunc"
        :showDetailFunc="showDetail"
        :limit="100"
      >
        <template v-slot:bar>
          <!-- <button v-if="isDirty" class="ui text icon green button right floated" @click="commit">
            <i class="fa-angle-double-up icon"></i> Commit
          </button>-->
          <button class="ui text icon button right floated" @click="addDistrict">
            <i class="fa-plus icon"></i> Tambah
          </button>
        </template>
      </AnsTable>

      <DialogModal
        modalName="AddDistrictModal"
        caption="Tambah Rekod"
        :withCloseButton="true"
        @onApprove="doAddDistrict"
        @opened="onAddDistrictOpened"
        :buttonsText="{reject: 'Cancel', approve: 'Ok'}"
      >
        <template v-slot:content>
          <h2 class="ui header">Tambah Kecamatan</h2>

          <div style="text-align: left;">
            <div class="ui form">
              <div class="field">
                <label>Nama Kecamatan:</label>
                <input
                  ref="addDistrictName"
                  type="text"
                  name="DistrictName"
                  id="DistrictName"
                  v-model="inputDistrictName"
                />
              </div>
              <div class="field">
                <label>Kota/Kabupaten:</label>
                <input ref="addCityInput" type="text" name="City" id="City" v-model="inputCityName" />
              </div>
              <div class="field">
                <label>Provinsi:</label>
                <input
                  ref="addProvinceInput"
                  type="text"
                  name="Province"
                  id="Province"
                  v-model="inputProvinceName"
                />
              </div>
              <div class="field">
                <div class="ui checkbox">
                  <input
                    type="checkbox"
                    name="CollectorOnly"
                    id="CollectorOnly"
                    v-model="inputCollectorOnly"
                  />
                  <label>Hanya Sebagai Kolektor (bukan kecamatan sebenarnya)</label>
                </div>
              </div>

              <div class="ui grid">
                <div class="six wide column">
                  <div class="field">
                    <label>Latitude:</label>
                    <input :disabled="inputCollectorOnly"
                      ref="addLatInput"
                      type="text"
                      name="Lat"
                      id="Lat"
                      v-model="inputLatitude"
                    />
                  </div>
                </div>
                <div class="six wide column">
                  <div class="field">
                    <label>Longitude:</label>
                    <input :disabled="inputCollectorOnly"
                      ref="addLonInput"
                      type="text"
                      name="Lon"
                      id="Lon"
                      v-model="inputLongitude"
                    />
                  </div>
                </div>
              </div>
              
            </div>
          </div>
        </template>
      </DialogModal>

      <ConfirmDialog
        modalName="Delete"
        caption="Confirmation"
        approveText="Hapus"
        :withCloseButton="true"
        @onApprove="doDelete"
      >
        <p>
          Yakin untuk menghapus kab/kota
          <strong>{{toDelete['name']}}</strong> ?
        </p>
      </ConfirmDialog>
    </div>
  </div>
</template>


<script>
import AnsTable from "@/components/AnsTable.vue";
import DialogModal from "@/components/modal/DialogModal.vue";
import ConfirmDialog from "@/components/modal/ConfirmDialog.vue";
import SatgasLogin from "@/components/SatgasLogin.vue";

export default {
  name: "Districts",
  components: {
    AnsTable,
    DialogModal,
    ConfirmDialog,
    SatgasLogin
  },
  data() {
    return {
      editedItem: { loc: "" },
      editedCatName: "",
      editedCat: "",
      commitLogs: {},
      isDirty: false,
      tableCitys: "-0",
      toDelete: { id: 0, loc: "" },
      inputDistrictName: "",
      inputCityName: "",
      inputProvinceName: "",
      inputLatitude: null,
      inputLongitude: null,
      inputCollectorOnly: false
    };
  },
  methods: {
    mapItemFunc(d) {
      return {
        id: d["id"],
        name: d["name"],
        city: d["city_name"],
        province: d["province_name"]
      };
    },
    addDistrict() {
      this.$modal.show("AddDistrictModal");
    },
    doAddDistrict() {
      var name = this.inputDistrictName,
        city = this.inputCityName,
        province = this.inputProvinceName,
        latitude = this.inputLatitude,
        longitude = this.inputLongitude;

      var payload = {
        name: name,
        city_name: city,
        province: province,
        collector_only: this.inputCollectorOnly
      };

      if (!this.inputCollectorOnly) {
        payload["latitude"] = latitude;
        payload["longitude"] = longitude;
      }

      this.$pandemia
        .api()
        .publicApi.post("/district/v1/add", payload)
        .then(resp => {
          if (resp.data.code == 0) {
            this.$modal.hide("AddDistrictModal");

            this.showSuccess("Data berhasil ditambahkan");
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
    onAddDistrictOpened() {
      this.$refs["addDistrictName"].focus();
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
    beforeOpenDialog(_) {},
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
      this.toDelete = item;
      this.$modal.show("Delete");
    },
    doDelete() {
      this.$modal.hide("Delete");
      this.$pandemia
        .api()
        .publicApi.post("/city/v1/delete", {
          id: this.toDelete["id"]
        })
        .then(resp => {
          // console.log(resp);
          if (resp.data.code == 0) {
            this.refreshTable();
            this.showSuccess("Kab/Kota telah berhasil dihapus");
          } else {
            this.showError(
              "Gagal menghapus kab/kota, hubungi sistem administrator"
            );
          }
        });
    },
    refreshTable() {
      this.tableDistricts = "A-" + new Date().getTime();
    }
  }
};
</script>

<style lang="less">
td.dirty {
  color: orange;
}
</style>