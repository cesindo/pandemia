import 'package:pandemia_mobile/core/smart_repo.dart';

import 'package:bloc/bloc.dart';
import 'package:pandemia_mobile/api/pandemia_api.dart';
import 'package:pandemia_mobile/blocs/feed/feed_event.dart';
import 'package:pandemia_mobile/blocs/feed/feed_state.dart';
import 'package:pandemia_mobile/models/feed.dart';

class FeedBloc extends Bloc<FeedEvent, FeedState> {
  PersistentSmartRepo repo;

  FeedBloc() {
    repo = PersistentSmartRepo("bloc_feed");
  }

  @override
  FeedState get initialState => FeedLoading();

  @override
  Stream<FeedState> mapEventToState(FeedEvent event) async* {
    if (event is LoadFeed) {
      yield* _mapLoadFeedToState(event);
    } else if (event is CreateFeed) {
      yield* _mapCreateFeedToState(event);
    } else if (event is DeleteFeed) {
      yield* _mapDeleteToState(event);
    } else if (event is RefreshFeed) {
      yield DoRefreshFeed();
    } else if (event is LoadMoreFeed && !_hasReachedMax(currentState)) {
      yield* _mapMoreFeedToState(event);
    }
  }

  bool _hasReachedMax(FeedState state) =>
      state is FeedsUpdated && state.hasReachedMax;

  Stream<FeedState> _mapLoadFeedToState(LoadFeed event) async* {
    yield FeedLoading();

    yield* repo
        .fetchGradually(
            "entries",
            () => PublicApi.get(
                "/feed/v1/query?exclude_loc=global&offset=0&limit=10"),
            force: event.force)
        .asyncExpand((d) async* {
      if (d != null) {
        final entries = (d.data["entries"] as List<dynamic>)
            .map((a) => Feed.fromMap(a))
            .toList();

        if (d.isLocal) {
          yield FeedsLoaded(entries);
        } else {
          yield FeedsUpdated(
            items: entries,
            hasReachedMax: false,
            isLoading: false,
          );
        }
      } else {
        yield FeedFailure(error: "Cannot get Feed data from server");
      }
    });

    // final data = await repo.fetchApi(
    //   "entries", "/feed/v1/query?loc=Indonesia&query=&offset=0&limit=10",
    //   force: event.force);

    // if (data != null) {
    //   yield FeedListLoaded((data["entries"] as List<dynamic>)
    //       .map((a) => Feed.fromMap(a))
    //       .toList());
    // } else {
    //   yield FeedFailure(error: "Cannot get feed data from server");
    // }
  }

  Stream<FeedState> _mapMoreFeedToState(LoadMoreFeed event) async* {
    yield (currentState as FeedsUpdated).copyWith(isLoading: true);

    // await new Future.delayed(new Duration(milliseconds: 300));

    final entries = await _fetchFeeds(
        (currentState as FeedsUpdated).items.length, 10, true);
    (currentState as FeedsUpdated).items.addAll(entries);
    var data = (currentState as FeedsUpdated).items;
    data = data.toSet().toList();
    repo.putData(
        "entries", {"entries": data.map((f) => f.toMap() as dynamic).toList()});

    yield entries.isEmpty
        ? (currentState as FeedsUpdated).copyWith(hasReachedMax: true)
        : FeedsUpdated(items: data, hasReachedMax: false, isLoading: false);
  }

  Future<List<Feed>> _fetchFeeds(int offset, int limit, bool force) async {
    final d = await repo.fetchApi("entries",
        "/feed/v1/query?exclude_loc=global&offset=$offset&limit=$limit",
        force: force);
    if (d != null) {
      return (d["entries"] as List<dynamic>)
          .map((a) => Feed.fromMap(a))
          .toList();
    } else {
      return null;
    }
  }

  Stream<FeedState> _mapCreateFeedToState(CreateFeed event) async* {
    yield FeedLoading();

    final data = await PublicApi.post("/feed/v1/add", {
      // @TODO(you): add params to post here
    });

    if (data != null) {
      print("resp data: $data");

      repo.updateEntriesItem("entries", data["result"]);

      yield FeedCreated(Feed.fromMap(data["result"]));

      dispatch(LoadFeed());
    } else {
      yield FeedFailure(error: "Cannot add Feed");
    }
  }

  Stream<FeedState> _mapDeleteToState(DeleteFeed event) async* {
    yield FeedLoading();

    final data = await PublicApi.post("/feed/v1/delete", {"id": event.feed.id});

    if (data != null) {
      await repo.deleteEntriesItem("entries", event.feed.toMap());

      yield FeedDeleted(event.feed);
      dispatch(LoadFeed(force: false));
    } else {
      yield FeedFailure(error: "Cannot delete Feed");
    }
  }
}
