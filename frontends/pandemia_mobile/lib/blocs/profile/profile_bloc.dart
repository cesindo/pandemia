import 'package:bloc/bloc.dart';
import 'package:location/location.dart';
import 'package:pandemia_mobile/api/pandemia_api.dart';
import 'package:pandemia_mobile/blocs/profile/profile_event.dart';
import 'package:pandemia_mobile/blocs/profile/profile_state.dart';
import 'package:pandemia_mobile/core/smart_repo.dart';
import 'package:pandemia_mobile/models/user.dart';
import 'package:pandemia_mobile/user_repository/user_repository.dart';

class ProfileBloc extends Bloc<ProfileEvent, ProfileState> {
  PersistentSmartRepo repo;

  ProfileBloc() {
    repo = PersistentSmartRepo("bloc_profile");
  }

  @override
  ProfileState get initialState => ProfileLoading();

  @override
  Stream<ProfileState> mapEventToState(ProfileEvent event) async* {
    if (event is LoadProfile) {
      yield* _mapLoadProfileToState(event);
    } else if (event is RegisterAsSatgas) {
      yield* _mapRegisterAsSatgasToState(event);
    }
  }

  Stream<ProfileState> _mapLoadProfileToState(LoadProfile event) async* {
    yield ProfileLoading();

    final data = await PublicApi.get("/user/v1/me/info");

    if (data != null) {
      yield ProfileLoaded(User.fromMap(data["result"]));
    } else {
      yield ProfileFailure(error: "Cannot get profile data from server");
    }
  }

  Stream<ProfileState> _mapRegisterAsSatgasToState(
      RegisterAsSatgas event) async* {
    yield ProfileUpdateLoading();
    final userRepository = UserRepository();
    final oldData = await userRepository.getLocalUserInfo();


    LocationData locationData;
    try {
      locationData = await Location().getLocation();
    } catch (e) {
      print("GET LOC ERROR: $e");
      yield ProfileFailure(error:
          "Gagal mendapatkan lokasi, pastikan Pandemia memiliki ijin untuk menggunakan lokasi di setelan HP Anda");
      return;
    }


    Map<String, dynamic> payload = {
      "full_name": event.user.fullName,
      "phone_num": event.user.phoneNum,
      "village": event.user.village,
      "loc_path": event.user.locPath,
      "latitude": locationData.latitude,
      "longitude": locationData.longitude,
      "area_code": event.areaCode,
      "is_medic": event.isMedic
    };

    if (event.user.email != "") {
      payload["email"] = event.user.email;
    }

    yield* PublicApi.post2("/user/v1/me/update", payload)
        .then((data) {
          if (data != null) {
            User updated = event.user.copy(
                isSatgas: true,
                settings: oldData.settings);
            userRepository.repo.putData("currentUser", updated.toMap());
            return ProfileUpdated(updated);
          } else {
            return ProfileFailure(
                error: "Tidak dapat mendaftar sebagai satgas.");
          }
        })
        .catchError((error) {
          return ProfileFailure(error: error.toString());
        })
        .whenComplete(() => dispatch(LoadProfile()))
        .asStream();
  }
}
