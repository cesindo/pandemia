import 'dart:async';
import 'dart:io';
import 'package:bloc/bloc.dart';
import 'package:firebase_messaging/firebase_messaging.dart';
import 'package:google_maps_flutter/google_maps_flutter.dart';
import 'package:location/location.dart';
import 'package:meta/meta.dart';
import 'package:package_info/package_info.dart';
import 'package:pandemia_mobile/api/pandemia_api.dart';
import 'package:pandemia_mobile/blocs/pandemia/pandemia_event.dart';
import 'package:pandemia_mobile/blocs/pandemia/pandemia_state.dart';
import 'package:pandemia_mobile/blocs/settings/settings_util.dart';
import 'package:pandemia_mobile/core/smart_repo.dart';
import 'package:pandemia_mobile/notification_util.dart';
import 'package:pandemia_mobile/throttle.dart';
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
    } else if (event is CheckForUpdate) {
      yield* _mapCheckForUpdateToState(event);
    }
  }

  Stream<PandemiaState> _loadUserSettings() async* {
    yield LoadingSetting();

    final resp = await PublicApi.get("/user/v1/settings");

    if (resp != null) {
      final settings = toUserSettings(resp["result"] as List<dynamic>);
      userRepository.currentUser =
          userRepository.currentUser.copy(settings: settings);
    }
  }

  Stream<PandemiaState> _mapStartupToState(StartupEvent event) async* {
    yield PandemiaLoading();

    if (Throttle.isReady("reinit_user")) {
      return;
    }

    final bool hasToken = await userRepository.hasToken();

    LocationData locationData;
    try {
      locationData = await Location().getLocation();
    } catch (e) {
      print("GET LOC ERROR: $e");
      yield PandemiaLocationFailure(
          "Gagal mendapatkan lokasi, pastikan Pandemia memiliki ijin untuk menggunakan lokasi di setelan HP Anda");
      return;
    }

    final GeoLocation geoLocName = await getLocationName(locationData);
    final deviceId = await DeviceUtil.getID();

    if (hasToken) {
      // validate token
      yield ValidateToken();

      // final latestLocation = await repo.getData("latest_loc");
      // if (latestLocation != null &&
      //     latestLocation["loc_name"] != geoLocName.city) {
        print("[LOC] Changing location...");
        PublicApi.post("/user/v1/me/update_loc", {
          'device_id': deviceId,
          'loc_name': geoLocName.city,
          'loc_name_full': geoLocName.toString()
        }).whenComplete(() {
          print("[LOC] Location changed");
          repo.putData("latest_loc", {"loc_name": geoLocName.city});
          repo.putData(
              "latest_loc_full", {"loc_full_name": geoLocName.toString()});
        }).catchError((err) => print("[LOC_ERROR]: $err"));
      // } else {
      //   print("[LOC] Location not changed");
      // }

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

        userRepository.currentUser = userRepository.currentUser
            .copy(loc: LatLng(locationData.latitude, locationData.longitude));
        return;
      }

      // yang ini akan meng-update currentUser di userRepository
      await userRepository.getUserInfo();

      yield* _loadUserSettings();

      userRepository.currentUser = userRepository.currentUser
          .copy(loc: LatLng(locationData.latitude, locationData.longitude));

      yield PandemiaReady();
      return;
    }

    yield AuthorizeToken();

    final fcmToken = await firebaseMessaging.getToken();

    final data = await PublicApi.post("/auth/v1/device/authorize", {
      "device_id": deviceId,
      "fcm_token": fcmToken,
      "platform": Platform.isAndroid ? "android" : "ios",
      "loc_name": geoLocName.city,
      "loc_name_full": geoLocName.toString(),
      "loc_lat": locationData.latitude,
      "loc_long": locationData.longitude,
    });

    if (data != null) {
      print("/authorize success, resp data: $data");
      final String accessToken = data["result"]["token"] as String;
      print("access token: $accessToken");

      userRepository.persistToken(accessToken);
      repo.putData("latest_loc", {"loc_name": geoLocName.city});
      repo.putData("latest_loc_full", {"loc_full_name": geoLocName.toString()});

      // yang ini akan meng-update currentUser di userRepository
      await userRepository.getUserInfo();

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

  Stream<PandemiaState> _mapCheckForUpdateToState(CheckForUpdate event) async* {
    PackageInfo pInfo = await PackageInfo.fromPlatform();

    String platform = "unknown";
    if (Platform.isAndroid) {
      platform = "android";
    } else if (Platform.isIOS) {
      platform = "ios";
    } else if (Platform.isWindows) {
      // realy?
      platform = "windows";
    }

    final data = await PublicApi.get(
        "/system/v1/check_version?version=${pInfo.version}&platform=$platform");

    if (data != null) {
      if (data["result"] != null) {
        if (data["result"]["new_update"] != null && data["result"]["new_update"] != "") {
          yield PandemiaNewUpdateAvailable(data["new_update"], data["notes"]);
          yield PandemiaReady();
        }
      }
    }
  }
}
