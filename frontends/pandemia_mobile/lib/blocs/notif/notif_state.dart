import 'package:equatable/equatable.dart';
import 'package:meta/meta.dart';
import 'package:pandemia_mobile/models/models.dart';

@immutable
abstract class NotifState {
  NotifState([List props = const []]);
}

class NotifListLoading extends NotifState {
  @override
  String toString() => "NotifLoading";
}

class NotifListLoaded extends NotifState {
  final List<NotifItem> notifs;

  NotifListLoaded(this.notifs) : super([notifs]);

  @override
  String toString() => "NotifListLoaded";
}

