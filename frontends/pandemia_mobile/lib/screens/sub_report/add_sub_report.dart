import 'package:flutter/material.dart';

class AddSubReportPage extends StatefulWidget {
  AddSubReportPage({Key key}) : super(key: key);

  @override
  _AddSubReportPageState createState() => _AddSubReportPageState();
}

class _AddSubReportPageState extends State<AddSubReportPage> {
  final _formKey = GlobalKey<FormState>();
  final _fullNameCtl = TextEditingController();
  final _birthDayCtl = TextEditingController();
  final _fromCtl = TextEditingController();
  final _comingDateCtl = TextEditingController();
  final _necessityCtl = TextEditingController();
  bool _isDryCough = false;
  bool _isFever = false;
  bool _isHardToBreath = false;
  String _valGender;
  String _valStatus;

  List gender = ["L", "P"];
  List status = ["ODP", "PDP", "Positif", "Sembuh"];

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text("Tambah Data"),
      ),
      body: _getBody(context),
    );
  }

  Widget _getBody(BuildContext context) {
    return ListView(
      children: <Widget>[
        Padding(
          padding: EdgeInsets.all(15.0),
          child: Form(
            key: _formKey,
            child: Column(
              children: <Widget>[
                TextFormField(
                  decoration: InputDecoration(labelText: "Nama Lengkap"),
                  controller: _fullNameCtl,
                ),
                TextFormField(
                  decoration:
                      InputDecoration(labelText: "Tempat Tanggal Lahir"),
                  controller: _birthDayCtl,
                ),
                Container(
                  margin: EdgeInsets.only(top: 10.0),
                  width: double.infinity,
                  child: DropdownButton(
                    hint: Text("Jenis Kelamin"),
                    value: _valGender,
                    items: gender.map((val) {
                      return DropdownMenuItem(
                        child: Text(val),
                        value: val,
                      );
                    }).toList(),
                    onChanged: (val) {
                      setState(() {
                        _valGender = val;
                      });
                    },
                    isExpanded: true,
                  ),
                ),
                TextFormField(
                  decoration: InputDecoration(labelText: "Dari"),
                  controller: _fromCtl,
                ),
                TextFormField(
                  decoration: InputDecoration(labelText: "Tanggal Kedatangan"),
                  controller: _comingDateCtl,
                ),
                TextFormField(
                  decoration:
                      InputDecoration(labelText: "Keperluan di Perantauan"),
                  controller: _necessityCtl,
                ),
                Container(
                  margin: EdgeInsets.only(top: 10.0),
                  width: double.infinity,
                  child: DropdownButton(
                    hint: Text("Status"),
                    value: _valStatus,
                    items: status.map((val) {
                      return DropdownMenuItem(
                        child: Text(val),
                        value: val,
                      );
                    }).toList(),
                    onChanged: (val) {
                      setState(() {
                        _valStatus = val;
                      });
                    },
                    isExpanded: true,
                  ),
                ),
                Container(
                  margin: EdgeInsets.only(top: 40.0),
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: <Widget>[
                      Text("Gejala"),
                      Container(
                        child: Row(
                          children: <Widget>[
                            Checkbox(
                              value: _isDryCough,
                              onChanged: (bool value) {
                                setState(() {
                                  _isDryCough = value;
                                });
                              },
                            ),
                            Text("Batuk kering"),
                          ],
                        ),
                      ),
                      Container(
                        child: Row(
                          children: <Widget>[
                            Checkbox(
                              value: _isFever,
                              onChanged: (bool value) {
                                setState(() {
                                  _isFever = value;
                                });
                              },
                            ),
                            Text("Demam"),
                          ],
                        ),
                      ),
                      Container(
                        child: Row(
                          children: <Widget>[
                            Checkbox(
                              value: _isHardToBreath,
                              onChanged: (bool value) {
                                setState(() {
                                  _isHardToBreath = value;
                                });
                              },
                            ),
                            Text("Susah bernafas"),
                          ],
                        ),
                      ),
                    ],
                  ),
                ),
                Container(
                  margin: EdgeInsets.only(top: 20.0, bottom: 50.0),
                  child: MaterialButton(
                    child: Text(
                      "Tambahkan",
                      style: TextStyle(color: Colors.white),
                    ),
                    minWidth: double.infinity,
                    onPressed: () {},
                    color: Theme.of(context).buttonColor,
                    height: 40.0,
                  ),
                ),
              ],
            ),
          ),
        ),
      ],
    );
  }
}