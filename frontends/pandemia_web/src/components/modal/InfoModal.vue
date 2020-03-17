<template>
  <Modal :modalName="modalName" :clickToClose="clickToClose">
    <div class="ui basic center aligned segment">
      <p>{{ textContent }}</p>
    </div>
    <div class="ui basic center aligned segment">
      <button class="ui primary button">{{ buttonsText }}</button>
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
      type: Boolean,
      default: false
    },
    textContent: String,
    buttonsText: {
      type: String,
      default: 'Close'
    }
  }
}
</script>


