import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:google_maps_flutter/google_maps_flutter.dart';
import 'package:intl/intl.dart';
import 'package:pandemia_mobile/blocs/map/map_bloc.dart';
import 'package:pandemia_mobile/blocs/map/map_event.dart';
import 'package:pandemia_mobile/blocs/map/map_state.dart';
import 'package:pandemia_mobile/core/core.dart';
import 'package:pandemia_mobile/core/map_marker_kind.dart';
import 'package:pandemia_mobile/models/map_marker.dart';
import 'package:pandemia_mobile/widgets/widgets.dart';

class BannerImages {
  static Map<int, Widget> _bannerImages = {
    MapMarkerKind.unknown: Image.asset(
      "assets/img/pandemia-logo-64.png",
      fit: BoxFit.cover,
    ),
    MapMarkerKind.pandemicInfo: Image.asset(
      "assets/img/pandemic-icon-128.png",
      fit: BoxFit.cover,
    ),
    MapMarkerKind.sick: Image.asset(
      "assets/img/sick-icon-128.png",
      fit: BoxFit.cover,
    ),
    MapMarkerKind.hospital: Image.asset(
      "assets/img/hospital-128.png",
      fit: BoxFit.cover,
    )
  };

  static Widget get(int kind) {
    if (_bannerImages.containsKey(kind)) {
      return _bannerImages[kind];
    } else {
      return _bannerImages[0];
    }
  }
}

class PinIcons {
  static final PinIcons _instance = PinIcons._internal();

  Map<int, BitmapDescriptor> _kindToPinImage = {
    MapMarkerKind.pandemicInfo: null,
    MapMarkerKind.sick: null,
    MapMarkerKind.hospital: null,
  };

  BitmapDescriptor _unknown;

  factory PinIcons() {
    BitmapDescriptor.fromAssetImage(ImageConfiguration(devicePixelRatio: 3.0),
            'assets/img/pandemia-logo-32.png')
        .then((icon) {
      _instance._unknown = icon;
    });

    BitmapDescriptor.fromAssetImage(ImageConfiguration(devicePixelRatio: 3.0),
            'assets/img/sick-pin-icon2.png')
        .then((icon) {
      _instance._kindToPinImage[MapMarkerKind.sick] = icon;
    });

    BitmapDescriptor.fromAssetImage(ImageConfiguration(devicePixelRatio: 3.0),
            'assets/img/pandemic-icon-32.png')
        .then((icon) {
      _instance._kindToPinImage[MapMarkerKind.pandemicInfo] = icon;
    });

    BitmapDescriptor.fromAssetImage(ImageConfiguration(devicePixelRatio: 3.0),
            'assets/img/medical-pin-32.png')
        .then((icon) {
      _instance._kindToPinImage[MapMarkerKind.hospital] = icon;
    });
    return _instance;
  }

  BitmapDescriptor get(int kind) {
    if (_kindToPinImage.containsKey(kind)) {
      return _kindToPinImage[kind];
    } else {
      return _unknown;
    }
  }

  PinIcons._internal() {}
}

class MapPage extends StatefulWidget {
  final MapBloc mapBloc;
  MapPage(this.mapBloc, {Key key}) : super(key: key);

  @override
  _MapPageState createState() => _MapPageState(mapBloc);
}

class _MapPageState extends State<MapPage> {
// class MapPage extends StatelessWidget {
  final MapBloc mapBloc;
  // BitmapDescriptor pinIcon;
  LatLng userPosition;
  LatLng pinPosition;
  LatLng movedPos;
  MapMarker curSel;
  TextStyle curSelTextStyle =
      TextStyle(fontWeight: FontWeight.bold, fontSize: 20);
  PinIcons pinIcons;

  _MapPageState(this.mapBloc) {}

  @override
  void initState() {
    super.initState();
    pinIcons = PinIcons();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(body: _getBody(context));
  }

