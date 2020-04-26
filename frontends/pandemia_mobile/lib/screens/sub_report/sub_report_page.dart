import 'dart:async';

import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:pandemia_mobile/api/pandemia_api.dart';
import 'package:pandemia_mobile/blocs/profile/profile_bloc.dart';
import 'package:pandemia_mobile/blocs/sub_report/sub_report.dart';
import 'package:pandemia_mobile/blocs/sub_report/sub_report_bloc.dart';
import 'package:pandemia_mobile/core/core.dart';
import 'package:pandemia_mobile/models/sub_report.dart';
import 'package:pandemia_mobile/screens/sub_report/add_sub_report.dart';
import 'package:pandemia_mobile/throttle.dart';
import 'package:pandemia_mobile/util/sub_report_util.dart';
import 'package:pandemia_mobile/widgets/widgets.dart';

class SubReportPage extends StatefulWidget {
  final ProfileBloc profileBloc;
  final SubReportBloc subReportBloc;
  SubReportPage({Key key, this.subReportBloc, this.profileBloc})
      : super(key: key);

  @override
  _SubReportPageState createState() =>
      _SubReportPageState(this.subReportBloc, this.profileBloc);
}

class _SubReportPageState extends State<SubReportPage>
    with TickerProviderStateMixin {
  final SubReportBloc subReportBloc;
  final ProfileBloc profileBloc;
  TabController _tabController;
  TextEditingController _odpSearchController = TextEditingController();
  TextEditingController _pdpSearchController = TextEditingController();
  int odpCount = 0;
  int pdpCount = 0;
  int otgCount = 0;

  _SubReportPageState(this.subReportBloc, this.profileBloc);

  @override
  void initState() {
    subReportBloc.dispatch(LoadSubReport(status: 'ODP'));
    _tabController = TabController(length: 4, vsync: this);
    _tabController.addListener(() {
      if (!Throttle.isReady("sub_report_load", within: 500)){
        return;
      }
      String status = 'ODP';
      if (_tabController.index == 1) {
        status = 'PDP';
      } else if (_tabController.index == 2) {
        status = 'OTG';
      } else if (_tabController.index == 3) {
        status = 'ALL';
      }
      subReportBloc.dispatch(LoadSubReport(status: status, withLoading: false));
    });
    super.initState();

    refreshState();
  }

  @override
  void dispose() {
    _tabController.dispose();
    super.dispose();
  }

  void refreshState() {
    PublicApi.get("/pandemia/v1/sub_report/count").then((data) {
      if (data["code"] == 0) {
        final result = data['result'];
        setState(() {
          this.odpCount = result['ODP'] != null ? result['ODP'] : 0;
          this.pdpCount = result['PDP'] != null ? result['PDP'] : 0;
          this.otgCount = result['OTG'] != null ? result['OTG'] : 0;
        });
      }
    });
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
              text: 'ODP ($odpCount)',
            ),
            Tab(
              text: 'PDP ($pdpCount)',
            ),
            Tab(
              text: 'OTG ($otgCount)',
            ),
            Tab(
              text: 'SEMUA',
            )
          ],
        ),
      ),
      body: TabBarView(
        physics: NeverScrollableScrollPhysics(),
        controller: _tabController,
        children: <Widget>[
          SubReportList(
            this,
            subReportBloc: subReportBloc,
            profileBloc: profileBloc,
            searchController: _odpSearchController,
            status: "ODP",
          ),
          SubReportList(this,
              subReportBloc: subReportBloc,
              profileBloc: profileBloc,
              searchController: _pdpSearchController,
              status: "PDP"),
          SubReportList(this,
              subReportBloc: subReportBloc,
              profileBloc: profileBloc,
              searchController: _pdpSearchController,
              status: "OTG"),
          SubReportList(this,
              subReportBloc: subReportBloc,
              profileBloc: profileBloc,
              searchController: _pdpSearchController,
              status: "ALL"),
        ],
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: () => Navigator.of(context)
            .push(MaterialPageRoute(
                builder: (context) =>
                    AddSubReportPage(subReportBloc: subReportBloc)))
            .then((_) {
          this.refreshState();
        }),
        child: Icon(Icons.person_add),
        tooltip: "Tambahkan data ODP/PDP/OTG",
      ),
    );
  }
}

