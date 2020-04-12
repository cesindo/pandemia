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
import 'package:pandemia_mobile/blocs/settings/settings_bloc.dart';
import 'package:pandemia_mobile/blocs/settings/settings_event.dart';
import 'package:pandemia_mobile/blocs/stats/stats_bloc.dart';
import 'package:pandemia_mobile/blocs/stats/stats_event.dart';
import 'package:pandemia_mobile/core/core.dart';
import 'package:pandemia_mobile/models/models.dart';
import 'package:pandemia_mobile/notification_util.dart';
import 'package:pandemia_mobile/screens/feed/feed_tab_screen.dart';
import 'package:pandemia_mobile/screens/issue/issue_page.dart';
import 'package:pandemia_mobile/screens/map/map_page.dart';
import 'package:pandemia_mobile/screens/profile/profile_edit_page.dart';
import 'package:pandemia_mobile/screens/setting/setting_page.dart';
import 'package:pandemia_mobile/user_repository/user_repository.dart';
import 'package:pandemia_mobile/widgets/widgets.dart';

import '../core/core.dart';
import 'package:pandemia_mobile/screens/stats/stats_page.dart';

@immutable
class HomeScreen extends StatelessWidget {
  final String title;
  final PandemiaBloc pandemiaBloc;

  HomeScreen({Key key, this.title, this.pandemiaBloc}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    // final pandemiaBloc = BlocProvider.of<PandemiaBloc>(context);
    final tabBloc = BlocProvider.of<TabBloc>(context);
    final notifBloc = BlocProvider.of<NotifBloc>(context);
    final statsBloc = BlocProvider.of<StatsBloc>(context);
    final feedBloc = BlocProvider.of<FeedBloc>(context);
    final issueBloc = BlocProvider.of<IssueBloc>(context);
    final mapBloc = BlocProvider.of<MapBloc>(context);
    final settingsBloc = BlocProvider.of<SettingsBloc>(context);
    final profileBloc = BlocProvider.of<ProfileBloc>(context);

    final feed = FeedTabScreen(feedBloc);
    final stats = StatsPage();
    final map = MapPage(mapBloc);
    final issue = IssuePage(issueBloc);
    final settings = SettingScreen(
      settingsBloc: settingsBloc,
    );
    final editProfile = ProfileEditPage(profileBloc: profileBloc);

    new Future.delayed(Duration.zero, () {
      NotificationUtil().init(context, notifBloc, feedBloc);
    });

    void _selectedChoice(String choice) {
      if (choice == CustomPopupMenu.profile) {
        Navigator.push(
            context, MaterialPageRoute(builder: (context) => editProfile));
      } else if (choice == CustomPopupMenu.about) {
        Navigator.of(context).pushNamed(PandemiaRoutes.about);
      } else {
        Navigator.of(context).pushNamed(PandemiaRoutes.odp);
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
          appBar: AppBar(
            elevation: 2.0,
            leading: Image.asset("assets/img/pandemia-logo-32.png"),
            title: Text(title, style: TextStyle()),
            titleSpacing: 0.0,
            actions: <Widget>[
              PopupMenuButton(
                onSelected: _selectedChoice,
                itemBuilder: (BuildContext context) {
                  return CustomPopupMenu.choices.map((String choice) {
                    return PopupMenuItem(
                      value: choice,
                      child: ListTile(
                        leading: choice == CustomPopupMenu.profile
                            ? Icon(Icons.edit)
                            : Icon(Icons.info),
                        title: Text(choice),
                      ),
                    );
                  }).toList();
                },
              )
              
            ],
          ),
          body: body,
          bottomNavigationBar: TabSelector(
            activeTab: activeTab,
            onTabSelected: (tab) => tabBloc.dispatch(UpdateTab(tab)),
          ),
        );
      },
    );
  }
}

class CustomPopupMenu {
  static const String odp = 'Data ODP/PDP';
  static const String profile = 'Edit Profil Satgas';
  static const String about = 'Tentang';

  static const List<String> choices = <String>[odp, profile, about];
}