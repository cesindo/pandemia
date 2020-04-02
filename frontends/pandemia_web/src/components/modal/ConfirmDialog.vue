<template>
  <Modal
    :modalName="modalName"
    :clickToClose="clickToClose"
    rejectText="Batal"
    approveText="Oke"
    @beforeOpen="beforeOpen"
    @beforeClose="beforeClose"
    @opened="opened"
  >
    <div class="ui basic center aligned segment">
      <slot name="content">
        <h2 class="ui header">{{ caption }}</h2>
      </slot>
      <slot name="default">
        <p>{{content}}</p>
      </slot>
    </div>
    <div class="ui basic center aligned segment">
      <button class="ui button" @click="onReject">{{ rejectText }}</button>
      <button class="ui primary button" @click="onApprove">{{ approveText }}</button>
    </div>
  </Modal>
</template>

<script>
import Modal from "@/components/modal/BasicSmallModal";

export default {
  components: {
    Modal
  },
  props: {
    modalName: {
      type: String,
      required: true
    },
    clickToClose: {
      type: Boolean,
      default: false
    },
    caption: String,
    content: String,
    approveText: {type: String, default: "Oke"},
    rejectText: {type: String, default: "Batal"},
  },
  methods: {
    closeModal() {
      this.$modal.hide(this.modalName);
    },
    onReject() {
      this.$emit("onReject");
      this.closeModal();
    },
    onApprove() {
      this.$emit("onApprove");
    },
    beforeOpen(event) {
      this.$emit("beforeOpen", event);
    },
    beforeClose() {
      this.$emit("beforeClose");
    },
    opened() {
      this.$emit("opened");
    }
  }
};
</script>


