import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:pandemia_mobile/blocs/blocs.dart';
import 'package:pandemia_mobile/blocs/feed/feed.dart';
import 'package:pandemia_mobile/blocs/feed/feed_bloc.dart';
import 'package:pandemia_mobile/blocs/issue/issue_bloc.dart';
import 'package:pandemia_mobile/blocs/map/map_bloc.dart';
import 'package:pandemia_mobile/blocs/map/map_event.dart';
import 'package:pandemia_mobile/blocs/notif/notif_bloc.dart';
import 'package:pandemia_mobile/blocs/pandemia/pandemia.dart';
import 'package:pandemia_mobile/blocs/profile/profile.dart';
import 'package:pandemia_mobile/blocs/report_note/report_note_bloc.dart';
import 'package:pandemia_mobile/blocs/report_note/report_note_event.dart';
import 'package:pandemia_mobile/blocs/settings/settings_bloc.dart';
import 'package:pandemia_mobile/blocs/settings/settings_event.dart';
import 'package:pandemia_mobile/blocs/stats/stats_bloc.dart';
import 'package:pandemia_mobile/blocs/stats/stats_event.dart';
import 'package:pandemia_mobile/blocs/sub_report/sub_report_bloc.dart';
import 'package:pandemia_mobile/core/core.dart';
import 'package:pandemia_mobile/models/models.dart';
import 'package:pandemia_mobile/notification_util.dart';
import 'package:pandemia_mobile/screens/feed/feed_tab_screen.dart';
import 'package:pandemia_mobile/screens/issue/issue_page.dart';
import 'package:pandemia_mobile/screens/map/map_page.dart';
import 'package:pandemia_mobile/screens/profile/profile_edit_page.dart';
import 'package:pandemia_mobile/screens/report/report_note_add_page.dart';
import 'package:pandemia_mobile/screens/setting/setting_page.dart';
import 'package:pandemia_mobile/screens/sub_report/add_sub_report.dart';
import 'package:pandemia_mobile/screens/sub_report/sub_report_page.dart';
import 'package:pandemia_mobile/screens/web/web_token_page.dart';
import 'package:pandemia_mobile/user_repository/user_repository.dart';
import 'package:pandemia_mobile/widgets/widgets.dart';

import '../core/core.dart';
import 'package:pandemia_mobile/screens/stats/stats_page.dart';

@immutable
class HomeScreen extends StatelessWidget {
  final String title;
  final PandemiaBloc pandemiaBloc;
  final UserRepository userRepository = UserRepository();
  final _scaffoldKey = GlobalKey<ScaffoldState>();

