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

      <h2>Wonosobo / Kalisuren</h2>

      <div class="ui grid">
        <div class="four wide column">
          <AnsTable
            :key="tableUsers"
            data-source-url="/analytic/v1/area?province=jawa-tengah&city=wonosobo"
            :columns="['Desa', 'ODP', 'PDP', 'Pos']"
            :searchable="true"
            :withActionButton="true"
            :mapItemFunc="userListAllMapper"
            :showDetailFunc="showDetail"
          />
        </div>
        <div class="eight wide column">dua</div>
      </div>
    </div>
  </div>
</template>

<script>
// @ is an alias to /src
// import AnsTable from "@/components/AnsTable.vue";
// import UserDetail from "@/components/UserDetail.vue";

export default {
  name: "Dashboard",
  components: {
    // AnsTable,
    // UserDetail,
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

    this.startLoginChecker();
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
</style>

