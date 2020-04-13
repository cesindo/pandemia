import 'package:google_maps_flutter/google_maps_flutter.dart';
import 'package:intl/intl.dart';

extension StringExtension on String {
  String capitalize() {
    return "${this[0].toUpperCase()}${this.substring(1)}";
  }
}

extension NumFormatExtension on String {
  String toNumberFormat() {
    final formatter = NumberFormat('#,###');
    return formatter.format(int.parse(this));
  }
}

extension LatLngToMapExtenstion on LatLng {
  Map<String, dynamic> toMap() {
    Map<String, dynamic> mapResult = {
      "loc": {
        "lat": this.latitude,
        "long": this.longitude,
      }
    };
    return mapResult;
  }
}
