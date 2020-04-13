import 'dart:async';

import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:pandemia_mobile/blocs/sub_report/sub_report.dart';
import 'package:pandemia_mobile/blocs/sub_report/sub_report_bloc.dart';
import 'package:pandemia_mobile/core/core.dart';
import 'package:pandemia_mobile/models/sub_report.dart';
import 'package:pandemia_mobile/widgets/widgets.dart';

class SubReportPage extends StatefulWidget {
  final SubReportBloc subReportBloc;
  SubReportPage({Key key, this.subReportBloc}) : super(key: key);

  @override
  _SubReportPageState createState() => _SubReportPageState(this.subReportBloc);
}

class _SubReportPageState extends State<SubReportPage>
    with TickerProviderStateMixin {
  final SubReportBloc subReportBloc;
  TabController _tabController;
  TextEditingController _odpSearchController = TextEditingController();
  TextEditingController _pdpSearchController = TextEditingController();

  _SubReportPageState(this.subReportBloc);

  @override
  void initState() {
    subReportBloc.dispatch(LoadSubReport(status: 0));
    _tabController = TabController(length: 2, vsync: this);
    _tabController.addListener(() {
      subReportBloc.dispatch(
          LoadSubReport(status: _tabController.index, withLoading: false));
    });
    super.initState();
  }

  @override
  void dispose() {
    _tabController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text("Data ODP/PDP"),
        bottom: TabBar(
          controller: _tabController,
          labelColor: Colors.white,
          tabs: <Widget>[
            Tab(
              text: 'ODP',
            ),
            Tab(
              text: 'PDP',
            )
          ],
        ),
      ),
      body: TabBarView(
        physics: NeverScrollableScrollPhysics(),
        controller: _tabController,
        children: <Widget>[
          ViewODPScreen(
            subReportBloc: subReportBloc,
            searchController: _odpSearchController,
          ),
          ViewPDPScreen(
            subReportBloc: subReportBloc,
            searchController: _pdpSearchController,
          ),
        ],
      ),
    );
  }
}

class ViewODPScreen extends StatefulWidget {
  final SubReportBloc subReportBloc;
  final TextEditingController searchController;

  ViewODPScreen({Key key, this.subReportBloc, this.searchController})
      : super(key: key);

  // ViewODPScreen({Key key}) : super(key: key);

  @override
  _ViewODPScreenState createState() => _ViewODPScreenState();
}

class _ViewODPScreenState extends State<ViewODPScreen> {
  FocusNode node = FocusNode();
  StreamSubscription _subs;

  @override
  void initState() {
    super.initState();
    _subs = widget.subReportBloc.state.listen((SubReportState state) {
      if (state is SubReportListUpdated || state is SubReportListLoaded) {
        Future.delayed(Duration(milliseconds: 1000), () {
          node.requestFocus();
        });
      }
    });
  }

  @override
  void dispose() {
    _subs.cancel();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<SubReportBloc, SubReportState>(
        bloc: this.widget.subReportBloc,
        builder: (context, state) {
          List<SubReport> items = [];
          if (state is SubReportListLoading) {
            return LoadingIndicator(key: PandemiaKeys.loading);
          } else if (state is SubReportListLoaded) {
            items = state.items;
          } else if (state is SubReportListUpdated) {
            items = state.items;
          }

          return Column(
            children: <Widget>[
              Padding(
                padding: EdgeInsets.only(left: 10),
                child: TextFormField(
                  focusNode: node,
                  autofocus: true,
                  controller: this.widget.searchController,
                  onFieldSubmitted: (text) {
                    print("submit $text");
                    widget.subReportBloc.dispatch(SubReportSearch(text, 0));
                  },
                  onChanged: (text) {
                    setState(() {});
                  },
                  decoration: InputDecoration(
                      //  border: InputBorder.none,
                      suffixIcon: widget.searchController.text != ""
                          ? IconButton(
                              icon: Icon(Icons.cancel),
                              onPressed: () {
                                setState(() {
                                  widget.searchController.clear();
                                  widget.subReportBloc
                                      .dispatch(LoadSubReport(status: 0));
                                  node.requestFocus();
                                });
                              })
                          : null,
                      hintText: 'Pencarian'),
                ),
              ),
              Expanded(
                child: items.isNotEmpty
                    ? ListView.builder(
                        itemCount: items.length,
                        itemBuilder: (context, index) {
                          final item = items[index];
                          return Card(
                            child: ListTile(
                              leading: Icon(
                                Icons.person_pin,
                                size: 30,
                              ),
                              title: Text("${item.fullName}"),
                              subtitle: Text("${item.residenceAddress}"),
                            ),
                          );
                        })
                    : Center(child: Text("Belum ada data")),
              )
            ],
          );
        });
  }
}

