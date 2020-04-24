import 'dart:async';

import 'package:autocomplete_textfield/autocomplete_textfield.dart';
import 'package:equatable/equatable.dart';
import 'package:flutter/material.dart';
import 'package:intl/intl.dart';
import 'package:pandemia_mobile/blocs/sub_report/sub_report.dart';
import 'package:pandemia_mobile/blocs/sub_report/sub_report_bloc.dart';
import 'package:pandemia_mobile/blocs/sub_report/sub_report_event.dart';
import 'package:pandemia_mobile/models/sub_report.dart';
import 'package:pandemia_mobile/models/user.dart';
import 'package:pandemia_mobile/screens/sub_report/data_kabupaten.dart';
import 'package:pandemia_mobile/user_repository/user_repository.dart';
import 'package:pandemia_mobile/util/text_formatter.dart';

class AddSubReportPage extends StatefulWidget {
  final SubReportBloc subReportBloc;
  final SubReport item;

  AddSubReportPage({Key key, @required this.subReportBloc, this.item})
      : super(key: key);

  @override
  _AddSubReportPageState createState() =>
      _AddSubReportPageState(this.subReportBloc, this.item);
}

class Gender extends Equatable {
  final String value;
  final String label;

  Gender(this.value, this.label);

  @override
  List get props => [this.value, this.label];
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
  Gender _valGender;
  String _valStatus;
  User currentUser;
  final _scaffoldKey = GlobalKey<ScaffoldState>();
  String currentText = "";
  GlobalKey<AutoCompleteTextFieldState<String>> key = new GlobalKey();
  final SubReport item;
  final bool editMode;

  List<Gender> gender = [
    Gender("L", "Laki-laki"),
    Gender("P", "Perempuan"),
  ];
  List<String> statuses = ["ODP", "PDP", 'OTG'];
  List<Map<String, String>> addInfo = [
    {"no": "traveler", "info": "Pelaku Perjalanan"},
    {"no": "from_red_zone", "info": "Datang Dari Zona Merah"},
    {"no": "has_symptoms", "info": "Bergejala COVID-19"},
  ];
  List<String> addInfoSelected = [];
  List<String> keluhan = [
    "Suhu di atas normal",
    "Demam",
    "Batuk Kering",
    "Sesak Nafas"
  ];
  List<String> keluhanSelected = [];

  _AddSubReportPageState(this.subReportBloc, this.item)
      : this.editMode = item != null {}

  @override
  void initState() {
    super.initState();
    currentUser = userRepository.currentUser;
    _subs = subReportBloc.state.listen((state) {
      if (state is SubReportFailure) {
        _scaffoldKey.currentState.showSnackBar(
            SnackBar(content: Text(state.error), backgroundColor: Colors.red));
      } else if (state is SubReportCreated) {
        _scaffoldKey.currentState.showSnackBar(SnackBar(
            content: Text("Data berhasil ditambahkan"),
            backgroundColor: Colors.green));
        // Navigator.of(context).pushReplacement(MaterialPageRoute(
        //     builder: (context) => BlocProvider<SubReportBloc>(
        //         builder: (ctx) => SubReportBloc(),
        //         child: SubReportPage(subReportBloc: subReportBloc))));
        Navigator.pop(context, true);
      } else if (state is SubReportUpdated) {
        _scaffoldKey.currentState.showSnackBar(SnackBar(
            content: Text("Data berhasil diperbarui"),
            backgroundColor: Colors.green));
        Navigator.pop(context, true);
      }
    });

    if (item != null) {
      Future.delayed(Duration(milliseconds: 300), () {
        setState(() {
          _fullNameCtl.text = item.fullName;
          _addrCtl.text = item.residenceAddress;
          _ageCtl.text = item.age.toString();
          _fromCtl.text = item.comingFrom;
          _comingDateCtl.text = item.arrivalDate;
          _necessityCtl.text = item.notes;
          if (item.gender == "L") {
            _valGender = gender[0];
          } else {
            _valGender = gender[1];
          }
          _valStatus = item.status;
          keluhanSelected =
              item.healthyNotes.split(',').map((a) => a.trim()).toList();
          if (item.fromRedZone){
            addInfoSelected.add("from_red_zone");
          }
          if (item.hasSymptoms){
            addInfoSelected.add("has_symptoms");
          }
          if (item.traveler){
            addInfoSelected.add("traveler");
          }
        });
      });
    }
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
        title: Text("${editMode ? "Edit" : "Tambah"} Data"),
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
                  autofocus: true,
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
                  inputFormatters: [new TitleCaseTextFormatter()],
                ),
                TextFormField(
                  decoration: InputDecoration(labelText: "Alamat Lengkap"),
                  maxLines: 3,
                  controller: _addrCtl,
                  textInputAction: TextInputAction.next,
                  keyboardType: TextInputType.text,
                  onFieldSubmitted: (_) => FocusScope.of(context).nextFocus(),
                  validator: (val) {
                    return val.isEmpty
                        ? "Alamat lengkap tidak boleh kosong"
                        : null;
                  },
                  inputFormatters: [new TitleCaseTextFormatter()],
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
                DropdownButtonFormField<Gender>(
                  decoration: InputDecoration(
                      contentPadding: EdgeInsets.fromLTRB(0, 4, 0, 4)),
                  hint: Text("Jenis Kelamin"),
                  value: _valGender,
                  items: gender.map((val) {
                    return DropdownMenuItem<Gender>(
                      child: Text(val.label),
                      value: val,
                    );
                  }).toList(),
                  onChanged: (Gender val) {
                    print(val);
                    setState(() {
                      _valGender = val;
                    });
                  },
                  isExpanded: true,
                  // validator: (val) {
                  //   return val.isEmpty
                  //       ? "Jenis kelamin tidak boleh kosong"
                  //       : null;
                  // },
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
                  // validator: (val) {
                  //   return val.isEmpty ? "Field tidak boleh kosong" : null;
                  // },
                ),
                DropdownButtonFormField(
                  decoration: InputDecoration(
                      contentPadding: EdgeInsets.fromLTRB(0, 4, 0, 4)),
                  hint: Text("Status"),
                  value: _valStatus,
                  items: statuses.map((val) {
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
                        Text("Info tambahan"),
                        Column(
                            children: addInfo
                                .map((a) => LabeledCheckbox(
                                      label: a["info"],
                                      value: addInfoSelected.contains(a["no"]),
                                      onChanged: (value) {
                                        setState(() {
                                          if (value) {
                                            addInfoSelected.add(a["no"]);
                                          } else {
                                            addInfoSelected.remove(a["no"]);
                                          }
                                        });
                                      },
                                    ))
                                .toList()),
                        Text("Keluhan Kesehatan:"),
                        Column(
                          // shrinkWrap: true,
                          // primary: false,
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
                        if (!this.editMode) {
                          subReportBloc.dispatch(CreateSubReport(
                              _fullNameCtl.text,
                              int.parse(_ageCtl.text),
                              _addrCtl.text,
                              _valGender.value,
                              _fromCtl.text,
                              _comingDateCtl.text,
                              _necessityCtl.text,
                              _valStatus,
                              keluhanSelected,
                              addInfoSelected));
                        } else {
                          subReportBloc.dispatch(UpdateSubReport(
                              item.id,
                              _fullNameCtl.text,
                              int.parse(_ageCtl.text),
                              _addrCtl.text,
                              _valGender.value,
                              _fromCtl.text,
                              _comingDateCtl.text,
                              _necessityCtl.text,
                              _valStatus,
                              keluhanSelected,
                              addInfoSelected));
                        }
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
