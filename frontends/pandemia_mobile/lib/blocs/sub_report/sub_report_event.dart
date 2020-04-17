import 'package:equatable/equatable.dart';
import 'package:meta/meta.dart';
import 'package:pandemia_mobile/models/sub_report.dart';

@immutable
abstract class SubReportEvent extends Equatable {
  SubReportEvent([List props = const []]) : super(props);
}

class LoadSubReport extends SubReportEvent {
  final bool force;
  final String status;
  final bool withLoading;
  LoadSubReport({this.status, this.force = false, this.withLoading = true});

  @override
  String toString() => "LoadSubReport";
}

class CreateSubReport extends SubReportEvent {
  final String fullName;
  final int age;
  final String residenceAddress;
  final String gender;
  final String comingFrom;
  final String arrivalDate;
  final String notes;
  final String status;
  final List<String> complaint;
  final List<String> addInfo;

  CreateSubReport(
      this.fullName,
      this.age,
      this.residenceAddress,
      this.gender,
      this.comingFrom,
      this.arrivalDate,
      this.notes,
      this.status,
      this.complaint,
      this.addInfo,
      );
  @override
  String toString() => "CreateSubReport";
}

class UpdateSubReport extends SubReportEvent {
  final int id;
  final String fullName;
  final int age;
  final String residenceAddress;
  final String gender;
  final String comingFrom;
  final String arrivalDate;
  final String notes;
  final String status;
  final List<String> complaint;
  final List<String> addInfo;

  UpdateSubReport(
      this.id,
      this.fullName,
      this.age,
      this.residenceAddress,
      this.gender,
      this.comingFrom,
      this.arrivalDate,
      this.notes,
      this.status,
      this.complaint,
      this.addInfo);
  @override
  String toString() => "UpdateSubReport";
}

/// Event to delete SubReport
class DeleteSubReport extends SubReportEvent {
  final SubReport subReport;
  DeleteSubReport(this.subReport);
  @override
  String toString() => "DeleteSubReport";
}

class SubReportSearch extends SubReportEvent {
  final String query;
  final int status;
  SubReportSearch(this.query, this.status);
  @override
  String toString() => "SubReportSearch";
}
