<template>
  <div>

    <button class="ui button" @click="openModal('basic-modal')">Open Modal</button>
    <Modal modalName="basic-modal" :withCloseButton="true">
      <template v-slot:default="{closeModal}">
        <h1 class="ui header">This is Basic Modal</h1>
        <button class="ui button" @click="closeModal()">Close</button>
      </template>
    </Modal>

    <button class="ui button" @click="openModal('small-modal')">Open Small Modal</button>
    <SmallModal modalName="small-modal" :withCloseButton="true">
      <template v-slot:default="{closeModal}">
        <h1 class="ui header">This is Small Modal</h1>
        <button class="ui button" @click="closeModal()">Close</button>
      </template>
    </SmallModal>

    <button class="ui button" @click="openModal('full-modal')">Open FullScreen Modal</button>
    <FullscreenModal modalName="full-modal" :withCloseButton="true">
      <template v-slot:default="{closeModal}">
        <h1 class="ui header">This is FullScreen Modal</h1>
        <button class="ui button" @click="closeModal()">Close</button>
      </template>
    </FullscreenModal>

    <button class="ui button" @click="openModal('dialog-modal')">Open Dialog Modal</button>
    <DialogModal textContent="Ini Adalah Dialog Modal" modalName="dialog-modal" :withCloseButton="true" @onApprove="approveDialog" />

    <button class="ui button" @click="openModal('image-modal')">Open Image Modal</button>
    <ImageViewerModal imageUrl="https://cdn.pixabay.com/photo/2019/07/17/09/49/susten-4343542_960_720.jpg" modalName="image-modal" />

  </div>
</template>

<script>
import Modal from '@/components/modal/BasicModal.vue'
import SmallModal from '@/components/modal/BasicSmallModal.vue'
import FullscreenModal from '@/components/modal/BasicFullscreenModal.vue'
import DialogModal from '@/components/modal/DialogModal.vue'
import ImageViewerModal from '@/components/modal/ImageViewerModal.vue'

export default {
  components: {
    Modal,
    SmallModal,
    FullscreenModal,
    DialogModal,
    ImageViewerModal,
  },
  methods: {
    openModal(modalName) {
      this.$modal.show(modalName)
    },
    approveDialog() {
      // do API call here...


      this.$modal.hide('dialog-modal')
    }
  },
}
</script>


