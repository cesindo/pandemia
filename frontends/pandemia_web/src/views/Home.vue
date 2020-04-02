<template>
  <div class="home">
    <Login title="Pandemia Control Center" desc="Track and monitor Pandemic Disease" />
  </div>
</template>

<script>
// @ is an alias to /src
import Login from '@/components/Login.vue'

export default {
  name: 'home',
  components: {
    Login
  }
}
</script>

