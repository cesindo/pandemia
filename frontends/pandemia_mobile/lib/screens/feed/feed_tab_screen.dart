import 'dart:async';

import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:pandemia_mobile/blocs/feed/feed_bloc.dart';
import 'package:pandemia_mobile/blocs/feed/feed_event.dart';
import 'package:pandemia_mobile/blocs/feed/feed_state.dart';
import 'package:pandemia_mobile/core/core.dart';
import 'package:pandemia_mobile/models/feed.dart';
import 'package:pandemia_mobile/widgets/bottom_loader.dart';
import 'package:pandemia_mobile/widgets/feed/feed_item_view.dart';
import 'package:pandemia_mobile/widgets/loading_indicator.dart';

class FeedTabScreen extends StatelessWidget {
  final FeedBloc feedBloc;
  final bool isLoading = false;
  final GlobalKey<RefreshIndicatorState> _refreshKey = new GlobalKey();
  final ScrollController _scrollController = new ScrollController();

  FeedTabScreen(this.feedBloc) {
    this.feedBloc.dispatch(LoadFeed(withLoading: true));
    _scrollController.addListener(_onScroll);
  }

  void _onScroll() {
    final maxScroll = _scrollController.position.maxScrollExtent;
    final currentScroll = _scrollController.position.pixels;
    if (currentScroll == maxScroll) {
      this.feedBloc.dispatch(LoadMoreFeed());
    }
  }

  @override
  Widget build(BuildContext context) {
    return new Container(
      margin: EdgeInsets.only(top: 10),
      child: _getBody(context),
    );
  }

  Widget _getBody(BuildContext context) {
    return BlocBuilder<FeedBloc, FeedState>(
      builder: (context, state) {
        print("[FEED_STATE] $state");
        List<Feed> feeds;
        bool hasReachedMax = false;

        if (state is FeedLoading) {
          return LoadingIndicator(key: PandemiaKeys.loading);
        } else if (state is FeedFailure) {
          return refreshableText(context,
              "Cannot fetch data from server :(\n please try again later");
        } else if (state is FeedsLoaded) {
          // _loaded = true;
          feeds = state.items;
        } else if (state is FeedsUpdated) {
          // _loaded = true;
          feeds = state.items;
          // isLoading = state.isLoading;
          hasReachedMax = state.hasReachedMax;
        } else {
          return Text("Unknown state");
        }

        if (feeds.length > 0) {
          return refreshable(
              context,
              ListView.builder(
                key: PandemiaKeys.notifList,
                controller: _scrollController,
                itemCount: hasReachedMax ? feeds.length : feeds.length + 1,
                itemBuilder: (BuildContext context, int index) {
                  if (index >= feeds.length) {
                    return BottomLoader(
                      isLoading: isLoading,
                    );
                  }

                  final item = feeds[index];
                  return new FeedItemView(item: item);
                },
              ));
        } else {
          return refreshableText(
              context, "Data masih kosong, coba beberapa saat lagi");
        }
      },
    );
  }

  Widget refreshable(BuildContext context, Widget child) {
    return new RefreshIndicator(
        child: child,
        key: _refreshKey,
        onRefresh: () {
          this.feedBloc.dispatch(LoadFeed(force: true));
          return Future<void>(() {});
        });
  }

  Widget refreshableText(BuildContext context, String text) {
    return refreshable(
        context,
        SingleChildScrollView(
          physics: AlwaysScrollableScrollPhysics(),
          child: Container(
              height: MediaQuery.of(context).size.height / 1.3,
              child: Center(
                child: Container(
                    width: MediaQuery.of(context).size.width / 1.3,
                    child: Text(
                      text,
                      textAlign: TextAlign.center,
                      maxLines: 2,
                    )),
              )),
        ));
  }
}
