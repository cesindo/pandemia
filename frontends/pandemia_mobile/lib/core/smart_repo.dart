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

import 'dart:convert';
import 'package:localstorage/localstorage.dart';
import 'package:pandemia_mobile/api/pandemia_api.dart';
import 'package:pandemia_mobile/db_helper.dart';
import 'package:sqflite/sqlite_api.dart';

abstract class SmartRepo {
  Future<T> fetch<T extends dynamic>(
      String storeName, Future<T> Function() dataRetriever,
      {force: bool});

  Future<dynamic> fetchApi(String storeName, String apiPath,
      {force: bool});

  void clear();
}

class LocalSmartRepo extends SmartRepo {
  final String key;
  final LocalStorage _storage;

  LocalSmartRepo(this.key) : _storage = LocalStorage(key) {
    var appConfig = LocalStorage("__app_config__");
    if (appConfig.getItem("resetData") == true) {
      this.clear();
    }
  }

  Future<T> fetch<T extends dynamic>(
      String storeName, Future<T> Function() dataRetriever,
      {force: bool}) async {
    T resultData = _storage.getItem(storeName);

    // print("resultData: $resultData");

    if (resultData == null || force == true) {
      final data = await dataRetriever();
      if (data != null) {
        print("data from dataRetriever($storeName): $data");
        resultData = data["result"];
      }
    }

    _storage.setItem(storeName, resultData);

    return resultData;
  }

  Future<Map<String, dynamic>> fetchApi(String storeName, String apiPath,
      {force: bool}) {
    return fetch(storeName, () => PublicApi.get(apiPath), force: force);
  }

  void clear() {
    _storage.clear();
  }
}

/// Return type data from fetchGradually
class RepoData<T> {
  final T data;
  bool isRemote;
  RepoData(this.data, this.isRemote);
  get isLocal => !this.isRemote;
}

class PersistentSmartRepo extends SmartRepo {
  final String key;

  Future<Database> get getDb async {
    var dbClient = await DatabaseHelper().db;
    // print("Create db table $key...");
    dbClient.execute(
        "CREATE TABLE IF NOT EXISTS $key (t_key TEXT PRIMARY KEY, t_val TEXT)");
    return dbClient;
  }

  PersistentSmartRepo(this.key) {
    // db.execute('DROP TABLE $key');
  }

  Future<T> fetch<T extends dynamic>(
      String storeName, Future<T> Function() dataRetriever,
      {force: bool}) async {
    final dbClient = await getDb;

    List<Map> result = await dbClient
        .rawQuery('SELECT * FROM $key WHERE t_key=\'$storeName\' LIMIT 1');

    T resultData;

    if (result.length > 0) {
      resultData = json.decode(result.first["t_val"]);
    } else {
      resultData = null;
    }

    // print("resultData: $resultData");

    if (resultData == null || force == true) {
      final data = await dataRetriever();
      if (data != null) {
        print("resp data: $data");

        // await dbClient.insert(
        //     key, {"t_key": storeName, "t_val": json.encode(data["result"])});

        final tVal = json.encode(data["result"]);

        await dbClient.rawInsert(
            "INSERT OR REPLACE INTO $key (t_key, t_val)VALUES(?, ?)",
            [storeName, tVal]);

        resultData = data["result"];
      }
    }

    return resultData;
  }

  /// Fetch data gradually return in stream (generator in Python term).
  /// first return will be data from local if any, otherwise from remote.
  /// second return will be data from remote
  Stream<RepoData<dynamic>> fetchGradually(
      String storeName, Future<dynamic> Function() dataRetriever,
      {force: bool}) async* {
    final dbClient = await getDb;

    List<Map> result = await dbClient
        .rawQuery('SELECT * FROM $key WHERE t_key=\'$storeName\' LIMIT 1');

    dynamic resultData;

    if (result.length > 0) {
      resultData = json.decode(result.first["t_val"]);
      yield RepoData(resultData, false);
    }

    // print("fetchGradually.resultData: $resultData");

    final data = await dataRetriever();
    if (data != null) {
      // print("resp data: $data");

      final tVal = json.encode(data["result"]);

      await dbClient.rawInsert(
          "INSERT OR REPLACE INTO $key (t_key, t_val)VALUES(?, ?)",
          [storeName, tVal]);

      resultData = data["result"];
      yield RepoData(resultData, true);
    }
  }