class ViewPDPScreen extends StatefulWidget {
  final SubReportBloc subReportBloc;
  final TextEditingController searchController;

  ViewPDPScreen({Key key, this.subReportBloc, this.searchController})
      : super(key: key);

  @override
  _ViewPDPScreenState createState() => _ViewPDPScreenState();
}

class _ViewPDPScreenState extends State<ViewPDPScreen> {
  FocusNode node = FocusNode();
  StreamSubscription _subs;

  @override
  void initState() {
    super.initState();
    _subs = widget.subReportBloc.state.listen((SubReportState state) {
      if (state is SubReportListUpdated || state is SubReportListLoaded) {
        Future.delayed(Duration(milliseconds: 1000), () {
          node.requestFocus();
        });
      }
    });
  }

  @override
  void dispose() {
    _subs.cancel();
    super.dispose();
  }

  // const ViewPDPScreen({Key key, this.subReportBloc}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<SubReportBloc, SubReportState>(
        bloc: widget.subReportBloc,
        builder: (context, state) {
          List<SubReport> items = [];
          if (state is SubReportListLoading) {
            return LoadingIndicator(key: PandemiaKeys.loading);
          } else if (state is SubReportListLoaded) {
            items = state.items;
          } else if (state is SubReportListUpdated) {
            items = state.items;
          }

          return Column(
            children: <Widget>[
              Padding(
                padding: EdgeInsets.only(left: 10),
                child: TextFormField(
                  focusNode: node,
                  autofocus: true,
                  controller: this.widget.searchController,
                  onFieldSubmitted: (text) {
                    print("submit $text");
                    widget.subReportBloc.dispatch(SubReportSearch(text, 1));
                  },
                  onChanged: (text) {
                    setState(() {});
                  },
                  decoration: InputDecoration(
                      //  border: InputBorder.none,
                      suffixIcon: widget.searchController.text != ""
                          ? IconButton(
                              icon: Icon(Icons.cancel),
                              onPressed: () {
                                setState(() {
                                  widget.searchController.clear();
                                  widget.subReportBloc
                                      .dispatch(LoadSubReport(status: 1));
                                  node.requestFocus();
                                });
                              })
                          : null,
                      hintText: 'Pencarian'),
                ),
              ),
              Expanded(
                  child: items.isNotEmpty
                      ? ListView.builder(
                          itemCount: items.length,
                          itemBuilder: (context, index) {
                            final item = items[index];
                            return Card(
                              child: ListTile(
                                leading: Icon(
                                  Icons.person_pin,
                                  size: 30,
                                ),
                                title: Text("${item.fullName}"),
                                subtitle: Text("${item.residenceAddress}"),
                              ),
                            );
                          })
                      : Center(child: Text("Belum ada data")))
            ],
          );

          //   if (items.isNotEmpty) {
          //     return ListView.builder(
          //         itemCount: items.length,
          //         itemBuilder: (context, index) {
          //           final item = items[index];
          //           return Card(
          //             child: ListTile(
          //               leading: Icon(
          //                 Icons.person_pin,
          //                 size: 30,
          //               ),
          //               title: Text("${item.fullName}"),
          //               subtitle: Text("${item.residenceAddress}"),
          //             ),
          //           );
          //         });
          //   } else {
          //     return Center(child: Text("Belum ada data"));
          //   }
        });
  }
}
