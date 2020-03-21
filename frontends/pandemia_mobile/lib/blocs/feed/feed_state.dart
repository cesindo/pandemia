
import 'package:equatable/equatable.dart';
import 'package:meta/meta.dart';
import 'package:pandemia_mobile/models/feed.dart';

@immutable
abstract class FeedState extends Equatable {
  FeedState([List props = const []]) : super(props);
}

/// Loading state
class FeedLoading extends FeedState {
  /// Set true to block screen with blocking loading modal box.
  final bool block;
  FeedLoading({this.block = false});
  @override
  String toString() => "FeedLoading";
}

class FeedsLoaded extends FeedState {
  final List<Feed> items;
  FeedsLoaded(this.items);
  @override
  String toString() => "FeedsLoaded";
}

class FeedsUpdated extends FeedState {
  final List<Feed> items;
  FeedsUpdated(this.items);
  @override
  String toString() => "FeedsUpdated";
}

/// State when error/failure occurred
class FeedFailure extends FeedState {
  final String error;
  FeedFailure({this.error}) : super([error]);
  @override
  String toString() => "FeedFailure";
}

class FeedCreated extends FeedState {
  final Feed item;
  FeedCreated(this.item);
  @override
  String toString() => "FeedCreated";
}

/// State when Feed already deleted
class FeedDeleted extends FeedState {
  final Feed feed;
  FeedDeleted(this.feed);
  @override
  String toString() => "FeedDeleted";
}
