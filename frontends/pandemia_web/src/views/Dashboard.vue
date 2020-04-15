<template>
  <div class="home">
    <div style="float: left;">
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
    </div>

    <div class="dashboard-inner" v-bind:style="customMargin">
      <h1>{{ pageTitle }}</h1>

      <div v-if="currentPage['/dashboard']">
        <div class="ui placeholder segment center aligned">
          <div class="ui header">Selamat datang di pusat kontrol Pandemia</div>
        </div>
      </div>

      <Admins
        v-if="currentPage['/dashboard/admins'] && (userId == 1 || userAccesses.indexOf('admins') > -1 )"
      />
      <Users
        v-if="currentPage['/dashboard/users'] && (userId == 1 || userAccesses.indexOf('users') > -1 )"
      />

      <Records v-if="currentPage['/dashboard/records']" />

      <AdminDetail
        baseApiUrl="/admin/v1/detail"
        v-if="$route.path.startsWith('/dashboard/admins/')"
        :userId="$route.params.id"
      />
      <UserDetail
        baseApiUrl="/user/v1/detail"
        v-if="$route.path.startsWith('/dashboard/users/')"
        :userId="$route.params.id"
      />

      <Villages v-if="currentPage['/dashboard/villages']" />

      <SubReports v-if="currentPage['/dashboard/data'] && userAccesses.indexOf('data') > -1" />
      <ReportNotes v-if="currentPage['/dashboard/reports'] && userAccesses.indexOf('report_notes') > -1" />
      

      <Logs v-if="currentPage['/dashboard/journal']" />
    </div>

    <notifications group="default" position="top center" classes="vue-notification" />
  </div>
</template>

<script>
// @ is an alias to /src
// import AnsTable from "@/components/AnsTable.vue";
import UserDetail from "@/components/UserDetail.vue";
import AdminDetail from "@/components/AdminDetail.vue";
import Records from "@/views/Records.vue";
import Users from "@/views/Users.vue";
import Admins from "@/views/Admins.vue";
import Villages from "@/views/Villages.vue";
import Logs from "@/views/Logs.vue";
import ReportNotes from "@/views/ReportNotes.vue";
import SubReports from "@/views/SubReports.vue";

export default {
  name: "Dashboard",
  components: {
    // AnsTable,
    UserDetail,
    AdminDetail,
    Records,
    Users,
    Admins,
    Villages,
    Logs,
    ReportNotes,
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
          href: "/dashboard/records",
          access: "records"
        },
        {
          title: "Satgas",
          icon: "fa fa-hiking",
          href: "/dashboard/satgas",
          access: "satgas"
        },
        {
          title: "Hospital",
          icon: "fa fa-hotel",
          href: "/dashboard/hospital",
          access: "hospital"
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
          title: "Data",
          icon: "fa fa-file-alt",
          href: "/dashboard/data",
          access: "data"
        },
        {
          title: "Laporan Satgas",
          icon: "fa fa-comment-dots",
          href: "/dashboard/reports",
          access: "report_notes"
        },
        {
          title: "Log/Journal",
          icon: "fa fa-book",
          href: "/dashboard/journal"
        },
        {
          title: "Logout",
          icon: "fa fa-sign-out-alt",
          access: '*'
        }
      ]
    };
  },
  computed: {
    menu: function() {
      if (this.userId != 1) {
        return this.menu_items.filter(a => {
          return this.userAccesses.indexOf(a.access) > -1 || a.access == '*';
        });
      } else {
        return this.menu_items;
      }
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

