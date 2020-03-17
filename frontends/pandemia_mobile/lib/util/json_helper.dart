

import 'dart:convert';

import 'package:pandemia_mobile/core/error.dart';


/// Try to decode json text into [Map<String, dynamic>]
/// Will raise [PandemiaException] when something went wrong.
Map<String, dynamic> tryDecode(String txtData){
  try {
    var data = json.decode(txtData);
    return data;
  } catch (e) {
    print("Cannot decode json message: " + txtData);
    throw PandemiaException("Cannot communicate with server", code: 4001);
  }
}
