
export default class Notif {
  static install(Vue) {
    Vue.prototype.$notif = {
      showAlert(msg = "data", title = "Info", type = "success", group = "alert") {
        this.$notify({
          group: group,
          title: title,
          type: type,
          text: msg
        });
      },
      showError(msg = "An error occured", title = "Error", group = "alert") {
        this.showAlert(msg, title, 'error', group)
      },
      showSuccess(msg = "", title = "Success", group = "alert") {
        this.showAlert(msg, title, 'success', group)
      },
      showWarning(msg = "", title = "Warning", group = "alert") {
        this.showAlert(msg, title, 'warning', group)
      },
      showErrorApi(msg = "An error occured when fetching data") {
        this.showError(msg)
      },
    }
  }

}



