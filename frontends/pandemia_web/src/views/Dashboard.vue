<template>
  <div class="home">
    <div style="float: left;">
      <sidebar-menu
        :menu="menu"
        @collapse="onCollapse"
        @itemClick="onItemClick"
        :collapsed="true"
        style="z-index: 1000;"
      />
    </div>

    <div class="dashboard-inner" v-bind:style="customMargin">
      <h1>{{ pageTitle }}</h1>

      <AnsTable
        v-if="currentPage['/dashboard']"
        data-source-url="/user/v1/users"
        :columns="['ID', 'Name', 'Email']"
        :searchable="true"
        :withActionButton="true"
        :mapItemFunc="userListAllMapper2"
      ></AnsTable>

      <AnsTable
        v-if="currentPage['/dashboard/users']"
        data-source-url="/user/v1/users"
        :columns="['ID', 'Name', 'Email', 'Phone', 'Active', 'Register']"
        :searchable="true"
        :withActionButton="true"
        :mapItemFunc="userListAllMapper"
      />

      <UserDetail v-if="$route.path.startsWith('/dashboard/users/')" :accountId="$route.params.id"/>
    </div>

    <notifications group="default" position="top center" classes="vue-notification" />
  </div>
</template>

<script>
// @ is an alias to /src
import AnsTable from "@/components/AnsTable.vue";
import UserDetail from "@/components/UserDetail.vue";

export default {
  name: "Dashboard",
  components: {
    AnsTable,
    UserDetail
  },
  data() {
    return {
      collapsed: true,
      customMargin: {},
      currentPage: {},
      pageTitle: this.pageTitle,
      menu: [
        {
          header: true,
          title: "Main Navigation"
        },
        {
          href: "/dashboard",
          title: "Dashboard",
          icon: "fa fa-user"
        },
        {
          title: "Users",
          icon: "fa fa-users",
          href: "/dashboard/users"
        },
        {
          title: "Logout",
          icon: "fa fa-sign-out-alt"
        }
      ]
    };
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
    txItemMap(item) {
      return item;
    },
    userListAllMapper(item) {
      return item;
    },
    userListAllMapper2(item) {
      return {
        id: item["id"],
        name: item["full_name"],
        email: item["email"]
      };
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
      }, 3000);
    },
    onCollapse(state) {
      this.collapsed = state;
      this.customMargin = {
        left: this.collapsed ? "70px" : "370px",
        position: "absolute"
      };
    },
    onItemClick(_event, item) {
      if (item.title == 'Logout'){
        this.$dialog.confirm("Are you sure to logout?")
          .then((_dialog) => {
            this.$pandemia.unauthorize();
          })
          .catch(()=>{});
      }
    }
  }
};
</script>


<style lang="less" scoped>
.dashboard-inner {
  width: 100%;
  transition: all 0.1s ease-in-out;
  -webkit-transition: all 0.1s ease-in-out; /** Chrome & Safari **/
  -moz-transition: all 0.1s ease-in-out; /** Firefox **/
  -o-transition: all 0.1s ease-in-out; /** Opera **/
}
</style>

