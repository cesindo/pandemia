import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:intl/intl.dart';
import 'package:pandemia_mobile/blocs/stats/stats_bloc.dart';
import 'package:pandemia_mobile/blocs/stats/stats_event.dart';
import 'package:pandemia_mobile/blocs/stats/stats_state.dart';
import 'package:pandemia_mobile/models/record.dart';
import 'package:pandemia_mobile/time_helper.dart';
import 'package:pandemia_mobile/widgets/stats/stats_item_view.dart';
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
          viewItems.add(StatsItemView(item: a));
        });
      } else if (state is StatsUpdated) {
        // print("state.items: ${state.items}");
        state.items.forEach((a) {
          viewItems.add(StatsItemView(item: a));
        });
        // print("viewItems: $viewItems");
      }

      return ListView(children: viewItems);
    });
    ;
  }
}


