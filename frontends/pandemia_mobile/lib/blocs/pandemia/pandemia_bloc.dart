import 'dart:async';
import 'dart:io';
import 'package:bloc/bloc.dart';
import 'package:firebase_messaging/firebase_messaging.dart';
import 'package:meta/meta.dart';
import 'package:pandemia_mobile/api/pandemia_api.dart';
import 'package:pandemia_mobile/blocs/pandemia/pandemia_event.dart';
import 'package:pandemia_mobile/blocs/pandemia/pandemia_state.dart';
import 'package:pandemia_mobile/core/smart_repo.dart';
import 'package:pandemia_mobile/notification_util.dart';
import 'package:pandemia_mobile/user_repository/user_repository.dart';
import 'package:pandemia_mobile/util/device_util.dart';

class PandemiaBloc extends Bloc<PandemiaEvent, PandemiaState> {
  final UserRepository userRepository;
  final PersistentSmartRepo repo = PersistentSmartRepo("pandemia");
  final NotificationUtil notifUtil = NotificationUtil();
  final FirebaseMessaging firebaseMessaging = new FirebaseMessaging();

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

  Stream<PandemiaState> _mapStartupToState(StartupEvent event) async* {
    yield PandemiaLoading();

    final bool hasToken = await userRepository.hasToken();

    if (hasToken) {
      // validate token
      yield ValidateToken();

      final user = await userRepository.getUserInfo().catchError((err) {
        print("error: $err");
      });

      if (user == null) {
        // invalid, reinitialize
        userRepository.deleteToken();
        yield* _mapStartupToState(event);
        return;
      }

      yield PandemiaReady();
      return;
    }

    yield AuthorizeToken();

    final fcmToken = await firebaseMessaging.getToken();

    final data = await PublicApi.post("/auth/v1/device/authorize", {
      "device_id": await DeviceUtil.getID(),
      "fcm_token": fcmToken,
      "platform": Platform.isAndroid ? "android" : "ios"
    });

    if (data != null) {
      print("/authorize success, resp data: $data");
      final String accessToken = data["result"]["token"] as String;
      print("access token: $accessToken");

      userRepository.persistToken(accessToken);

      yield PandemiaReady();
    } else {
      yield PandemiaFailure("Initialization failed");
    }
  }

  // Stream<PandemiaState> _mapLoggedOutToState(LoggedOut event) async* {
  //   yield AuthenticationLoading();
  //   await userRepository.deleteToken();
  //   ApiResource.accessToken = "";
  //   yield AuthenticationUnauthenticated();
  // }

}
