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
        <img
          class="ui image pandemia-logo mobile hidden"
          src="/img/logo-64.png"
          alt="pandemia logo"
        />
        <div id="MaintTitle" class="content">Kabupaten Wonosobo - COVID-19 Information Center</div>
      </h1>

      <!-- <div class="ui divider"></div> -->

      <div class="ui center aligned stackable celled grid">
        <div class="row">
          <div class="four wide column">
            <h3>
              <i class="icon fa-bell"></i> Kabar Terbaru:
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

            <div class="four wide mobile hidden column">
              <h2>Data Per Desa</h2>
              <table class="ui celled unstackable table village-data">
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
              <!-- <center>
                <a href="/lihatsemua">Lihat Semua</a>
              </center>-->
            </div>
          </div>

          <div class="eight wide column">
            <div class="ui center aligned grid">
              <div class="nine wide column">
                <div class="ui statistics">
                  <table class="statistics" style="border: 1px solid #cacaca;">
                    <tr>
                      <td>
                        <div class="blue statistic">
                          <div class="value stat-num">{{total_odp}}</div>
                          <div class="label">ODP</div>
                        </div>
                      </td>
                      <td>
                        <div class="orange statistic">
                          <div class="value stat-num">{{total_pdp}}</div>
                          <div class="label">PDP</div>
                        </div>
                      </td>
                      <td>
                        <div class="red statistic">
                          <div class="value stat-num">{{total_cases}}</div>
                          <div class="label">COVID19</div>
                        </div>
                      </td>
                      <td>
                        <div class="green statistic">
                          <div class="value stat-num">{{total_recovered}}</div>
                          <div class="label">Sembuh</div>
                        </div>
                      </td>
                      <td>
                        <div class="grey statistic">
                          <div class="value stat-num">{{total_deaths}}</div>
                          <div class="label">Meninggal</div>
                        </div>
                      </td>
                    </tr>
                  </table>
                </div>
              </div>
            </div>

            <div class="ui center aligned grid">
              <div class="sixteen wide column">
                <center>
                  <div class="map-view">
                    <div
                      v-for="m in districtData"
                      v-bind:key="m.id"
                      class="marker"
                      :id=" 'M' + m.district_name"
                    >
                      <div v-if="m.cases > 0" class="ui red circular label">{{m.cases}}</div>
                      <div v-if="m.cases == 0" class="ui circular label">{{m.cases}}</div>
                    </div>
                  </div>
                </center>
              </div>
            </div>
          </div>

          <div class="four wide column">
            <div class="four wide column">
              <h2>Data Per Kecamatan</h2>
              <table class="ui celled unstackable table district-data">
                <thead>
                  <tr>
                    <th>Kecamatan</th>
                    <th>O</th>
                    <th>P</th>
                    <th>C</th>
                    <th>S</th>
                    <th class="death">M</th>
                  </tr>
                </thead>
                <tbody>
                  <tr v-for="item in districtData" v-bind:key="item.id">
                    <td class="district-name">{{item.district_name}}</td>
                    <td>{{item.odp}}</td>
                    <td>{{item.pdp}}</td>
                    <td class="positive">{{item.cases}}</td>
                    <td class="recover">{{item.recovered}}</td>
                    <td class="death">{{item.deaths}}</td>
                  </tr>
                </tbody>
              </table>
            </div>
          </div>
        </div>

        <div class="row mobile only">
          <div class="four wide mobile only column">
            <h2>Data Per Desa</h2>
            <table class="ui celled unstackable table village-data">
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
            <!-- <center>
              <a href="/lihatsemua">Lihat Semua</a>
            </center>-->
          </div>
        </div>
      </div>

      <div class="ui stackable three column grid">
        <div class="row">
          <div class="column">
            <highcharts :options="allDataPipeChart"></highcharts>
          </div>
          <div class="column">
            <highcharts :options="generalTrendData"></highcharts>
          </div>
          <div class="column">
            <highcharts :options="travelerTrendData"></highcharts>
          </div>
        </div>
      </div>

      <!-- <area-chart
        id="area"
        :data="areaData"
        xkey="year"
        ykeys="["value"]"
        line-colors="["#30b7ff", "#ff974d", "#FF6384", "#4ba63f", "#DEDEDE"]"
        grid="true"
        resize="true"
      ></area-chart>-->
    </div>
  </div>
</template>

<script>
// @ is an alias to /src
// import AnsTable from "@/components/AnsTable.vue";
// import UserDetail from "@/components/UserDetail.vue";
// import LineChart from "@/components/LineChart.vue";

import Raphael from "raphael/raphael";
global.Raphael = Raphael;
// import { /*DonutChart, BarChart, LineChart,*/ AreaChart } from "vue-morris";
// import { Chart } from "highcharts-vue";

