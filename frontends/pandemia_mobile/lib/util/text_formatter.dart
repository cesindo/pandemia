import 'package:flutter/services.dart';

class UpperCaseTextFormatter extends TextInputFormatter {
  @override
  TextEditingValue formatEditUpdate(
      TextEditingValue oldValue, TextEditingValue newValue) {
    return TextEditingValue(
      text: newValue.text?.toUpperCase(),
      selection: newValue.selection,
    );
  }
}

class TitleCaseTextFormatter extends TextInputFormatter {
  @override
  TextEditingValue formatEditUpdate(
      TextEditingValue oldValue, TextEditingValue newValue) {
    return TextEditingValue(
      text: titleCase(newValue.text),
      selection: newValue.selection,
    );
  }
}

String titleCase(String subject, [List<String> notSplitList = const []]) {
  if (subject is! String || subject.length == 0) {
    return '';
  }

  RegExp _wordPattern = RegExp("[a-zA-Z]+");

  int index = 0;

  String replacer(Match m) {
    String subString = m[0];
    index = subject.indexOf(subString, index);
    int previousIndex = index - 1;
    if (previousIndex >= 0 &&
        notSplitList.indexOf(subject[previousIndex]) >= 0) {
      index += subString.length;
      return subString.toLowerCase();
    } else {
      index += subString.length;
      return capitalize(subString, true);
    }
  }

  return subject.replaceAllMapped(_wordPattern, replacer);
}

String capitalize(String subject, [bool lowerRest = false]) {
  if (subject is! String || subject.length == 0) {
    return '';
  }

  if (lowerRest) {
    return subject[0].toUpperCase() + subject.substring(1).toLowerCase();
  } else {
    return subject[0].toUpperCase() + subject.substring(1);
  }
}
