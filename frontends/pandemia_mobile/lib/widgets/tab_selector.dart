import 'package:flutter/cupertino.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:pandemia_mobile/core/core.dart';
import 'package:pandemia_mobile/models/models.dart';

class TabSelector extends StatelessWidget {
  final AppTab activeTab;
  final Function(AppTab) onTabSelected;

  TabSelector({
    Key key,
    @required this.activeTab,
    @required this.onTabSelected,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return BottomNavigationBar(
      key: PandemiaKeys.tabs,
      currentIndex: AppTab.values.indexOf(activeTab),
      onTap: (index) => onTabSelected(AppTab.values[index]),
      items: AppTab.values.map((tab) {
        IconData icon;
        Key key;
        String title;
        if (tab == AppTab.timeline){
          icon = Icons.rss_feed;
          key = PandemiaKeys.timelineTab;
          title = "Timeline";
        }else if (tab == AppTab.notif){
          icon = Icons.notifications;
          key = PandemiaKeys.notifTab;
          title = "Notif";
        }else if (tab == AppTab.todo){
          icon = Icons.list;
          key = PandemiaKeys.todoTab;
          title = "Todo";
        }else{
          icon = Icons.dashboard;
          key = PandemiaKeys.dashboardTab;
          title = "Menu";
        }
        return BottomNavigationBarItem(
          icon: Icon(
            icon,
            key: key,
          ),
          title: Text(title),
        );
      }).toList(),
    );
  }
}

