import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:google_maps_flutter/google_maps_flutter.dart';
import 'package:pandemia_mobile/blocs/map/map_bloc.dart';
import 'package:pandemia_mobile/blocs/map/map_event.dart';
import 'package:pandemia_mobile/blocs/map/map_state.dart';
import 'package:pandemia_mobile/core/core.dart';
import 'package:pandemia_mobile/widgets/widgets.dart';

// class MapPage extends StatefulWidget {
//   final MapBloc mapBloc;
//   MapPage(this.mapBloc, {Key key}) : super(key: key);

//   @override
//   _MapPageState createState() => _MapPageState(this.mapBloc);
// }

// class _MapPageState extends State<MapPage> {
class MapPage extends StatelessWidget {
  final MapBloc mapBloc;
  BitmapDescriptor pinIcon;
  LatLng userPosition;
  LatLng pinPosition;
  LatLng movedPos;

  MapPage(this.mapBloc);

  // @override
  // void initState() {
  //   super.initState();
  // }

  @override
  Widget build(BuildContext context) {
    return Scaffold(body: _getBody(context));
  }

  Widget _getBody(BuildContext context) {
    Set<Marker> markers;

    BitmapDescriptor.fromAssetImage(
            ImageConfiguration(
                devicePixelRatio: MediaQuery.of(context).devicePixelRatio),
            'assets/img/sick-pin-icon2.png')
        .then((icon) {
      // setState(() {
      pinIcon = icon;
      // });
    });

    return BlocBuilder<MapBloc, MapState>(
        builder: (BuildContext context, MapState state) {
      if (state is MapLoading) {
        return LoadingIndicator(key: PandemiaKeys.loading);
      } else if (state is MapLoaded) {
        pinPosition = LatLng(state.location.lat, state.location.long);
        userPosition = pinPosition;

        markers = state.markers
            .map((a) => Marker(
                markerId: MarkerId(a.caption),
                position: LatLng(a.latitude, a.longitude),
                icon: pinIcon,
                infoWindow: InfoWindow(title: a.caption)))
            .toSet();
      } else if (state is MapUpdated) {
        pinPosition = LatLng(state.location.lat, state.location.long);

        markers = state.markers
            .map((a) => Marker(
                markerId: MarkerId(a.caption),
                position: LatLng(a.latitude - 0.000520, a.longitude - 0.000010),
                icon: pinIcon,
                infoWindow: InfoWindow(title: a.caption)))
            .toSet();
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
        onMapCreated: (GoogleMapController controller) {
          // _controller.complete(controller);
        },
        onCameraIdle: () {
          mapBloc.dispatch(LoadMap(movedPos, withLoading: false));
        },
        onCameraMove: (CameraPosition camPos) {
          // setState(() {
          movedPos = LatLng(camPos.target.latitude, camPos.target.longitude);
          // });
        },
      );
    });
  }
}
