import 'dart:async';
import 'package:flutter_typeahead/flutter_typeahead.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:pandemia_mobile/api/pandemia_api.dart';
import 'package:pandemia_mobile/blocs/profile/profile.dart';
import 'package:pandemia_mobile/core/error.dart';
import 'package:pandemia_mobile/core/smart_repo.dart';
import 'package:pandemia_mobile/models/sub_report.dart';
import 'package:pandemia_mobile/models/user.dart';
import 'package:pandemia_mobile/user_repository/user_repository.dart';
import 'package:pandemia_mobile/util/text_formatter.dart';
import 'package:pandemia_mobile/widgets/loading_indicator.dart';
import 'package:pandemia_mobile/util/string_extension.dart';

class ProfileEditPage extends StatefulWidget {
  final ProfileBloc profileBloc;
  final SubReport item;

  ProfileEditPage({this.profileBloc, this.item, Key key}) : super(key: key);

  @override
  _ProfileEditPageState createState() =>
      _ProfileEditPageState(profileBloc, this.item);
}

class _ProfileEditPageState extends State<ProfileEditPage> {
  final ProfileBloc profileBloc;

  // final _villageCtl = TextEditingController();
  final UserRepository userRepository = UserRepository();
  final _formKey = GlobalKey<FormState>();
  final _fullNameCtl = TextEditingController();
  final _emailCtl = TextEditingController();
  final _phoneCtl = TextEditingController();
  // final _locCtl = TextEditingController();
  final _villageCtl = TextEditingController();
  final _areaCodeCtl = TextEditingController();
  final _scaffoldKey = GlobalKey<ScaffoldState>();
  StreamSubscription subs;
  // LatLng location;
  User currentUser;
  bool _isLoading = false;
  // bool _medical = false;
  final SubReport item;
  FocusNode node = FocusNode();

  _ProfileEditPageState(this.profileBloc, this.item);

  @override
  void initState() {
    super.initState();

    currentUser = userRepository.currentUser;

    subs = profileBloc.state.listen((ProfileState state) {
      if (state is ProfileUpdated) {
        setState(() => _isLoading = false);
        Navigator.pop(context, [state.profile, _villageCtl.text]);
      } else if (state is ProfileFailure) {
        _scaffoldKey.currentState.showSnackBar(
            SnackBar(content: Text(state.error), backgroundColor: Colors.red));
        setState(() {
          _isLoading = false;
        });
      } else if (state is ProfileUpdateLoading) {
        setState(() => _isLoading = true);
      }
    });
    if (item != null) {
      _fullNameCtl.text = item.fullName;
    }
  }

  @override
  void dispose() {
    super.dispose();
    subs.cancel();
  }

  final appRepo = PersistentSmartRepo("pandemia");

  Future<List<dynamic>> getVillageSuggestions(String query) async {
    final geoLoc = await appRepo.getData("latest_loc_full");
    final locPath = geoLoc["loc_path"];
    return appRepo
        .fetchApi("village_suggestions",
            "/village/v1/search?query=$query&scope=$locPath&offset=0&limit=10",
            force: false)
        .then((data) async {
      // return PublicApi.get(
      //         "/village/v1/search?query=$query&scope=${currentUser.locPath}&offset=0&limit=10")
      //     .then((data) async {
      if (data != null) {
        List<dynamic> entries = data["entries"] as List;
        if (entries.length == 0) {
          // coba listing semuanya

          return await PublicApi.get(
                  "/village/v1/search?query=$query&offset=0&limit=10")
              .then((data2) {
            if (data2 != null) {
              List<dynamic> entries = data2["result"]["entries"] as List;
              return entries
                  .map((d) =>
                      "${d["name"]}, ${d["district_name"]}, ${d["city"]}, ${d["province"]}")
                  .toList();
            }
            return [];
          });
        } else {
          return entries
              .map((d) =>
                  "${d["name"]}, ${d["district_name"]}, ${d["city"]}, ${d["province"]}")
              .toList();
        }
      } else {
        throw PandemiaException(
            "Cannot contact API server for getting suggestions");
      }
    });
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
                      autofocus: true,
                      onFieldSubmitted: (_) =>
                          FocusScope.of(context).nextFocus(),
                      validator: (val) {
                        return val.isEmpty
                            ? "Nama lengkap tidak boleh kosong"
                            : null;
                      },
                      inputFormatters: [new TitleCaseTextFormatter()],
                    ),
                    TextFormField(
                      keyboardType: TextInputType.emailAddress,
                      textInputAction: TextInputAction.next,
                      decoration: InputDecoration(labelText: 'Alamat Email'),
                      controller: _emailCtl,
                      onFieldSubmitted: (_) =>
                          FocusScope.of(context).nextFocus(),
                      // validator: (val) {
                      //   return val.isEmpty
                      //       ? "Alamat email tidak boleh kosong"
                      //       : null;
                      // },
                    ),
                    TextFormField(
                      keyboardType: TextInputType.number,
                      textInputAction: TextInputAction.next,
                      decoration: InputDecoration(labelText: 'Nomor Telepon'),
                      controller: _phoneCtl,
                      onFieldSubmitted: (_) =>
                          FocusScope.of(context).nextFocus(),
                      validator: (val) {
                        return val.isEmpty
                            ? "Nomor telepon tidak boleh kosong"
                            : null;
                      },
                    ),