  Future<dynamic> fetchApi(String storeName, String apiPath,
      {force: bool}) {
    return fetch(storeName, () => PublicApi.get(apiPath), force: force);
  }

  Future<Map<String, dynamic>> getEntriesItem(
      String storeName, dynamic id) async {
    final dbClient = await getDb;

    List<Map> result = await dbClient
        .rawQuery('SELECT * FROM $key WHERE t_key=\'$storeName\' LIMIT 1');

    Map<String, dynamic> resultData;

    if (result.length > 0) {
      Map<String, dynamic> entriesData = json.decode(result.first["t_val"]);
      resultData =
          (entriesData["entries"] as List).where((a) => a["id"] == id).first;
    } else {
      resultData = null;
    }
    return resultData;
  }

  /// Update entries ini akan me-replace data di dalam array
  /// dilakukan dengan cara pengecheckan berdasarkan id-nya.
  Future<void> updateEntriesItem(
      String storeName, Map<String, dynamic> value) async {
    final dbClient = await getDb;

    List<Map> result = await dbClient
        .rawQuery('SELECT * FROM $key WHERE t_key=\'$storeName\' LIMIT 1');

    Map<String, dynamic> resultData;

    if (result.length > 0) {
      resultData = json.decode(result.first["t_val"]);
    } else {
      resultData = null;
    }

    // print("updateEntriesItem>resultData: $resultData");

    if (resultData != null) {
      bool replaced = false;
      List<dynamic> newEntries = (resultData["entries"] as List).map((a) {
        if (a["id"] == value["id"]) {
          replaced = true;
          return value;
        } else {
          return a;
        }
      }).toList();

      // jika belum ada maka tambahkan
      if (!replaced) {
        newEntries.add(value);
      }

      // await dbClient.insert(
      //       key, {"t_key": storeName, "t_val": json.encode(newEntries)});
      final tVal = json.encode({"entries": newEntries});

      await dbClient.rawInsert(
          "INSERT OR REPLACE INTO $key (t_key, t_val)VALUES(?, ?)",
          [storeName, tVal]);
    }
  }

  Future<void> deleteEntriesItem(
      String storeName, Map<String, dynamic> value) async {
    final dbClient = await getDb;

    List<Map> result = await dbClient
        .rawQuery('SELECT * FROM $key WHERE t_key=\'$storeName\' LIMIT 1');

    Map<String, dynamic> resultData;

    if (result.length > 0) {
      resultData = json.decode(result.first["t_val"]);
    } else {
      resultData = null;
    }

    // print("deleteEntriesItem>resultData: $resultData");

    if (resultData != null) {
      List<dynamic> newEntries = (resultData["entries"] as List)
          .where((a) => a["id"] != value["id"])
          .toList();

      // print("deleteEntriesItem>newEntries: $newEntries");

      final tVal = json.encode({"entries": newEntries});

      await dbClient.rawQuery(
          "INSERT OR REPLACE INTO $key (t_key, t_val)VALUES(?, ?)",
          [storeName, tVal]);
    }
  }

  Future<void> putData(String storeName, Map<String, dynamic> data) async {
    final dbClient = await getDb;

    final tVal = json.encode(data);
    await dbClient.rawQuery(
        "INSERT OR REPLACE INTO $key (t_key, t_val)VALUES(?, ?)",
        [storeName, tVal]);
  }

  Future<Map<String, dynamic>> getData(String storeName) async {
    final dbClient = await getDb;
    List<Map> result = await dbClient
        .rawQuery('SELECT * FROM $key WHERE t_key=\'$storeName\' LIMIT 1');

    Map<String, dynamic> resultData;

    if (result.length > 0) {
      resultData = json.decode(result.first["t_val"]);
    } else {
      resultData = null;
    }

    return resultData;
  }

  void clear() async {
    final dbClient = await getDb;
    dbClient.rawQuery("DROP TABLE $key");
  }
}

