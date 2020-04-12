import 'package:flutter/material.dart';
import 'package:flutter_typeahead/flutter_typeahead.dart';
import 'package:google_maps_flutter/google_maps_flutter.dart';
import 'package:location/location.dart';
import 'package:pandemia_mobile/models/suggestion.dart';
import 'package:pandemia_mobile/util/address_util.dart';

class LocationPicker extends StatefulWidget {
  final LatLng pinPosition;
  LocationPicker({Key key, @required this.pinPosition}) : super(key: key);

  @override
  _LocationPickerState createState() => _LocationPickerState();
}

class _LocationPickerState extends State<LocationPicker> {
  Set<Marker> markers;
  GoogleMapController _controller;
  GeoLocation geoLoc;
  LatLng selPos;
  final _textController = TextEditingController();

  @override
  void initState() {
    super.initState();
    markers = {
      Marker(
        markerId: MarkerId("currentLocation"),
        position:
            LatLng(widget.pinPosition.latitude, widget.pinPosition.longitude),
        // infoWindow: InfoWindow(title: a.caption))
      )
    };
  }

  @override
  Widget build(BuildContext context) {
    List<Widget> widgets = [];
    widgets.add(GoogleMap(
      mapType: MapType.normal,
      markers: markers,
      initialCameraPosition: CameraPosition(
        target: widget.pinPosition,
        zoom: 16,
        bearing: 30,
      ),
      myLocationEnabled: true,
      onMapCreated: (GoogleMapController controller) {
        this._controller = controller;
      },
      onTap: pickPosition,
    ));

    widgets.add(Card(
      elevation: 3,
      margin: EdgeInsets.all(10),
      child: TypeAheadFormField<Suggestion>(
          textFieldConfiguration: TextFieldConfiguration(
            controller: _textController,
            decoration: InputDecoration(
              border: InputBorder.none,
              contentPadding:
                  EdgeInsets.symmetric(horizontal: 22, vertical: 14),
              hintText: 'Cari...',
              prefixIcon: Icon(Icons.location_on),
              suffixIcon: InkWell(
                child: Icon(Icons.clear),
                onTap: () {
                  WidgetsBinding.instance.addPostFrameCallback((_) {
                    _textController.clear();
                  });
                },
              ),
            ),
          ),
          itemBuilder: (BuildContext context, itemData) {
            return ListTile(
              title:
                  Text(itemData.label.split(", ").reversed.toList().join(", ")),
              subtitle: Text(
                  "${itemData.address.county ?? "-"}, ${itemData.address.country}",
                  style: TextStyle(fontSize: 14)),
            );
          },
          onSuggestionSelected: (suggestion) {
            getLatLongPosition(suggestion.locationId).then((ll) {
              if (mounted) {
                setState(() {
                  geoLoc = null;
                  _controller.moveCamera(CameraUpdate.newLatLng(ll));
                  markers = {
                    Marker(
                      markerId: MarkerId("currentLocation"),
                      position: LatLng(ll.latitude, ll.longitude),
                      infoWindow: InfoWindow(title: suggestion.label),
                    )
                  };
                });
              }
            });
          },
          suggestionsCallback: (String pattern) async {
            return await searchLocation(pattern);
          }),
    ));

    if (geoLoc != null) {
      Widget infoBar = AnimatedPositioned(
          duration: Duration(milliseconds: 200),
          child: Align(
            alignment: Alignment.bottomCenter,
            child: Card(
                margin: EdgeInsets.symmetric(horizontal: 8, vertical: 10),
                child: ListTile(
                  onTap: () {
                    Navigator.of(context).pop({
                      "latlng": selPos,
                      "geoloc": geoLoc,
                    });
                  },
                  contentPadding: EdgeInsets.all(8.0),
                  leading: Icon(Icons.location_searching, size: 40),
                  title: Text("Pilih lokasi ini"),
                  subtitle: Text(
                      "${geoLoc.subdistrict ?? "-"}, ${geoLoc.district ?? "-"}, ${geoLoc.city}, ${geoLoc.country}"),
                  trailing: IconButton(
                      icon: Icon(Icons.cancel),
                      onPressed: () {
                        if (mounted) {
                          setState(() {
                            geoLoc = null;
                          });
                        }
                      }),
                )),
          ));
      widgets.add(infoBar);
    }

    return Scaffold(
        appBar: AppBar(title: Text("Pilih Lokasi Anda")),
        body: Stack(overflow: Overflow.clip, children: widgets));
  }

  void pickPosition(LatLng ll) async {
    GeoLocation res = await getLocationName(LocationData.fromMap({
      "latitude": ll.latitude,
      "longitude": ll.longitude,
    }));

    if (mounted) {
      setState(() {
        geoLoc = res;
        selPos = ll;
        _controller.moveCamera(CameraUpdate.newLatLng(ll));
        markers = {
          Marker(
            markerId: MarkerId("currentLocation"),
            position: LatLng(ll.latitude, ll.longitude),
            infoWindow: InfoWindow(title: geoLoc.city),
          )
        };
      });
    }
  }
}
