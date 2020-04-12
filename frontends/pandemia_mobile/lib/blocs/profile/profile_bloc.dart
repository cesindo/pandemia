
import 'package:bloc/bloc.dart';
import 'package:pandemia_mobile/api/pandemia_api.dart';
import 'package:pandemia_mobile/blocs/profile/profile_event.dart';
import 'package:pandemia_mobile/blocs/profile/profile_state.dart';
import 'package:pandemia_mobile/core/smart_repo.dart';
import 'package:pandemia_mobile/models/user.dart';

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
    }
    
  }

  Stream<ProfileState> _mapLoadProfileToState(LoadProfile event) async* {
    yield ProfileListLoading();

    final data = await PublicApi.get("/user/v1/me/info");

    if (data != null) {
      yield ProfileListLoaded((data["result"]["entries"] as List<dynamic>)
          .map((a) => User.fromMap(a))
          .toList());
    } else {
      yield ProfileFailure(error: "Cannot get profile data from server");
    }
  } 
}