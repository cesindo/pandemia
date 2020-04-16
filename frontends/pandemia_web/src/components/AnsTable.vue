<template>
  <div>
    <div class="ui grid">
      <div class="ten wide column">
        <div v-if="searchable" class="ui icon input">
          <input
            type="text"
            placeholder="Search..."
            v-on:keyup.13="doSearch"
            ref="inputSearch"
            v-on:keyup="checkHasText"
          />

          <a href="javascript://" v-show="hasText" @click="resetSearch" class="search reset"><i class="search remove icon" ></i></a>

          <i v-if="!hasText" class="search icon"></i>
        </div>

        <slot name="bar">
          <div class="ui mini statistic">
            <div class="value">{{items.length}}</div>
            <div class="label">Total</div>
          </div>
        </slot>

        <div>
          <slot name="bellow-search"></slot>
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
                <td v-if="withActionButton">
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
  items: [],
  count: 0,
  hasText: false
};
export default {
  name: "AnsTable",
  props: {
    dataSourceUrl: String,
    addParams: String,
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
  // computed: {
  //   hasText() {
  //     return (
  //       this.$refs.inputSearch != null &&
  //       this.$refs.inputSearch.value.length > 0
  //     );
  //   }
  // },
  mounted() {},
  methods: {
    checkHasText() {
      this.hasText = this.$refs.inputSearch.value.length > 0;
    },
    resetSearch() {
      this.$refs.inputSearch.value = "";
      this.hasText = "";
      this.doSearch();
      this.$refs.inputSearch.focus();
    },
    doSearch() {
      var url =
        this.dataSourceUrl +
        `?query=${this.$refs.inputSearch.value}&offset=${this.offset}&limit=${this.limit}`;

      if (this.addParams != null) {
        url = url + "&" + this.addParams;
      }
      this.apiScopeBuilder(this)
        .get(url)
        .then(resp => {
          if (resp.data.code == 0) {
            this.items = resp.data.result.entries.map(this.mapItemFunc);
            this.count = resp.data.result.count;
          } else {
            this.showError("Gagal mendapatkan data dari server");
          }
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

    if (this.addParams != null) {
      url = url + "&" + this.addParams;
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
.mini.statistic {
  margin-left: 10px !important;
}
a.search.reset {
  font-size: 0.857143em;
  float: left;
  margin: 0;
  padding: 0;
  right: 1em;
  top: 0.7em;
  z-index: 1000 !important;
  position: absolute;
  opacity: 0.5;
  cursor: pointer;
}
</style>

