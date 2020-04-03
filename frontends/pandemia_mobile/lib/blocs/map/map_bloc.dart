import 'package:bloc/bloc.dart';
import 'package:google_maps_flutter/google_maps_flutter.dart';
import 'package:pandemia_mobile/api/pandemia_api.dart';
import 'package:pandemia_mobile/blocs/map/map_event.dart';
import 'package:pandemia_mobile/blocs/map/map_state.dart';
import 'package:pandemia_mobile/core/smart_repo.dart';
import 'package:pandemia_mobile/models/map_location.dart';
import 'package:pandemia_mobile/models/map_marker.dart';

class MapBloc extends Bloc<MapEvent, MapState> {
  PersistentSmartRepo repo;

  MapBloc() {
    repo = PersistentSmartRepo("bloc_map");
  }

  @override
  MapState get initialState => MapLoading();

  @override
  Stream<MapState> mapEventToState(MapEvent event) async* {
    if (event is LoadMap) {
      yield* _mapLoadMapToState(event);
    }
  }

  Stream<MapState> _mapLoadMapToState(LoadMap event) async* {
    if (event.withLoading) {
      yield MapLoading();
    }

    // final data = await Location().getLocation();

    // print(data);

    if (event.location != null) {
      final location =
          MapLocation(1, event.location.latitude, event.location.longitude);

      final currentLocation = await repo.getData("lat_long");
      final markers = await getMarkers(event.location);

      // if (currentLocation != null &&
      //     currentLocation["lat_long"] == location.toMap()) {

      if (currentLocation != null) {
        yield MapLoaded(MapLocation.fromMap(currentLocation), markers);
      }

      repo.putData("lat_long", location.toMap());
      // final markers = await getMarkers(event.location);
      yield MapUpdated(location, markers);
    } else {
      yield MapFailure(error: "Cannot get current location");
    }
  }

  Future<List<MapMarker>> getMarkers(LatLng location) async {
    final data = await PublicApi.get(
        "/map_area/v1/search?longitude=${location.longitude}&latitude=${location.latitude}&offset=0&limit=10");
    List<MapMarker> markers = [];

    if (data != null) {
      markers = (data["result"] as List<dynamic>)
          .map((a) => MapMarker.fromMap(a))
          .toList();
    }

    return markers;
  }
}
