import 'package:pandemia_mobile/blocs/settings/settings_util.dart';
import 'package:pandemia_mobile/core/smart_repo.dart';

import 'package:bloc/bloc.dart';
import 'package:pandemia_mobile/api/pandemia_api.dart';
import 'package:pandemia_mobile/blocs/settings/settings_event.dart';
import 'package:pandemia_mobile/blocs/settings/settings_state.dart';

class SettingsBloc extends Bloc<SettingsEvent, SettingsState> {
  PersistentSmartRepo repo;

  SettingsBloc() {
    repo = PersistentSmartRepo("bloc_settings");
  }

  @override
  SettingsState get initialState => SettingsLoading();

  @override
  Stream<SettingsState> mapEventToState(SettingsEvent event) async* {
    if (event is LoadSettings) {
      yield* _mapLoadSettingsToState(event);
    } else if (event is SetSetting) {
      yield* _mapSetSettingToState(event);
    }
  }

  Stream<SettingsState> _mapLoadSettingsToState(LoadSettings event) async* {
    yield SettingsLoading();

    final data =
        await repo.fetchApi("entries", "/user/v1/settings", force: event.force);

    if (data != null) {
      final settings = toUserSettings(data as List<dynamic>);
      yield SettingsLoaded(settings);
    } else {
      yield SettingsFailure(error: "Cannot get settings data from server");
    }
  }

  Stream<SettingsState> _mapSetSettingToState(SetSetting event) async* {
    yield SettingsLoading();

    final data = await PublicApi.post(
        "/user/v1/update_setting", {"key": event.key, "value": event.value});

    if (data != null) {
      print("resp data: $data");
      yield SettingsUpdated(event.key, event.value);
    } else {
      yield SettingsFailure(error: "Cannot add Model");
    }
  }
}

class CreateSetting {}
