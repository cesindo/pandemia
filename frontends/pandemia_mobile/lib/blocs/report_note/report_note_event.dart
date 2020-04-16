
import 'package:equatable/equatable.dart';
import 'package:meta/meta.dart';
import 'package:pandemia_mobile/models/report_note.dart';

@immutable
abstract class ReportNoteEvent extends Equatable {
  ReportNoteEvent([List props = const []]) : super(props);
}

class LoadReportNote extends ReportNoteEvent {
  final bool force;
  LoadReportNote({this.force=false});

  @override
  String toString() => "LoadReportNote";
}

class CreateReportNote extends ReportNoteEvent {
  final String text;
  CreateReportNote(this.text);
  @override
  String toString() => "CreateReportNote";
}

/// Event to delete ReportNote
class DeleteReportNote extends ReportNoteEvent {
  final ReportNote reportNote;
  DeleteReportNote(this.reportNote);
  @override
  String toString() => "DeleteReportNote";
}
  
