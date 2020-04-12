import 'dart:async';

import 'package:flutter/material.dart';
import 'package:flutter/scheduler.dart';
import 'package:google_maps_flutter/google_maps_flutter.dart';
import 'package:location/location.dart';
import 'package:pandemia_mobile/blocs/profile/profile.dart';
import 'package:pandemia_mobile/models/user.dart';
import 'package:pandemia_mobile/screens/profile/location_picker.dart';
import 'package:pandemia_mobile/user_repository/user_repository.dart';
import 'package:pandemia_mobile/util/address_util.dart';
import 'package:pandemia_mobile/widgets/loading_indicator.dart';

class ProfileEditPage extends StatefulWidget {
  final ProfileBloc profileBloc;

  ProfileEditPage({this.profileBloc, Key key}) : super(key: key);

  @override
  _ProfileEditPageState createState() => _ProfileEditPageState(profileBloc);
}

class _ProfileEditPageState extends State<ProfileEditPage> {
  final ProfileBloc profileBloc;
  final UserRepository userRepository = UserRepository();
  final _formKey = GlobalKey<FormState>();
  final _fullNameCtl = TextEditingController();
  final _emailCtl = TextEditingController();
  final _phoneCtl = TextEditingController();
  final _locCtl = TextEditingController();
  final _scaffoldKey = GlobalKey<ScaffoldState>();
  StreamSubscription subs;
  LatLng location;
  User currentUser;
  bool _isLoading = false;

  _ProfileEditPageState(this.profileBloc);

  @override
  void initState() {
    currentUser = userRepository.currentUser;
    // _fullNameCtl.text = currentUser.fullName;
    // _emailCtl.text = currentUser.email;
    // _phoneCtl.text = currentUser.phoneNum;

    subs = profileBloc.state.listen((ProfileState state) {
      if (state is ProfileUpdated) {
        setState(() => _isLoading = false);
        Navigator.pop(context, state.profile);
      } else if (state is ProfileFailure) {
        _scaffoldKey.currentState.showSnackBar(
            SnackBar(content: Text(state.error), backgroundColor: Colors.red));
      } else if (state is ProfileUpdateLoading) {
        setState(() => _isLoading = true);
      }
    });
    super.initState();
  }

  @override
  void dispose() {
    super.dispose();
    subs.cancel();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      key: _scaffoldKey,
      appBar: AppBar(title: Text("Daftar Sebagai Satgas")),
      body: Builder(builder: (context) {
        return Center(
          child: ListView(children: <Widget>[
            Padding(
              padding: EdgeInsets.all(15.0),
              child: Form(
                key: _formKey,
                child: Column(
                  children: <Widget>[
                    TextFormField(
                      keyboardType: TextInputType.text,
                      textInputAction: TextInputAction.next,
                      decoration: InputDecoration(labelText: 'Nama Lengkap'),
                      controller: _fullNameCtl,
                      onFieldSubmitted: (_) => FocusScope.of(context).nextFocus(),
                      validator: (val) {
                        return val.isEmpty
                            ? "Nama lengkap tidak boleh kosong"
                            : null;
                      },
                    ),
                    TextFormField(
                      keyboardType: TextInputType.emailAddress,
                      textInputAction: TextInputAction.next,
                      decoration: InputDecoration(labelText: 'Alamat Email'),
                      controller: _emailCtl,
                      onFieldSubmitted: (_) => FocusScope.of(context).nextFocus(),
                      validator: (val) {
                        return val.isEmpty
                            ? "Alamat email tidak boleh kosong"
                            : null;
                      },
                    ),
                    TextFormField(
                      keyboardType: TextInputType.number,
                      textInputAction: TextInputAction.next,
                      decoration: InputDecoration(labelText: 'Nomor Telepon'),
                      controller: _phoneCtl,
                      onFieldSubmitted: (_) => FocusScope.of(context).nextFocus(),
                      validator: (val) {
                        return val.isEmpty
                            ? "Nomor telepon tidak boleh kosong"
                            : null;
                      },
                    ),
                    TextFormField(
                      controller: _locCtl,
                      readOnly: true,
                      onTap: () => showPlacePicker(),
                      validator: (val) {
                        return val.isEmpty ? "Lokasi tidak boleh kosong" : null;
                      },
                      decoration: InputDecoration(
                          labelText: 'Lokasi',
                          hintText: 'Pilih lokasi Anda',
                          suffixIcon: Icon(Icons.location_searching)),
                    ),
                    Container(
                      margin: EdgeInsets.only(top: 20.0, bottom: 10.0),
                      child: MaterialButton(
                        child: Text("Daftar",
                            style: TextStyle(color: Colors.white)),
                        minWidth: double.infinity,
                        height: 40.0,
                        color: Theme.of(context).buttonColor,
                        onPressed: () {
                          if (_formKey.currentState.validate()) {
                            profileBloc.dispatch(RegisterAsSatgas(
                                currentUser.copy(
                                    fullName: _fullNameCtl.text,
                                    email: _emailCtl.text,
                                    phoneNum: _phoneCtl.text),
                                location));
                          }
                        },
                      ),
                    ),
                    _isLoading == true ? LoadingIndicator() : Container()
                  ],
                ),
              ),
            )
          ]),
        );
      }),
    );
  }

  void showPlacePicker() async {
    final locationData = await Location().getLocation();
    Navigator.of(context)
        .push(MaterialPageRoute(
            builder: (ctx) => LocationPicker(
                  pinPosition: LatLng(locationData.latitude, locationData.longitude),
                )))
        .then((result) {
      if (result != null) {
        GeoLocation geoLoc = result["geoloc"];
        LatLng latLng = result["latlng"];
        _locCtl.text =
            "${geoLoc.subdistrict ?? "-"}, ${geoLoc.district ?? "-"}, ${geoLoc.city}, ${geoLoc.country}";
        setState(() => location = latLng);
      }
    });
  }
}
