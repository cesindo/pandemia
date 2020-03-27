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
