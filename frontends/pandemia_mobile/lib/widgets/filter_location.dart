import 'package:flutter/material.dart';
import 'package:multiselect_formfield/multiselect_formfield.dart';

class FilterLocation extends StatefulWidget {
  final bool disabled;
  FilterLocation({Key key, this.disabled}) : super(key: key);

  @override
  _FilterLocationState createState() => _FilterLocationState();
}

class _FilterLocationState extends State<FilterLocation> {
  List _locations;
  final formKey = new GlobalKey<FormState>();

  @override
  void initState() {
    super.initState();
    _locations = [];
  }

  @override
  Widget build(BuildContext context) {
    return IgnorePointer(
      ignoring: widget.disabled,
      child: Form(
      key: formKey,
      child: MultiSelectFormField(
        autovalidate: false,
        titleText: 'Filter lokasi',
        validator: (value) {
          if (value == null || value.length == 0) {
            return 'Pilih satu daerah atau banyak daerah';
          }
        },
        dataSource: [
          {
            "display": "Jakarta",
            "value": "Jakarta",
          },
          {
            "display": "Bandung",
            "value": "Bandung",
          },
          {
            "display": "Bogor",
            "value": "Bogor",
          },
          {
            "display": "Semarang",
            "value": "Semarang",
          },
          {
            "display": "Solo",
            "value": "Solo",
          },
          {
            "display": "Magelang",
            "value": "Magelang",
          },
          {
            "display": "Yogyakarta",
            "value": "Yogyakarta",
          },
        ],
        textField: 'display',
        valueField: 'value',
        okButtonLabel: 'Ok',
        cancelButtonLabel: 'Batal',
        // required: true,
        hintText: 'Pilih hanya daerah tertentu saja',
        value: _locations,
        onSaved: (value) {
          if (value == null) return;
          setState(() {
            _locations = value;
          });
        },
      ),
    ),
    );
  }
}
