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

import 'package:pandemia_mobile/core/error.dart';
import 'package:pandemia_mobile/models/user.dart';
import 'package:pandemia_mobile/util/json_helper.dart';

import './pandemia_api.dart';
import './session.dart';

class Auth {
  static Future<Session> doLogin(String email, String password) {
    return ApiClient().public().post("/auth/v1/authorize",
        body: {'email': email, 'password': password}).then((resp) {
      // print("resp: $resp");
      final data = tryDecode(resp.body);
      print("login data: $data.body");
      if (data['code'] != 0) {
        switch (data['code']) {
          case ErrorCode.Unauthorized:
          case ErrorCode.NotFound:
            throw PandemiaException.fromResp(data)
                .withMsg("Wrong email or password");
            break;
          default:
            throw PandemiaException.fromResp(data);
        }
      }
      final d = data['result'];
      return new Session(d['account_id'], d['token']);
    }).catchError(handleError);
  }

  // load account information
  static Future<User> getMeInfo() {
    print("loading user information...");
    return ApiClient().public().get("/user/v1/me/info").then((resp) {
      print("resp: $resp");
      final data = tryDecode(resp.body);
      print("login data: $data");
      if (data['code'] != 0) {
        switch (data['code']) {
          case ErrorCode.Unauthorized:
          case ErrorCode.NotFound:
            throw PandemiaException.fromResp(data).withMsg("User not found");
            break;
          default:
            throw PandemiaException.fromResp(data);
        }
      }
      return User.fromMap(data["result"]);
    });
  }

  static Future<void> doLogout() {
    return ApiClient().public().post("/auth/v1/unauthorize").then((resp) {
      print("resp: ${resp.body}");
      final data = tryDecode(resp.body);
      checkValidResp(data);
    }).catchError(handleError, test: (_) => true);
  }
}

