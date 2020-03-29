<template>
  <modal
    :name="modalName"
    width="95%"
    :max-width="800"
    :min-width="320"
    :clickToClose="clickToClose"
    :scrollable="true"
    :adaptive="true"
    height="auto"
    @before-open="beforeOpen"
    @before-close="beforeClose"
    @opened="opened"
  >
    <div class="size-modal-content">
      <div class="ui fullscreen modal transition visible active">
        <i class="close icon" @click="closeModal()" v-if="withCloseButton"></i>
        <div class="header" v-if="headerTitle">{{ headerTitle }}</div>
        <div class="content" :class="{'without-header': !headerTitle && withCloseButton}">
          <slot name="content"></slot>
          <slot :closeModal="closeModal"></slot>
        </div>
      </div>
    </div>
  </modal>
</template>
<script>
export default {
  props: {
    modalName: {
      type: String,
      required: true
    },
    headerTitle: {
      type: String
    },
    withCloseButton: {
      type: Boolean,
      default: false
    },
    clickToClose: {
      type: Boolean,
      default: false
    }
  },
  methods: {
    closeModal() {
      this.$modal.hide(this.modalName);
    },
    beforeOpen() {
      this.$emit("beforeOpen");
    },
    beforeClose() {
      this.$emit("beforeClose");
    },
    opened(){
      this.$emit("opened");
    }
  }
};
</script>


