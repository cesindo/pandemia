import 'package:bloc/bloc.dart';
import 'package:location/location.dart';
import 'package:pandemia_mobile/blocs/map/map_event.dart';
import 'package:pandemia_mobile/blocs/map/map_state.dart';
import 'package:pandemia_mobile/core/smart_repo.dart';
import 'package:pandemia_mobile/models/map_location.dart';

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
    final data = await Location().getLocation();

    print(data);

    if (data != null) {
      final location = MapLocation(1, data.latitude, data.longitude);
      final currentLocation = await repo.getData("location");
      if (currentLocation != null &&
          currentLocation["lat_long"] == location.toMap()) {
        yield MapLoaded(MapLocation.fromMap(currentLocation["lat_long"]));
      } else {
        repo.putData("lat_long", location.toMap());
        yield MapUpdated(location);
      }
    } else {
      yield MapFailure(error: "Cannot get current location");
    }
  }
}
