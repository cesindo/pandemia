<template>
  <div class="home">
    <div class="ui two column stackable grid">
      <div class="column">
        <div class="ui form">
          <div class="field">
            <label>Kode Daerah:</label>
            <div>
              <strong class="area-code">{{areaCode}}</strong>
            </div>
            <p>Kode area ini digunakan oleh satgas untuk mendaftar menjadi penginput data di lapangan.</p>
            <button
              :disabled="isLoading"
              class="ui button icon"
              @click="updateAreaCode"
            >Ganti Kode Area</button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  name: "AreaSettings",
  components: {},
  props: {},
  data() {
    return { areaCode: "loading...", isLoading: false };
  },
  computed: {
    cityId() {
      return this.$session.get("admin").city_id;
    }
  },
  mounted() {
    // let city_id = this.$session.get("admin").city_id;
    this.isLoading = true;
    this.$pandemia
      .api()
      .publicApi.get(`/auth/v1/get_area_code?id=${this.cityId}`)
      .then(resp => {
        this.isLoading = false;
        if (resp.data.code == 0) {
          this.areaCode = resp.data.result;
        } else {
          this.showError(resp.data.description);
        }
      });
  },
  methods: {
    updateAreaCode() {
      this.isLoading = true;
      this.$pandemia
        .api()
        .publicApi.post(`/auth/v1/reset_area_code`, {
          id: this.cityId,
          area_code: this.areaCode
        })
        .then(resp => {
          this.isLoading = false;
          if (resp.data.code == 0) {
            this.areaCode = resp.data.result;
          } else {
            this.showError(resp.data.description);
          }
        });
    }
  }
};
</script>


<style lang="less" scoped>
.area-code {
  font-weight: bold;
  font-size: 30px;
}
</style>
