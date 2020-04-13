
import 'package:equatable/equatable.dart';
import 'package:meta/meta.dart';
import 'package:pandemia_mobile/models/sub_report.dart';

@immutable
abstract class SubReportState extends Equatable {
  SubReportState([List props = const []]) : super(props);
}

/// Loading state
class SubReportLoading extends SubReportState {
  /// Set true to block screen with blocking loading modal box.
  final bool block;
  SubReportLoading({this.block = false});
  @override
  String toString() => "SubReportLoading";
}

class SubReportListLoading extends SubReportState {
  @override
  String toString() => "SubReportListLoading";
}

class SubReportListLoaded extends SubReportState {
  final List<SubReport> items;
  SubReportListLoaded(this.items);
  @override
  String toString() => "SubReportListLoaded";
}

class SubReportListUpdated extends SubReportState {
  final List<SubReport> items;
  SubReportListUpdated(this.items);
  @override
  String toString() => "SubReportListUpdated";
}

/// State when error/failure occurred
class SubReportFailure extends SubReportState {
  final String error;
  SubReportFailure({this.error}) : super([error]);
  @override
  String toString() => "SubReportFailure";
}

class SubReportCreated extends SubReportState {
  SubReportCreated();
  @override
  String toString() => "SubReportCreated";
}

/// State when SubReport already deleted
class SubReportDeleted extends SubReportState {
  final SubReport subReport;
  SubReportDeleted(this.subReport);
  @override
  String toString() => "SubReportDeleted";
}


class SearchLoading extends SubReportState {
  @override
  String toString() => "SearchLoading";
}