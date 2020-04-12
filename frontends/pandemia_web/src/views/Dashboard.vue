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

      <Users v-if="currentPage['/dashboard/users'] && currentUserId == 1" />

      <Records v-if="currentPage['/dashboard/records']" />

      <UserDetail v-if="$route.path.startsWith('/dashboard/users/')" :userId="$route.params.id" />
    </div>

    <notifications group="default" position="top center" classes="vue-notification" />
  </div>
</template>

<script>
// @ is an alias to /src
// import AnsTable from "@/components/AnsTable.vue";
import UserDetail from "@/components/UserDetail.vue";
import Records from "@/views/Records.vue";
import Users from "@/views/Users.vue";

export default {
  name: "Dashboard",
  components: {
    // AnsTable,
    UserDetail,
    Records,
    Users
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
          title: "Log/Journal",
          icon: "fa fa-book",
          href: "/dashboard/log"
        },
        {
          title: "Logout",
          icon: "fa fa-sign-out-alt"
        }
      ]
    };
  },
  computed: {
    menu: function(){
      if (this.currentUserId != 1){
        return this.menu_items.filter(a => !a.adminOnly);
      }else{
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
    startLoginChecker() {
      var self = this;
      this.loginCheckerIval = setInterval(() => {
        this.$pandemia.isLoggedIn(loggedIn => {
          if (!loggedIn) {
            self.$router.replace("/");
          }
        });
      }, 6000);
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

      this.menu = [        {
        title: "Logout",
        icon: "fa fa-sign-out-alt"
      }];
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

