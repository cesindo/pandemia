import 'package:flutter/material.dart';

class SettingScreen extends StatefulWidget {
  SettingScreen({Key key}) : super(key: key);

  @override
  _SettingScreenState createState() => _SettingScreenState();
}

class _SettingScreenState extends State<SettingScreen> {
  bool _onTap = false;
  bool _pushIsChecked = false;
  bool _petaIsChecked = false;
  bool _isBatuk = false;
  bool _isDemam = false;
  bool _isFlu = false;
  bool _isPusing = false;

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
            padding: EdgeInsets.only(bottom: 10),
            child: Center(
              child: InkWell(
                child: Text(
                  "Pilih hanya daerah tertentu saja",
                  style: TextStyle(
                    fontSize: 16,
                  ),
                ),
                onTap: _onTap
                    ? () {
                        print("=======> show filter screen");
                      }
                    : null,
              ),
            ),
          ),
          Divider(),
          Padding(
            padding: EdgeInsets.only(left: 15, top: 0, bottom: 10),
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
                      print("value peta sekaran : $_petaIsChecked");

                      _isBatuk == false ? null : _isBatuk = !_isBatuk;
                      _isDemam == false ? null : _isDemam = !_isDemam;
                      _isFlu == false ? null : _isFlu = !_isFlu;
                      _isPusing == false ? null : _isPusing = !_isPusing;
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
                        value: _isFlu,
                        onChanged: _petaIsChecked
                            ? (value) {
                                setState(() {
                                  _isFlu = value;
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

Widget _buildInfoDialog(BuildContext context) {
  return new AlertDialog(
    title: const Text("Info"),
    content: new Container(
      child: _buildText(context),
    ),
    actions: <Widget>[
      new Center(
        child: FlatButton(
          child: const Text("OKE"),
          onPressed: () {
            Navigator.of(context).pop();
          },
        ),
      )
    ],
  );
}

Widget _buildText(BuildContext context) {
  return new RichText(
      text: TextSpan(
          text:
              "Fitur ini memungkinkan anda untuk mendapatkan informasi Covid-19 ( Corona ) di daerah sekitar kita."));
}
