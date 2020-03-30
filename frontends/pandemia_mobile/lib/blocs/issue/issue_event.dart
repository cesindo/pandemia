import 'package:equatable/equatable.dart';
import 'package:meta/meta.dart';
import 'package:pandemia_mobile/models/issue.dart';

@immutable
abstract class IssueEvent extends Equatable {
  IssueEvent([List props = const []]) : super(props);
}

class LoadIssue extends IssueEvent {
  final bool force;
  LoadIssue({this.force = false});

  @override
  String toString() => "LoadIssue";
}

class LoadDetailIssue extends IssueEvent {
  final bool force;
  final String id;
  LoadDetailIssue(this.id, {this.force = false});

  @override
  String toString() => "LoadDetailIssue";
}

class LoadMoreIssue extends IssueEvent {
  LoadMoreIssue();

  @override
  String toString() => "LoadMoreIssue";
}

class CreateIssue extends IssueEvent {
  final int id;
  final String text;
  CreateIssue(this.id, this.text);
  @override
  String toString() => "CreateIssue";
}

/// Event to delete Issue
class DeleteIssue extends IssueEvent {
  final Issue issue;
  DeleteIssue(this.issue);
  @override
  String toString() => "DeleteIssue";
}
