<template>
  <div class="home">
    <!-- <div style="float: left;">
      <sidebar-menu
        :menu="menu"
        @collapse="onCollapse"
        @item-click="onItemClick"
        :collapsed="true"
        :disableHover="true"
        style="z-index: 1000;"
      >
        <div slot="header"></div>
      </sidebar-menu>
    </div>-->

    <div class="analytic-inner">
      <!-- <h1>{{ pageTitle }}</h1> -->

      <h1>Kabupaten Wonosobo - COVID-19 Control Center</h1>

      <!-- <div class="ui divider"></div> -->

      <div class="ui internally stackable celled grid">
        <div class="row">
          <div class="four wide column">
            <h3>
              <i class="icon fa-bell"></i> Terbaru:
            </h3>

            <div class="ui divided list">
              <div
                v-for="(item, idx) in feeds"
                v-bind:key="item.id"
                class="item"
                :id="'Item-' + idx"
              >
                <div class="content">
                  <h4 class="ui sub header">{{item.location}}</h4>
                  <div class="ui feed">
                    <div class="event">
                      <div class="content">
                        <div class="summary">{{item.notes}}</div>
                      </div>
                    </div>
                  </div>
                  <div class="meta">
                    <span>{{item.ts}}</span>
                  </div>
                </div>
              </div>
            </div>

            <center>
              <a href="/lihatsemua">Lihat Semua</a>
            </center>
          </div>

          <div class="eight wide column">
            <div class="ui statistics">
              <div class="blue statistic">
                <div class="value">{{total_odp}}</div>
                <div class="label">ODP</div>
              </div>
              <div class="orange statistic">
                <div class="value">{{total_pdp}}</div>
                <div class="label">PDP</div>
              </div>
              <div class="red statistic">
                <div class="value">{{total_cases}}</div>
                <div class="label">COVID-19</div>
              </div>
              <div class="green statistic">
                <div class="value">{{total_recovered}}</div>
                <div class="label">Sembuh</div>
              </div>
              <div class="grey statistic">
                <div class="value">{{total_deaths}}</div>
                <div class="label">Meninggal</div>
              </div>
            </div>
            
            
            <div>
              <div class="map-view"></div>
            </div>

            
          </div>
        </div>

        <div class="row">
          <div class="four wide column">
            <table class="ui celled table">
              <thead>
                <tr>
                  <th>Desa</th>
                  <th>ODP</th>
                  <th>PDP</th>
                  <th>Pos</th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="item in village_data" v-bind:key="item.id">
                  <td>{{item.village_name}}</td>
                  <td>{{item.odp}}</td>
                  <td>{{item.pdp}}</td>
                  <td>{{item.cases}}</td>
                </tr>
              </tbody>
            </table>
            <center>
              <a href="/lihatsemua">Lihat Semua</a>
            </center>
          </div>
          <div class="eight wide column">
            <line-chart :chart-data="dataCollection" :responsive="true" width="700px"></line-chart>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
// @ is an alias to /src
// import AnsTable from "@/components/AnsTable.vue";
// import UserDetail from "@/components/UserDetail.vue";
import LineChart from "@/components/LineChart.vue";

export default {
  name: "Dashboard",
  components: {
    // AnsTable,
    // UserDetail,
    LineChart
  },
  props: ["province", "city"],
  data() {
    return {
      currentUserId: this.$session.get("user_id"),
      feeds: [],
      village_data: [],
      total_odp: 0,
      total_pdp: 0,
      total_cases: 0,
      total_recovered: 0,
      total_deaths: 0
    };
  },
  computed: {},
  created() {
    // this.customMargin = {
    //   left: "70px",
    //   position: "absolute"
    // };
    this.currentPage = {};
    this.$set(this.currentPage, this.$route.path, true);
    this.pageTitle = this.$router.history.current.name;

    // this.startLoginChecker();
  },
  destroyed() {
    clearInterval(this.loginCheckerIval);
  },
  methods: {
    publicApiScope(self) {
      return self.$pandemia.api().publicApi;
    },
    privateApiScope(self) {
      return self.$pandemia.api().privateApi;
    },
    isCurrentPage(title) {
      return this.currentPage == title;
    },
    onCollapse(state) {
      this.collapsed = state;
      this.customMargin = {
        left: this.collapsed ? "70px" : "370px",
        position: "absolute"
      };
    },
    onItemClick(_event, item) {
      console.log(_event);
      if (item.title == "Logout") {
        this.$dialog
          .confirm("Are you sure to logout?")
          .then(_dialog => {
            this.$pandemia.unauthorize();
            this.$router.replace("/");
          })
          .catch(() => {});
      }
    },
    loadData() {
      this.$pandemia
        .api()
        .publicApi.get(
          `/analytic/v1/area?province=${this.province}&city=${this.city}&offset=0&limit=5`
        )
        .then(resp => {
          if (resp.data.code == 0) {
            this.village_data = resp.data.result.entries;
          }
        });

      this.$pandemia
        .api()
        .publicApi.get(
          `/analytic/v1/report_notes?province=${this.province}&city=${this.city}&offset=0&limit=5`
        )
        .then(resp => {
          if (resp.data.code == 0) {
            this.feeds = resp.data.result.entries;
          }
        });

      this.$pandemia
        .api()
        .publicApi.get(
          `/analytic/v1/total?province=${this.province}&city=${this.city}`
        )
        .then(resp => {
          if (resp.data.code == 0) {
            var d = resp.data.result;
            this.total_odp = d.odp;
            this.total_pdp = d.pdp;
            this.total_cases = d.cases;
            this.total_recovered = d.recovered;
            this.total_deaths = d.deaths;
          } else {
            console.log("Gagal mendapatkan data total. e: " + resp.data.description);
          }
        });
    }
  },
  mounted() {
    this.loadData();
  }
};
</script>


<style lang="less" scoped>
h1 {
  padding-left: 10px;
}
h2 {
  padding: 0 !important;
  margin: 0 !important;
  font-weight: 100;
}

.dashboard-inner {
  width: 100%;
  transition: all 0.1s ease-in-out;
  -webkit-transition: all 0.1s ease-in-out; /** Chrome & Safari **/
  -moz-transition: all 0.1s ease-in-out; /** Firefox **/
  -o-transition: all 0.1s ease-in-out; /** Opera **/
}
.v-sidebar-menu .vsm--header {
  display: none;
}
.card .content {
  text-align: left;
}
div.map-view {
  width: 100%;
  height: 1000px;
  background-image: url("/img/peta-wonosobo.jpg");
  background-repeat: no-repeat;
}
</style>

