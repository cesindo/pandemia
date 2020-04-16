
import 'package:equatable/equatable.dart';
import 'package:meta/meta.dart';
import 'package:pandemia_mobile/models/report_note.dart';

@immutable
abstract class ReportNoteState extends Equatable {
  ReportNoteState([List props = const []]) : super(props);
}

/// Loading state
class ReportNoteLoading extends ReportNoteState {
  /// Set true to block screen with blocking loading modal box.
  final bool block;
  ReportNoteLoading({this.block = false});
  @override
  String toString() => "ReportNoteLoading";
}

class ReportNoteReady extends ReportNoteState {
  @override
  String toString() => "ReportNoteReady";
}

class ReportNoteListLoading extends ReportNoteState {
  @override
  String toString() => "ReportNoteListLoading";
}

class ReportNoteListLoaded extends ReportNoteState {
  final List<ReportNote> items;
  ReportNoteListLoaded(this.items);
  @override
  String toString() => "ReportNoteListLoaded";
}

/// State when error/failure occurred
class ReportNoteFailure extends ReportNoteState {
  final String error;
  ReportNoteFailure({this.error}) : super([error]);
  @override
  String toString() => "ReportNoteFailure";
}

class ReportNoteCreated extends ReportNoteState {
  final ReportNote item;
  ReportNoteCreated(this.item);
  @override
  String toString() => "ReportNoteCreated";
}

/// State when ReportNote already deleted
class ReportNoteDeleted extends ReportNoteState {
  final ReportNote reportNote;
  ReportNoteDeleted(this.reportNote);
  @override
  String toString() => "ReportNoteDeleted";
}
