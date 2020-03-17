<template>
  <div>
    <div class="ui grid">
      <div class="ten wide column">
        <div v-if="searchable" class="ui icon input">
          <input type="text" placeholder="Search..." v-on:keyup.13="doSearch" ref="inputSearch">
          <i class="search icon"></i>
        </div>

        <table class="ui celled table">
          <thead>
            <tr>
              <th v-for="col in columns" v-bind:key="col">{{col}}</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="item in items" v-bind:key="item.id">
              <td v-for="td in item" v-bind:key="td">{{td}}</td>
              <td><button v-on:click="showDetail(item)">detail</button></td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  </div>
</template>

<script>
const initialState = {
  items: []
};
export default {
  name: "AnsTable",
  props: {
    dataSourceUrl: String,
    columns: Array,
    searchable: Boolean,
    withActionButton: Boolean,
    mapItemFunc: Function
  },
  data() {
    return initialState;
  },
  methods: {
    doSearch() {
      var url =
        this.dataSourceUrl +
        `?query=${this.$refs.inputSearch.value}&page=${this.page}&limit=${this.limit}`;
      this.$pandemia
        .api()
        .privateApi.get(url)
        .then(resp => {
          this.items = resp.data.result.entries.map(this.mapItemFunc);
        });
    },
    showDetail(item){
      this.$router.push("/dashboard/users/" + item.id);
    }
  },
  created() {
    this.items = [];
    this.page = 0;
    this.limit = 5;
    var self = this;
    var url;

    if (this.searchable && this.query) {
      url = this.dataSourceUrl + "?q=" + this.query + "&page=0&limit=10";
    } else {
      url = this.dataSourceUrl + "?page=0&limit=10";
    }

    if (this.withActionButton){
      this.columns.push('Action');
    }

    this.$pandemia
      .api()
      .privateApi.get(url)
      .then(resp => {
        self.items = resp.data.result.entries.map(this.mapItemFunc);
      });
  }

};
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped lang="less">
</style>

