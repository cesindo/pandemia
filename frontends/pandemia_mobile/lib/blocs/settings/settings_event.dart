
import 'package:equatable/equatable.dart';
import 'package:meta/meta.dart';
import 'package:pandemia_mobile/models/user_settings.dart';

@immutable
abstract class SettingsEvent extends Equatable {
  SettingsEvent([List props = const []]) : super(props);
}

class LoadSettings extends SettingsEvent {
  final bool force;
  LoadSettings({this.force=false});

  @override
  String toString() => "LoadSettings";
}

class SetSetting extends SettingsEvent {
  final String key;
  final String value;
  SetSetting(this.key, this.value);
  @override
  String toString() => "SetSetting";
}

/// Event to delete Settings
class DeleteSettings extends SettingsEvent {
  final UserSettings settings;
  DeleteSettings(this.settings);
  @override
  String toString() => "DeleteSettings";
}
  
