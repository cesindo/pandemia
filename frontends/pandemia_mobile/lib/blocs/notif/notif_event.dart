import 'package:equatable/equatable.dart';
import 'package:meta/meta.dart';

@immutable
abstract class NotifEvent extends Equatable {
  NotifEvent([List props = const []]) : super(props);
}

class LoadNotif extends NotifEvent {
  LoadNotif();
  @override
  String toString() => "LoadNotif";
}

