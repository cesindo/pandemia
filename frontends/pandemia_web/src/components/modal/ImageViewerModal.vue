<template>
  <modal :name="modalName" width="100%" :clickToClose="clickToClose" :scrollable="true" :adaptive="true" height="auto" @before-open="beforeOpen" @before-close="beforeClose">
    <div class="size-modal-content">
    <div class="ui fullscreen basic modal transition visible active scrolling">
      <i class="close icon" @click="closeModal()" v-if="withCloseButton"></i>
      <div class="header" v-if="headerTitle">{{ headerTitle }}</div>
      <div class="content" :class="{'without-header': !headerTitle && withCloseButton}">
        <slot name="content-wrapper">   
          <div class="ui center aligned container" style="padding:2rem 1rem">
            <div class="ui basic centered segment">
            <slot name="content">
              <img class="ui centered image" :src="imageUrl" alt="image">
            </slot>
            </div>
            <slot name="confirm-button"><button class="ui big inverted green button" @click="closeModal()">Close</button></slot>
          </div>
        </slot>
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
      type: String,
    },
    withCloseButton: {
      type:Boolean,
      default: false
    },
    clickToClose: {
      type:Boolean,
      default: false
    },
    imageUrl: {
      type: String,
      required: true
    }
  },
  methods: {
    closeModal() {
      this.$modal.hide(this.modalName)
    },
    beforeOpen() {
      this.$emit('beforeOpen')
    },
    beforeClose() {
      this.$emit('beforeClose')
    },
  }
};
</script>

<style lang="less" scoped>
  .ui.centered.image {
    max-width: 100%;
  }
</style>


