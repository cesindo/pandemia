import 'dart:async';

import 'package:autocomplete_textfield/autocomplete_textfield.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:intl/intl.dart';
import 'package:pandemia_mobile/blocs/sub_report/sub_report.dart';
import 'package:pandemia_mobile/blocs/sub_report/sub_report_bloc.dart';
import 'package:pandemia_mobile/blocs/sub_report/sub_report_event.dart';
import 'package:pandemia_mobile/models/user.dart';
import 'package:pandemia_mobile/screens/sub_report/data_kabupaten.dart';
import 'package:pandemia_mobile/screens/sub_report/sub_report_page.dart';
import 'package:pandemia_mobile/user_repository/user_repository.dart';

class AddSubReportPage extends StatefulWidget {
  final SubReportBloc subReportBloc;
  AddSubReportPage({Key key, @required this.subReportBloc}) : super(key: key);

  @override
  _AddSubReportPageState createState() =>
      _AddSubReportPageState(this.subReportBloc);
}

class _AddSubReportPageState extends State<AddSubReportPage> {
  final SubReportBloc subReportBloc;
  final UserRepository userRepository = UserRepository();
  final _formKey = GlobalKey<FormState>();
  final _fullNameCtl = TextEditingController();
  final _addrCtl = TextEditingController();
  final _ageCtl = TextEditingController();
  final _fromCtl = TextEditingController();
  final _comingDateCtl = TextEditingController();
  final _necessityCtl = TextEditingController();
  StreamSubscription _subs;
  Map<dynamic, String> _valGender;
  String _valStatus;
  User currentUser;
  final _scaffoldKey = GlobalKey<ScaffoldState>();
  String currentText = "";
  GlobalKey<AutoCompleteTextFieldState<String>> key = new GlobalKey();

  List<Map<dynamic, String>> gender = [
    {"value": "L", "label": "Laki-laki"},
    {"value": "P", "label": "Perempuan"}
  ];
  List<String> status = ["ODP", "PDP", "Positif", "Sembuh"];
  List<String> keluhan = [
    "Suhu di atas normal",
    "Demam",
    "Batuk Kering",
    "Sesak Nafas"
  ];
  List<String> keluhanSelected = [];

  _AddSubReportPageState(this.subReportBloc);

  @override
  void initState() {
    currentUser = userRepository.currentUser;
    _subs = subReportBloc.state.listen((state) {
      if (state is SubReportFailure) {
        _scaffoldKey.currentState.showSnackBar(
            SnackBar(content: Text(state.error), backgroundColor: Colors.red));
      } else if (state is SubReportCreated) {
        _scaffoldKey.currentState.showSnackBar(SnackBar(
            content: Text("Data berhasil ditambahkan"),
            backgroundColor: Colors.green));
        Navigator.of(context).pushReplacement(MaterialPageRoute(
            builder: (context) => BlocProvider<SubReportBloc>(
                builder: (ctx) => SubReportBloc(),
                child: SubReportPage(subReportBloc: subReportBloc))));
      }
    });
    super.initState();
  }

