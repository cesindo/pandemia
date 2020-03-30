import 'package:flutter/widgets.dart';

class PandemiaKeys {
  // Home Screens
  static final homeScreen = const Key('__homeScreen__');
  static final snackbar = const Key('__snackbar__');
  static Key snackbarAction(String id) => Key('__snackbar_action_${id}__');

  // Timeline
  static final timelineLoading = const Key('__timelineLoading__');
  static final timelineItem = (String id) => Key('TimelineItem__$id');
  static final timelineItemTask = (String id) => Key('TimelineItem__${id}__Task');
  static final timelineItemNote = (String id) => Key('TimelineItem__${id}__Note');
  static final timelineList = Key('__timelineList__');

  // Screen
  static final addCommentScreen = const Key('__addCommentScreen__');

  // Tabs
  static final tabs = const Key('__tabs__');
  static final updatesTab = const Key('updatesTab__');
  static final statsTab = const Key('statsTab__');
  static final settingsTab = const Key('__dashboardTab__');
  static final hoaxTab = const Key('__hoaxTab__');
  static final mapTab = const Key('__mapTab__');

  // Notif
  static final notifList = const Key('__notifList__');

  // etc
  static final commentField = const Key('__commentField__');
  static final loading = const Key('__loading__');
  static final logo = "__logo__";
}

class PandemiaRoutes {
  static final login = "/login";  
  static final about = "/about";
}
