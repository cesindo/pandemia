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

class FeedTabScreen extends StatefulWidget {
  final BuildContext context;
  final FeedBloc feedBloc;

  FeedTabScreen(this.context, this.feedBloc);

  @override
  _FeedTabScreenState createState() => _FeedTabScreenState();
}

class _FeedTabScreenState extends State<FeedTabScreen> {
  StreamSubscription _subs;
  List<Feed> feeds = [];
  bool hasReachedMax = false;
  bool isLoading = false;
  final GlobalKey<RefreshIndicatorState> _refreshKey = new GlobalKey();
  final ScrollController _scrollController = new ScrollController();

  _FeedTabScreenState();

  @override
  void initState() {
    this.widget.feedBloc.dispatch(LoadFeed());
    _subs = this.widget.feedBloc.state.listen((state) {
      if (state is DoRefreshFeed) {
        this.widget.feedBloc.dispatch(LoadFeed(force: true));
      }
    });

    _scrollController.addListener(_onScroll);
    super.initState();
  }

  @override
  void dispose() {
    _subs.cancel();
    super.dispose();
  }

  void _onScroll() {
    final maxScroll = _scrollController.position.maxScrollExtent;
    final currentScroll = _scrollController.position.pixels;
    if (currentScroll == maxScroll) {
      if (feeds.isNotEmpty) {
        this.widget.feedBloc.dispatch(LoadMoreFeed());
      }
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
        if (state is FeedLoading) {
          return LoadingIndicator(key: PandemiaKeys.loading);
        } else if (state is FeedFailure) {
          return refreshableText(
              "Cannot fetch data from server :(\n please try again later");
        } else if (state is FeedsLoaded) {
          feeds = state.items;
        } else if (state is FeedsUpdated) {
          feeds = state.items;
          isLoading = state.isLoading;
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
          return refreshableText("No updates yet");
        }
      },
    );
  }

  Widget refreshable(BuildContext context, Widget child) {
    return new RefreshIndicator(
        child: child,
        key: _refreshKey,
        onRefresh: () {
          this.widget.feedBloc.dispatch(LoadFeed(force: true));
          return Future<void>(() {});
        });
  }

  Widget refreshableText(String text) {
    return refreshable(
        context,
        SingleChildScrollView(
          physics: AlwaysScrollableScrollPhysics(),
          child: Container(
            height: MediaQuery.of(context).size.height / 1.3,
            child: Center(
              child: Text(
                text,
                textAlign: TextAlign.center,
              ),
            ),
          ),
        ));
  }
}