export default {
  name: "Dashboard",
  components: {
    // AnsTable,
    // UserDetail,
    // LineChart
    // AreaChart,
    // Chart
  },
  props: ["province", "city"],
  data() {
    return {
      generalTrendData: {
        chart: {
          type: "line"
        },
        title: {
          text: "TREND"
        },
        // subtitle: {
        //   text: "Source: WorldClimate.com"
        // },
        xAxis: {
          categories: [
            "Jan",
            "Feb",
            "Mar",
            "Apr",
            "May",
            "Jun",
            "Jul",
            "Aug",
            "Sep",
            "Oct",
            "Nov",
            "Dec"
          ]
        },
        yAxis: {
          title: {
            text: "Jumlah"
          }
        },
        plotOptions: {
          line: {
            dataLabels: {
              enabled: true
            },
            enableMouseTracking: true
          }
        },
        colors: ["#166c91", "#f2711c", "#db2828", "#21ba45", "#767676"],
        series: [
          {
            name: "ODP",
            data: [
              7.0,
              6.9,
              9.5,
              14.5,
              18.4,
              21.5,
              25.2,
              26.5,
              23.3,
              18.3,
              13.9,
              9.6
            ]
          },
          {
            name: "PDP",
            data: [
              3.9,
              4.2,
              5.7,
              8.5,
              11.9,
              15.2,
              17.0,
              16.6,
              14.2,
              10.3,
              6.6,
              4.8
            ]
          },
          {
            name: "SEMBUH",
            data: [
              2.9,
              3.2,
              5.7,
              8.5,
              1.9,
              20.2,
              19.0,
              40.6,
              30.2,
              31.3,
              90.6,
              100.8
            ]
          }
        ]
      },
      travelerTrendData: {
        chart: {
          type: "line"
        },
        title: {
          text: "TREND PELAKU PERJALANAN"
        },
        // subtitle: {
        //   text: "Source: WorldClimate.com"
        // },
        colors: ["#db2828", "#4d9ba3", "#f2711c"],
        xAxis: {
          categories: [
            "Jan",
            "Feb",
            "Mar",
            "Apr",
            "May",
            "Jun",
            "Jul",
            "Aug",
            "Sep",
            "Oct",
            "Nov",
            "Dec"
          ]
        },
        yAxis: {
          title: {
            text: "Jumlah"
          }
        },
        plotOptions: {
          line: {
            dataLabels: {
              enabled: true
            },
            enableMouseTracking: true
          }
        },
        series: [
          {
            name: "Pelaku Perjalanan Dari Wilayah Terjangkit",
            data: [
              7.0,
              6.9,
              9.5,
              14.5,
              18.4,
              21.5,
              25.2,
              26.5,
              23.3,
              18.3,
              13.9,
              9.6
            ]
          },
          {
            name: "ODP",
            data: [
              3.9,
              4.2,
              5.7,
              8.5,
              11.9,
              15.2,
              17.0,
              16.6,
              14.2,
              10.3,
              6.6,
              4.8
            ]
          },
          {
            name: "Pelaku Perjalanan Dari Wilayah Terjangkit Tanpa Gejala",
            data: [
              2.9,
              3.2,
              5.7,
              8.5,
              1.9,
              20.2,
              19.0,
              40.6,
              30.2,
              31.3,
              90.6,
              100.8
            ]
          }
        ]
      },
      allDataPipeChart: {
        chart: {
          plotBackgroundColor: null,
          plotBorderWidth: null,
          plotShadow: false,
          type: "pie"
        },
        title: {
          text: "COVID19 - Wonosobo"
        },
        tooltip: {
          pointFormat:
            "{series.name}: ({point.y}) <b>{point.percentage:.1f}%</b>"
        },
        colors: [/*"#166c91",*/ "#f2711c", "#db2828", "#21ba45", "#767676"],
        accessibility: {
          point: {
            valueSuffix: "%"
          }
        },
        plotOptions: {
          pie: {
            allowPointSelect: true,
            cursor: "pointer",
            dataLabels: {
              enabled: true,
              format: "<b>{point.name}</b>: {point.percentage:.1f} %"
            }
          }
        },
        series: [
          {
            name: "Data",
            colorByPoint: true,
            data: [
              {
                name: "ODP",
                y: 0,
                sliced: true,
                selected: true
              },
              {
                name: "PDP",
                y: 0
              },
              {
                name: "COVID19",
                y: 0
              },
              {
                name: "SEMBUH",
                y: 0
              },
              {
                name: "MENINGGAL",
                y: 0
              }
            ]
          }
        ]
      },
      areaData: [
        { year: "2008", value: 20 },
        { year: "2009", value: 10 },
        { year: "2010", value: 5 },
        { year: "2011", value: 5 },
        { year: "2012", value: 20 }
      ],
      currentUserId: this.$session.get("user_id"),
      feeds: [],
      village_data: [],
      total_odp: 0,
      total_pdp: 0,
      total_cases: 0,
      total_recovered: 0,
      total_deaths: 0,
      districtData: [
        // {
        //   name: "Kejajar",
        //   odp: 2,
        //   pdp: 3,
        //   positive: 2
        // },
        // {
        //   name: "Kepil",
        //   odp: 2,
        //   pdp: 3,
        //   positive: 1
        // },
        // {
        //   name: "Sapuran",
        //   odp: 2,
        //   pdp: 3,
        //   positive: 4
        // },
        // {
        //   name: "Kaliwiro",
        //   odp: 2,
        //   pdp: 3,
        //   positive: 3
        // },
        // {
        //   name: "Wadaslintang",
        //   odp: 2,
        //   pdp: 3,
        //   positive: 0
        // }
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
      // console.log(_event);
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
          `/analytic/v1/area?province=${this.province}&city=${this.city}&offset=0&limit=10`
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

            this.$set(this.allDataPipeChart, "series", [
              {
                name: "Data",
                colorByPoint: true,
                data: [
                  // {
                  //   name: "ODP",
                  //   y: d.odp,
                  //   sliced: true,
                  //   selected: true
                  // },
                  {
                    name: "PDP",
                    y: d.pdp
                  },
                  {
                    name: "COVID19",
                    y: d.cases
                  },
                  {
                    name: "SEMBUH",
                    y: d.recovered
                  },
                  {
                    name: "MENINGGAL",
                    y: d.deaths
                  }
                ]
              }
            ]);
          } else {
            // console.log(
            //   "Gagal mendapatkan data total. e: " + resp.data.description
            // );
          }
        });

      this.$pandemia
        .api()
        .publicApi.get(
          `/analytic/v1/trend/traveler?province=${this.province}&city=${this.city}&offset=0&limit=30`
        )
        .then(resp => {
          if (resp.data.code == 0) {
            // console.log(resp.data.result);
            let ent = resp.data.result;
            // console.log(ent);

            this.$set(this.travelerTrendData.xAxis, "categories", ent.cats.map((a) => a.replace(/-/g, '/')));
            this.$set(this.travelerTrendData, "series", ent.series);
          } else {
            this.showError(resp.data.description);
          }
        });

      this.$pandemia
        .api()
        .publicApi.get(
          `/analytic/v1/trend/general?province=${this.province}&city=${this.city}&offset=0&limit=30`
        )
        .then(resp => {
          if (resp.data.code == 0) {
            // console.log(resp.data.result);
            let ent = resp.data.result;
            // console.log(ent);

            this.$set(this.generalTrendData.xAxis, "categories", ent.cats.map((a) => a.replace(/-/g, '/')));
            this.$set(this.generalTrendData, "series", ent.series);
          } else {
            this.showError(resp.data.description);
          }
        });

      this.loaddistrictData();
    },
    loaddistrictData() {
      this.$pandemia
        .api()
        .publicApi.get(
          `/analytic/v1/district_data?province=${this.province}&city=${this.city}&offset=0&limit=30`
        )
        .then(resp => {
          if (resp.data.code == 0) {
            this.districtData = resp.data.result.entries;
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
  width: 620px;
  height: 700px;
  background-image: url("/img/peta-wonosobo.jpeg");
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
  top: 20px;
  right: 120px;

  // &:hover {
  //   background-color: white;
  //   // opacity: 50%;
  // }
}

.marker#MSapuran {
  bottom: 170px;
  right: 150px;
}

.marker#MKaliwiro {
  bottom: 150px;
  left: 145px;
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
  top: 320px;
  left: 30px;
}
.marker#MLeksono {
  top: 310px;
  left: 135px;
}
.marker#MKepil {
  bottom: 140px;
  right: 70px;
}
.marker#MKalikajar {
  top: 280px;
  right: 70px;
}
.marker#MKertek {
  top: 220px;
  right: 130px;
}
.marker#MSelomerto {
  top: 335px;
  right: 280px;
}
.marker#MWonosobo {
  top: 250px;
  right: 255px;
}
.marker#MKalibawang {
  bottom: 50px;
  right: 250px;
}

th,
td {
  text-align: center !important;
}

table.village-data,
table.district-data {
  td.village-name,
  td.district-name {
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

#app div.stat-num {
  font-size: 2rem !important;
}
table.statistics tr td {
  padding: 0 10px;
}

@media only screen and (max-width: 767px) {
  [class*="mobile hidden"],
  [class*="tablet only"]:not(.mobile),
  [class*="computer only"]:not(.mobile),
  [class*="large monitor only"]:not(.mobile),
  [class*="widescreen monitor only"]:not(.mobile),
  [class*="or lower hidden"] {
    display: none !important;
  }

  #MaintTitle {
    font-size: 0.6em !important;
    text-align: center !important;
    display: block;
  }
}
</style>

