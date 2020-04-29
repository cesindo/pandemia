<template>
  <div>
    <div id="Main">
      <AnsTable
        :key="tableSubReports"
        data-source-url="/pandemia/v1/sub_report/search"
        :add-params="'status=-1&city_id=' + cityId"
        :columns="['ID', 'Nama', 'Desa', 'Umur', 'Tempat Tinggal', 'JK', 'Status', 'Catatan & Info Tambahan', 'Pendata', 'Operasi']"
        :searchable="true"
        :withActionButton="false"
        :limit="100"
      >
        <template v-slot:bellow-search>
          <small>
            <i class="ui icon fa-info"></i>
            <a href="javascript://void();" @click="searchTips">Tips pencarian</a>
          </small>
        </template>
        <template v-slot:bar>
          <!-- <button v-if="isDirty" class="ui text icon green button right floated" @click="commit">
            <i class="fa-angle-double-up icon"></i> Commit
          </button>-->

          <button v-if="addable" class="ui text icon button right floated" @click="addSubReport">
            <i class="fa-plus icon"></i> Tambah
          </button>
        </template>

        <template v-slot:tdmap="self">
          <td>{{self.item['id']}}</td>
          <td>{{self.item['full_name']}}</td>
          <td>{{self.item['reporter_village']}}</td>
          <td>{{self.item['age']}}</td>
          <td>{{self.item['residence_address']}}</td>
          <td>{{self.item['gender'] == 'L' ? 'Laki-laki' : 'Perempuan' }}</td>
          <td>{{ statusIdNameToLabel(self.item['status']) }}</td>
          <td>
            {{self.item['notes'] }}
            <div v-if="self.item['coming_from'] != ''">
              datang dari:
              {{self.item['coming_from'] }}
              <div>
                {{
                self.item['coming_from'] != null && self.item['coming_from'].length > 0 ?
                " @ " + self.item['arrival_date'] : "-"
                }}
              </div>
            </div>

            <div>
              {{self.item['healthy_notes']}}
              <br />
              {{self.item['from_red_zone'] ? 'Dari zona merah' : 'Tidak dari zona merah'}},
              <br />
              {{self.item['has_symptoms'] ? 'Bergejala' : 'Tidak bergejala'}}
              <br />
            </div>
          </td>
          <td>{{self.item['creator_name']}} {{ self.item['created_by_admin'] ? '(admin)' : '' }}</td>
          <td style="width: 120px;">
            <!-- <button
              v-if="self.item['status'] != 'approved'"
              class="ui icon button"
              title="Approve"
              @click="approve(self.item)"
            >
              <i class="check icon"></i>
            </button>-->
            <button class="ui icon button" title="Edit" @click="edit(self.item)">
              <i class="edit icon"></i>
            </button>
            <button class="ui icon button" title="Hapus" @click="confirmDelete(self.item)">
              <i class="trash icon"></i>
            </button>
          </td>
        </template>
      </AnsTable>

      <modal
        name="AddData"
        width="95%"
        :max-width="1000"
        :min-width="500"
        :clickToClose="true"
        :scrollable="true"
        :adaptive="true"
        height="auto"
        @opened="onAddDataOpened"
      >
        <div class="size-modal-content">
          <div class="ui fullscreen modal transition visible active">
            <i class="close icon" @click="onAddDataCanceled()"></i>
            <div class="content">
              <h2 class="ui header">{{ editMode ? 'Edit Data' : 'Tambah Laporan' }}</h2>

              <div style="text-align: left;">
                <div class="ui form">
                  <div class="ui grid">
                    <div class="eight wide column">
                      <div class="field">
                        <label>Nama:</label>
                        <input ref="nameInput" type="text" name="Name" id="Name" autofocus />
                      </div>

                      <div v-if="adminMode" class="field">
                        <label>Desa:</label>
                        <!-- <input ref="villageInput" type="text" name="Village" id="Village" /> -->

                        <!-- <div class="autosuggest-container"> -->
                        <vue-autosuggest
                          v-model="queryVillage"
                          :suggestions="filteredVillage"
                          @click="clickHandler"
                          @input="onInputChange"
                          @selected="onSelected"
                          :get-suggestion-value="getSuggestionValue"
                          :input-props="{id:'autosuggest__input', placeholder:'Desa'}"
                        >
                          <div
                            slot-scope="{suggestion}"
                            style="display: flex; align-items: center;"
                          >
                            <div
                              style="{ display: 'flex', color: 'navyblue'}"
                            >{{suggestion.item.address}}</div>
                          </div>
                        </vue-autosuggest>
                        <!-- </div> -->
                      </div>

                      <div class="field">
                        <label>Alamat:</label>
                        <!-- <input ref="addAddressInput" type="text" name="Address" id="Address" /> -->
                        <textarea ref="addAddressInput" name="Address" id cols="30" rows="3"></textarea>
                      </div>
                      <div class="field">
                        <label>Usia:</label>
                        <input ref="addAgeInput" type="text" name="Age" id="Age" />
                      </div>
                      <div class="field">
                        <label>Jenis Kelamin:</label>
                        <select ref="addGender" name="addGender" id="Gender">
                          <option value="L">Laki-laki</option>
                          <option value="P">Perempuan</option>
                        </select>
                      </div>
                    </div>
                    <div class="eight wide column">
                      <div class="field">
                        <label>Datang Dari:</label>
                        <input ref="addComeFromInput" type="text" name="ComeFrom" id="ComeFrom" />
                      </div>
                      <div class="field">
                        <label>Tanggal Kedatangan:</label>
                        <!-- <input
                      ref="addArrivalDateInput"
                      type="text"
                      name="ArrivalDate"
                      id="ArrivalDate"
                        />-->
                        <datetime v-model="date"></datetime>
                      </div>
                      <div class="field">
                        <label>Catatan:</label>
                        <!-- <input ref="addNotesInput" type="text" name="Notes" id="Notes" /> -->
                        <textarea ref="addNotesInput" name="Notes" id="Notes" cols="30" rows="3"></textarea>
                      </div>
                      <div class="field">
                        <label>Status:</label>
                        <select ref="addStatus" name="AddStatus" id="Status" v-model="addStatus">
                          <option value="odp">ODP</option>
                          <option value="odps">ODP Selesai Pemantauan</option>
                          <option value="pdp">PDP</option>
                          <option value="pdps" v-if="adminMode">PDP Sembuh</option>
                          <option value="pdpm" v-if="adminMode">PDP Meninggal</option>
                          <option value="otg">OTG</option>
                          <option v-if="adminMode" value="positive">Positif</option>
                          <option v-if="adminMode" value="recovered">Positif Sembuh</option>
                          <option v-if="adminMode" value="death">Positif Meninggal</option>
                        </select>
                      </div>

                      <div class="ui top attached segment">
                        <div class="ui header">Info Tambahan</div>

                        <div class="field">
                          <div class="ui checkbox">
                            <input type="checkbox" name="Traveler" id="Traveler" v-model="traveler" />
                            <label>Pelaku Perjalanan</label>
                          </div>
                        </div>

                        <div class="field">
                          <div class="ui checkbox">
                            <input
                              type="checkbox"
                              name="FromRedZone"
                              id="FromRedZone"
                              v-model="fromRedZone"
                            />
                            <label>Dari zona merah</label>
                          </div>
                        </div>

                        <div class="field">
                          <div class="ui checkbox">
                            <input
                              type="checkbox"
                              name="DasSymptoms"
                              id="DasSymptoms"
                              v-model="hasSymptoms"
                            />
                            <label>Punya gejala COVID-19</label>
                          </div>
                        </div>
                      </div>
                    </div>
                  </div>
                </div>
              </div>

              <div class="ui basic center aligned segment">
                <button :disabled="isLoading" class="ui button" @click="onAddDataCanceled">Batal</button>
                <button
                  :disabled="isLoading"
                  v-if="!editMode"
                  class="ui primary button"
                  @click="onAddDataApproved"
                >Tambah</button>
                <button
                  :disabled="isLoading"
                  v-if="editMode"
                  class="ui blue button"
                  @click="() => onDataUpdate('correction')"
                >Koreksi</button>
                <button
                  :disabled="isLoading"
                  v-if="editMode"
                  class="ui green button"
                  @click="() => onDataUpdate('update')"
                >Pembaharuan</button>
              </div>
            </div>
          </div>
        </div>
      </modal>
      <!-- <DialogModal
        modalName="AddData"
        caption="Tambah Laporan"
        :withCloseButton="true"
        @onApprove="onAddDataApproved"
        @opened="onAddDataOpened"
        :buttonsText="{reject: 'Batal', approve: 'Tambah'}"
        width="1700"
      >
        
      </DialogModal>-->

      <BasicModal modalName="SearchTips" caption="Tips Pencarian" :clickToClose="true">
        <p>
          <strong>Tips pencarian</strong>
        </p>

        <ul>
          <li>tt: - untuk mencari dengan kriteria tempat tinggal, contoh: tt:jakarta</li>
          <li>desa: - untuk mencari hanya dari desa asal laporan dibuat, contoh: desa:kalikajar</li>
          <li>kcm: - untuk mencari hanya dari kecamatan asal laporan dibuat, contoh: kcm:watumalang</li>
          <li>umur: - untuk mencari dengan kriteria umur, contoh: umur:33</li>
          <li>jk: - untuk mencari dengan kriteria jenis kelamin, contoh: jk:L untuk laki-laki, jk:P untuk perempuan</li>
          <li>dari: - untuk mencari dengan kriteria nama daerah kedatangan, contoh: dari:bandung</li>
          <li>status: - untuk mencari dengan kriteria status odp/pdp/positive/recovered/death nya, contoh: status:odp</li>
        </ul>

        <strong>Contoh menggabungkan kriteria pencarian:</strong>
        <p class="sample">dari:jakarta status:odp nama:anto</p>
        <center>
          <button class="ui button" @click="closeTips">Close</button>
        </center>
      </BasicModal>

      <ConfirmDialog
        modalName="Delete"
        caption="Konfirmasi"
        approveText="Hapus"
        :withCloseButton="true"
        @onApprove="doDelete"
      >
        <p>
          Yakin untuk menghapus data id
          <strong>{{toProcess["id"]}}</strong> atas nama
          <strong>{{toProcess['full_name']}}</strong> ?
        </p>
      </ConfirmDialog>

      <!-- <ConfirmDialog
        modalName="Approve"
        caption="Konfirmasi"
        approveText="Approve"
        :withCloseButton="true"
        @onApprove="doApprove"
      >
        <p>Yakin untuk meng-approve laporan ini? :</p>
        <strong>"{{toProcess['notes']}}"</strong>

        <p>
          dari
          <strong>{{toProcess['creator_name']}}</strong> -
          <strong>{{toProcess['location']}}</strong>
        </p>
      </ConfirmDialog>-->
    </div>
  </div>