class SubReportList extends StatefulWidget {
  final SubReportBloc subReportBloc;
  final ProfileBloc profileBloc;
  final TextEditingController searchController;
  final String status;
  final _SubReportPageState parent;

  SubReportList(this.parent,
      {Key key,
      this.subReportBloc,
      this.profileBloc,
      this.searchController,
      @required this.status})
      : super(key: key);

  // SubReportList({Key key}) : super(key: key);

  @override
  _SubReportListState createState() => _SubReportListState();
}

class _SubReportListState extends State<SubReportList> {
  FocusNode node = FocusNode();
  List<SubReport> items = [];
  // StreamSubscription _subs;

  // @override
  // void initState() {
  //   super.initState();
  //   // _subs = widget.subReportBloc.state.listen((SubReportState state) {
  //   //   if (state is SubReportListUpdated || state is SubReportListLoaded) {
  //   //     Future.delayed(Duration(milliseconds: 1000), () {
  //   //       node.requestFocus();
  //   //     });
  //   //   }
  //   // });
  // }

  // @override
  // void dispose() {
  //   // _subs.cancel();
  //   super.dispose();
  // }

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<SubReportBloc, SubReportState>(
        bloc: this.widget.subReportBloc,
        builder: (context, state) {
          if (state is SubReportListLoading) {
            return LoadingIndicator(key: PandemiaKeys.loading);
          } else if (state is SubReportListLoaded) {
            if (state.status == this.widget.status) {
              items = state.items;
            }
          } else if (state is SubReportListUpdated) {
            if (state.status == this.widget.status) {
              items = state.items;
            }
          } else if (state is SubReportFailure) {
            return Container(
                child: Center(
                    child: Text(
              "Gagal memuat data,\n periksa kembali koneksi Anda",
              textAlign: TextAlign.center,
            )));
          }

          return Column(
            children: <Widget>[
              Padding(
                padding: EdgeInsets.only(left: 10),
                child: TextFormField(
                  focusNode: node,
                  // autofocus: true,
                  controller: this.widget.searchController,
                  onFieldSubmitted: (text) {
                    print("submit $text");
                    widget.subReportBloc
                        .dispatch(SubReportSearch(text, this.widget.status));
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
                                  widget.subReportBloc.dispatch(LoadSubReport(
                                      status: this.widget.status));
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
                                Icons.person,
                                size: 30,
                              ),
                              title: Text("${item.fullName}",
                                  style: TextStyle(
                                      fontWeight: FontWeight.bold,
                                      fontSize: 17,
                                      fontFamily:
                                          "Google Sans, Roboto, sans-serif")),
                              subtitle: Column(mainAxisAlignment: MainAxisAlignment.start, crossAxisAlignment: CrossAxisAlignment.start,children: <Widget>[
                                Text("${item.residenceAddress}",
                                  style: TextStyle(fontSize: 15)),
                                  Text(SubReportUtil().statusIdNameToLabel[item.status.toLowerCase()],
                                  style: TextStyle(fontSize: 15))
                              ],),
                              onTap: () {
                                Navigator.of(context)
                                    .push(MaterialPageRoute(
                                        builder: (context) => AddSubReportPage(
                                            subReportBloc: widget.subReportBloc,
                                            key: Key("sub-report-" +
                                                item.id.toString()),
                                            item: item)))
                                    .then((updated) {
                                  if (updated == true) {
                                    widget.subReportBloc.dispatch(LoadSubReport(
                                        status: this.widget.status,
                                        withLoading: false));
                                    this.widget.parent.refreshState();
                                  }
                                });
                              },
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

// class ViewPDPScreen extends StatefulWidget {
//   final SubReportBloc subReportBloc;
//   final ProfileBloc profileBloc;
//   final TextEditingController searchController;

//   ViewPDPScreen(
//       {Key key, this.subReportBloc, this.profileBloc, this.searchController})
//       : super(key: key);

//   @override
//   _ViewPDPScreenState createState() => _ViewPDPScreenState();
// }

// class _ViewPDPScreenState extends State<ViewPDPScreen> {
//   FocusNode node = FocusNode();
//   StreamSubscription _subs;

//   @override
//   void initState() {
//     super.initState();
//     // _subs = widget.subReportBloc.state.listen((SubReportState state) {
//     //   if (state is SubReportListUpdated || state is SubReportListLoaded) {
//     //     Future.delayed(Duration(milliseconds: 1000), () {
//     //       node.requestFocus();
//     //     });
//     //   }
//     // });
//   }

//   @override
//   void dispose() {
//     // _subs.cancel();
//     super.dispose();
//   }

//   // const ViewPDPScreen({Key key, this.subReportBloc}) : super(key: key);

//   @override
//   Widget build(BuildContext context) {
//     return BlocBuilder<SubReportBloc, SubReportState>(
//         bloc: widget.subReportBloc,
//         builder: (context, state) {
//           List<SubReport> items = [];
//           if (state is SubReportListLoading) {
//             return LoadingIndicator(key: PandemiaKeys.loading);
//           } else if (state is SubReportListLoaded) {
//             items = state.items;
//           } else if (state is SubReportListUpdated) {
//             items = state.items;
//           }

//           return Column(
//             children: <Widget>[
//               Padding(
//                 padding: EdgeInsets.only(left: 10),
//                 child: TextFormField(
//                   focusNode: node,
//                   // autofocus: true,
//                   controller: this.widget.searchController,
//                   onFieldSubmitted: (text) {
//                     print("submit $text");
//                     widget.subReportBloc.dispatch(SubReportSearch(text, 'pdp'));
//                   },
//                   onChanged: (text) {
//                     setState(() {});
//                   },
//                   decoration: InputDecoration(
//                       //  border: InputBorder.none,
//                       suffixIcon: widget.searchController.text != ""
//                           ? IconButton(
//                               icon: Icon(Icons.cancel),
//                               onPressed: () {
//                                 setState(() {
//                                   widget.searchController.clear();
//                                   widget.subReportBloc
//                                       .dispatch(LoadSubReport(status: 'PDP'));
//                                   node.requestFocus();
//                                 });
//                               })
//                           : null,
//                       hintText: 'Pencarian'),
//                 ),
//               ),
//               Expanded(
//                   child: items.isNotEmpty
//                       ? ListView.builder(
//                           itemCount: items.length,
//                           itemBuilder: (context, index) {
//                             final item = items[index];
//                             return Card(
//                               child: ListTile(
//                                 leading: Icon(
//                                   Icons.person_pin,
//                                   size: 30,
//                                 ),
//                                 title: Text("${item.fullName}"),
//                                 subtitle: Text("${item.residenceAddress}"),
//                                 onTap: () {
//                                   Navigator.of(context).push(MaterialPageRoute(
//                                       builder: (context) => AddSubReportPage(
//                                           subReportBloc: widget.subReportBloc,
//                                           key: Key("sub-report-" +
//                                               item.id.toString()),
//                                           item: item)));
//                                 },
//                               ),
//                             );
//                           })
//                       : Center(child: Text("Belum ada data")))
//             ],
//           );

//           //   if (items.isNotEmpty) {
//           //     return ListView.builder(
//           //         itemCount: items.length,
//           //         itemBuilder: (context, index) {
//           //           final item = items[index];
//           //           return Card(
//           //             child: ListTile(
//           //               leading: Icon(
//           //                 Icons.person_pin,
//           //                 size: 30,
//           //               ),
//           //               title: Text("${item.fullName}"),
//           //               subtitle: Text("${item.residenceAddress}"),
//           //             ),
//           //           );
//           //         });
//           //   } else {
//           //     return Center(child: Text("Belum ada data"));
//           //   }
//         });
//   }
// }
