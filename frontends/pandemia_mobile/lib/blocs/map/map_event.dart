import 'package:equatable/equatable.dart';
import 'package:google_maps_flutter/google_maps_flutter.dart';
import 'package:meta/meta.dart';

@immutable
abstract class MapEvent extends Equatable {
  MapEvent([List props = const []]) : super(props);
}

class LoadMap extends MapEvent {
  final bool withLoading;
  final LatLng location;

  LoadMap(this.location, {this.withLoading = true});

  @override
  String toString() => "LoadMap";
}