                    TypeAheadFormField(
                      textFieldConfiguration: TextFieldConfiguration(
                        focusNode: node,
                        controller: this._villageCtl,
                        onChanged: (_) {
                          setState(() {}); // just to trigger state change
                        },
                        decoration: InputDecoration(
                            labelText: "Desa",
                            //  border: InputBorder.none,
                            suffixIcon: this._villageCtl.text != ""
                                ? IconButton(
                                    icon: Icon(Icons.cancel),
                                    onPressed: () {
                                      setState(() {
                                        this._villageCtl.clear();
                                        node.requestFocus();
                                      });
                                    })
                                : null,
                            hintText: 'Alamat Desa'),
                      ),
                      suggestionsCallback: (query) {
                        return getVillageSuggestions(query);
                      },
                      itemBuilder: (context, suggestion) {
                        return ListTile(
                          title: Text(suggestion),
                        );
                      },
                      transitionBuilder: (context, suggestionsBox, controller) {
                        return suggestionsBox;
                      },
                      onSuggestionSelected: (suggestion) {
                        this._villageCtl.text = suggestion;
                      },
                      validator: (val) {
                        return val.isEmpty ? "Desa tidak boleh kosong" : null;
                      },
                      onSaved: (value) {},
                    ),

                    // TextFormField(
                    //   keyboardType: TextInputType.text,
                    //   textInputAction: TextInputAction.done,
                    //   decoration: InputDecoration(labelText: 'Nama Desa'),
                    //   controller: _villageCtl,
                    //   onFieldSubmitted: (_) => FocusScope.of(context).unfocus(),
                    //   validator: (val) {
                    //     return val.isEmpty
                    //         ? "Nama Desa tidak boleh kosong"
                    //         : null;
                    //   },
                    //   inputFormatters: [new TitleCaseTextFormatter()],
                    // ),

                    // TextFormField(
                    //   controller: _locCtl,
                    //   readOnly: true,
                    //   onTap: () => showPlacePicker(),
                    //   validator: (val) {
                    //     return val.isEmpty ? "Lokasi tidak boleh kosong" : null;
                    //   },
                    //   decoration: InputDecoration(
                    //       labelText: 'Lokasi',
                    //       hintText: 'Pilih lokasi Anda',
                    //       suffixIcon: Icon(Icons.location_searching)),
                    // ),
                    TextFormField(
                      textCapitalization: TextCapitalization.characters,
                      inputFormatters: [new UpperCaseTextFormatter()],
                      keyboardType: TextInputType.text,
                      textInputAction: TextInputAction.done,
                      decoration: InputDecoration(labelText: 'Kode Daerah'),
                      controller: _areaCodeCtl,
                      onFieldSubmitted: (_) => FocusScope.of(context).unfocus(),
                      validator: (val) {
                        return val.isEmpty
                            ? "Kode daerah tidak boleh kosong"
                            : null;
                      },
                    ),
                    Text("Dapatkan kode daerah dari pemerintah daerah Anda",
                        style: TextStyle(fontSize: 15)),

                    // LabeledCheckbox(
                    //   label: "Saya Petugas Medis",
                    //   value: this._medical,
                    //   onChanged: (value) {
                    //     setState(() {
                    //       this._medical = value;
                    //     });
                    //   },
                    // ),
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
                            showDialog(
                                context: context,
                                builder: (BuildContext context) {
                                  return AlertDialog(
                                    title: Text("Apakah Anda Petugas Medis?"),
                                    content: Text("Apakah Anda Petugas Medis?"),
                                    actions: <Widget>[
                                      new FlatButton(
                                          onPressed: () {
                                            List<String> s =
                                                _villageCtl.text.split(", ");
                                            String _village = _villageCtl.text;
                                            if (s.length > 2) {
                                              _village = s[0];
                                            }

                                            profileBloc.dispatch(
                                                RegisterAsSatgas(
                                                    currentUser.copy(
                                                        fullName:
                                                            _fullNameCtl.text,
                                                        email: _emailCtl.text
                                                            .trim(),
                                                        phoneNum:
                                                            _phoneCtl.text,
                                                        locPath: _villageCtl
                                                            .text
                                                            .capitalize(),
                                                        village: _village),
                                                    _areaCodeCtl.text,
                                                    true));
                                            Navigator.pop(context);
                                          },
                                          child: Text("Ya")),
                                      new FlatButton(
                                          onPressed: () {
                                            List<String> s =
                                                _villageCtl.text.split(", ");
                                            String _village = _villageCtl.text;
                                            if (s.length > 2) {
                                              _village = s[0];
                                            }

                                            profileBloc.dispatch(
                                                RegisterAsSatgas(
                                                    currentUser.copy(
                                                        fullName:
                                                            _fullNameCtl.text,
                                                        email: _emailCtl.text
                                                            .trim(),
                                                        phoneNum:
                                                            _phoneCtl.text,
                                                        locPath: _villageCtl
                                                            .text
                                                            .capitalize(),
                                                        village: _village),
                                                    _areaCodeCtl.text,
                                                    false));
                                            Navigator.pop(context);
                                          },
                                          child: Text("Bukan"))
                                    ],
                                  );
                                });
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

//   void showPlacePicker() async {
//     final locationData = await Location().getLocation();
//     Navigator.of(context)
//         .push(MaterialPageRoute(
//             builder: (ctx) => LocationPicker(
//                   pinPosition:
//                       LatLng(locationData.latitude, locationData.longitude),
//                 )))
//         .then((result) {
//       if (result != null) {
//         GeoLocation geoLoc = result["geoloc"];
//         LatLng latLng = result["latlng"];
//         _locCtl.text =
//             "${geoLoc.subdistrict ?? "-"}, ${geoLoc.district ?? "-"}, ${geoLoc.city}, ${geoLoc.country}";
//         setState(() => location = latLng);
//       }
//     });
//   }
}
