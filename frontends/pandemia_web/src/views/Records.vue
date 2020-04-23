<template>
  <div>
    <AnsTable
      :key="tableRecords"
      data-source-url="/pandemia/v1/search_records"
      :columns="['ID', 'Lokasi', 'Jenis', 'ODP', 'PDP', 'Positive', 'Sembuh', 'Meninggal', 'Action']"
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
        <td>
          <strong>{{self.item['loc']}}</strong>
          <br />
          <span>{{self.item['loc_scope']}}</span>
          <br />
          <small>updated: {{self.item['last_updated']}}</small>
        </td>
        <td>
          <span v-if="self.item['loc_kind'] == 0">Global</span>
          <span v-if="self.item['loc_kind'] == 1">Benua</span>
          <span v-if="self.item['loc_kind'] == 2">Negara</span>
          <span v-if="self.item['loc_kind'] == 3">Provinsi</span>
          <span v-if="self.item['loc_kind'] == 4">Kabupaten/Kota</span>
          <span v-if="self.item['loc_kind'] == 5">Kecamatan</span>
        </td>

        <td
          :class="commitLogs[self.item['id']] != null && commitLogs[self.item['id']]['odp'] != self.item['odp'] ? 'dirty': '' "
        >
          <a
            href="javascript://"
            v-on:click="editValue(self,'positif','odp',self.item['odp']);"
          >{{self.item['odp']}}</a>
          <span
            v-if="commitLogs[self.item['id']] != null && commitLogs[self.item['id']]['odp'] != self.item['odp']"
          >
            <i class="ui icon fa-arrow-right"></i>
            {{commitLogs[self.item['id']]['odp']}}
          </span>
        </td>

        <td
          :class="commitLogs[self.item['id']] != null && commitLogs[self.item['id']]['pdp'] != self.item['pdp'] ? 'dirty': '' "
        >
          <a
            href="javascript://"
            v-on:click="editValue(self,'positif','pdp',self.item['pdp']);"
          >{{self.item['pdp']}}</a>
          <span
            v-if="commitLogs[self.item['id']] != null && commitLogs[self.item['id']]['pdp'] != self.item['pdp']"
          >
            <i class="ui icon fa-arrow-right"></i>
            {{commitLogs[self.item['id']]['pdp']}}
          </span>
        </td>

        <td
          :class="commitLogs[self.item['id']] != null && commitLogs[self.item['id']]['total_cases'] != self.item['total_cases'] ? 'dirty': '' "
        >
          <a
            href="javascript://"
            v-on:click="editValue(self,'positif','total_cases',self.item['total_cases']);"
          >{{self.item['total_cases']}}</a>
          <span
            v-if="commitLogs[self.item['id']] != null && commitLogs[self.item['id']]['total_cases'] != self.item['total_cases']"
          >
            <i class="ui icon fa-arrow-right"></i>
            {{commitLogs[self.item['id']]['total_cases']}}
          </span>
        </td>
        <td
          :class="commitLogs[self.item['id']] != null && commitLogs[self.item['id']]['total_recovered'] != self.item['total_recovered'] ? 'dirty': '' "
        >
          <a
            href="javascript://"
            v-on:click="editValue(self,'sembuh','total_recovered',self.item['total_recovered']);"
            class="dirty"
          >{{self.item['total_recovered']}}</a>

          <span
            v-if="commitLogs[self.item['id']] != null && commitLogs[self.item['id']]['total_recovered'] != self.item['total_recovered']"
          >
            <i class="ui icon fa-arrow-right"></i>
            {{commitLogs[self.item['id']]['total_recovered']}}
          </span>
        </td>
        <td
          :class="commitLogs[self.item['id']] != null && commitLogs[self.item['id']]['total_deaths'] != self.item['total_deaths'] ? 'dirty': '' "
        >
          <a
            href="javascript://"
            v-on:click="editValue(self,'meninggal','total_deaths',self.item['total_deaths']);"
          >{{self.item['total_deaths']}}</a>

          <span
            v-if="commitLogs[self.item['id']] != null && commitLogs[self.item['id']]['total_deaths'] != self.item['total_deaths']"
          >
            <i class="ui icon fa-arrow-right"></i>
            {{commitLogs[self.item['id']]['total_deaths']}}
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
        <h2 class="ui header">Tambah Entri Baru</h2>

        <div style="text-align: left;">
          <div class="ui form">
            <!-- <div class="field">
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
            </div>-->

            <div class="field">
              <vue-autosuggest
                v-model="addLocAddress"
                :suggestions="filteredAddresses"
                @click="clickHandler"
                @selected="onSelected"
                :get-suggestion-value="getSuggestionValue"
                :input-props="{id:'autosuggest__input', placeholder:'Alamat Lokasi'}"
              >
                <div slot-scope="{suggestion}" style="display: flex; align-items: center;">
                  <div
                    style="{ display: 'flex', color: 'navyblue'}"
                  >{{suggestion.item.address}} {{suggestion.item.kind == 4 ? '(Kab/Kota)' : '(Kecamatan)' }}</div>
                </div>
              </vue-autosuggest>
            </div>

            <div class="ui grid">
              <div class="three wide column">
                <div class="field">
                  <label>ODP:</label>
                  <input ref="addRecOdp" type="text" name="Odp" id="Odp" v-model="addOdp" />
                </div>
              </div>
              <div class="three wide column">
                <div class="field">
                  <label>PDP:</label>
                  <input ref="addRecPdp" type="text" name="Pdp" id="Pdp" v-model="addPdp" />
                </div>
              </div>
              <div class="three wide column">
                <div class="field">
                  <label>Positive:</label>
                  <input
                    ref="addRecPositive"
                    type="text"
                    name="Positive"
                    id="Positive"
                    v-model="addPositive"
                  />
                </div>
              </div>
              <div class="three wide column">
                <div class="field">
                  <label>Sembuh:</label>
                  <input
                    ref="addRecTotalRecovered"
                    type="text"
                    name="Recovered"
                    id="Recovered"
                    v-model="addRecovered"
                  />
                </div>
              </div>
              <div class="three wide column">
                <div class="field">
                  <label>Meninggal:</label>
                  <input
                    ref="addRecTotalDeaths"
                    type="text"
                    name="Deaths"
                    id="Deaths"
                    v-model="addDeaths"
                  />
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
</template>


