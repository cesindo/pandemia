<template>
  <div>
    <AnsTable
      :key="tableRecords"
      data-source-url="/village/v1/village_data/search"
      :columns="['ID', 'Desa', 'Kecamatan', 'PPDWT', 'PPTB', 'ODP', 'ODP-SP', 'PDP', 'PDP-S', 'PDP-M', 'OTG', 'Positive', 'Sembuh', 'Meninggal', 'Action']"
      :columnsInfo="{'PPDWT':'Pelaku Perjalanan Dari Wilayah Terjangkit', 'PPTB': 'Pelaku Perjalanan Tak Bergejala', 'ODP-SP': 'ODP Selesai Pemantauan', 'PDP-S': 'PDP Sembuh', 'PDP-M': 'PDP Meninggal', 'OTG': 'Orang Tidak Bergejala'}"
      :searchable="true"
      :withActionButton="false"
      :showDetailFunc="showDetail"
      :limit="10"
    >
      <template v-slot:bar>
        <button v-if="isDirty" class="ui text icon green button right floated" @click="commit">
          <i class="fa-angle-double-up icon"></i> Commit
        </button>
        <button class="ui text icon button right floated" @click="addRecord">
          <i class="fa-plus icon"></i> Tambah
        </button>
      </template>
      <template v-slot:tdmap="self">
        <td>{{self.item['id']}}</td>
        <td style="width: 160px !important;">
          <strong>{{self.item['village_name']}}</strong>
          <br />
          <span>{{self.item['loc_scope']}}</span>
          <br />
          <small>updated: {{self.item['last_updated']}}</small>
        </td>
        <td>{{self.item['district_name']}}</td>

        <td
          :class="commitLogs[self.item['id']] != null && commitLogs[self.item['id']]['ppdwt'] != self.item['ppdwt'] ? 'dirty': '' "
        >
          <a
            href="javascript://"
            v-on:click="editValue(self,'ppdwt','ppdwt',self.item['ppdwt']);"
          >{{self.item['ppdwt']}}</a>
          <span
            v-if="commitLogs[self.item['id']] != null && commitLogs[self.item['id']]['ppdwt'] != self.item['ppdwt']"
          >
            <i class="ui icon fa-arrow-right"></i>
            {{commitLogs[self.item['id']]['ppdwt']}}
          </span>
        </td>

        <td
          :class="commitLogs[self.item['id']] != null && commitLogs[self.item['id']]['pptb'] != self.item['pptb'] ? 'dirty': '' "
        >
          <a
            href="javascript://"
            v-on:click="editValue(self,'pptb','pptb',self.item['pptb']);"
          >{{self.item['pptb']}}</a>
          <span
            v-if="commitLogs[self.item['id']] != null && commitLogs[self.item['id']]['pptb'] != self.item['pptb']"
          >
            <i class="ui icon fa-arrow-right"></i>
            {{commitLogs[self.item['id']]['pptb']}}
          </span>
        </td>

        <td
          :class="commitLogs[self.item['id']] != null && commitLogs[self.item['id']]['odp'] != self.item['odp'] ? 'dirty': '' "
        >
          <a
            href="javascript://"
            v-on:click="editValue(self,'odp','odp',self.item['odp']);"
          >{{self.item['odp']}}</a>
          <span
            v-if="commitLogs[self.item['id']] != null && commitLogs[self.item['id']]['odp'] != self.item['odp']"
          >
            <i class="ui icon fa-arrow-right"></i>
            {{commitLogs[self.item['id']]['odp']}}
          </span>
        </td>

        <td
          :class="commitLogs[self.item['id']] != null && commitLogs[self.item['id']]['odpsp'] != self.item['odpsp'] ? 'dirty': '' "
        >
          <a
            href="javascript://"
            v-on:click="editValue(self,'odpsp','odpsp',self.item['odpsp']);"
          >{{self.item['odpsp']}}</a>
          <span
            v-if="commitLogs[self.item['id']] != null && commitLogs[self.item['id']]['odpsp'] != self.item['odpsp']"
          >
            <i class="ui icon fa-arrow-right"></i>
            {{commitLogs[self.item['id']]['odpsp']}}
          </span>
        </td>

        <td
          :class="commitLogs[self.item['id']] != null && commitLogs[self.item['id']]['pdp'] != self.item['pdp'] ? 'dirty': '' "
        >
          <a
            href="javascript://"
            v-on:click="editValue(self,'pdp','pdp',self.item['pdp']);"
          >{{self.item['pdp']}}</a>
          <span
            v-if="commitLogs[self.item['id']] != null && commitLogs[self.item['id']]['pdp'] != self.item['pdp']"
          >
            <i class="ui icon fa-arrow-right"></i>
            {{commitLogs[self.item['id']]['pdp']}}
          </span>
        </td>

        <td
          :class="commitLogs[self.item['id']] != null && commitLogs[self.item['id']]['pdps'] != self.item['pdps'] ? 'dirty': '' "
        >
          <a
            href="javascript://"
            v-on:click="editValue(self,'pdps','pdps',self.item['pdps']);"
            class="dirty"
          >{{self.item['pdps']}}</a>

          <span
            v-if="commitLogs[self.item['id']] != null && commitLogs[self.item['id']]['pdps'] != self.item['pdps']"
          >
            <i class="ui icon fa-arrow-right"></i>
            {{commitLogs[self.item['id']]['pdps']}}
          </span>
        </td>

        <td
          :class="commitLogs[self.item['id']] != null && commitLogs[self.item['id']]['pdpm'] != self.item['pdpm'] ? 'dirty': '' "
        >
          <a
            href="javascript://"
            v-on:click="editValue(self,'pdpm','pdpm',self.item['pdpm']);"
            class="dirty"
          >{{self.item['pdpm']}}</a>

          <span
            v-if="commitLogs[self.item['id']] != null && commitLogs[self.item['id']]['pdpm'] != self.item['pdpm']"
          >
            <i class="ui icon fa-arrow-right"></i>
            {{commitLogs[self.item['id']]['pdpm']}}
          </span>
        </td>

        <td
          :class="commitLogs[self.item['id']] != null && commitLogs[self.item['id']]['otg'] != self.item['otg'] ? 'dirty': '' "
        >
          <a
            href="javascript://"
            v-on:click="editValue(self,'otg','otg',self.item['otg']);"
            class="dirty"
          >{{self.item['otg']}}</a>

          <span
            v-if="commitLogs[self.item['id']] != null && commitLogs[self.item['id']]['otg'] != self.item['otg']"
          >
            <i class="ui icon fa-arrow-right"></i>
            {{commitLogs[self.item['id']]['otg']}}
          </span>
        </td>

        <td
          :class="commitLogs[self.item['id']] != null && commitLogs[self.item['id']]['cases'] != self.item['cases'] ? 'dirty': '' "
        >
          <a
            href="javascript://"
            v-on:click="editValue(self,'cases','cases',self.item['cases']);"
            class="dirty"
          >{{self.item['cases']}}</a>

          <span
            v-if="commitLogs[self.item['id']] != null && commitLogs[self.item['id']]['cases'] != self.item['cases']"
          >
            <i class="ui icon fa-arrow-right"></i>
            {{commitLogs[self.item['id']]['cases']}}
          </span>
        </td>

        <td
          :class="commitLogs[self.item['id']] != null && commitLogs[self.item['id']]['recovered'] != self.item['recovered'] ? 'dirty': '' "
        >
          <a
            href="javascript://"
            v-on:click="editValue(self,'sembuh','recovered',self.item['recovered']);"
            class="dirty"
          >{{self.item['recovered']}}</a>

          <span
            v-if="commitLogs[self.item['id']] != null && commitLogs[self.item['id']]['recovered'] != self.item['recovered']"
          >
            <i class="ui icon fa-arrow-right"></i>
            {{commitLogs[self.item['id']]['recovered']}}
          </span>
        </td>
        <td
          :class="commitLogs[self.item['id']] != null && commitLogs[self.item['id']]['deaths'] != self.item['deaths'] ? 'dirty': '' "
        >
          <a
            href="javascript://"
            v-on:click="editValue(self,'meninggal','deaths',self.item['deaths']);"
          >{{self.item['deaths']}}</a>

          <span
            v-if="commitLogs[self.item['id']] != null && commitLogs[self.item['id']]['deaths'] != self.item['deaths']"
          >
            <i class="ui icon fa-arrow-right"></i>
            {{commitLogs[self.item['id']]['deaths']}}
          </span>
        </td>
        <td>
          <button class="ui icon button" @click="confirmDelete(self.item)">
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
      :buttonsText="{reject: 'Batal', approve: 'Ok'}"
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
            <input
              ref="newValue"
              type="text"
              name="NewValue"
              id="NewValue"
              @keyup.enter="approveDialog"
            />
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
      modalName="CommitModal"
      caption="Commit data"
      approveText="Lanjut"
      :withCloseButton="true"
      @onApprove="doCommit"
    >
      <p>Yakin untuk melakukan commit? Semua perubahan akan di-simpan ke server, pastikan koneksi internet Anda lancar.</p>

      <div v-if="isSuperUser">
        <div class="ui form">
          <div class="field">
            <label>Nama provinsi:</label>
            <input ref="provNameInput" v-model="provName" type="text" name="ProvName" id="ProvName" />
          </div>
          <div class="field">
            <label>Nama Kab/Kota:</label>
            <input
              ref="cityNameInput"
              v-model="cityName"
              type="text"
              name="CityName"
              id="CityName"
              @keyup.enter="doCommit"
            />
          </div>
        </div>
      </div>
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
</template>


