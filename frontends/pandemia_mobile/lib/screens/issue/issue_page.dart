import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:pandemia_mobile/blocs/issue/issue.dart';
import 'package:pandemia_mobile/blocs/issue/issue_bloc.dart';
import 'package:pandemia_mobile/core/core.dart';
import 'package:pandemia_mobile/models/issue.dart';
import 'package:pandemia_mobile/widgets/bottom_loader.dart';
import 'package:pandemia_mobile/widgets/issue/issue_item_view.dart';
import 'package:pandemia_mobile/widgets/loading_indicator.dart';

class IssuePage extends StatefulWidget {
  final IssueBloc issueBloc;
  const IssuePage(this.issueBloc);

  @override
  _IssuePageState createState() => _IssuePageState();
}

class _IssuePageState extends State<IssuePage> {
  List<Issue> issues = [];
  bool isLoading = false;
  bool hasReachedMax = false;
  final ScrollController _scrollController = new ScrollController();

  _IssuePageState();

  @override
  void initState() {
    this.widget.issueBloc.dispatch(LoadIssue());
    _scrollController.addListener(_onScroll);
    super.initState();
  }

  void _onScroll() {
    final maxScroll = _scrollController.position.maxScrollExtent;
    final currentScroll = _scrollController.position.pixels;
    if (currentScroll == maxScroll) {
      if (issues.isNotEmpty) {
        this.widget.issueBloc.dispatch(LoadMoreIssue());
      }
    }
  }

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<IssueBloc, IssueState>(
      builder: (BuildContext context, state) {
        if (state is IssueLoading) {
          return LoadingIndicator(key: PandemiaKeys.loading);
        } else if (state is IssueFailure) {
          return Center(
              child: refreshable(
                  context,
                  Text(
                    "Cannot fetch data from server :(\n please try again later",
                    textAlign: TextAlign.center,
                  )));
        } else if (state is IssueListLoaded) {
          issues = state.items;
        } else if (state is IssueListUpdated) {
          issues = state.items;
          isLoading = state.isLoading;
          hasReachedMax = state.hasReachedMax;
        } else {
          return Text("Unknown state");
        }

        if (issues.length > 0) {
          return refreshable(
              context,
              ListView.builder(
                key: PandemiaKeys.notifList,
                controller: _scrollController,
                itemCount: hasReachedMax ? issues.length : issues.length + 1,
                itemBuilder: (BuildContext context, int index) {
                  if (index >= issues.length) {
                    return BottomLoader(
                      isLoading: isLoading,
                    );
                  }

                  final item = issues[index];
                  return new IssueItemView(
                      item: item, issueBloc: this.widget.issueBloc);
                },
              ));
        } else {
          return Center(
              child: refreshable(
                  context,
                  Text(
                    "No Hoax Issues",
                    textAlign: TextAlign.center,
                  )));
        }
      },
    );
  }

  Widget refreshable(BuildContext context, Widget child) {
    return new RefreshIndicator(
        child: child,
        onRefresh: () {
          return Future<void>(() {
            this.widget.issueBloc.dispatch(LoadIssue(force: true));
          });
        });
  }
}
