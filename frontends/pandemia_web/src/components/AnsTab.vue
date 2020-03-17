<template>
  <div class="ans-tab">
    <div class="ui top attached tabular menu">
      <div class="active item">Tab #1</div>
      <div class="item">Tab #2</div>
    </div>
    <div class="ui bottom active attached tab segment">
      <slot></slot>
    </div>
  </div>
</template>

<script>
export default {
  name: "AnsTab",
  props: {

  },
  created(){

  }
}
</script>
