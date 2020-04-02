import 'dart:async';
import 'dart:io';
import 'package:bloc/bloc.dart';
import 'package:firebase_messaging/firebase_messaging.dart';
import 'package:location/location.dart';
import 'package:meta/meta.dart';
import 'package:pandemia_mobile/api/pandemia_api.dart';
import 'package:pandemia_mobile/blocs/pandemia/pandemia_event.dart';
import 'package:pandemia_mobile/blocs/pandemia/pandemia_state.dart';
import 'package:pandemia_mobile/core/smart_repo.dart';
import 'package:pandemia_mobile/models/user_settings.dart';
import 'package:pandemia_mobile/notification_util.dart';
import 'package:pandemia_mobile/user_repository/user_repository.dart';
import 'package:pandemia_mobile/util/address_util.dart';
import 'package:pandemia_mobile/util/device_util.dart';

class PandemiaBloc extends Bloc<PandemiaEvent, PandemiaState> {
  final UserRepository userRepository;
  final PersistentSmartRepo repo = PersistentSmartRepo("pandemia");
  final NotificationUtil notifUtil = NotificationUtil();
  final FirebaseMessaging firebaseMessaging = new FirebaseMessaging();
  int _totalRetries = 0;

  PandemiaBloc({@required this.userRepository})
      : assert(userRepository != null);

  @override
  PandemiaState get initialState => PandemiaLoading();

  @override
  Stream<PandemiaState> mapEventToState(PandemiaEvent event) async* {
    // if (event is LoggedIn) {
    //   yield* _mapLoginPandemiaToState(event);
    // } else if (event is StartupEvent) {
    if (event is StartupEvent) {
      print("Got startup event");
      yield* _mapStartupToState(event);
      // } else if (event is LoggedOut) {
      //   yield* _mapLoggedOutToState(event);
    }
  }

  Stream<PandemiaState> _loadUserSettings() async* {
    yield LoadSettings();

    final resp = await PublicApi.get("/user/v1/settings");

    if (resp != null) {
      final List<dynamic> ss = resp["result"] as List<dynamic>;
      UserSettings settings =
          UserSettings(0, true, false, false, false, false, false);
      for (final s in ss) {
        if (s["s_key"] == "enable_push_notif") {
          settings = settings.copy(enablePushNotif: s["s_value"] == "true");
        } else if (s["s_key"] == "complaint_map") {
          settings = settings.copy(complaintMap: s["s_value"] == "true");
        } else if (s["s_key"] == "has_cough") {
          settings = settings.copy(hasCough: s["s_value"] == "true");
        } else if (s["s_key"] == "has_fever") {
          settings = settings.copy(hasFever: s["s_value"] == "true");
        } else if (s["s_key"] == "has_flu") {
          settings = settings.copy(hasFlu: s["s_value"] == "true");
        } else if (s["s_key"] == "has_headache") {
          settings = settings.copy(hasHeadache: s["s_value"] == "true");
        }
      }
      userRepository.currentUser =
          userRepository.currentUser.copy(settings: settings);
    }
  }

  Stream<PandemiaState> _mapStartupToState(StartupEvent event) async* {
    yield PandemiaLoading();

    final bool hasToken = await userRepository.hasToken();
    final locationData = await Location().getLocation();
    final locationName = await getLocationName(locationData);
    final deviceId = await DeviceUtil.getID();

    if (hasToken) {
      // validate token
      yield ValidateToken();

      final latestLocation = await repo.getData("latest_location");
      if (latestLocation != null &&
          latestLocation["loc_name"] != locationName) {
        print("[LOC] Changing location...");
        PublicApi.post("/user/v1/me/update_loc", {
          'device_id': deviceId,
          'location_name': locationName
        }).whenComplete(() {
          print("[LOC] Location changed");
          repo.putData("latest_location", {"loc_name": locationName});
        }).catchError((err) => print("[LOC_ERROR]: $err"));
      } else {
        print("[LOC] Location not changed");
      }

      // untuk memeriksa apakah access token-nya masih valid
      final user = await PublicApi.get("/user/v1/me/info").catchError((err) {
        print("error: $err");
      });

      if (user == null) {
        // invalid, reinitialize
        userRepository.deleteToken();
        ApiResource.accessToken = "";

        if (++_totalRetries == 5) {
          yield PandemiaFailure("Cannot connect to server :(");
          return;
        }

        yield* _mapStartupToState(event);
        return;
      }

      // yang ini akan meng-update currentUser di userRepository
      await userRepository.getUserInfo();

      yield* _loadUserSettings();

      yield PandemiaReady();
      return;
    }

    yield AuthorizeToken();

    final fcmToken = await firebaseMessaging.getToken();

    final data = await PublicApi.post("/auth/v1/device/authorize", {
      "device_id": deviceId,
      "fcm_token": fcmToken,
      "platform": Platform.isAndroid ? "android" : "ios",
      "location_name": locationName,
    });

    if (data != null) {
      print("/authorize success, resp data: $data");
      final String accessToken = data["result"]["token"] as String;
      print("access token: $accessToken");

      userRepository.persistToken(accessToken);
      repo.putData("latest_location", {"loc_name": locationName});

      yield* _loadUserSettings();

      yield PandemiaReady();
    } else {
      yield PandemiaFailure("Initialization failed :(");
    }
  }

  // Stream<PandemiaState> _mapLoggedOutToState(LoggedOut event) async* {
  //   yield AuthenticationLoading();
  //   await userRepository.deleteToken();
  //   ApiResource.accessToken = "";
  //   yield AuthenticationUnauthenticated();
  // }

}