<script>
import AnsTable from "@/components/AnsTable.vue";
import DialogModal from "@/components/modal/DialogModal.vue";
import ConfirmDialog from "@/components/modal/ConfirmDialog.vue";
import { obMapToArrayValues } from "@/utils/utils";
// import _axios from "axios";

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

      province: "",
      city: "",
      addLocAddressId: 0,
      addLocAddress: "",
      districtSuggestions: [],

      addPpdwt: "0",
      addPptb: "0",
      addOdp: "0",
      addOdpsp: "0",
      addPdp: "0",
      addPdps: "0",
      addPdpm: "0",
      addOtg: "0",
      addPositive: "0",
      addRecovered: "0",
      addDeaths: "0"
    };
  },
  computed: {
    filteredAddresses() {
      return [
        {
          data: this.districtSuggestions.filter(d => {
            return (
              d.address
                .toLowerCase()
                .indexOf(this.addLocAddress.toLowerCase()) > -1
            );
          })
        }
      ];
    },
    isAdmin(){
      return this.$session.get("is_admin");
    }
  },
  mounted() {
    this.loadData();
  },
  methods: {
    loadData() {
      this.province = localStorage.province || localStorage.province;
      this.city = localStorage.city || localStorage.city;
      this.province = this.province || this.$session.get("user_province");
      this.city = this.city || this.$session.get("user_city");

      var multiLoc = this.$session.get("user_accesses").indexOf('multi_location') > -1;

      if (this.province && this.city && !multiLoc) {
        this.$pandemia
          .api()
          .publicApi.get(`/analytic/v1/data/districts?province=${this.province}&city=${this.city}&offset=0&limit=1000`)
          .then(resp => {
            if (resp.data.code == 0) {
              this.districtSuggestions = resp.data.result;
            } else {
              this.showError(resp.data.description);
            }
          });
      } else if (this.province && !multiLoc) {
        this.$pandemia
          .api()
          .publicApi.get(`/analytic/v1/data/location_address?province=${this.province}`)
          .then(resp => {
            if (resp.data.code == 0) {
              this.districtSuggestions = resp.data.result;
            } else {
              this.showError(resp.data.description);
            }
          });
      }else{
        this.$pandemia
          .api()
          .publicApi.get(`/analytic/v1/data/location_address`)
          .then(resp => {
            if (resp.data.code == 0) {
              this.districtSuggestions = resp.data.result;
            } else {
              this.showError(resp.data.description);
            }
          });
      }
    },
    normalizePath(q) {
      return q.replace(/\W+/g, "-").toLowerCase();
    },
    getSuggestionValue(suggestion) {
      return suggestion.item.address;
    },
    clickHandler(_) {},
    onSelected(item) {
      this.addLocAddress = item.item.address;
      this.addLocItem = item.item;
    },
    addRecord() {
      this.$modal.show("AddRecordModal");
    },
    doAddRecord() {
      // var loc = this.addLocAddress,
      var totalCases = parseInt(this.$refs["addRecPositive"].value) || 0,
        totalDeaths = parseInt(this.addDeaths) || 0,
        totalRecovered = parseInt(this.addRecovered) || 0;
      this.$pandemia
        .api()
        .publicApi.post("/pandemia/v1/add_record", {
          loc: this.addLocItem.name,
          loc_id: this.addLocItem.id,
          // loc_scope: locScope,
          loc_kind: this.addLocItem.kind,
          loc_path: this.addLocItem.path,

          total_cases: totalCases,
          total_deaths: totalDeaths,
          total_recovered: totalRecovered,
          active_cases: 0,
          critical_cases: 0,
          ppdwt: parseInt(this.addPpdwt),
          pptb: parseInt(this.addPptb),
          odp: parseInt(this.addOdp),
          odpsp: parseInt(this.addOdpsp),
          pdp: parseInt(this.addPdp),
          pdps: parseInt(this.addPdps),
          pdpm: parseInt(this.addPdpm),
          otg: parseInt(this.addOtg)
        })
        .then(resp => {
          if (resp.data.code == 0) {
            this.$modal.hide("AddRecordModal");

            this.showSuccess("Rekod berhasil ditambahkan");
            this.refreshTable();
            this.resetData();
          } else {
            var suggest = "";
            if (resp.data.description.indexOf("Invalid") > -1) {
              suggest = "Mohon periksa kembali data masukan Anda";
            }
            this.showError("Gagal menambahkan rekod. " + suggest);
          }
        });
    },
    resetData() {
      this.totalCases = 0;
      this.totalDeaths = 0;
      this.totalRecovered = 0;
      this.addPpdwt = 0;
      this.addPptb = 0;
      this.addOdp = 0;
      this.addOdpsp = 0;
      this.addPdp = 0;
      this.addPdps = 0;
      this.addPdpm = 0;
      this.addOtg = 0;
    },
    onAddRecordOpened() {
      // this.$refs["addRecLocInput"].focus();
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
    }
  }
};
</script>

<style lang="less">
td.dirty {
  color: orange;
}

.autosuggest-container,
.autosuggest__results {
  position: absolute;
  justify-content: center;
  width: 400px;
  background-color: white;
  border: 1px solid #cacaca;
  z-index: 90000;
}

#autosuggest {
  width: 100%;
  display: block;
  z-index: 90000;
}
.autosuggest__results-item--highlighted {
  background-color: rgba(51, 217, 178, 0.2);
  z-index: 90000;
}
</style>