<template>
  <div class="home">

    <SatgasLogin v-if="!loggedIn" title="Pandemia" desc="Satgas Login" />

    <div v-if="loggedIn" style="float: left;">
      <sidebar-menu
        :menu="menu"
        @collapse="onCollapse"
        @item-click="onItemClick"
        :collapsed="true"
        :disableHover="true"
        style="z-index: 1000;"
        
      >
        <!-- <div slot="header"></div> -->
      </sidebar-menu>
    </div>

    <div v-if="loggedIn" class="dashboard-inner" v-bind:style="customMargin">
      <h1>{{ pageTitle }} - desa {{ villageName }}</h1>

      <div v-if="currentPage['/dashboard']">
        <div class="ui placeholder segment center aligned">
          <div class="ui header">Selamat datang di pusat kontrol Pandemia</div>
        </div>
      </div>

      <SubReports v-if="currentPage['/satgas/data']" :addable="true" />

      <ReportNotes
        v-if="currentPage['/satgas/reports']"
      />

    </div>

    <notifications group="default" position="top center" classes="vue-notification" />
  </div>
</template>

<script>
// @ is an alias to /src
// import AnsTable from "@/components/AnsTable.vue";
// import Records from "@/views/Records.vue";
// import ReportNotes from "@/views/ReportNotes.vue";
import SubReports from "@/views/SubReports.vue";
import SatgasLogin from "@/components/SatgasLogin.vue";

export default {
  name: "Dashboard",
  components: {
    SatgasLogin,
    SubReports
  },
  data() {
    return {
      collapsed: true,
      customMargin: {},
      currentPage: {},
      pageTitle: this.pageTitle,
      userId: this.$session.get("user_id"),
      userAccesses: this.$session.get("user_accesses"),
      menu: [
        {
          header: true,
          title: "Main Navigation"
        },
        {
          title: "Data",
          icon: "fa fa-address-card",
          href: "/satgas/data",
          access: "records"
        },
        {
          title: "Rumah Sakit",
          icon: "fa fa-hotel",
          href: "/satgas/hospital",
          access: "hospital"
        },
        {
          title: "Lapor",
          icon: "fa fa-comment-dots",
          href: "/satgas/reports",
          access: "report_notes"
        },
        {
          title: "Logout",
          icon: "fa fa-sign-out-alt",
          access: "*"
        }
      ]
    };
  },
  computed: {
    loggedIn(){
      return this.$session.get('user_id') != null;
    },
    villageName(){
      return this.$session.get('user_village');
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
    // startLoginChecker() {
    //   var self = this;
    //   this.loginCheckerIval = setInterval(() => {
    //     this.$pandemia.isLoggedIn(loggedIn => {
    //       if (!loggedIn) {
    //         self.$router.replace("/");
    //       }
    //     });
    //   }, 6000);
    // },
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
</style>

