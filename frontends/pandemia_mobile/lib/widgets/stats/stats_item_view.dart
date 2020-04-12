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
    return Card(
      elevation: 2.0,
      margin: new EdgeInsets.fromLTRB(16.0, 16.0, 16.0, 0),
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
          Padding(padding: EdgeInsets.all(10), child: _buildCount(context)),
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

  Widget _buildCount(BuildContext context) {
    final numfa = new NumberFormat("#,##0", "en_US");
    var size = MediaQuery.of(context).size;
    final double itemHeight = (size.height - kToolbarHeight - 24) / 3.5;
    final double itemWidth = size.width / 1;

    return GridView.count(
      shrinkWrap: true,
      primary: false,
      crossAxisCount: 3,
      childAspectRatio: (itemWidth / itemHeight),
      children: <Widget>[
        Padding(
          padding: const EdgeInsets.symmetric(horizontal: 5.0),
          child: Column(children: [
            Text(numfa.format(item.latestRecord.totalCases),
                style: TextStyle(fontSize: 20, color: Colors.red)),
            SizedBox(height: 5),
            Text(
              "Positif",
              textAlign: TextAlign.center,
              style: TextStyle(
                fontSize: 15,
                fontWeight: FontWeight.w300,
              ),
            ),
          ]),
        ),
        Padding(
          padding: const EdgeInsets.symmetric(horizontal: 5.0),
          child: Column(children: [
            Text(numfa.format(item.latestRecord.totalRecovered),
                style: TextStyle(fontSize: 20, color: Colors.green)),
            SizedBox(height: 5),
            Text(
              "Sembuh",
              textAlign: TextAlign.center,
              style: TextStyle(
                fontSize: 15,
                fontWeight: FontWeight.w300,
              ),
            ),
          ]),
        ),
        Padding(
          padding: const EdgeInsets.symmetric(horizontal: 5.0),
          child: Column(children: [
            Text(numfa.format(item.latestRecord.totalDeaths),
                style: TextStyle(fontSize: 20, color: Colors.grey)),
            SizedBox(height: 5),
            Text(
              "Meninggal",
              textAlign: TextAlign.center,
              style: TextStyle(
                fontSize: 15,
                fontWeight: FontWeight.w300,
              ),
            ),
          ]),
        ),
      ],
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
