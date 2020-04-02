import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:pandemia_mobile/blocs/stats/stats_bloc.dart';
import 'package:pandemia_mobile/blocs/stats/stats_state.dart';
import 'package:pandemia_mobile/widgets/stats/stats_item_view.dart';
import 'package:pandemia_mobile/widgets/widgets.dart';

class StatsPage extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
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

      if (viewItems.isEmpty) {
        return Container(
          child: Center(
            child: Container(
              child: Text("Belum ada data masuk", textAlign: TextAlign.center,),
              width: MediaQuery.of(context).size.width / 1.3,
            ),
          ),
        );
      }

      return ListView(children: viewItems);
    });
  }
}
