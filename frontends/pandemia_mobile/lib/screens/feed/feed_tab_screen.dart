import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:pandemia_mobile/blocs/feed/feed_bloc.dart';
import 'package:pandemia_mobile/blocs/feed/feed_event.dart';
import 'package:pandemia_mobile/blocs/feed/feed_state.dart';
import 'package:pandemia_mobile/core/core.dart';
import 'package:pandemia_mobile/widgets/feed/feed_item_view.dart';
import 'package:pandemia_mobile/widgets/loading_indicator.dart';

class FeedTabScreen extends StatelessWidget {
  final BuildContext context;

  FeedTabScreen(this.context);

  @override
  Widget build(BuildContext context) {
    return BlocProvider(
        builder: (BuildContext context) {
          return FeedBloc()..dispatch(LoadFeed());
        },
        child: new Container(child: _getBody(context),));
    ;
  }

  Widget _getBody(BuildContext context) {
    return BlocBuilder<FeedBloc, FeedState>(
      builder: (context, state) {
        print(state);
        if (state is FeedLoading) {
          return LoadingIndicator(key: PandemiaKeys.loading);
        } else if (state is FeedFailure) {
          return Center(
              child: refreshable(
                  context,
                  Text(
                    "Cannot fetch data from server :(\n please try again later",
                    textAlign: TextAlign.center,
                  )));
        } else if (state is FeedListLoaded) {
          // return Text("satu");
          final notifs = state.items;
          return refreshable(
              context,
              ListView.builder(
                key: PandemiaKeys.notifList,
                itemCount: notifs.length,
                itemBuilder: (BuildContext context, int index) {
                  final item = notifs[index];
                  return new FeedItemView(item: item);
                },
              ));
        } else {
          return Text("Unknown state");
        }
      },
    );
  }

  Widget refreshable(BuildContext context, Widget child) {
    return new RefreshIndicator(
        child: child,
        onRefresh: () {
          BlocProvider.of<FeedBloc>(context).dispatch(LoadFeed(force: true));
          return Future<void>((){});
        });
  }
}
