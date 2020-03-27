import 'package:equatable/equatable.dart';
import 'package:meta/meta.dart';
import 'package:pandemia_mobile/models/feed.dart';

@immutable
abstract class FeedEvent extends Equatable {
  FeedEvent([List props = const []]) : super(props);
}

class LoadFeed extends FeedEvent {
  final bool force;
  LoadFeed({this.force = false});

  @override
  String toString() => "LoadFeed";
}

class LoadMoreFeed extends FeedEvent {
  LoadMoreFeed();
  @override
  String toString() => "LoadMoreFeed";
}

class RefreshFeed extends FeedEvent {
  RefreshFeed();
  @override
  String toString() => "RefreshFeed";
}

class CreateFeed extends FeedEvent {
  final int id;
  final String text;
  CreateFeed(this.id, this.text);
  @override
  String toString() => "CreateFeed";
}

/// Event to delete Feed
class DeleteFeed extends FeedEvent {
  final Feed feed;
  DeleteFeed(this.feed);
  @override
  String toString() => "DeleteFeed";
}
