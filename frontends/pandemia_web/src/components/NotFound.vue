<template>
<div class="ui center aligned grid container">
  <div class="fourteen wide column">
    <img src="/img/404.png" alt="image" class="ui centered image" />
  </div>
  <div class="six wide column">
    <h1 class="ui header">Oops!</h1>
    <p>{{ textContent }}</p>    
    <slot></slot>
  </div>
</div>
</template>

<script>
export default {
  props: {
    textContent: {
      type: String,
      default: 'Halaman yang Kamu cari tidak ditemukan.'
    },
  }
}
</script>

<style lang="less" scoped>
  p {
    font-size: 18px;
  }
</style>

