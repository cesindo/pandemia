<template>
  <div>
    <div class="ui grid">
      <div class="ten wide column">
        <div v-if="searchable" class="ui icon input">
          <input type="text" placeholder="Search..." v-on:keyup.13="doSearch" ref="inputSearch" />
          <i class="search icon"></i>
        </div>

        <table class="ui celled table">
          <thead>
            <tr>
              <th v-for="col in columns" v-bind:key="col">{{col}}</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="(item,a_idx) in items" v-bind:key="item.id">
              <slot name="tdmap" v-bind:item="item">
                <td v-for="(td,b_idx) in item" v-bind:key="item.id + '-' + a_idx + '-' + b_idx">
                  <div v-if="b_idx.endsWith('_raw')" v-html="td"></div>
                  <div v-if="b_idx.endsWith('_func')" v-html="td(td)"></div>
                  <div v-if="!b_idx.endsWith('_raw') && !b_idx.endsWith('_func')">{{td}}</div>
                </td>
                <td>
                  <button v-on:click="showDetail(item)">detail</button>
                </td>
              </slot>
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
    mapItemFunc: {
      type: Function,
      default: a => a
    },
    limit: { type: Number, default: 10 },
    showDetailFunc: Function,
    apiScopeBuilder: {
      type: Function,
      default: a => {
        return a.$pandemia.api().publicApi;
      }
    }
  },
  data() {
    return initialState;
  },
  methods: {
    doSearch() {
      var url =
        this.dataSourceUrl +
        `?query=${this.$refs.inputSearch.value}&offset=${this.offset}&limit=${this.limit}`;
      this.apiScopeBuilder(this)
        .get(url)
        .then(resp => {
          this.items = resp.data.result.entries.map(this.mapItemFunc);
        });
    },
    showDetail(item) {
      this.showDetailFunc(item);
    }
  },
  created() {
    this.items = [];
    this.offset = 0;
    var self = this;
    var url;

    if (this.searchable && this.query) {
      url =
        this.dataSourceUrl +
        "?q=" +
        this.query +
        `&offset=0&limit=${this.limit}`;
    } else {
      url = this.dataSourceUrl + "?offset=0&limit=10";
    }

    if (this.withActionButton) {
      this.columns.push("Action");
    }

    this.apiScopeBuilder(this)
      .get(url)
      .then(resp => {
        self.items = resp.data.result.entries.map(this.mapItemFunc);
      });
  }
};
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped lang="less">
</style>