</template>


<script>
import AnsTable from "@/components/AnsTable.vue";
// import DialogModal from "@/components/modal/DialogModal.vue";
import BasicModal from "@/components/modal/BasicModal.vue";
import ConfirmDialog from "@/components/modal/ConfirmDialog.vue";
// import _axios from "axios";

export default {
  name: "Satgas",
  components: {
    AnsTable,
    // DialogModal,
    ConfirmDialog,
    BasicModal
  },
  props: {
    addable: { type: Boolean, default: false },
    adminMode: { type: Boolean, default: false }
  },
  data() {
    return {
      editedItem: { loc: "" },
      editedCatName: "",
      editedCat: "",
      commitLogs: {},
      isDirty: false,
      tableSubReports: "-0",
      toProcess: { id: 0, loc: "" },
      date: null,
      editMode: false,
      toEdit: null,
      isLoading: false,

      queryVillage: "",
      selectedVillage: {},
      // queryDistrict: "",
      villageName: "",
      districtName: "",
      villageSuggestions: [],
      districtSuggestions: [],

      traveler: false,
      fromRedZone: false,
      hasSymptoms: false,
      addStatus: ""
    };
  },
  computed: {
    filteredVillage() {
      return [
        {
          data: this.villageSuggestions.filter(d => {
            return (
              d.address.toLowerCase().indexOf(this.queryVillage.toLowerCase()) >
              -1
            );
          })
        }
      ];
    },
    cityId(){
      return this.$session.get("user_city_id");
    }
    // filteredDistricts() {
    //   return [
    //     {
    //       data: this.districtSuggestions.filter(option => {
    //         return (
    //           option.toLowerCase().indexOf(this.queryDistrict.toLowerCase()) >
    //           -1
    //         );
    //       })
    //     }
    //   ];
    // }
  },
  mounted() {
    // _axios.get("/json/wonosobo-villages.json").then(response => {
    //   // console.log(response);
    //   this.villageSuggestions = response.data.villages;
    // });
    // _axios.get("/json/wonosobo-districts.json").then(response => {
    //   // console.log(response);
    //   this.districtSuggestions = response.data.districts;
    // });

    this.province = localStorage.province || localStorage.province;
    this.city = localStorage.city || localStorage.city;
    this.province = this.province || this.$session.get("user_province");
    this.city = this.city || this.$session.get("user_city");

    if (this.province && this.city) {
      this.$pandemia
        .api()
        .publicApi.get(
          `/village/v1/village_address?scope=/Indonesia/${this.province}/${this.city}&offset=0&limit=1000`
        )
        .then(resp => {
          // console.log(resp);
          if (resp.data.code == 0) {
            this.villageSuggestions = resp.data.result.entries;
          } else {
            this.showError(resp.data.description);
          }
        });
    } else {
      // console.log(
      //   "Cannot fetch village-address data, province or city not defined"
      // );
      // console.log(this.$session);
    }
  },
  methods: {
    clickHandler(_) {},
    onSelected(item) {
      this.villageName = item.item.address;
      this.queryVillage = item.item.address;
    },
    onDistrictSelected(item) {
      this.districtName = item.item;
      this.queryDistrict = item.item;
    },
    onInputChange(_text) {
      // console.log(text);
    },
    getSuggestionValue(suggestion) {
      this.selectedVillage = suggestion.item;
      return suggestion.item.address;
    },
    // focusMe(e) {
    //   console.log(e); // FocusEvent
    // },
    edit(item) {
      this.editMode = true;
      this.toEdit = item;
      this.traveler = item["traveler"] == true;
      this.fromRedZone = item["from_red_zone"] == true;
      this.hasSymptoms = item["has_symptoms"] == true;
      this.villageName = item["reporter_village"];
      this.districtName = item["reporter_district"];
      this.addStatus = item["status"].toLowerCase();
      this.$modal.show("AddData");
    },
    approve(item) {
      this.toProcess = item;
      this.$modal.show("Approve");
    },
    // doApprove() {
    //   this.$pandemia
    //     .api()
    //     .publicApi.post("/pandemia/v1/report_note/update_state", {
    //       id: this.toProcess["id"],
    //       state: "approved"
    //     })
    //     .then(resp => {
    //       // console.log(resp);
    //       if (resp.data.code == 0) {
    //         this.refreshTable();
    //         this.$modal.hide("Approve");
    //         this.showSuccess("Laporan telah berhasil di-aprove");
    //       } else {
    //         this.showError(
    //           "Gagal meng-approve laporan, hubungi sistem administrator"
    //         );
    //       }
    //     });
    // },
    // onAddVillageOpened() {
    //   this.$refs["addRecLocInput"].focus();
    // },
    // showDetail(item) {
    //   this.$router.push("/dashboard/villages/" + item.id);
    // },
    // editValue(self, catName, cat) {
    //   this.editedItem = self.item;
    //   this.editedCatName = catName;
    //   this.editedCat = cat;
    //   this.$modal.show("EditValueModal", { item: self.item });
    // },
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
    // editValueDialogOpened(_) {
    //   this.$refs["newValue"].focus();
    // },
    // approveDialog() {
    //   var commitLog = this.commitLogs[this.editedItem["id"]];

    //   if (commitLog == undefined) {
    //     commitLog = Object.assign({}, this.editedItem);
    //     this.isDirty = true;
    //   }

    //   commitLog[this.editedCat] = parseInt(this.$refs.newValue.value);

    //   this.$set(this.commitLogs, this.editedItem["id"], commitLog);

    //   this.isDirty = true;

    //   this.$modal.hide("EditValueModal");
    // },
    confirmDelete(item) {
      this.toProcess = item;
      this.$modal.show("Delete");
    },
    doDelete() {
      this.$modal.hide("Delete");
      this.$pandemia
        .api()
        .publicApi.post("/pandemia/v1/sub_report/delete", {
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
      this.tableSubReports = "A-" + new Date().getTime();
    },
    searchTips() {
      this.$modal.show("SearchTips");
    },
    closeTips() {
      this.$modal.hide("SearchTips");
    },
    addSubReport() {
      this.editMode = false;
      this.$modal.show("AddData");
    },
    onAddDataApproved() {
      this.isLoading = true;

      var name = this.$refs["nameInput"].value,
        address = this.$refs["addAddressInput"].value,
        age = this.$refs["addAgeInput"].value,
        addGender = this.$refs["addGender"].value,
        addComeFrom = this.$refs["addComeFromInput"].value,
        // arrivalDate = this.date,
        notes = this.$refs["addNotesInput"].value;
      // addStatus = this.$refs["addStatus"].value;

      var addInfo = [];

      if (this.traveler) {
        addInfo.push("traveler");
      }
      if (this.fromRedZone) {
        addInfo.push("from_red_zone");
      }
      if (this.hasSymptoms) {
        addInfo.push("has_symptoms");
      }

      var payload = {
        full_name: name,
        age: parseInt(age),
        residence_address: address,
        gender: addGender,
        coming_from: addComeFrom,

        notes: notes,
        status: this.addStatus,
        add_info: addInfo
      };

      if (this.adminMode) {
        var village = this.villageName,
          district = this.districtName;

        if (village != null && village != "") {
          payload["village_name"] = village;
        }
        if (district != null && district != "") {
          payload["district_name"] = district;
        }
        if (this.selectedVillage["village_id"] != null) {
          payload["village_id"] = parseInt(this.selectedVillage["village_id"]);
        }
      }

      let arrivalDate = this.date.replace(/T.+$/, "").trim();

      if (arrivalDate != "") {
        payload["arrival_date"] = arrivalDate;
      }

      this.$pandemia
        .api()
        .publicApi.post(`/pandemia/v1/sub_report/add`, payload)
        .then(resp => {
          this.isLoading = false;
          if (resp.data.code == 0) {
            this.showSuccess("Berhasil menambahkan data");
            this.refreshTable();
            this.$modal.hide("AddData");
          } else {
            this.showError(resp.data.description);
          }
        })
        .catch(_ => {
          this.isLoading = false;
        });
    },
    onDataUpdate(updateMethod) {
      var name = this.$refs["nameInput"].value,
        address = this.$refs["addAddressInput"].value,
        age = this.$refs["addAgeInput"].value,
        addGender = this.$refs["addGender"].value,
        addComeFrom = this.$refs["addComeFromInput"].value,
        // arrivalDate = this.date,
        notes = this.$refs["addNotesInput"].value;

      var addInfo = [`update_method=${updateMethod}`];

      var payload = {
        full_name: name,
        age: parseInt(age),
        residence_address: address,
        gender: addGender,
        coming_from: addComeFrom,
        notes: notes,
        status: this.addStatus,
        add_info: addInfo
      };

      if (this.adminMode) {
        // var village = this.$refs["villageInput"].value,
        //   district = this.$refs["districtInput"].value;

        var village = this.villageName,
          district = this.districtName;

        payload["village_name"] = village;
        payload["district_name"] = district;
      }

      let arrivalDate = this.date.replace(/T.+$/, "").trim();

      if (arrivalDate != "") {
        payload["arrival_date"] = arrivalDate;
      }

      if (this.editMode) {
        payload["id"] = this.toEdit["id"];
      }

      this.$pandemia
        .api()
        .publicApi.post(`/pandemia/v1/sub_report/update`, payload)
        .then(resp => {
          if (resp.data.code == 0) {
            this.showSuccess("Berhasil memperbaharui data");
            this.refreshTable();
            this.$modal.hide("AddData");
          } else {
            this.showError(resp.data.description);
          }
        });
    },
    onAddDataOpened() {
      this.$refs["nameInput"].focus();

      if (this.toEdit != null) {
        this.query = this.toEdit.reporter_village;
        this.villageName = this.toEdit.reporter_village;
        this.queryDistrict = this.toEdit.reporter_district;
        this.districtName = this.toEdit.reporter_district;
        this.$refs["nameInput"].value = this.toEdit.full_name;
        this.$refs["addAddressInput"].value = this.toEdit.residence_address;
        this.$refs["addAgeInput"].value = this.toEdit.age;
        this.$refs["addGender"].value = this.toEdit.gender;
        this.$refs["addComeFromInput"].value = this.toEdit.coming_from;
        this.date = this.toEdit.arrival_date;
        this.$refs["addNotesInput"].value = this.toEdit.notes;
        this.$refs["addStatus"].value = this.toEdit.status.toLowerCase();
      }
    },
    onAddDataCanceled() {
      this.$modal.hide("AddData");
    },
    statusIdNameToLabel(statusId) {
      return statusMap[statusId.toLowerCase()];
    },
    statusLabelToIdName(label) {
      return statusMapReversed[label];
    }
  }
};

function swap(json) {
  var ret = {};
  for (var key in json) {
    ret[json[key]] = key;
  }
  return ret;
}

let statusMap = {
  odp: "ODP",
  odpsp: "ODP Selesai Pemantauan",
  pdp: "PDP",
  pdps: "PDP Sembuh",
  pdpm: "PDP Meninggal",
  otg: "OTG",
  positive: "Positif",
  recovered: "Sembuh",
  death: "Meninggal"
};

let statusMapReversed = swap(statusMap);
</script>

<style lang="less">
td.dirty {
  color: orange;
}
p.sample {
  background-color: grey;
  color: white;
  padding: 10px;
}

ul {
  width: 100%;
  color: rgba(30, 39, 46, 1);
  list-style: none;
  margin: 0;
  padding: 0.5rem 0 0.5rem 0;
}
li {
  margin: 0 0 0 0;
  border-radius: 5px;
  padding: 0.75rem 0 0.75rem 0.75rem;
  display: flex;
  align-items: center;
}
li:hover {
  cursor: pointer;
}

.autosuggest-container,
.autosuggest__results {
  position: absolute;
  justify-content: center;
  width: 400px;
  background-color: white;
  border: 1px solid #cacaca;
}

#autosuggest {
  width: 100%;
  display: block;
}
.autosuggest__results-item--highlighted {
  background-color: rgba(51, 217, 178, 0.2);
}
</style>