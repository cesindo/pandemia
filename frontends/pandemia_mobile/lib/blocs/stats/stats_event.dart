
import 'package:equatable/equatable.dart';
import 'package:meta/meta.dart';

@immutable
abstract class StatsEvent extends Equatable {
  StatsEvent([List props = const []]) : super(props);
}

class LoadStats extends StatsEvent {
  final bool force;
  final bool withLoading;
  LoadStats({this.force=false, this.withLoading=true});

  @override
  String toString() => "LoadStats";
}
