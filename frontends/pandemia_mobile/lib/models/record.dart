// this code is autogenerated by ansvia-vscode extension.
// please don't edit this by hand
// use 'ansvia-vscode extension > Edit Model fields' instead.
import 'package:equatable/equatable.dart';
import 'package:meta/meta.dart';

/// Model for Record
@immutable
class Record extends Equatable {
  final int id;
  final String loc;
  final int locKind;
  final int totalCases;
  final int totalDeaths;
  final int totalRecovered;
  final int activeCases;
  final int criticalCases;
  final double casesToPop;
  final List<String> meta;
  final String lastUpdated;

  Record(
      this.id,
      this.loc,
      this.locKind,
      this.totalCases,
      this.totalDeaths,
      this.totalRecovered,
      this.activeCases,
      this.criticalCases,
      this.casesToPop,
      this.meta,
      this.lastUpdated)
      : super([
          id,
          loc,
          locKind,
          totalCases,
          totalDeaths,
          totalRecovered,
          activeCases,
          criticalCases,
          casesToPop,
          meta,
          lastUpdated
        ]);

  Map<String, dynamic> toMap() {
    Map<String, dynamic> data = Map();
    data["id"] = this.id;
    data["loc"] = this.loc;
    data["loc_kind"] = this.locKind;
    data["total_cases"] = this.totalCases;
    data["total_deaths"] = this.totalDeaths;
    data["total_recovered"] = this.totalRecovered;
    data["active_cases"] = this.activeCases;
    data["critical_cases"] = this.criticalCases;
    data["cases_to_pop"] = this.casesToPop;
    data["meta"] = this.meta;
    data["last_updated"] = this.lastUpdated;
    return data;
  }

  static Record fromMap(Map<String, dynamic> data) {
    assert(data['loc'] != null, "Record.loc is null");
    assert(data['loc_kind'] != null, "Record.loc_kind is null");
    assert(data['total_cases'] != null, "Record.total_cases is null");
    assert(data['total_deaths'] != null, "Record.total_deaths is null");
    assert(data['total_recovered'] != null, "Record.total_recovered is null");
    assert(data['active_cases'] != null, "Record.active_cases is null");
    assert(data['critical_cases'] != null, "Record.critical_cases is null");
    assert(data['cases_to_pop'] != null, "Record.cases_to_pop is null");
    assert(data['last_updated'] != null, "Record.last_updated is null");
    return Record(
        data['id'] as int,
        data['loc'] as String,
        data['loc_kind'] as int,
        data['total_cases'] as int,
        data['total_deaths'] as int,
        data['total_recovered'] as int,
        data['active_cases'] as int,
        data['critical_cases'] as int,
        data['cases_to_pop'] as double,
        data['meta'] != null ? List.from(data['meta']) : [],
        data['last_updated'] as String);
  }

  Record copy(
      {String loc,
      int locKind,
      int totalCases,
      int totalDeaths,
      int totalRecovered,
      int activeCases,
      int criticalCases,
      double casesToPop,
      List<String> meta,
      String lastUpdated}) {
    return Record(
        this.id,
        loc ?? this.loc,
        locKind ?? this.locKind,
        totalCases ?? this.totalCases,
        totalDeaths ?? this.totalDeaths,
        totalRecovered ?? this.totalRecovered,
        activeCases ?? this.activeCases,
        criticalCases ?? this.criticalCases,
        casesToPop ?? this.casesToPop,
        meta ?? this.meta,
        lastUpdated ?? this.lastUpdated);
  }
}