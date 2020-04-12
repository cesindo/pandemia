import 'dart:async';

import 'package:flutter/material.dart';
import 'package:multiselect_formfield/multiselect_formfield.dart';
import 'package:pandemia_mobile/blocs/settings/settings.dart';
import 'package:pandemia_mobile/blocs/settings/settings_bloc.dart';
import 'package:pandemia_mobile/user_repository/user_repository.dart';
import 'package:pandemia_mobile/widgets/filter_location.dart';

class SettingScreen extends StatefulWidget {
  final SettingsBloc settingsBloc;
  SettingScreen({Key key, @required this.settingsBloc}) : super(key: key);

  @override
  _SettingScreenState createState() => _SettingScreenState(this.settingsBloc);
}

class _SettingScreenState extends State<SettingScreen> {
  final UserRepository _userRepo = UserRepository();
  final SettingsBloc settingsBloc;
  bool _onTap = false;
  bool _pushIsChecked = false;
  bool _petaIsChecked = false;
  bool _isBatuk = false;
  bool _isDemam = false;
  bool _isCold = false;
  bool _isPusing = false;
  StreamSubscription<SettingsState> subs;

  _SettingScreenState(this.settingsBloc) {}

  @override
  void initState() {
    super.initState();
    _pushIsChecked = _userRepo.currentUser.settings.enablePushNotif;
    _petaIsChecked = _userRepo.currentUser.settings.complaintMap;
    _isBatuk = _userRepo.currentUser.settings.hasCough;
    _isDemam = _userRepo.currentUser.settings.hasFever;
    _isCold = _userRepo.currentUser.settings.hasCold;
    _isPusing = _userRepo.currentUser.settings.hasHeadache;

    subs = settingsBloc.state.listen((SettingsState state) {
      if (state is SettingsUpdated) {
        if (state.key == "enable_push_notif") {
          _userRepo.currentUser = _userRepo.currentUser.copy(
              settings: _userRepo.currentUser.settings
                  .copy(enablePushNotif: state.value == "true"));
        } else if (state.key == "complaint_map") {
          _userRepo.currentUser = _userRepo.currentUser.copy(
              settings: _userRepo.currentUser.settings
                  .copy(complaintMap: state.value == "true"));
        } else if (state.key == "has_cough") {
          _userRepo.currentUser = _userRepo.currentUser.copy(
              settings: _userRepo.currentUser.settings
                  .copy(hasCough: state.value == "true"));
        } else if (state.key == "has_fever") {
          _userRepo.currentUser = _userRepo.currentUser.copy(
              settings: _userRepo.currentUser.settings
                  .copy(hasFever: state.value == "true"));
        } else if (state.key == "has_cold") {
          _userRepo.currentUser = _userRepo.currentUser.copy(
              settings: _userRepo.currentUser.settings
                  .copy(hasCold: state.value == "true"));
        } else if (state.key == "has_headache") {
          _userRepo.currentUser = _userRepo.currentUser.copy(
              settings: _userRepo.currentUser.settings
                  .copy(hasHeadache: state.value == "true"));
        }
      }
    });
  }

