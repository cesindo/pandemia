<template>
  <div>
    
    <AnsTable
      :key="tableLogs"
      data-source-url="/pandemia/v1/journal/search"
      :columns="['ID', 'Time', 'Activity']"
      :searchable="true"
      :withActionButton="false"
      :mapItemFunc="dataMapper"
    />

  </div>
</template>

<script>
import AnsTable from "@/components/AnsTable.vue";
// import DialogModal from "@/components/modal/DialogModal.vue";
// import ConfirmDialog from "@/components/modal/ConfirmDialog.vue";

export default {
  name: "Users",
  components: {
    AnsTable,
    // DialogModal,
    // ConfirmDialog
  },
  data() {
    return {
      tableLogs: "-0"
    };
  },
  methods: {
    onOpened() {},
    txItemMap(item) {
      return item;
    },
    dataMapper(item) {
      return {
        "id": item["id"],
        "time": item["ts"],
        "activity": item["activity"],
      };
    },
    
    refreshTable() {
      this.tableLogs = "A-" + new Date().getTime();
    }
  }
};
</script>
