import 'dart:async';

import 'package:flutter/material.dart';
import 'package:google_maps_flutter/google_maps_flutter.dart';
import 'package:location/location.dart';
import 'package:pandemia_mobile/core/core.dart';
import 'package:pandemia_mobile/widgets/widgets.dart';

class MapPage extends StatefulWidget {
  @override
  _MapPageState createState() => _MapPageState();
}

class _MapPageState extends State<MapPage> {
  Completer<GoogleMapController> _controller = Completer();
  Location location = new Location();
  bool _serviceEnabled;
  PermissionStatus _permissionGranted;
  LocationData _locationData;
  Set<Marker> _markers = {};

  @override
  void initState() {
    super.initState();
    _initService();
    location.onLocationChanged.listen((LocationData currentLocation) {
      // listen change location
      if(mounted) {
        setState(() => _locationData = currentLocation);
      }
    });
  }

  _initService() async {
    _serviceEnabled = await location.serviceEnabled();
    if (!_serviceEnabled) {
      _serviceEnabled = await location.requestService();
      if (!_serviceEnabled) {
        return;
      }
    }

    _permissionGranted = await location.hasPermission();
    if (_permissionGranted == PermissionStatus.denied) {
      _permissionGranted = await location.requestPermission();
      if (_permissionGranted != PermissionStatus.granted) {
        return;
      }
    }

    _locationData = await location.getLocation();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(body: _getBody(context));
  }

  Widget _getBody(BuildContext context) {
    if (_locationData == null) {
      return LoadingIndicator(key: PandemiaKeys.loading);
    }
    var pinPosition = LatLng(_locationData.latitude, _locationData.longitude);
    return GoogleMap(
      mapType: MapType.normal,
      markers: _markers,
      initialCameraPosition: CameraPosition(
        target: pinPosition,
        zoom: 16,
        bearing: 30,
      ),
      myLocationEnabled: true,
      onMapCreated: (GoogleMapController controller) {
        _controller.complete(controller);
        setState(() {
          _markers.add(Marker(
              markerId: MarkerId("currentLocation"),
              position: pinPosition,
              icon: BitmapDescriptor.defaultMarker));
        });
      },
    );
  }
}
