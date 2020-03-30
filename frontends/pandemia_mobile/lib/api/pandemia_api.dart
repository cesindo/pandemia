// Copyright (C) 2016 Muqorrobien Ma'rufi
// All Rights Reserved.
//
// NOTICE: All information contained herein is, and remains
// the property of Muqorrobien Ma'rufi.
// The intellectual and technical concepts contained
// herein are proprietary to Muqorrobien Ma'rufi
// and are protected by trade secret or copyright law.
// Dissemination of this information or reproduction of this material
// is strictly forbidden unless prior written permission is obtained
// from Muqorrobien Ma'rufi (obin.mf@gmail.com).
//

export './api_client.dart';
export './auth.dart';
export './session.dart';

import 'package:pandemia_mobile/api/api_client.dart';
import 'package:pandemia_mobile/core/error.dart';
import 'package:pandemia_mobile/util/json_helper.dart';

class DetaxApi {
  static Future<Map<String, dynamic>> get(String path) async {
    // print("GET $path (public)");
    return ApiClient().detax().get(path).then((resp) {
      if (resp == null || resp.body == null){
        throw PandemiaException("Cannot connect to server (code: 5832)");
      }
      // print("GET resp: ${resp.body}");
      final respData = tryDecode(resp.body);
      if (respData == null){
        throw PandemiaException("Cannot connect to server (code: 5832)");
      }
      checkValidResp(respData);
      checkValidResultResp(respData);
      return respData;
    }).catchError(handleError);
  }

  static Future<Map<String, dynamic>> post(
      String path, Map<String, dynamic> data) async {
    // print("POST $path (public)");
    final rv = await ApiClient().detax().post(path, body: data).then((resp) {
      if (resp == null || resp.body == null){
        throw PandemiaException("Cannot connect to server (code: 5832)");
      }
      final respData = tryDecode(resp.body);
      if (respData == null){
        throw PandemiaException("Cannot connect to server (code: 5832)");
      }
      checkValidResp(respData);
      checkValidResultResp(respData);
      return respData;
    }).catchError(handleError);
    return rv;
  }
}

class PublicApi {
  static Future<Map<String, dynamic>> get(String path) async {
    // print("GET $path (public)");
    return ApiClient().public().get(path).then((resp) {
      if (resp == null || resp.body == null){
        throw PandemiaException("Cannot connect to server (code: 5832)");
      }
      // print("GET resp: ${resp.body}");
      final respData = tryDecode(resp.body);
      if (respData == null){
        throw PandemiaException("Cannot connect to server (code: 5832)");
      }
      checkValidResp(respData);
      checkValidResultResp(respData);
      return respData;
    }).catchError(handleError);
  }

  static Future<Map<String, dynamic>> post(
      String path, Map<String, dynamic> data) async {
    // print("POST $path (public)");
    final rv = await ApiClient().public().post(path, body: data).then((resp) {
      if (resp == null || resp.body == null){
        throw PandemiaException("Cannot connect to server (code: 5832)");
      }
      final respData = tryDecode(resp.body);
      if (respData == null){
        throw PandemiaException("Cannot connect to server (code: 5832)");
      }
      checkValidResp(respData);
      checkValidResultResp(respData);
      return respData;
    }).catchError(handleError);
    return rv;
  }
}

class PrivateApi {
  static Future<Map<String, dynamic>> get(String path) async {
    // print("GET $path (private)");
    final data = await ApiClient().private().get(path).then((resp) {
      if (resp == null || resp.body == null){
        throw PandemiaException("Cannot connect to server (code: 5832)");
      }
      final respData = tryDecode(resp.body);
      if (respData == null){
        throw PandemiaException("Cannot connect to server (code: 5832)");
      }
      checkValidResp(respData);
      checkValidResultResp(respData);
      return respData;
    }).catchError(handleError);
    return data;
  }

  static Future<Map<String, dynamic>> post(
      String path, Map<String, dynamic> data) async {
    // print("POST $path (private)");
    final rv = await ApiClient().private().post(path, body: data).then((resp) {
      if (resp == null || resp.body == null){
        throw PandemiaException("Cannot connect to server (code: 5832)");
      }
      final respData = tryDecode(resp.body);
      if (respData == null){
        throw PandemiaException("Cannot connect to server (code: 5832)");
      }
      checkValidResp(respData);
      checkValidResultResp(respData);
      return respData;
    }).catchError(handleError);
    return rv;
  }
}

/// Automatically throw exception with server error information wrapped
/// into [PandemiaException]
void checkValidResp(Map<String, dynamic> respData) {
  int code = respData['code'] as int;
  if (code != 0) {
    throw PandemiaException.fromResp(respData);
  }
}

void checkValidResultResp(Map<String, dynamic> respData) {
  if (!respData.containsKey("result")) {
    print('Invalid server response: $respData');
    throw PandemiaException("Invalid server response, please report to developer");
  }
}

