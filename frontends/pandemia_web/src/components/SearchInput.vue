<template>
  <div class="ui icon input" :class="{loading:isProcessing}">
    <input type="text" placeholder="Search..." @keyup="search" v-model="searchText">
    <i class="search icon"></i>
    <div class="results" v-show="searchText.length > 0 && !isProcessing && !hideResults" :class="{upward:direction == 'up'}">
      <div class="item" v-for="(item, idx) in items" :key="idx" @click="doSelect(item)"> {{ item.text }}</div>
      <div v-if="items.length == 0 && !isProcessing && !hideResults" class="ui basic center aligned segment"><em><b>Empty result</b></em></div>
    </div>
  </div>
</template>

<script>
import debounce from 'lodash/debounce'

export default {
  data() {
    return {
      searchText:'',
      isProcessing:false,
      entries: [],
      hideResults: true
    }
  },
  props:{
    url: {
      type:String,
      required:true
    },
    mapItemFunc: {
      type:Function,
      default: (x) => {return x} 
    },
    direction: {
      type:String,
      default:'bottom'
    }
  },
   methods: {
    search() {
      this.hideResults = true
      this.doSearch()
    },
    doSearch: debounce(function () {
      if (!this.isProcessing && this.searchText.length > 0) {
        this.isProcessing = true
        this.$pandemia.api().publicApi.get(this.url.replace(/{{query}}/g, this.searchText))
        .then(resp => {
          const data = resp.data
          if (data.code == 0) {
            this.entries = data.result.entries
          }
        })
        .then(_ => {
          this.isProcessing = false
          this.hideResults = false
        })
      }
    }, 300),
    doSelect(val) {
      this.hideResults = true
      this.searchText = val.text
      this.$emit('onSelected', val)
    },
  },
  computed: {
    items() {
      return this.entries.map(this.mapItemFunc)
    },
  }
}
</script>


<style lang="less" scoped>
.results {
    position: absolute;
    top: 100%;
    left: 0;
    transform-origin: center top;
    white-space: normal;
    text-align: left;
    text-transform: none;
    background: #fff;
    margin-top: .5em;
    border-radius: .28571429rem;
    box-shadow: 0 2px 4px 0 rgba(34,36,38,.12), 0 2px 10px 0 rgba(34,36,38,.15);
    border: 1px solid #d4d4d5;
    z-index: 998;

    &.upward {
      top:unset;
      bottom: 100%;
      margin-bottom: .5em;
    }

    .ui.segment {
      width: 250px
    }

    > .item {
      padding: 10px;
      cursor: pointer;
      border-bottom: 1px solid @grey-color;

      &:hover {
        background-color: @grey-color;
      }
    }
}
</style>

