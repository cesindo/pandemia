import 'dart:io';
import 'package:bloc/bloc.dart';
import 'package:firebase_messaging/firebase_messaging.dart';
import 'package:pandemia_mobile/api/pandemia_api.dart';
import 'package:pandemia_mobile/blocs/fcm/fcm_event.dart';
import 'package:pandemia_mobile/blocs/fcm/fcm_state.dart';
import 'package:pandemia_mobile/throttle.dart';
import 'package:pandemia_mobile/util/device_util.dart';

class FcmBloc extends Bloc<FcmEvent, FcmState> {
  final FirebaseMessaging firebaseMessaging = new FirebaseMessaging();

  @override
  FcmState get initialState => FcmLoading();

  @override
  Stream<FcmState> mapEventToState(FcmEvent event) async* {
    if (event is CreateFcm) {
      yield* _mapCreateFcmToState(event);
    } else if (event is DeleteFcm) {
      yield* _mapDeleteToState(event);
    }
  }

  Stream<FcmState> _mapCreateFcmToState(CreateFcm event) async* {
    yield FcmLoading();

    if (!Throttle.isReady("bloc_notif.LoadNotif", within: 10000)) {
      print("bloc_notif.loadNotif throttled.");
      return;
    }

    String fcmToken = await firebaseMessaging.getToken();

    final data = await PublicApi.post("/user/v1/me/connect/create", {
      "device_id": await DeviceUtil.getID(),
      "app_id": fcmToken,
      "provider_name": Platform.isAndroid ? "android" : "ios"
    });
    print("APP_ID: $fcmToken");

    if (data != null) {
      print("resp data: $data");

      //repo.updateEntriesItem("entries", data["result"]);

      //yield FcmCreated(Fcm.fromMap(data["result"]));

      // dispatch(LoadFcm());
    } else {
      yield FcmFailure(error: "Cannot add Fcm");
    }
  }

  Stream<FcmState> _mapDeleteToState(DeleteFcm event) async* {
    yield FcmLoading();
    String fcmToken = await firebaseMessaging.getToken();

    final data = await PublicApi.post("/user/v1/me/connect/remove", {
      "app_id": fcmToken,
      "provider_name": Platform.isAndroid ? "android" : "iOS"
    });

    if (data != null) {
      firebaseMessaging.deleteInstanceID();
      yield FcmDeleted();
      // dispatch(LoadFcm(force: false));
    } else {
      yield FcmFailure(error: "Cannot delete Fcm");
    }
  }
}