  HomeScreen({Key key, this.title, this.pandemiaBloc}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    User currentUser = userRepository.currentUser;
    // final pandemiaBloc = BlocProvider.of<PandemiaBloc>(context);
    final tabBloc = BlocProvider.of<TabBloc>(context);
    final notifBloc = BlocProvider.of<NotifBloc>(context);
    final statsBloc = BlocProvider.of<StatsBloc>(context);
    final feedBloc = BlocProvider.of<FeedBloc>(context);
    final issueBloc = BlocProvider.of<IssueBloc>(context);
    final mapBloc = BlocProvider.of<MapBloc>(context);
    final settingsBloc = BlocProvider.of<SettingsBloc>(context);
    final profileBloc = BlocProvider.of<ProfileBloc>(context);
    final subReportBloc = BlocProvider.of<SubReportBloc>(context);

    final feed = FeedTabScreen(feedBloc);
    final stats = StatsPage();
    final map = MapPage(mapBloc);
    final issue = IssuePage(issueBloc);
    final settings = SettingScreen(
      settingsBloc: settingsBloc,
    );

    new Future.delayed(Duration.zero, () {
      NotificationUtil().init(context, notifBloc, feedBloc);
      pandemiaBloc.state.listen((PandemiaState state) {
        if (state is PandemiaNewUpdateAvailable) {
          _scaffoldKey.currentState.showSnackBar(SnackBar(
            content: Text(
                "Pandemia versi ${state.version} telah tersedia, segera lakukan update!"),
            backgroundColor: Colors.blue,
            behavior: SnackBarBehavior.fixed,
          ));
        }
      });
      pandemiaBloc.dispatch(CheckForUpdate());
    });

    List<CustomPopupMenuItem> choices = [];

    if (currentUser?.isSatgas != true || currentUser?.isBlocked == true) {
      choices.add(CustomPopupMenuItem(0, "Daftar Satgas", Icons.edit));
    } else {
      choices.add(CustomPopupMenuItem(1, "Data ODP/PDP", Icons.list));
      choices.add(CustomPopupMenuItem(2, "Buat Laporan", Icons.comment));
      choices.add(CustomPopupMenuItem(3, "Login Web", Icons.lock));
    }
    choices.add(CustomPopupMenuItem(4, "Tentang", Icons.info));

    void _selectedChoice(CustomPopupMenuItem choice) {
      if (choice.index == 0 || choice.index == 1) {
        if (currentUser?.isBlocked == true) {
          _scaffoldKey.currentState.showSnackBar(SnackBar(
            content: Text("Akun anda telah diblokir"),
            backgroundColor: Colors.red,
          ));
          return;
        }
      }
      if (choice.index == 0) {
        Navigator.push(
            context,
            MaterialPageRoute(
                builder: (context) =>
                    ProfileEditPage(profileBloc: profileBloc))).then((result) {
          if (result != null) {
            choices.clear();
            choices.add(CustomPopupMenuItem(1, "Data ODP/PDP", Icons.list));
            choices.add(CustomPopupMenuItem(2, "Buat Laporan", Icons.comment));
            choices.add(CustomPopupMenuItem(3, "Login Web", Icons.lock));
            choices.add(CustomPopupMenuItem(4, "Tentang", Icons.info));
            User user = result[0];
            String villageName = result[1];
            currentUser = currentUser.copy(
                fullName: user.fullName,
                email: user.email,
                phoneNum: user.phoneNum,
                isSatgas: true);
            _scaffoldKey.currentState.showSnackBar(SnackBar(
                content: Text(
                    "Anda telah terdaftar sebagai Satgas COVID-19 di daerah $villageName. Kini Anda bisa melakukan input data ODP/PDP."),
                backgroundColor: Colors.green));
          }
        });
      } else if (choice.index == 1) {
        Navigator.of(context).push(MaterialPageRoute(
            builder: (context) => SubReportPage(
                subReportBloc: subReportBloc, profileBloc: profileBloc)));
      } else if (choice.index == 2) {
        Navigator.of(context)
            .push(MaterialPageRoute(
                builder: (context) => BlocProvider(
                    builder: (BuildContext context) {
                      return ReportNoteBloc()..dispatch(LoadReportNote());
                    },
                    child: ReportNoteAddPage())))
            .then((result) {
          if (result != null) {
            _scaffoldKey.currentState.showSnackBar(SnackBar(
              content: Text("Terimakasih, laporan Anda telah terkirim"),
              backgroundColor: Colors.green,
            ));
          } //else{
          //   _scaffoldKey.currentState.showSnackBar(SnackBar(
          //     content: Text("Laporan gagal terkirim"),
          //     backgroundColor: Colors.red,
          //   ));
          // }
        });
      } else if (choice.index == 3) {
        Navigator.of(context)
            .push(MaterialPageRoute(builder: (context) => WebTokenPage()));
      } else if (choice.index == 4) {
        Navigator.of(context).pushNamed(PandemiaRoutes.about);
      }
    }

    return BlocBuilder<TabBloc, AppTab>(
      builder: (context, activeTab) {
        Widget body;
        if (activeTab == AppTab.updates) {
          feedBloc.dispatch(LoadFeed(withLoading: false));
          body = feed;
        } else if (activeTab == AppTab.stats) {
          statsBloc.dispatch(LoadStats(withLoading: false));
          body = stats;
        } else if (activeTab == AppTab.map) {
          mapBloc.dispatch(
              LoadMap(UserRepository().currentUser.loc, withLoading: false));
          body = map;
        } else if (activeTab == AppTab.hoax) {
          body = issue;
        } else if (activeTab == AppTab.settings) {
          settingsBloc.dispatch(LoadSettings(force: true));
          body = settings;
        }
        return Scaffold(
          key: _scaffoldKey,
          appBar: AppBar(
            elevation: 2.0,
            leading: Image.asset("assets/img/pandemia-logo-32.png"),
            title: Text(title, style: TextStyle()),
            titleSpacing: 0.0,
            actions: <Widget>[
              PopupMenuButton<CustomPopupMenuItem>(
                onSelected: _selectedChoice,
                itemBuilder: (context) => choices.map((choice) {
                  return PopupMenuItem<CustomPopupMenuItem>(
                    value: choice,
                    child: ListTile(
                      leading: Icon(choice.icon),
                      title: Text(choice.title),
                    ),
                  );
                }).toList(),
              )
            ],
          ),
          body: body,
          floatingActionButton: (currentUser?.isSatgas == true &&
                  activeTab != AppTab.settings &&
                  activeTab != AppTab.map)
              ? FloatingActionButton(
                  tooltip: "Tambahkan data ODP/PDP",
                  child: Icon(Icons.person_add),
                  onPressed: () => Navigator.of(context).push(MaterialPageRoute(
                      builder: (context) =>
                          AddSubReportPage(subReportBloc: subReportBloc))))
              : null,
          bottomNavigationBar: TabSelector(
            activeTab: activeTab,
            onTabSelected: (tab) => tabBloc.dispatch(UpdateTab(tab)),
          ),
        );
      },
    );
  }
}

class CustomPopupMenuItem {
  final int index;
  final String title;
  final IconData icon;

  CustomPopupMenuItem(this.index, this.title, this.icon);
}
