import 'package:pandemia_mobile/core/smart_repo.dart';

import 'package:bloc/bloc.dart';
import 'package:pandemia_mobile/api/pandemia_api.dart';
import 'package:pandemia_mobile/blocs/report_note/report_note_event.dart';
import 'package:pandemia_mobile/blocs/report_note/report_note_state.dart';
import 'package:pandemia_mobile/models/report_note.dart';

class ReportNoteBloc extends Bloc<ReportNoteEvent, ReportNoteState> {
  PersistentSmartRepo repo;

  ReportNoteBloc() {
    repo = PersistentSmartRepo("bloc_report_note");
  }

  @override
  ReportNoteState get initialState => ReportNoteLoading();

  @override
  Stream<ReportNoteState> mapEventToState(ReportNoteEvent event) async* {
    if (event is LoadReportNote) {
      yield* _mapLoadReportNoteToState(event);
    } else if (event is CreateReportNote) {
      yield* _mapCreateReportNoteToState(event);
    } else if (event is DeleteReportNote) {
      yield* _mapDeleteToState(event);
    }
  }

  Stream<ReportNoteState> _mapLoadReportNoteToState(
      LoadReportNote event) async* {
    // yield ReportNoteListLoading();

    // final data = await repo.fetchApi(
    //     "entries", "/report_note/v1/list?offset=0&limit=10",
    //     force: event.force);

    // if (data != null) {
    //   yield ReportNoteListLoaded((data["result"]["entries"] as List<dynamic>)
    //       .map((a) => ReportNote.fromMap(a))
    //       .toList());
    // } else {
    //   yield ReportNoteFailure(error: "Cannot get report_note data from server");
    // }
    yield ReportNoteReady();
  }

  Stream<ReportNoteState> _mapCreateReportNoteToState(
      CreateReportNote event) async* {
    yield ReportNoteLoading();

    final data = await PublicApi.post(
        "/pandemia/v1/report_note/add", {'notes': event.text});

    if (data != null) {
      print("resp data: $data");

      repo.updateEntriesItem("entries", data["result"]);

      yield ReportNoteCreated(ReportNote.fromMap(data["result"]));

      dispatch(LoadReportNote());
    } else {
      yield ReportNoteFailure(error: "Cannot add ReportNote");
    }
  }

  Stream<ReportNoteState> _mapDeleteToState(DeleteReportNote event) async* {
    yield ReportNoteLoading();

    final data = await PublicApi.post(
        "/report_note/v1/delete", {"id": event.reportNote.id});

    if (data != null) {
      await repo.deleteEntriesItem("entries", event.reportNote.toMap());

      yield ReportNoteDeleted(event.reportNote);
      dispatch(LoadReportNote(force: false));
    } else {
      yield ReportNoteFailure(error: "Cannot delete ReportNote");
    }
  }
}
