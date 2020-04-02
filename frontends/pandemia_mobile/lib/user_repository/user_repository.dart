import 'dart:async';

import 'package:meta/meta.dart';
import 'package:pandemia_mobile/api/pandemia_api.dart';
import 'package:pandemia_mobile/core/smart_repo.dart';
import 'package:pandemia_mobile/models/models.dart';
import 'package:pandemia_mobile/core/error.dart';

class UserRepository {
  static UserRepository _singleton;
  PersistentSmartRepo repo;
  User currentUser;

  factory UserRepository() {
    if (_singleton == null) {
      _singleton = UserRepository._internal();
    }
    return _singleton;
  }

  UserRepository._internal() {
    repo = new PersistentSmartRepo('User');
  }

  Future<Session> authenticate({
    @required String email,
    @required String password,
  }) async {
    return Auth.doLogin(email, password).then((session) async {
      if (session != null) {
        repo.putData("accessToken", session.toMap());
      } else {
        throw PandemiaException("Cannot contact server");
      }
      return session;
    }).whenComplete(() async {
      repo.fetchGradually(
          "currentUser", () => PublicApi.get("/user/v1/me/info"),
          force: true);
    });
  }

  Future<void> deleteToken() async {
    await Auth.doLogout();
    repo.clear();
    return;
  }

  Future<void> persistToken(String token) async {
    repo.putData("accessToken", {"token": token});
    return;
  }

  Future<bool> hasToken() async {
    var token = await getToken();
    return token != null;
  }

  Future<User> getUserInfo() {
    return repo
        .fetchGradually("currentUser", () => PublicApi.get("/user/v1/me/info"),
            force: true)
        .map((a){
          currentUser = User.fromMap(a.data);
          return currentUser;
        })
        .first;
  }

  Future<User> getLocalUserInfo() async {
    return repo.getData("currentUser").then((data) => User.fromMap(data));
  }

  Future<String> getToken() async {
    return repo.getData("accessToken").then((data) {
      print("get accessToken in getToken(): $data");
      if (data != null) {
        return data["token"] as String;
      } else {
        return null;
      }
    });
  }
}
