<template>
  <div>
    <div class="ui grid right floated">
      <div class="ten wide column">
        <button class="ui text icon green button right floated" @click="commit">
          <i class="fa-angle-double-up icon"></i> Commit
        </button>
      </div>
    </div>
    <AnsTable :key="tableRecords"
      data-source-url="/pandemia/v1/search_records"
      :columns="['ID', 'Lokasi', 'Jenis', 'ODP', 'PDP', 'Positive', 'Sembuh', 'Meninggal', 'Action']"
      :searchable="true"
      :withActionButton="false"
      :showDetailFunc="showDetail"
    >
      <template v-slot:tdmap="self">
        <td>{{self.item['id']}}</td>
        <td>
          <strong>{{self.item['loc']}}</strong>
          <br />
          <small>updated: {{self.item['last_updated']}}</small>
        </td>
        <td>
          <span v-if="self.item['loc_kind'] == 0">Global</span>
          <span v-if="self.item['loc_kind'] == 1">Benua</span>
          <span v-if="self.item['loc_kind'] == 2">Negara</span>
          <span v-if="self.item['loc_kind'] == 3">Provinsi</span>
          <span v-if="self.item['loc_kind'] == 4">Kota</span>
        </td>
        <td>0</td>
        <td>0</td>
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
          <button class="ui icon button">
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

    <ConfirmDialog
      modalName="Commit"
      caption="Commit data"
      approveText="Lanjut"
      :withCloseButton="true"
      @onApprove="doCommit"
    >
      <p>Yakin untuk melakukan commit? Semua perubahan akan di-simpan ke server, pastikan koneksi internet Anda lancar.</p>
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
      tableRecords: "-0"
    };
  },
  methods: {
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
      // console.log("approved");

      // var d = Object.assign(this.commitLogs[this.editedItem["id"]] || {}, this.editedItem);

      // d = Object.assign({}, this.editedItem);

      var commitLog = this.commitLogs[this.editedItem["id"]];

      if (commitLog == undefined) {
        commitLog = Object.assign({}, this.editedItem);
      }

      commitLog[this.editedCat] = parseInt(this.$refs.newValue.value);

      // logs = this.commitLogs;

      // logs[this.editedItem["id"]] = d;

      // this.commitLogs = Object.assign(logs, this.commitLogs);
      this.$set(this.commitLogs, this.editedItem["id"], commitLog);

      this.$modal.hide("EditValueModal");
    },
    commit() {
      this.$modal.show("Commit");
    },
    doCommit() {
      this.$modal.hide("Commit");
      console.log(obMapToArrayValues(this));
      var normCommitLogs = obMapToArrayValues(this.commitLogs);
      this.$pandemia
        .api()
        .publicApi.post("/pandemia/v1/update_records", {
          records: normCommitLogs
        })
        .then(resp => {
          console.log(resp);
          this.commitLogs = {};
          this.tableRecords = 'A-' + new Date().getTime();
        });
    }
  }
};
</script>

<style lang="less">
td.dirty {
  color: orange;
}
</style>