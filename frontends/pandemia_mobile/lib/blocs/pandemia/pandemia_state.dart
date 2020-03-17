import 'package:equatable/equatable.dart';
import 'package:meta/meta.dart';
import 'package:pandemia_mobile/models/models.dart';
import 'package:pandemia_mobile/api/pandemia_api.dart';

@immutable
abstract class PandemiaState extends Equatable {
  PandemiaState([List props = const []]): super(props);
}

class PandemiaLoading extends PandemiaState {
  @override
  String toString() => "PandemiaLoading";
}

// class NotifListLoading extends PandemiaState {
//   @override
//   String toString() => "NotifListLoading";
// }

class AuthenticationUninitialized extends PandemiaState {
  @override
  String toString() => 'AuthenticationUninitialized';
}

class AuthenticationAuthenticated extends PandemiaState {
  @override
  String toString() => 'AuthenticationAuthenticated';
}

class AuthenticationUnauthenticated extends PandemiaState {
  @override
  String toString() => 'AuthenticationUnauthenticated';
}

class AuthenticationLoading extends PandemiaState {
  @override
  String toString() => 'AuthenticationLoading';
}


class LoginFailed extends PandemiaState {
  @override
  String toString() => "LoginFailed";
}

class LoginSuccess extends PandemiaState {
  final Session session;
  
  LoginSuccess(this.session);

  @override
  String toString() => "LoginSuccess { session: $session }";
}

class TimelineLoading extends PandemiaState {
  @override
  String toString() => "TimelineLoading";
}

