import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:pandemia_mobile/blocs/blocs.dart';
import 'package:pandemia_mobile/blocs/pandemia/pandemia.dart';
import 'package:pandemia_mobile/core/core.dart';
import 'package:pandemia_mobile/models/models.dart';
import 'package:pandemia_mobile/widgets/widgets.dart';

@immutable
class HomeScreen extends StatelessWidget {
  final String title;
  final PandemiaBloc pandemiaBloc;

  HomeScreen({Key key, this.title, this.pandemiaBloc}) : super(key: key) {
    print("init HomeScreen()");
  }

  @override
  Widget build(BuildContext context) {
    // final pandemiaBloc = BlocProvider.of<PandemiaBloc>(context);
    final tabBloc = BlocProvider.of<TabBloc>(context);

    return BlocBuilder<TabBloc, AppTab>(
      builder: (context, activeTab) {
        Widget body;
        if (activeTab == AppTab.timeline) {
          body = Timeline();
        } else if (activeTab == AppTab.notif) {
          body = NotifList(context);
        } else {
          // @TODO(*): fix this
          body = Timeline();
        }
        return Scaffold(
          appBar: AppBar(
            title: Text(title),
            // actions: [
            //   FilterButton(visible: activeTab == AppTab.todos),
            //   ExtraActions(),
            // ],
          ),
          drawer: new Drawer(
            child: ListView(
              children: <Widget>[
                new DrawerHeader(child: new Text("Pandemia Header")),
                new ListTile(
                    title: new Text("Users"),
                    onTap: () {
                      Navigator.pop(context);
                      // Navigator.of(context).pushNamed(PandemiaRoutes.taskMan);
                    }),
                new ListTile(title: new Text("Analytics"), onTap: () {}),
                new Divider(),
                new ListTile(title: new Text("Notification"), onTap: () {}),
                new ListTile(title: new Text("Profile"), onTap: () {}),
                new ListTile(title: new Text("Security"), onTap: () {}),
                new Divider(),
                new ListTile(
                    title: new Text("Logout"),
                    onTap: () {
                      Navigator.pop(context);
                      pandemiaBloc.dispatch(LoggedOut());
                      Navigator.pushReplacementNamed(context, PandemiaRoutes.login);
                    }),
              ],
            ),
          ),
          body: body,
          // floatingActionButton: activeTab == AppTab.timeline
          //     ? FloatingActionButton(
          //         key: PandemiaKeys.updateStatusFab,
          //         onPressed: () {
          //           Navigator.pushNamed(context, PandemiaRoutes.updateStatus);
          //         },
          //         child: Icon(Icons.add),
          //         tooltip: "Add comment",
          //       )
          //     : null,
          bottomNavigationBar: TabSelector(
            activeTab: activeTab,
            onTabSelected: (tab) => tabBloc.dispatch(UpdateTab(tab)),
          ),
        );
      },
    );
  }


}