  @override
  void dispose() {
    _subs.cancel();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      key: _scaffoldKey,
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
                  textInputAction: TextInputAction.next,
                  keyboardType: TextInputType.text,
                  onFieldSubmitted: (_) => FocusScope.of(context).nextFocus(),
                  validator: (val) {
                    return val.isEmpty
                        ? "Nama lengkap tidak boleh kosong"
                        : null;
                  },
                ),
                TextFormField(
                  decoration: InputDecoration(labelText: "Alamat Lengkap"),
                  controller: _addrCtl,
                  textInputAction: TextInputAction.next,
                  keyboardType: TextInputType.text,
                  onFieldSubmitted: (_) => FocusScope.of(context).nextFocus(),
                  validator: (val) {
                    return val.isEmpty
                        ? "Alamat lengkap tidak boleh kosong"
                        : null;
                  },
                ),
                TextFormField(
                  decoration: InputDecoration(labelText: "Usia"),
                  controller: _ageCtl,
                  textInputAction: TextInputAction.next,
                  keyboardType: TextInputType.number,
                  onFieldSubmitted: (_) => FocusScope.of(context).nextFocus(),
                  validator: (val) {
                    return val.isEmpty ? "Usia tidak boleh kosong" : null;
                  },
                ),
                DropdownButtonFormField(
                  decoration: InputDecoration(
                      contentPadding: EdgeInsets.fromLTRB(0, 4, 0, 4)),
                  hint: Text("Jenis Kelamin"),
                  value: _valGender,
                  items: gender.map((val) {
                    return DropdownMenuItem<Map<dynamic, String>>(
                      child: Text(val["label"]),
                      value: val,
                    );
                  }).toList(),
                  onChanged: (Map<dynamic, String> val) {
                    setState(() {
                      _valGender = val;
                    });
                  },
                  isExpanded: true,
                  validator: (val) {
                    return val.isEmpty
                        ? "Jenis kelamin tidak boleh kosong"
                        : null;
                  },
                ),
                // TextFormField(
                //   decoration: InputDecoration(labelText: "Datang Dari"),
                //   controller: _fromCtl,
                //   textInputAction: TextInputAction.next,
                //   keyboardType: TextInputType.text,
                //   onFieldSubmitted: (_) => FocusScope.of(context).nextFocus(),
                //   validator: (val) {
                //     return val.isEmpty ? "Field tidak boleh kosong" : null;
                //   },
                // ),
                SimpleAutoCompleteTextField(
                  key: key,
                  suggestions: kab,
                  textChanged: (text) => currentText = text,
                  decoration: new InputDecoration(labelText: "Datang Dari"),
                  controller: _fromCtl,
                ),
                TextFormField(
                  decoration: InputDecoration(labelText: "Tanggal Kedatangan"),
                  controller: _comingDateCtl,
                  readOnly: true,
                  onTap: () => _datePicker(),
                  validator: (val) {
                    return val.isEmpty ? "Field tidak boleh kosong" : null;
                  },
                ),
                TextFormField(
                  decoration:
                      InputDecoration(labelText: "Keperluan di Perantauan"),
                  controller: _necessityCtl,
                  textInputAction: TextInputAction.done,
                  keyboardType: TextInputType.text,
                  onFieldSubmitted: (_) => FocusScope.of(context).unfocus(),
                  validator: (val) {
                    return val.isEmpty ? "Field tidak boleh kosong" : null;
                  },
                ),
                DropdownButtonFormField(
                  decoration: InputDecoration(
                      contentPadding: EdgeInsets.fromLTRB(0, 4, 0, 4)),
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
                Container(
                    margin: EdgeInsets.only(top: 10.0),
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: <Widget>[
                        Text("Keluhan:"),
                        ListView(
                          shrinkWrap: true,
                          primary: false,
                          children: keluhan
                              .map((f) => LabeledCheckbox(
                                  label: f,
                                  value: keluhanSelected.contains(f),
                                  onChanged: (value) {
                                    setState(() {
                                      if (value) {
                                        keluhanSelected.add(f);
                                      } else {
                                        keluhanSelected.remove(f);
                                      }
                                    });
                                  }))
                              .toList(),
                        )
                      ],
                    )),
                Container(
                  margin: EdgeInsets.only(top: 20.0, bottom: 50.0),
                  child: MaterialButton(
                    child: Text(
                      "Simpan",
                      style: TextStyle(color: Colors.white),
                    ),
                    minWidth: double.infinity,
                    onPressed: () {
                      if (_formKey.currentState.validate()) {
                        subReportBloc.dispatch(CreateSubReport(
                            _fullNameCtl.text,
                            int.parse(_ageCtl.text),
                            _addrCtl.text,
                            _valGender["value"],
                            _fromCtl.text,
                            _comingDateCtl.text,
                            _necessityCtl.text,
                            status.indexOf(this._valStatus),
                            keluhanSelected));
                      }
                    },
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

  _datePicker() async {
    DateTime selectedDate = await showDatePicker(
      context: context,
      initialDate: DateTime.now(),
      firstDate: DateTime(2018),
      lastDate: DateTime(2030),
    );
    if (selectedDate != null) {
      var dateTime = DateFormat("yyyy-MM-dd").format(selectedDate);
      setState(() => _comingDateCtl.text = dateTime.toString());
    }
  }
}

class LabeledCheckbox extends StatelessWidget {
  final String label;
  final bool value;
  final Function onChanged;

  const LabeledCheckbox({
    this.label,
    this.value,
    this.onChanged,
  });

  @override
  Widget build(BuildContext context) {
    return InkWell(
      onTap: () {
        onChanged(!value);
      },
      child: Row(
        children: <Widget>[
          Checkbox(
            value: value,
            materialTapTargetSize: MaterialTapTargetSize.shrinkWrap,
            onChanged: (bool newValue) {
              onChanged(newValue);
            },
          ),
          Text(
            label,
            style: TextStyle(fontSize: 15),
          ),
        ],
      ),
    );
  }
}
