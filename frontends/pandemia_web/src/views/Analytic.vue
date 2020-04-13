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

    <div class="dashboard-inner" v-bind:style="customMargin">
      <!-- <h1>{{ pageTitle }}</h1> -->

      <h1>Kabupaten Wonosobo - COVID-19 Control Center</h1>

      <!-- <div class="ui divider"></div> -->

      <div class="ui internally celled grid">
        <div class="row">
          <div class="four wide column">
            <!-- <AnsTable
            :key="tableUsers"
            data-source-url="/analytic/v1/area?province=jawa-tengah&city=wonosobo"
            :columns="['Desa', 'ODP', 'PDP', 'Pos']"
            :searchable="true"
            :withActionButton="true"
            :mapItemFunc="userListAllMapper"
            :showDetailFunc="showDetail"
            />-->

            <h3>
              <i class="icon fa-bell"></i> Terbaru:
            </h3>

            <div class="ui relaxed divided list">
              <div class="ui item">
                <div class="content">
                  <h4 class="ui sub header">Ngalian</h4>

                  <div class="ui feed">
                    <div class="event">
                      <div class="content">
                        <div class="summary">
                          <strong>3</strong> pemudik baru masuk dari Jakarta
                        </div>
                      </div>
                    </div>
                  </div>
                  <div class="meta">
                    <span>3 menit yang lalu</span>
                  </div>
                </div>
              </div>

              <div class="ui item">
                <div class="content">
                  <h4 class="ui sub header">Sariyoso</h4>

                  <div class="ui feed">
                    <div class="event">
                      <div class="content">
                        <div class="summary">
                          <strong>1</strong> orang dirujuk ke RSUD Setjonegoro
                        </div>
                      </div>
                    </div>
                  </div>
                  <div class="meta">
                    <span>3 menit yang lalu</span>
                  </div>
                </div>
              </div>

              <div class="ui item">
                <div class="content">
                  <h4 class="ui sub header">Tawangsari</h4>

                  <div class="ui feed">
                    <div class="event">
                      <div class="content">
                        <div class="summary">
                          <strong>10</strong> pemudik baru masuk dari Bandung
                        </div>
                      </div>
                    </div>
                  </div>
                  <div class="meta">
                    <span>5 menit yang lalu</span>
                  </div>
                </div>
              </div>

              <div class="ui item">
                <div class="content">
                  <h4 class="ui sub header">Kuripan</h4>

                  <div class="ui feed">
                    <div class="event">
                      <div class="content">
                        <div class="summary">
                          <strong>1</strong> orang PDP sembuh
                        </div>
                      </div>
                    </div>
                  </div>
                  <div class="meta">
                    <span>10 menit yang lalu</span>
                  </div>
                </div>
              </div>
            </div>
            <center>
              <a href="/lihatsemua">Lihat Semua</a>
            </center>
          </div>
          <div class="eight wide column">
            <iframe
              width="100%"
              height="95%"
              frameborder="0"
              scrolling="no"
              marginheight="0"
              marginwidth="0"
              src="https://www.openstreetmap.org/export/embed.html?bbox=109.83718872070314%2C-7.423070318382198%2C109.98138427734375%2C-7.302025398075166&amp;layer=mapnik"
              style="border: 1px solid black"
            ></iframe>
            <br />
            <small>
              <a href="https://www.openstreetmap.org/#map=13/-7.3626/109.9093">View Larger Map</a>
            </small>
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
                <tr>
                  <td>Kuripan</td>
                  <td>30</td>
                  <td>3</td>
                  <td>0</td>
                </tr>
                <tr>
                  <td>Kalisuren</td>
                  <td>23</td>
                  <td>3</td>
                  <td>0</td>
                </tr>
                <tr>
                  <td>Reco</td>
                  <td>20</td>
                  <td>4</td>
                  <td>0</td>
                </tr>
                <tr>
                  <td>Tambi</td>
                  <td>18</td>
                  <td>3</td>
                  <td>0</td>
                </tr>
                <tr>
                  <td>Ngalian</td>
                  <td>15</td>
                  <td>2</td>
                  <td>0</td>
                </tr>
                <tr>
                  <td>Bumiroso</td>
                  <td>10</td>
                  <td>1</td>
                  <td>0</td>
                </tr>
                <tr>
                  <td>Tawangsari</td>
                  <td>10</td>
                  <td>3</td>
                  <td>0</td>
                </tr>
                <tr>
                  <td>Tawangsari</td>
                  <td>12</td>
                  <td>3</td>
                  <td>0</td>
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

