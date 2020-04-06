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

      <div v-if="currentPage['/dashboard']">
        <div class="ui placeholder segment center aligned">
          <div class="ui header">
            Selamat datang di pusat kontrol Pandemia
          </div>
        </div>
      </div>

      <AnsTable
        v-if="currentPage['/dashboard/users']"
        data-source-url="/user/v1/users"
        :columns="['ID', 'Name', 'Email', 'Phone', 'Active', 'Register']"
        :searchable="true"
        :withActionButton="true"
        :mapItemFunc="userListAllMapper"
        :showDetailFunc="showDetail"
      />

      <Records v-if="currentPage['/dashboard/records']" />

      <UserDetail v-if="$route.path.startsWith('/dashboard/users/')" :userId="$route.params.id" />
    </div>

    <notifications group="default" position="top center" classes="vue-notification" />
  </div>
</template>

<script>
// @ is an alias to /src
import AnsTable from "@/components/AnsTable.vue";
import UserDetail from "@/components/UserDetail.vue";
import Records from "@/views/Records.vue";

export default {
  name: "Dashboard",
  components: {
    AnsTable,
    UserDetail,
    Records
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
          icon: "fa fa-list"
        },
        {
          title: "Users",
          icon: "fa fa-users",
          href: "/dashboard/users"
        },
        {
          title: "Records",
          icon: "fa fa-address-card",
          href: "/dashboard/records"
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
    publicApiScope(self) {
      return self.$pandemia.api().publicApi;
    },
    privateApiScope(self) {
      return self.$pandemia.api().privateApi;
    },
    showDetail(item) {
      this.$router.push("/dashboard/users/" + item.id);
    },
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
      if (item.title == "Logout") {
        this.$dialog
          .confirm("Are you sure to logout?")
          .then(_dialog => {
            this.$pandemia.unauthorize();
          })
          .catch(() => {});
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

