<template>
  <div id="UserDetail">
    <AnsTab>
      <div class="ui grid">
        <div class="six wide column">
          <table class="ui celled table">
            <tbody>
              <tr>
                <td data-label="ID">ID:</td>
                <td class="value">{{d.id}}</td>
              </tr>
              <tr>
                <td data-label="Name">Full name:</td>
                <td class="value">{{d.full_name}}</td>
              </tr>
              <tr>
                <td data-label="Email">Email:</td>
                <td class="value">{{d.email}}</td>
              </tr>
              <tr>
                <td data-label="Phone">Phone:</td>
                <td class="value">{{d.phone_num}}</td>
              </tr>
              <tr>
                <td data-label="Active">Active:</td>
                <td class="value">{{d.active ? "YES" : "NO"}}</td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>
    </AnsTab>
  </div>
</template>

<script>
import AnsTab from "@/components/AnsTab";

export default {
  name: "UserDetail",
  components: {
    AnsTab
  },
  props: {
    userId: String
  },
  data() {
    return {
      d: {}
    };
  },
  created() {
    if (!this.userId) return;
    this.$pandemia
      .api()
      .privateApi.get(`/user/v1/user/info?id=${this.userId}`)
      .then(resp => {
        this.d = resp.data.result;
      });
  },
  methods: {
  }
};
</script>

<style lang="less" scoped>
.value {
  font-weight: bold;
}
tr td:first-child {
  text-align: right !important;
}
</style>

