<template>
  <div id="app">
    <router-view :key="$route.path"/>
    <notifications group="auth" position="top center" classes="vue-notification" />
    <notifications group="alert" position="top center" classes="vue-notification"/>
  </div>
</template>

<style lang="less">

@import url('assets/less/main.less');

</style>

