
import 'package:equatable/equatable.dart';
import 'package:meta/meta.dart';
import 'package:pandemia_mobile/models/info_location.dart';

@immutable
abstract class StatsState extends Equatable {
  StatsState([List props = const []]) : super(props);
}

/// Loading state
class StatsLoading extends StatsState {
  /// Set true to block screen with blocking loading modal box.
  final bool block;
  StatsLoading({this.block = false});
  @override
  String toString() => "StatsLoading";
}

class StatsLoaded extends StatsState {
  final List<InfoLocation> items;
  StatsLoaded(this.items);
  @override
  String toString() => "StatsLoaded";
}

class StatsUpdated extends StatsState {
  final List<InfoLocation> items;
  StatsUpdated(this.items);
  @override
  String toString() => "StatsUpdated";
}

/// State when error/failure occurred
class StatsFailure extends StatsState {
  final String error;
  StatsFailure({this.error}) : super([error]);
  @override
  String toString() => "StatsFailure";
}
