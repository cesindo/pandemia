
import 'package:equatable/equatable.dart';
import 'package:meta/meta.dart';

@immutable
abstract class StatsEvent extends Equatable {
  StatsEvent([List props = const []]) : super(props);
}

class LoadStats extends StatsEvent {
  final bool force;
  LoadStats({this.force=false});

  @override
  String toString() => "LoadStats";
}
