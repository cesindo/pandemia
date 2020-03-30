import 'package:equatable/equatable.dart';
import 'package:meta/meta.dart';
import 'package:pandemia_mobile/models/issue.dart';
import 'package:pandemia_mobile/models/issue_detail.dart';

@immutable
abstract class IssueState extends Equatable {
  IssueState([List props = const []]) : super(props);
}

/// Loading state
class IssueLoading extends IssueState {
  /// Set true to block screen with blocking loading modal box.
  final bool block;
  IssueLoading({this.block = false});
  @override
  String toString() => "IssueLoading";
}

class IssueListLoading extends IssueState {
  @override
  String toString() => "IssueListLoading";
}

class IssueListLoaded extends IssueState {
  final List<Issue> items;
  IssueListLoaded(this.items);
  @override
  String toString() => "IssueListLoaded";
}

class IssueListUpdated extends IssueState {
  final List<Issue> items;
  final bool hasReachedMax;
  final bool isLoading;

  IssueListUpdated({this.items, this.hasReachedMax, this.isLoading})
      : super([items, hasReachedMax, isLoading]);

  IssueListUpdated copyWith({
    List<Issue> items,
    bool hasReachedMax,
    bool isLoading,
  }) {
    return IssueListUpdated(
        items: items ?? this.items,
        hasReachedMax: hasReachedMax ?? this.hasReachedMax,
        isLoading: isLoading ?? this.isLoading);
  }

  @override
  String toString() =>
      'IssueListUpdated { Issues: ${items.length}, hasReachedMax: $hasReachedMax, isLoading: $isLoading}';
}

/// State when error/failure occurred
class IssueFailure extends IssueState {
  final String error;
  IssueFailure({this.error}) : super([error]);
  @override
  String toString() => "IssueFailure";
}

class IssueCreated extends IssueState {
  final Issue item;
  IssueCreated(this.item);
  @override
  String toString() => "IssueCreated";
}

class IssueDetailLoaded extends IssueState {
  final IssueDetail item;
  IssueDetailLoaded(this.item);
  @override
  String toString() => "IssueDetailLoaded";
}

/// State when Issue already deleted
class IssueDeleted extends IssueState {
  final Issue issue;
  IssueDeleted(this.issue);
  @override
  String toString() => "IssueDeleted";
}
