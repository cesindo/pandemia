import 'dart:io';

import 'package:firebase_messaging/firebase_messaging.dart';
import 'package:flushbar/flushbar.dart';
import 'package:flutter/material.dart';
import 'package:flutter/widgets.dart';
import 'package:flutter_ringtone_player/flutter_ringtone_player.dart';
import 'package:pandemia_mobile/blocs/feed/feed_bloc.dart';
import 'package:pandemia_mobile/blocs/feed/feed_event.dart';
import 'package:pandemia_mobile/blocs/notif/notif_bloc.dart';
import 'package:pandemia_mobile/feed_attributes.dart';
import 'package:pandemia_mobile/user_repository/user_repository.dart';

class NotificationUtil {
  static NotificationUtil _singleton;

  FirebaseMessaging _firebaseMessaging = FirebaseMessaging();
  UserRepository _userRepository = UserRepository();
  NotifBloc _notifBloc;
  FeedBloc _feedBloc;
  bool _initialized = false;

  factory NotificationUtil() {
    if (_singleton == null) {
      _singleton = NotificationUtil._internal();
    }
    return _singleton;
  }

  NotificationUtil._internal();

  void init(BuildContext context, NotifBloc notifBloc, FeedBloc feedBloc) {
    if (_initialized){
      //throw Exception("Notification already initialized");
      return;
    }
    _initialized = true;
    // _userRepository.getUserInfo();
    // _chatBloc = BlocProvider.of<ChatBloc>(context);
    _notifBloc = notifBloc;
    _feedBloc = feedBloc;

    // if (Platform.isIOS) {
    //   getIOSPermrission(context);
    // } else {
    //   _sendFCMToken(context);
    // }

    _firebaseMessaging.configure(
      onMessage: (Map<String, dynamic> message) async {
        print("[NotifUtil] got firebase message: $message");
        buildLocalNotif(context, message);
      },
      onResume: (Map<String, dynamic> message) async {
        print('=> on resume $message');
        // navigateToItemDetail(context, message, _ws, true);
      },
      onLaunch: (Map<String, dynamic> message) async {
        print('=> on launch $message');
        // navigateToItemDetail(context, message, _ws, true);
      },
    );
  }

  void buildLocalNotif(BuildContext context, Map<String, dynamic> message) {
    try {
      int kind;
      if (Platform.isIOS) {
        kind = int.parse(message["kind"]);
      } else {
        kind = int.parse(message["data"]["kind"]);
      }

      FlutterRingtonePlayer.play(
        android: AndroidSounds.notification,
        ios: IosSounds.bell,
        looping: false,
        volume: 1,
      );

      String msgTitle = "";
      String msgBody = "";
      if (message.containsKey('aps')) {
        msgTitle = message['aps']['alert']['title'] as String;
        msgBody = message['aps']['alert']['body'] as String;
      } else {
        msgTitle = message['notification']['title'] as String;
        msgBody = message['notification']['body'] as String;
      }

      Flushbar(
          title: msgTitle,
          message: msgBody,
          // margin: EdgeInsets.only(top: 55),
          padding: EdgeInsets.symmetric(horizontal: 26, vertical: 20),
          flushbarStyle: FlushbarStyle.GROUNDED,
          flushbarPosition: FlushbarPosition.TOP,
          backgroundColor: Colors.blue.shade600,
          icon: Icon(
            IconsByKind[kind],
            size: 32,
            color: Colors.white,
          ),
          // leftBarIndicatorColor: Colors.white,
          onTap: (obj) {
            obj.dismiss();
          },
          duration: Duration(seconds: 5))
        ..show(context);

      _feedBloc.dispatch(LoadFeed());
    } catch (e) {
      print("ERROR: $e");
    }
  }

  // void _sendFCMToken(BuildContext context) {
  //   var fcmBloc = BlocProvider.of<FcmBloc>(context);
  //   fcmBloc.dispatch(CreateFcm());
  // }

  void getIOSPermission() {
    print("=> checking IOS permission");
    _firebaseMessaging.requestNotificationPermissions(
        IosNotificationSettings(sound: true, badge: true, alert: true));
    _firebaseMessaging.onIosSettingsRegistered
        .listen((IosNotificationSettings settings) {
      print("=> Settings registered: $settings");
      // _sendFCMToken(context);
    });
  }
}
