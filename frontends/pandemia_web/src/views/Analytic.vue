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

      <h1 class="ui header">
        <img class="ui image pandemia-logo" src="/img/logo-64.png" alt="pandemia logo" />
        <div class="content">Kabupaten Wonosobo - COVID-19 Control Center</div>
      </h1>

      <!-- <div class="ui divider"></div> -->

      <div class="ui center aligned stackable celled grid">
        <div class="row">
          <div class="four wide column">
            <h3>
              <i class="icon fa-bell"></i> Terbaru:
            </h3>

            <div class="ui divided list feeds" style="text-align: left !important;">
              <div
                v-for="(item, idx) in feeds"
                v-bind:key="item.id"
                class="item"
                :id="'Item-' + idx"
              >
                <div class="content">
                  <h4 class="ui sub header">{{item.location}}:</h4>
                  <div class="ui feed">
                    <div class="event">
                      <div class="content">
                        <div class="summary">{{item.notes}}</div>
                      </div>
                    </div>
                  </div>
                  <div class="meta">
                    <span>{{item.ts | moment("from")}}</span>
                  </div>
                </div>
              </div>
            </div>

            <center>
              <a href="/lihatsemua">Lihat Semua</a>
            </center>

            <div class="ui divider"></div>

            <div class="four wide column">
              <h2>Per Desa</h2>
              <table class="ui celled table village-data">
                <thead>
                  <tr>
                    <th>Desa</th>
                    <th>O</th>
                    <th>P</th>
                    <th>C</th>
                    <th>S</th>
                    <th class="death">M</th>
                  </tr>
                </thead>
                <tbody>
                  <tr v-for="item in village_data" v-bind:key="item.id">
                    <td class="village-name">{{item.village_name}}</td>
                    <td>{{item.odp}}</td>
                    <td>{{item.pdp}}</td>
                    <td class="positive">{{item.cases}}</td>
                    <td class="recover">{{item.recovered}}</td>
                    <td class="death">{{item.deaths}}</td>
                  </tr>
                </tbody>
              </table>
              <center>
                <a href="/lihatsemua">Lihat Semua</a>
              </center>
            </div>
          </div>

          <div class="seven wide column">
            <div class="ui center aligned grid">
              <div class="thirteen wide column">
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
              </div>
            </div>

            <div class="ui center aligned grid">
              <div class="fourteen wide column">
                <div class="map-view">
                  <div
                    v-for="m in mapMarkers"
                    v-bind:key="m.id"
                    class="marker"
                    :id=" 'M' + m.district_name"
                  >
                    <div v-if="m.cases > 0" class="ui red label">{{m.cases}}</div>
                    <div v-if="m.cases == 0" class="ui label">{{m.cases}}</div>
                  </div>
                </div>
              </div>
            </div>
          </div>

          <div class="five wide column">
            <h1>Per Kecamatan</h1>
          </div>
        </div>

        <!-- <div class="row">
          <div class="eight wide column">
            <line-chart :chart-data="dataCollection" :responsive="true" width="700px"></line-chart>
          </div>
        </div>-->
      </div>
    </div>
  </div>
</template>

<script>
// @ is an alias to /src
// import AnsTable from "@/components/AnsTable.vue";
// import UserDetail from "@/components/UserDetail.vue";
// import LineChart from "@/components/LineChart.vue";

export default {
  name: "Dashboard",
  components: {
    // AnsTable,
    // UserDetail,
    // LineChart
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
      total_deaths: 0,
      mapMarkers: [
        {
          name: "Kejajar",
          odp: 2,
          pdp: 3,
          positive: 2
        },
        {
          name: "Kepil",
          odp: 2,
          pdp: 3,
          positive: 1
        },
        {
          name: "Sapuran",
          odp: 2,
          pdp: 3,
          positive: 4
        },
        {
          name: "Kaliwiro",
          odp: 2,
          pdp: 3,
          positive: 3
        },
        {
          name: "Wadaslintang",
          odp: 2,
          pdp: 3,
          positive: 0
        }
      ]
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
            console.log(
              "Gagal mendapatkan data total. e: " + resp.data.description
            );
          }
        });

      this.loadMapMarkers();
    },
    loadMapMarkers() {
      this.$pandemia
        .api()
        .publicApi.get(
          `/analytic/v1/district_data?province=${this.province}&city=${this.city}&offset=0&limit=30`
        )
        .then(resp => {
          if (resp.data.code == 0) {
            this.mapMarkers = resp.data.result.entries;
          } else {
            this.showError(resp.data.description);
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
.pandemia-logo {
  padding: 5px !important;
  margin-left: 10px !important;
  height: 50px !important;
  width: 50px !important;
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
  width: 520px;
  height: 600px;
  background-image: url("/img/peta-wonosobo.png");
  background-repeat: no-repeat;
}

.feeds {
  .header {
    font-size: 1em;
    color: #1349d1 !important;
    font-style: italic;
    padding: 0 !important;
    margin: 0 !important;
  }
  .feed {
    padding: 7px 0;
    margin: 0 !important;
  }
  .meta {
    text-align: right;
    font-style: italic;
  }
}
div.map-view {
  position: relative;
}
.marker {
  font-family: Verdana, Arial;
  font-weight: bold;
  background-color: transparent;
  color: white;
  width: 150px;
  height: 85px;
  // background-color: transparent;
  position: absolute !important;
  // -webkit-text-fill-color: white; /* Will override color (regardless of order) */
  // -webkit-text-stroke-width: 1px;
  // -webkit-text-stroke-color: black;
  .label {
    font-size: 15px !important;
    border: 2px solid orange !important;
  }
}

.marker#MKejajar {
  top: 10px;
  right: 40px;

  &:hover {
    background-color: white;
    // opacity: 50%;
  }
}

.marker#MKepil {
  bottom: 50px;
  right: 5px;
}

.marker#MSapuran {
  bottom: 140px;
  right: 90px;
}

.marker#MKaliwiro {
  bottom: 130px;
  left: 90px;
}

.marker#MWadaslintang {
  bottom: 10px;
  left: 30px;
}

.marker#MGarung {
  top: 80px;
  right: 20px;
}
.marker#MMojotengah {
  top: 40px;
  left: 60px;
}
.marker#MWatumalang {
  top: 100px;
  left: 10px;
}
.marker#MSukoharjo {
  top: 180px;
  left: 0px;
}
.marker#MSukoharjo {
  top: 180px;
  left: 0px;
}
.marker#MLeksono {
  top: 310px;
  left: 25px;
}
.marker#MKepil {
  top: 470px;
  right: 0px;
}
.marker#MKalikajar {
  top: 200px;
  right: -5px;
}
.marker#MKertek {
  top: 130px;
  right: -5px;
}
.marker#MSelomerto {
  top: 270px;
  right: 180px;
}
.marker#MWonosobo {
  top: 200px;
  right: 175px;
}
.marker#MKalibawang {
  top: 482px;
  right: 181px;
}

th,
td {
  text-align: center !important;
}

table.village-data {
  td.village-name {
    font-weight: bold !important;
    text-align: left !important;
  }

  th.death,
  td.death {
    color: grey !important;
    background-color: #ebe8e8 !important;
  }

  th.positive,
  td.positive {
    color: #c9160c !important;
    background-color: #ffdede !important;
  }

  th.recover,
  td.recover {
    color: #13870b !important;
    background-color: #c7f7c3 !important;
  }
}
</style>

