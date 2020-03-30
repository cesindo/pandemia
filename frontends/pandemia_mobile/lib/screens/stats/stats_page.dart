import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:intl/intl.dart';
import 'package:pandemia_mobile/blocs/stats/stats_bloc.dart';
import 'package:pandemia_mobile/blocs/stats/stats_event.dart';
import 'package:pandemia_mobile/blocs/stats/stats_state.dart';
import 'package:pandemia_mobile/models/record.dart';
import 'package:pandemia_mobile/time_helper.dart';
import 'package:pandemia_mobile/widgets/widgets.dart';
import 'package:timeago/timeago.dart' as timeago;

class StatsPage extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    BlocProvider.of<StatsBloc>(context).dispatch(LoadStats());
    return _getBody(context);
  }

  Widget _getBody(BuildContext context) {
    return BlocBuilder<StatsBloc, StatsState>(
        builder: (BuildContext context, StatsState state) {
      List<Widget> viewItems = [];

      if (state is StatsLoading) {
        return Center(
          child: LoadingIndicator(),
        );
      } else if (state is StatsLoaded) {
        // print("state.items: ${state.items}");
        state.items.forEach((a) {
          viewItems.add(Section(a.loc.toUpperCase(), a));
        });
      } else if (state is StatsUpdated) {
        // print("state.items: ${state.items}");
        state.items.forEach((a) {
          viewItems.add(Section(a.loc.toUpperCase(), a));
        });
        // print("viewItems: $viewItems");
      }

      return ListView(children: viewItems);
    });
    ;
  }
}

class Section extends StatelessWidget {
  final String title;
  final Record data;

  const Section(this.title, this.data);

  @override
  Widget build(BuildContext context) {
    final numfa = new NumberFormat("#,##0", "en_US");
    return Card(
      elevation: 2.0,
      margin: new EdgeInsets.symmetric(horizontal: 16.0, vertical: 5.0),
      child: Column(
        mainAxisAlignment: MainAxisAlignment.start,
        crossAxisAlignment: CrossAxisAlignment.start,
        children: <Widget>[
          Padding(
            padding: EdgeInsets.all(10),
            child: Row(children: <Widget>[
              Expanded(
                  child: Text(
                this.title,
                style: TextStyle(fontWeight: FontWeight.bold, fontSize: 20.0),
                textAlign: TextAlign.left,
              )),
              Text(
                timeago.format(TimeHelper.parseAsUtc(this.data.lastUpdated)),
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
                  Text(numfa.format(data.totalCases))
                ]),
                TableRow(children: [
                  Text("Total kematian"),
                  Text(":"),
                  Text(numfa.format(data.totalDeaths))
                ]),
                TableRow(children: [
                  Text("Total sembuh"),
                  Text(":"),
                  Text(numfa.format(data.totalRecovered))
                ])
              ],
            ),
          )
        ],
      ),
    );
  }
}
