
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
  final bool hasReachedMax;
  final bool isLoading;

  FeedsUpdated({this.items, this.hasReachedMax, this.isLoading})
      : super([items, hasReachedMax, isLoading]);

  FeedsUpdated copyWith({
    List<Feed> items,
    bool hasReachedMax,
    bool isLoading,
  }) {
    return FeedsUpdated(
        items: items ?? this.items,
        hasReachedMax: hasReachedMax ?? this.hasReachedMax,
        isLoading: isLoading ?? this.isLoading);
  }

  @override
  String toString() =>
      'FeedsUpdated { Feeds: ${items.length}, hasReachedMax: $hasReachedMax, isLoading: $isLoading}';
}

/// State when error/failure occurred
class FeedFailure extends FeedState {
  final String error;
  FeedFailure({this.error}) : super([error]);
  @override
  String toString() => "FeedFailure";
}
