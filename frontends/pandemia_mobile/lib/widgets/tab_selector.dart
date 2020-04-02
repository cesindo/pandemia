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
      unselectedItemColor: Colors.grey,
      unselectedLabelStyle: TextStyle(color: Colors.grey),
      selectedItemColor: Theme.of(context).primaryColor,
      type: BottomNavigationBarType.fixed,
      items: AppTab.values.map((tab) {
        IconData icon;
        Key key;
        String title;
        if (tab == AppTab.updates) {
          icon = Icons.notifications;
          key = PandemiaKeys.updatesTab;
          title = "Terbaru";
        } else if (tab == AppTab.stats) {
          icon = Icons.assessment;
          key = PandemiaKeys.statsTab;
          title = "Data";
        } else if (tab == AppTab.map) {
          icon = Icons.map;
          key = PandemiaKeys.mapTab;
          title = "Peta";
        } else if (tab == AppTab.hoax) {
          icon = Icons.burst_mode;
          key = PandemiaKeys.hoaxTab;
          title = "Hoax/Fakta";
        } else if (tab == AppTab.settings) {
          icon = Icons.dashboard;
          key = PandemiaKeys.settingsTab;
          title = "Setelan";
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
