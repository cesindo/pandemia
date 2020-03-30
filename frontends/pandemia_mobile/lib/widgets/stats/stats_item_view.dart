import 'package:flutter/material.dart';
import 'package:flutter/widgets.dart';
import 'package:intl/intl.dart';
import 'package:pandemia_mobile/models/info_location.dart';
import 'package:pandemia_mobile/time_helper.dart';
import 'package:timeago/timeago.dart' as timeago;
import 'package:charts_flutter/flutter.dart' as charts;

class StatsItemView extends StatelessWidget {
  final InfoLocation item;

  StatsItemView({Key key, @required this.item}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    final numfa = new NumberFormat("#,##0", "en_US");
    return Card(
      elevation: 2.0,
      margin: new EdgeInsets.fromLTRB(16.0,16.0,16.0,0),
      child: Column(
        mainAxisAlignment: MainAxisAlignment.start,
        crossAxisAlignment: CrossAxisAlignment.start,
        children: <Widget>[
          Padding(
            padding: EdgeInsets.all(10),
            child: Row(children: <Widget>[
              Expanded(
                  child: Text(
                this.item.name.toUpperCase(),
                style: TextStyle(fontWeight: FontWeight.bold, fontSize: 20.0),
                textAlign: TextAlign.left,
              )),
              Text(
                timeago.format(
                    TimeHelper.parseAsUtc(this.item.latestRecord.lastUpdated)),
                style: TextStyle(fontSize: 15, color: Colors.grey),
              )
            ]),
          ),
          Padding(
            padding: EdgeInsets.all(10),
            child: Table(
              columnWidths: {0: FlexColumnWidth(0.7), 1: FixedColumnWidth(20)},
              children: [
                TableRow(children: [
                  Text("Total kasus"),
                  Text(":"),
                  Text(numfa.format(item.latestRecord.totalCases))
                ]),
                TableRow(children: [
                  Text("Total kematian"),
                  Text(":"),
                  Text(numfa.format(item.latestRecord.totalDeaths))
                ]),
                TableRow(children: [
                  Text("Total sembuh"),
                  Text(":"),
                  Text(numfa.format(item.latestRecord.totalRecovered))
                ])
              ],
            ),
          ),
          Center(
            child: SizedBox(
              height: 150,
              width: MediaQuery.of(context).size.width - 50,
              child: _buildCharts(),
            ),
          )
        ],
      ),
    );
  }

  Widget _buildCharts() {
    List<PandemiHistory> cases = [];
    List<PandemiHistory> deaths = [];
    List<PandemiHistory> recovered = [];

    item.history.forEach((h) {
      final dt = TimeHelper.parseAsUtc(h.lastUpdated);
      cases.add(PandemiHistory(
          new DateTime(dt.year, dt.month, dt.day), h.totalCases));
    });

    item.history.forEach((h) {
      final dt = TimeHelper.parseAsUtc(h.lastUpdated);
      deaths.add(PandemiHistory(
          new DateTime(dt.year, dt.month, dt.day), h.totalDeaths));
    });

    item.history.forEach((h) {
      final dt = TimeHelper.parseAsUtc(h.lastUpdated);
      recovered.add(PandemiHistory(
          new DateTime(dt.year, dt.month, dt.day), h.totalRecovered));
    });

    return new charts.TimeSeriesChart(
      [
        new charts.Series<PandemiHistory, DateTime>(
            data: cases,
            colorFn: (_, __) => charts.MaterialPalette.red.shadeDefault,
            domainFn: (PandemiHistory datum, int index) => datum.time,
            measureFn: (PandemiHistory datum, int index) => datum.count,
            id: "Kasus"),
        new charts.Series<PandemiHistory, DateTime>(
            data: deaths,
            colorFn: (_, __) => charts.MaterialPalette.gray.shadeDefault,
            domainFn: (PandemiHistory datum, int index) => datum.time,
            measureFn: (PandemiHistory datum, int index) => datum.count,
            id: "Meninggal"),
        new charts.Series<PandemiHistory, DateTime>(
            data: deaths,
            domainFn: (PandemiHistory datum, int index) => datum.time,
            colorFn: (_, __) => charts.MaterialPalette.green.shadeDefault,
            measureFn: (PandemiHistory datum, int index) => datum.count,
            id: "Sembuh")
      ],
      animate: false,
      defaultRenderer: new charts.LineRendererConfig(includeArea: true),
      // dateTimeFactory: const charts.LocalDateTimeFactory(),
    );
  }
}

class PandemiHistory {
  final DateTime time;
  final int count;

  PandemiHistory(this.time, this.count);
}