  Widget _getBody(BuildContext context) {
    Set<Marker> markers;

    return BlocBuilder<MapBloc, MapState>(
        builder: (BuildContext context, MapState state) {
      if (state is MapLoading) {
        return LoadingIndicator(key: PandemiaKeys.loading);
      } else if (state is MapLoaded) {
        pinPosition = LatLng(state.location.lat, state.location.long);
        userPosition = pinPosition;

        markers = state.markers
            .map((a) => Marker(
                markerId: MarkerId("marker-${a.kind}-${a.caption}"),
                position: LatLng(a.latitude, a.longitude),
                icon: pinIcons.get(a.kind),
                infoWindow: InfoWindow(title: a.caption)))
            .toSet();
      } else if (state is MapUpdated) {
        pinPosition = LatLng(state.location.lat, state.location.long);

        markers = state.markers
            .map((a) => Marker(
                markerId: MarkerId("marker-${a.kind}-${a.caption}"),
                position: LatLng(a.latitude, a.longitude),
                icon: pinIcons.get(a.kind),
                infoWindow: InfoWindow(title: a.caption),
                onTap: () {
                  final _curSel = a;
                  setState(() {
                    curSel = _curSel;
                  });
                }))
            .toSet();
      }

      Widget map = GoogleMap(
        mapType: MapType.normal,
        markers: markers,
        initialCameraPosition: CameraPosition(
          target: pinPosition,
          zoom: 16,
          bearing: 30,
        ),
        myLocationEnabled: true,
        onTap: (ll) {
          setState(() {
            curSel = null;
          });
        },
        // onMapCreated: (GoogleMapController controller) {
        //   // _controller.complete(controller);
        // },
        onCameraIdle: () {
          mapBloc.dispatch(LoadMap(movedPos, withLoading: false));
        },
        onCameraMove: (CameraPosition camPos) {
          movedPos = LatLng(camPos.target.latitude, camPos.target.longitude);
        },
      );

      List<Widget> widgets = [map];

      if (curSel != null) {
        Widget infoBar = AnimatedPositioned(
            duration: Duration(milliseconds: 200),
            child: Align(
              alignment: Alignment.bottomCenter,
              child: Container(
                  // color: Colors.white,
                  margin: EdgeInsets.all(5),
                  height: 250,
                  decoration: BoxDecoration(
                      color: Colors.white,
                      borderRadius: BorderRadius.all(Radius.circular(20)),
                      boxShadow: <BoxShadow>[
                        BoxShadow(
                            blurRadius: 20,
                            offset: Offset.zero,
                            color: Colors.grey.withOpacity(0.5))
                      ]),
                  child: Column(
                    children: <Widget>[
                      Row(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        mainAxisAlignment: MainAxisAlignment.start,
                        children: <Widget>[
                          Container(
                              child: Align(
                                  alignment: Alignment.topCenter,
                                  child: Padding(
                                    padding: EdgeInsets.all(10),
                                    child: BannerImages.get(curSel.kind),
                                  )),
                              width: 70),
                          Expanded(
                              child: Padding(
                            padding: EdgeInsets.only(top: 10, left: 10),
                            child: Align(
                              alignment: Alignment.topLeft,
                              child: Column(
                                crossAxisAlignment: CrossAxisAlignment.start,
                                mainAxisAlignment: MainAxisAlignment.start,
                                children: <Widget>[
                                  Text(
                                    curSel.caption,
                                    style: curSelTextStyle,
                                    textAlign: TextAlign.left,
                                  ),
                                  Text(curSel.desc,
                                      style: TextStyle(fontSize: 18)),
                                ],
                              ),
                            ),
                          ))
                        ],
                      ),
                      Padding(
                          padding: EdgeInsets.only(top: 10),
                          child: _buildDetailInfo(context))
                    ],
                  )),
            ));
        widgets.add(infoBar);
      }

      return Stack(
        children: widgets,
      );
    });
  }

  final numfa = new NumberFormat("#,##0", "en_US");

  Widget _buildDetailInfo(BuildContext context) {
    // var size = MediaQuery.of(context).size;
    // final double itemHeight = (size.height - kToolbarHeight - 24) / 3.5;
    // final double itemWidth = size.width / 1;

    if (curSel.kind == MapMarkerKind.pandemicInfo) {
      return _buildPandemicInfo(context);
    } else if (curSel.kind == MapMarkerKind.hospital) {
      return _buildHospitalInfo(context);
    } else {
      return Container();
    }
  }

