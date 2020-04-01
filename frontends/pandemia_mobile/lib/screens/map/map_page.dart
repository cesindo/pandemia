
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:google_maps_flutter/google_maps_flutter.dart';
import 'package:pandemia_mobile/blocs/map/map_bloc.dart';
import 'package:pandemia_mobile/blocs/map/map_state.dart';
import 'package:pandemia_mobile/core/core.dart';
import 'package:pandemia_mobile/widgets/widgets.dart';

class MapPage extends StatelessWidget {
  final MapBloc mapBloc;

  MapPage(this.mapBloc, {Key key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Scaffold(body: _getBody(context));
  }

  Widget _getBody(BuildContext context) {
    Set<Marker> markers;
    LatLng pinPosition;

    return BlocBuilder<MapBloc, MapState>(
        builder: (BuildContext context, MapState state) {
      if (state is MapLoading) {
        return LoadingIndicator(key: PandemiaKeys.loading);
      } else if (state is MapLoaded) {
        pinPosition = LatLng(state.location.lat, state.location.long);
        markers = {
          Marker(
              markerId: MarkerId("currentLocation"),
              position: pinPosition,
              icon: BitmapDescriptor.defaultMarker)
        };
      } else if (state is MapUpdated) {
        pinPosition = LatLng(state.location.lat, state.location.long);
        markers = {
          Marker(
              markerId: MarkerId("currentLocation"),
              position: pinPosition,
              icon: BitmapDescriptor.defaultMarker)
        };
      }

      return GoogleMap(
        mapType: MapType.normal,
        markers: markers,
        initialCameraPosition: CameraPosition(
          target: pinPosition,
          zoom: 16,
          bearing: 30,
        ),
        myLocationEnabled: true,
        // onMapCreated: (GoogleMapController controller) {
        //   _controller.complete(controller);
        // },
      );
    });
  }
}
