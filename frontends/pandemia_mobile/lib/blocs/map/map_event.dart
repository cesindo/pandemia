import 'package:equatable/equatable.dart';
import 'package:meta/meta.dart';

@immutable
abstract class MapEvent extends Equatable {
  MapEvent([List props = const []]) : super(props);
}

class LoadMap extends MapEvent {
  final bool withLoading;
  LoadMap({this.withLoading=true});

  @override
  String toString() => "LoadMap";
}