  Widget _buildHospitalInfo(BuildContext context) {
    var size = MediaQuery.of(context).size;
    return Column(
      mainAxisAlignment: MainAxisAlignment.start,
      crossAxisAlignment: CrossAxisAlignment.start,
      children: <Widget>[
        Padding(
            padding: EdgeInsets.only(left: 10,bottom: 10),
            child: Text("Tempat Tidur:", style: TextStyle(fontSize: 15))),
        GridView.count(
          shrinkWrap: true,
          primary: false,
          crossAxisCount: 4,
          childAspectRatio: size.height / 500,
          children: <Widget>[

            Padding(
              padding: const EdgeInsets.symmetric(horizontal: 5.0),
              child: Column(children: [
                Text(numfa.format(curSel.occupationDetail.usedTotal),
                    style: TextStyle(fontSize: 20)),
                SizedBox(height: 5),
                Expanded(
                  child: Text(
                    "Terisi",
                    textAlign: TextAlign.center,
                    style: TextStyle(
                      fontSize: 15,
                      fontWeight: FontWeight.w300,
                    ),
                  ),
                ),
              ]),
            ),
            Padding(
              padding: const EdgeInsets.symmetric(horizontal: 5.0),
              child: Column(children: [
                Text(numfa.format(curSel.occupationDetail.vacTotal),
                    style: TextStyle(fontSize: 20)),
                SizedBox(height: 5),
                Expanded(
                  child: Text(
                    "Kosong",
                    textAlign: TextAlign.center,
                    style: TextStyle(
                      fontSize: 15,
                      fontWeight: FontWeight.w300,
                    ),
                  ),
                ),
              ]),
            ),
            Padding(
              padding: const EdgeInsets.symmetric(horizontal: 1.0),
              child: Column(children: [
                Text(
                    numfa.format(curSel.occupationDetail.usedTotal +
                        curSel.occupationDetail.vacTotal),
                    style: TextStyle(fontSize: 20)),
                SizedBox(height: 5),
                Expanded(
                  child: Text(
                    "Total",
                    textAlign: TextAlign.center,
                    style: TextStyle(
                      fontSize: 15,
                      fontWeight: FontWeight.w300,
                    ),
                  ),
                ),
              ]),
            ),
            Padding(
              padding: const EdgeInsets.symmetric(horizontal: 5.0),
              child: Column(children: [
                Text(numfa.format(curSel.occupationDetail.waiting),
                    style: TextStyle(fontSize: 20)),
                SizedBox(height: 5),
                Expanded(
                  child: Text(
                    "Antri",
                    textAlign: TextAlign.center,
                    style: TextStyle(
                      fontSize: 15,
                      fontWeight: FontWeight.w300,
                    ),
                  ),
                ),
              ]),
            ),
          ],
        ),
        Align(alignment: Alignment.bottomCenter, child: Text("terakhir diperbaharui: " + curSel.occupationDetail.lastUpdated, style: TextStyle(fontSize: 11)))
      ],
    );
  }

  Widget _buildPandemicInfo(BuildContext context) {
    var size = MediaQuery.of(context).size;
    return GridView.count(
      shrinkWrap: true,
      primary: false,
      crossAxisCount: 3,
      childAspectRatio: size.height / 400,
      children: <Widget>[
        Padding(
          padding: const EdgeInsets.symmetric(horizontal: 1.0),
          child: Column(children: [
            Text(numfa.format(curSel.pandemicDetail.totalCases),
                style: TextStyle(fontSize: 26, color: Colors.red)),
            SizedBox(height: 5),
            Expanded(
              child: Text(
                "Positif",
                textAlign: TextAlign.center,
                style: TextStyle(
                  fontSize: 15,
                  fontWeight: FontWeight.w300,
                ),
              ),
            ),
          ]),
        ),
        Padding(
          padding: const EdgeInsets.symmetric(horizontal: 5.0),
          child: Column(children: [
            Text(numfa.format(curSel.pandemicDetail.totalRecovered),
                style: TextStyle(fontSize: 26, color: Colors.green)),
            SizedBox(height: 5),
            Expanded(
              child: Text(
                "Sembuh",
                textAlign: TextAlign.center,
                style: TextStyle(
                  fontSize: 15,
                  fontWeight: FontWeight.w300,
                ),
              ),
            ),
          ]),
        ),
        Padding(
          padding: const EdgeInsets.symmetric(horizontal: 5.0),
          child: Column(children: [
            Text(numfa.format(curSel.pandemicDetail.totalDeaths),
                style: TextStyle(fontSize: 26, color: Colors.grey)),
            SizedBox(height: 5),
            Expanded(
              child: Text(
                "Meninggal",
                textAlign: TextAlign.center,
                style: TextStyle(
                  fontSize: 15,
                  fontWeight: FontWeight.w300,
                ),
              ),
            ),
          ]),
        ),
      ],
    );
  }
}
