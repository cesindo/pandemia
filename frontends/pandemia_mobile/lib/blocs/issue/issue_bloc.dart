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
    } 
  }

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

  Stream<IssueState> _mapLoadIssueToState(LoadIssue event) async* {
    yield IssueLoading();

    yield* repo
        .fetchGradually(
            "entries",
            () => DetaxApi.get(
                "/detax/v1/issue/search?query=covid-19&offset=0&limit=20"),
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
