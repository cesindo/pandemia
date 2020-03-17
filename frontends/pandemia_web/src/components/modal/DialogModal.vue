<template>
  <Modal :modalName="modalName" :clickToClose="clickToClose" @before-open="beforeOpen" @before-close="beforeClose">
    <div class="ui basic center aligned segment">
      <h2 class="ui header">{{ textContent }}</h2>
    </div>
    <div class="ui basic center aligned segment">
      <button class="ui button" @click="onReject">{{ buttonsText.reject }}</button>
      <button class="ui primary button" @click="onApprove">{{ buttonsText.approve }}</button>
    </div> 
  </Modal>  
</template>

<script>
import Modal from '@/components/modal/BasicSmallModal'

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
      type:Boolean,
      default: false
    },
    textContent: String,
    buttonsText: {
      type: Object,
      default: function() {
        return {
          reject: 'No',
          approve: 'Yes'
        }
      }
    }
  },
  methods: {
    closeModal() {
      this.$modal.hide(this.modalName)
    },
    onReject() {
      this.$emit('onReject')
      this.closeModal()
    },
    onApprove() {
      this.$emit('onApprove')
    },
    beforeOpen() {
      this.$emit('beforeOpen')
    },
    beforeClose() {
      this.$emit('beforeClose')
    }
  },
}
</script>


