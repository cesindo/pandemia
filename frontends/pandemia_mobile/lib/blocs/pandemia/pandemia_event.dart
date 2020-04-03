
import 'package:equatable/equatable.dart';
import 'package:meta/meta.dart';


@immutable
abstract class PandemiaEvent extends Equatable {
  PandemiaEvent([List props = const []]) : super(props);
}

class StartupEvent extends PandemiaEvent {
  StartupEvent();
}

class LoginInfo {
  String email;
  String password;

  LoginInfo(this.email, this.password);
}

class AddComment extends PandemiaEvent {
  final String text;
  AddComment(this.text);
  @override
  String toString() => "AddComment";
}

class LoggedIn extends PandemiaEvent {
  final String token;

  LoggedIn({@required this.token}) : super([token]);

  @override
  String toString() => 'LoggedIn { token: $token }';
}

class LoggedOut extends PandemiaEvent {
  @override
  String toString() => 'LoggedOut';
}

