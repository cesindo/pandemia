<template>
  <div id="app">
    <div id="nav">
      <!-- <router-link to="/">Home</router-link> |
      <router-link to="/about">About</router-link> -->
    </div>
    <router-view :key="$route.path"/>
    <notifications group="auth" position="top center" classes="vue-notification" />
    <notifications group="alert" position="top center" classes="vue-notification"/>
  </div>
</template>

<style lang="less">

@import url('assets/less/main.less');

</style>

