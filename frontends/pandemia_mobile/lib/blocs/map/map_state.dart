import 'package:equatable/equatable.dart';
import 'package:meta/meta.dart';
import 'package:pandemia_mobile/models/map_location.dart';
import 'package:pandemia_mobile/models/map_marker.dart';

@immutable
abstract class MapState extends Equatable {
  MapState([List props = const []]) : super(props);
}

/// Loading state
class MapLoading extends MapState {
  /// Set true to block screen with blocking loading modal box.
  final bool block;
  MapLoading({this.block = false});
  @override
  String toString() => "MapLoading";
}

class MapLoaded extends MapState {
  final MapLocation location;
  final List<MapMarker> markers;

  MapLoaded(this.location, this.markers);

  @override
  String toString() => "MapLoaded";
}

class MapUpdated extends MapState {
  final MapLocation location;
  final List<MapMarker> markers;

  MapUpdated(this.location, this.markers);
  @override
  String toString() => "MapUpdated";
}

/// State when error/failure occurred
class MapFailure extends MapState {
  final String error;
  MapFailure({this.error}) : super([error]);
  @override
  String toString() => "MapFailure";
}