<script>
import AnsTable from "@/components/AnsTable.vue";
import DialogModal from "@/components/modal/DialogModal.vue";
import ConfirmDialog from "@/components/modal/ConfirmDialog.vue";
import { obMapToArrayValues } from "@/utils/utils";

export default {
  name: "Records",
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
      toDelete: { id: 0, loc: "" },
      provName: "",
      cityName: ""
    };
  },
  computed: {
    isSuperUser() {
      return this.$session.get("user_id") == 1;
    }
  },
  mounted() {
    if (localStorage.commitLogs != null && localStorage.commitLogs != "") {
      try {
        this.commitLogs = JSON.parse(localStorage.commitLogs);
        if (this.commitLogs == null) {
          this.commitLogs = {};
        } else {
          this.isDirty = true;
        }
      } catch {
        this.commitLogs = {};
      }
    }
  },
  methods: {
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
        .publicApi.post("/village/v1/add", {
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
      this.$router.push("/dashboard/records/" + item.id);
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
      this.persist();

      this.$modal.hide("EditValueModal");
    },
    commit() {
      this.$modal.show("CommitModal");
    },
    doCommit() {
      if (this.isSupperUser) {
        if (this.provName == "") {
          this.showError(
            "Anda perlu memasukkan nama provinsi untuk data tersebut"
          );
          return;
        }
        if (this.cityName == "") {
          this.showError(
            "Anda perlu memasukkan nama kota/kab untuk data tersebut"
          );
          return;
        }
      }

      this.$modal.hide("CommitModal");
      var normCommitLogs = obMapToArrayValues(this.commitLogs);
      this.$pandemia
        .api()
        .publicApi.post("/village/v1/commit", {
          province: this.provName,
          city: this.cityName,
          records: normCommitLogs
        })
        .then(resp => {
          // console.log(resp);
          if (resp.data.code == 0) {
            this.isDirty = false;
            this.commitLogs = {};
            this.persist();
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
        .publicApi.post("/pandemia/v1/village_data/delete", {
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
    persist() {
      localStorage.commitLogs = JSON.stringify(this.commitLogs);
    }
  }
};
</script>

<style lang="less">
td.dirty {
  color: orange;
}
</style>