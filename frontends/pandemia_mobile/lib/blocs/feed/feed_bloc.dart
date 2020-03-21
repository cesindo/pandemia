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
    }
  }

  Stream<FeedState> _mapLoadFeedToState(LoadFeed event) async* {
    yield FeedLoading();

    yield* repo
        .fetchGradually(
            "entries",
            () => PublicApi.get(
                "/feed/v1/query?loc=Indonesia&query=&offset=0&limit=10"),
            force: event.force)
        .asyncExpand((d) async* {
      if (d != null) {
        final entries = (d.data["entries"] as List<dynamic>)
            .map((a) => Feed.fromMap(a))
            .toList();

        if (d.isLocal) {
          yield FeedsLoaded(entries);
        } else {
          yield FeedsUpdated(entries);
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
