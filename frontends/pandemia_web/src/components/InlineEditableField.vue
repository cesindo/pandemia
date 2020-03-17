<template>
  <div>
    <div class="content-wrapper" v-if="!editing" >
      <slot :toggleEdit="toggleEdit"></slot>
      <a href="javascript:void(0)" @click="toggleEdit" title="Edit"><i class="pencil icon"></i></a>
    </div>
    <div class="ui form" v-if="editing">
      <div class="field">
        <slot name="input" :inputValue="inputValue" :onInputChange="onInputChange" :onKeyPress="onKeyPress">
          <input type="text" :value="inputValue" @input="onInputChange" @keyup="onKeyPress">
        </slot>
      </div>
      <slot name="save">
        <div class="field">
          <div class="ui small button" @click="cancel()">Cancel</div>
          <div class="ui primary small button" @click="save()">Save</div>
        </div>
      </slot>
    </div>
  </div>
</template>

<script>
export default {
  data() {
    return {
      editing:false,
    }
  },
  props:{
    inputValue: {
      type:String
    }
  },
  methods: {
    toggleEdit() {
      this.editing = !this.editing
      this.$emit('onToggle')
    },
    save() {
      this.$emit('onSave')
    },
    cancel() {
      this.editing = false
      this.$emit('onCancel')
    },
    close() {
      this.editing = false
    },
    onInputChange(ev) {
      this.$emit('update:inputValue', ev.target.value)
    },
    onKeyPress(ev){
      this.$emit("onKeyPress", ev)
    }
  },
  created() {
    this.initValue = this.inputValue
  },
}
</script>

<style lang="less" scoped>
  .content-wrapper {
    position: relative;
    padding-right: 28px;
  }
</style>

