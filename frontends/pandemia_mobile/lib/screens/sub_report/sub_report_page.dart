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
          ViewODPScreen(subReportBloc: subReportBloc),
          ViewPDPScreen(subReportBloc: subReportBloc),
        ],
      ),
    );
  }
}

class ViewODPScreen extends StatelessWidget {
  final SubReportBloc subReportBloc;

  const ViewODPScreen({Key key, this.subReportBloc}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<SubReportBloc, SubReportState>(
        bloc: subReportBloc,
        builder: (context, state) {
          List<SubReport> items = [];
          if (state is SubReportListLoading) {
            return LoadingIndicator(key: PandemiaKeys.loading);
          } else if (state is SubReportListLoaded) {
            items = state.items;
          } else if (state is SubReportListUpdated) {
            items = state.items;
          }

          if (items.isNotEmpty) {
            return ListView.builder(
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
                });
          } else {
            return Center(child: Text("Belum ada data"));
          }
        });
  }
}

class ViewPDPScreen extends StatelessWidget {
  final SubReportBloc subReportBloc;

  const ViewPDPScreen({Key key, this.subReportBloc}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<SubReportBloc, SubReportState>(
        bloc: subReportBloc,
        builder: (context, state) {
          List<SubReport> items = [];
          if (state is SubReportListLoading) {
            return LoadingIndicator(key: PandemiaKeys.loading);
          } else if (state is SubReportListLoaded) {
            items = state.items;
          } else if (state is SubReportListUpdated) {
            items = state.items;
          }

          if (items.isNotEmpty) {
            return ListView.builder(
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
                });
          } else {
            return Center(child: Text("Belum ada data"));
          }
        });
  }
}
