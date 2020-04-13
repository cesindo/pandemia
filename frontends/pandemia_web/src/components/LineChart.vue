<script>
import { Line, mixins } from 'vue-chartjs';
const { reactiveProp } = mixins;


// function getRandomInt(min = 1, max = 10000) {
//   var min = Math.ceil(min),
//     max = Math.floor(max);
//   return Math.floor(Math.random() * (max - min + 1)) + min;
// }


export default {
  extends: Line,
  mixins: [reactiveProp],
  props: ['options'],
  mounted () {
    // this.chartData = {
    //     labels: [getRandomInt(), getRandomInt()],
    //     datasets: [
    //       {
    //         label: "ODP",
    //         backgroundColor: "blue",
    //         data: [getRandomInt(), getRandomInt()]
    //       },
    //       {
    //         label: "PDP",
    //         backgroundColor: "yellow",
    //         data: [getRandomInt(), getRandomInt()]
    //       },
    //       {
    //         label: "Positif",
    //         backgroundColor: "red",
    //         data: [getRandomInt(), getRandomInt()]
    //       },
    //       {
    //         label: "Sembuh",
    //         backgroundColor: "green",
    //         data: [getRandomInt(), getRandomInt()]
    //       },
    //       {
    //         label: "Meninggal",
    //         backgroundColor: "black",
    //         data: [getRandomInt(), getRandomInt()]
    //       }
    //     ]
    //   };
    this.renderChart(this.chartData, this.options)
  }
}
</script>

<style>
  .small {
    max-width: 600px;
    margin:  150px auto;
  }
</style>