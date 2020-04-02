<template>
  <Modal
    :modalName="modalName"
    :clickToClose="clickToClose"
    @beforeOpen="beforeOpen"
    @beforeClose="beforeClose"
    @opened="opened"
  >
    <div class="ui basic center aligned segment">
      <slot name="content">
        <h2 class="ui header">{{ caption }}</h2>
      </slot>
    </div>
    <div class="ui basic center aligned segment">
      <button class="ui button" @click="onReject">{{ buttonsText.reject }}</button>
      <button class="ui primary button" @click="onApprove">{{ buttonsText.approve }}</button>
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
    buttonsText: {
      type: Object,
      default: function() {
        return {
          reject: "No",
          approve: "Yes"
        };
      }
    }
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
    opened(){
      this.$emit("opened");
    }
  }
};
</script>


