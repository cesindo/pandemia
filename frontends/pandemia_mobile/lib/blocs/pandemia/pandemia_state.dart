import 'package:equatable/equatable.dart';
import 'package:meta/meta.dart';

@immutable
abstract class PandemiaState extends Equatable {
  PandemiaState([List props = const []]) : super(props);
}

class PandemiaLoading extends PandemiaState {
  @override
  String toString() => "PandemiaLoading";
}

class ValidateToken extends PandemiaState {
  @override
  String toString() => "ValidateToken";
}

class AuthorizeToken extends PandemiaState {
  @override
  String toString() => "AuthorizeToken";
}

class PandemiaReady extends PandemiaState {
  @override
  String toString() => "PandemiaReady";
}

class TimelineLoading extends PandemiaState {
  @override
  String toString() => "TimelineLoading";
}

class PandemiaFailure extends PandemiaState {
  final String error;

  PandemiaFailure(this.error);

  @override
  String toString() => "PandemiaFailure";
}

class PandemiaLocationFailure extends PandemiaState {
  final String error;

  PandemiaLocationFailure(this.error);

  @override
  String toString() => "PandemiaLocationFailure";
}

class LoadingSetting extends PandemiaState {
  @override
  String toString() => "LoadingSetting";
}

class PandemiaNewUpdateAvailable extends PandemiaState {
  final String version;
  final String notes;

  PandemiaNewUpdateAvailable(this.version, this.notes);

  @override
  String toString() => "PandemiaNewUpdateAvailable";
}
