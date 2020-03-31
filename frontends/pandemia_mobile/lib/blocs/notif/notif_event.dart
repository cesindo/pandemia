import 'package:equatable/equatable.dart';
import 'package:meta/meta.dart';

@immutable
abstract class NotifEvent {
  NotifEvent([List props = const []]);
}

class LoadNotif extends NotifEvent {
  LoadNotif();
  @override
  String toString() => "LoadNotif";
}

