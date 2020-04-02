import 'package:equatable/equatable.dart';
import 'package:meta/meta.dart';
import 'package:pandemia_mobile/models/user_settings.dart';

@immutable
abstract class SettingsState extends Equatable {
  SettingsState([List props = const []]) : super(props);
}

/// Loading state
class SettingsLoading extends SettingsState {
  /// Set true to block screen with blocking loading modal box.
  final bool block;
  SettingsLoading({this.block = false});
  @override
  String toString() => "SettingsLoading";
}

class SettingsLoaded extends SettingsState {
  final UserSettings items;
  SettingsLoaded(this.items);
  @override
  String toString() => "SettingsLoaded";
}

class SettingsUpdated extends SettingsState {
  final String key;
  final String value;

  SettingsUpdated(this.key, this.value);

  @override
  String toString() => "SettingsUpdated";
}

/// State when error/failure occurred
class SettingsFailure extends SettingsState {
  final String error;
  SettingsFailure({this.error}) : super([error]);
  @override
  String toString() => "SettingsFailure";
}