  @override
  void dispose() {
    subs.cancel();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Container(
      child: Column(
        mainAxisAlignment: MainAxisAlignment.start,
        crossAxisAlignment: CrossAxisAlignment.start,
        children: <Widget>[
          Padding(
            padding: EdgeInsets.only(left: 15, top: 20, bottom: 5),
            child: Row(
              children: <Widget>[
                Checkbox(
                    value: _pushIsChecked,
                    onChanged: (value) {
                      setState(() {
                        _pushIsChecked = value;
                        settingsBloc.dispatch(SetSetting("enable_push_notif",
                            _pushIsChecked ? "true" : "false"));
                        _onTap = true;
                      });
                    }),
                // Text("[ ]"),
                Text(
                  "Push Notif",
                  style: TextStyle(
                    fontSize: 18,
                  ),
                )
              ],
            ),
          ),
          Padding(
            padding: EdgeInsets.only(bottom: 10, left: 30, right: 30),
            child: Center(
              child: _pushIsChecked == true
                  ? FilterLocation(disabled: false)
                  : FilterLocation(disabled: true),
            ),
          ),
          Divider(),
          Padding(
            padding: EdgeInsets.only(left: 15, top: 0, bottom: 0),
            child: Row(
              children: <Widget>[
                Checkbox(
                  value: _petaIsChecked,
                  onChanged: (value) {
                    _petaIsChecked == true
                        ? null
                        : showDialog(
                            context: context,
                            builder: (BuildContext context) {
                              return AlertDialog(
                                title: Row(
                                  children: <Widget>[
                                    Padding(
                                      padding: EdgeInsets.only(right: 10),
                                      child: Icon(
                                        Icons.info,
                                        color: Colors.purple[400],
                                      ),
                                    ),
                                    Text(
                                      "Informasi",
                                      style: TextStyle(color: Colors.purple),
                                    )
                                  ],
                                ),
                                content: Text(
                                    "Fitur ini memungkinkan anda untuk mendapatkan info daerah sekitar kita tentang pandemi Covid-19 ( Corona )"),
                                actions: <Widget>[
                                  Center(
                                    child: FlatButton(
                                      child: Text("OKE"),
                                      onPressed: () {
                                        Navigator.of(context).pop();
                                      },
                                    ),
                                  )
                                ],
                              );
                            });

                    setState(() {
                      _petaIsChecked = value;
                      settingsBloc.dispatch(SetSetting(
                          "complaint_map", _petaIsChecked ? "true" : "false"));
                    });
                  },
                ),
                Text(
                  "Peta Keluhan",
                  style: TextStyle(
                    fontSize: 18,
                  ),
                )
              ],
            ),
          ),
          Padding(
            padding: EdgeInsets.only(left: 30, right: 30, bottom: 15),
            child: Text(
                "Menandai daerah keberadaan kita dengan keluhan kita, data hanya dalam bentuk " +
                    "statistik anonim (tidak ada data pribadi yang ditampilkan), " +
                    "fitur ini mempermudah kita dalam melakukan tracing.",
                maxLines: 5,
                style: TextStyle(fontSize: 16)),
          ),
          Padding(
            padding: EdgeInsets.only(left: 30, bottom: 15),
            child: Text(
              "Keluhan saya :",
              style: TextStyle(
                fontSize: 18,
              ),
            ),
          ),
          Padding(
            padding: EdgeInsets.only(left: 45),
            child: Column(
              children: <Widget>[
                Container(
                  height: 35,
                  child: Row(
                    children: <Widget>[
                      Checkbox(
                        value: _isBatuk,
                        onChanged: _petaIsChecked
                            ? (value) {
                                setState(() {
                                  _isBatuk = value;
                                  settingsBloc.dispatch(SetSetting("has_cough",
                                      _isBatuk ? "true" : "false"));
                                });
                              }
                            : null,
                      ),
                      Text(
                        "Batuk",
                        style: TextStyle(
                          fontSize: 16,
                        ),
                      )
                    ],
                  ),
                ),
                Container(
                  height: 35,
                  child: Row(
                    children: <Widget>[
                      Checkbox(
                        value: _isDemam,
                        onChanged: _petaIsChecked
                            ? (value) {
                                setState(() {
                                  _isDemam = value;
                                  settingsBloc.dispatch(SetSetting("has_fever",
                                      _isDemam ? "true" : "false"));
                                });
                              }
                            : null,
                      ),
                      Text(
                        "Demam",
                        style: TextStyle(
                          fontSize: 16,
                        ),
                      )
                    ],
                  ),
                ),
                Container(
                  height: 35,
                  child: Row(
                    children: <Widget>[
                      Checkbox(
                        value: _isCold,
                        onChanged: _petaIsChecked
                            ? (value) {
                                setState(() {
                                  _isCold = value;
                                  settingsBloc.dispatch(SetSetting(
                                      "has_cold", _isCold ? "true" : "false"));
                                });
                              }
                            : null,
                      ),
                      Text(
                        "Flu",
                        style: TextStyle(
                          fontSize: 16,
                        ),
                      )
                    ],
                  ),
                ),
                Container(
                  height: 35,
                  child: Row(
                    children: <Widget>[
                      Checkbox(
                        value: _isPusing,
                        onChanged: _petaIsChecked
                            ? (value) {
                                setState(() {
                                  _isPusing = value;
                                  settingsBloc.dispatch(SetSetting(
                                      "has_headache",
                                      _isPusing ? "true" : "false"));
                                });
                              }
                            : null,
                      ),
                      Text(
                        "Pusing",
                        style: TextStyle(
                          fontSize: 16,
                        ),
                      )
                    ],
                  ),
                )
              ],
            ),
          )
        ],
      ),
    );
  }
}

// Widget _buildInfoDialog(BuildContext context) {
//   return new AlertDialog(
//     title: const Text("Info"),
//     content: new Container(
//       child: _buildText(context),
//     ),
//     actions: <Widget>[
//       new Center(
//         child: FlatButton(
//           child: const Text("OKE"),
//           onPressed: () {
//             Navigator.of(context).pop();
//           },
//         ),
//       )
//     ],
//   );
// }

// Widget _buildText(BuildContext context) {
//   return new RichText(
//       text: TextSpan(
//           text:
//               "Fitur ini memungkinkan anda untuk mendapatkan informasi Covid-19 ( Corona ) di daerah sekitar kita."));
// }
