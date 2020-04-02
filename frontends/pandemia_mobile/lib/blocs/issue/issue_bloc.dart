import 'package:bloc/bloc.dart';
import 'package:pandemia_mobile/api/pandemia_api.dart';
import 'package:pandemia_mobile/blocs/issue/issue_event.dart';
import 'package:pandemia_mobile/blocs/issue/issue_state.dart';
import 'package:pandemia_mobile/core/smart_repo.dart';
import 'package:pandemia_mobile/models/issue.dart';
import 'package:pandemia_mobile/models/issue_detail.dart';

class IssueBloc extends Bloc<IssueEvent, IssueState> {
  PersistentSmartRepo repo;

  IssueBloc() {
    repo = PersistentSmartRepo("bloc_issue");
  }

  @override
  IssueState get initialState => IssueLoading();

  @override
  Stream<IssueState> mapEventToState(IssueEvent event) async* {
    if (event is LoadIssue) {
      yield* _mapLoadIssueToState(event);
    } else if (event is CreateIssue) {
      yield* _mapCreateIssueToState(event);
    } else if (event is DeleteIssue) {
      yield* _mapDeleteToState(event);
    } else if (event is LoadDetailIssue) {
      yield* _mapDetailToState(event);
    } else if (event is LoadMoreIssue && !_hasReachedMax(currentState)) {
      yield* _mapMoreIssueToState(event);
    }
  }

  bool _hasReachedMax(IssueState state) =>
      state is IssueListUpdated && state.hasReachedMax;

  Stream<IssueState> _mapDetailToState(LoadDetailIssue event) async* {
    yield IssueLoading();

    final data = await DetaxApi.get("/detax/v1/issue/${event.id}");
    if (data != null) {
      var result = IssueDetail.fromJson(data["result"]);
      yield IssueDetailLoaded(result);
    } else {
      yield IssueFailure(error: "Cannot get issue data from server");
    }
  }

  Stream<IssueState> _mapMoreIssueToState(LoadMoreIssue event) async* {
    yield (currentState as IssueListUpdated).copyWith(isLoading: true);

    // await new Future.delayed(new Duration(milliseconds: 300));

    final entries = await _fetchIssues(
        (currentState as IssueListUpdated).items.length, 10, true);
    (currentState as IssueListUpdated).items.addAll(entries);
    var data = (currentState as IssueListUpdated).items;
    data = data.toSet().toList();
    repo.putData(
        "entries", {"entries": data.map((f) => f.toMap() as dynamic).toList()});

    yield entries.isEmpty
        ? (currentState as IssueListUpdated).copyWith(hasReachedMax: true)
        : IssueListUpdated(items: data, hasReachedMax: false, isLoading: false);
  }

  Future<List<Issue>> _fetchIssues(int offset, int limit, bool force) async {
    final d = await DetaxApi.get(
        "/detax/v1/issue/search?query=covid-19&offset=$offset&limit=$limit");
    if (d != null) {
      return (d["result"]["entries"] as List<dynamic>)
          .map((a) => Issue.fromMap(a))
          .toList();
    } else {
      return null;
    }
  }

  Stream<IssueState> _mapLoadIssueToState(LoadIssue event) async* {
    yield IssueLoading();

    yield* repo
        .fetchGradually(
            "entries",
            () => DetaxApi.get(
                "/detax/v1/issue/search?query=covid-19&offset=0&limit=10"),
            force: event.force)
        .map((d) {
      if (d != null) {
        final entries = (d.data["entries"] as List<dynamic>)
            .map((a) => Issue.fromMap(a))
            .toList();

        if (d.isLocal) {
          return IssueListLoaded(entries);
        } else {
          return IssueListUpdated(
            items: entries,
            hasReachedMax: false,
            isLoading: false,
          );
        }
      } else {
        return IssueFailure(error: "Cannot get issue data from server");
      }
    });
  }

  Stream<IssueState> _mapCreateIssueToState(CreateIssue event) async* {
    yield IssueLoading();

    final data = await PublicApi.post("/issue/v1/add", {
      // @TODO(you): add params to post here
    });

    if (data != null) {
      print("resp data: $data");

      repo.updateEntriesItem("entries", data["result"]);

      yield IssueCreated(Issue.fromMap(data["result"]));

      dispatch(LoadIssue());
    } else {
      yield IssueFailure(error: "Cannot add Issue");
    }
  }

  Stream<IssueState> _mapDeleteToState(DeleteIssue event) async* {
    yield IssueLoading();

    final data =
        await PublicApi.post("/issue/v1/delete", {"id": event.issue.id});

    if (data != null) {
      await repo.deleteEntriesItem("entries", event.issue.toMap());

      yield IssueDeleted(event.issue);
      dispatch(LoadIssue(force: false));
    } else {
      yield IssueFailure(error: "Cannot delete Issue");
    }
  }
}