// function getRandomInt(min = 1, max = 10000) {
//   var min = Math.ceil(min),
//     max = Math.floor(max);
//   return Math.floor(Math.random() * (max - min + 1)) + min;
// }

export default {
  name: "Dashboard",
  components: {
    // AnsTable,
    // UserDetail,
    LineChart
  },
  data() {
    return {
      collapsed: true,
      customMargin: {},
      currentPage: {},
      pageTitle: this.pageTitle,
      currentUserId: this.$session.get("user_id"),
      menu_items: [
        {
          header: true,
          title: "Main Navigation"
        },
        {
          href: "/dashboard",
          title: "Dashboard",
          icon: "fa fa-list"
        },
        {
          title: "Admins",
          icon: "fa fa-user-cog",
          href: "/dashboard/admins",
          adminOnly: true
        },
        {
          title: "Users",
          icon: "fa fa-users",
          href: "/dashboard/users",
          adminOnly: true
        },
        {
          title: "Records",
          icon: "fa fa-address-card",
          href: "/dashboard/records"
        },
        {
          title: "Satgas",
          icon: "fa fa-hiking",
          href: "/dashboard/satgas"
        },
        {
          title: "Hospital",
          icon: "fa fa-hotel",
          href: "/dashboard/hospital"
        },
        {
          title: "Map",
          icon: "fa fa-globe-asia",
          href: "/dashboard/map"
        },
        {
          title: "Villages",
          icon: "fa fa-campground",
          href: "/dashboard/villages"
        },
        {
          title: "Log/Journal",
          icon: "fa fa-book",
          href: "/dashboard/journal"
        },
        {
          title: "Logout",
          icon: "fa fa-sign-out-alt"
        }
      ]
    };
  },
  computed: {
    menu: function() {
      if (this.currentUserId != 1) {
        return this.menu_items.filter(a => !a.adminOnly);
      } else {
        return this.menu_items;
      }
    },
    dataCollection: function() {
      return {
        labels: ["9 Apr", "10 Apr", "11 Apr", "12 Apr", "13 Apr", "14 Apr"],
        datasets: [
          {
            label: "Meninggal",
            backgroundColor: "black",
            data: [0, 0, 0, 0, 0, 0]
          },
          {
            label: "Sembuh",
            backgroundColor: "green",
            data: [0, 0, 0, 1, 1, 1]
          },
          {
            label: "Positive",
            backgroundColor: "red",
            data: [1, 1, 1, 4, 4, 4]
          },
          {
            label: "PDP",
            backgroundColor: "orange",
            data: [3, 10, 22, 27, 30, 35]
          },
          {
            label: "ODP",
            backgroundColor: "yellow",
            data: [3, 10, 33, 60, 70, 100]
          }
        ]
      };
    }
  },
  created() {
    this.customMargin = {
      left: "70px",
      position: "absolute"
    };
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
    mounted() {
      var menu = [
        {
          header: true,
          title: "Main Navigation"
        },
        {
          href: "/dashboard",
          title: "Dashboard",
          icon: "fa fa-list"
        }
      ];

      if (this.$session.get("user_id") == 1) {
        menu.push({
          title: "Users",
          icon: "fa fa-users",
          href: "/dashboard/users"
        });
      }

      menu.push({
        title: "Records",
        icon: "fa fa-address-card",
        href: "/dashboard/records"
      });
      menu.push({
        title: "Logout",
        icon: "fa fa-sign-out-alt"
      });

      this.menu = [
        {
          title: "Logout",
          icon: "fa fa-sign-out-alt"
        }
      ];
    }
  }
};
</script>


<style lang="less">
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
</style>

