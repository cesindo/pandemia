import 'package:pandemia_mobile/api/pandemia_api.dart';
import 'package:pandemia_mobile/core/smart_repo.dart';

import 'package:bloc/bloc.dart';
import 'package:pandemia_mobile/blocs/stats/stats_event.dart';
import 'package:pandemia_mobile/blocs/stats/stats_state.dart';
import 'package:pandemia_mobile/models/info_location.dart';
import 'package:pandemia_mobile/user_repository/user_repository.dart';

class StatsBloc extends Bloc<StatsEvent, StatsState> {
  PersistentSmartRepo repo;

  StatsBloc() {
    repo = PersistentSmartRepo("bloc_stats");
    // @TODO(*): remove this, for dev purpose
    repo.clear();
  }

  @override
  StatsState get initialState => StatsLoading();

  @override
  Stream<StatsState> mapEventToState(StatsEvent event) async* {
    if (event is LoadStats) {
      yield* _mapLoadStatsToState(event);
    }
  }

  Stream<StatsState> _mapLoadStatsToState(LoadStats event) async* {
    if (event.withLoading) {
      yield StatsLoading();
    }

    final locPath = UserRepository().currentUser.locPath;
    final s = locPath.split("/");
    String provPath = "/Indonesia";
    if (s.length > 1){
      provPath = "/${s[0]}/${s[1]}";
    }

    yield* repo
        .fetchGradually(
            "entries",
            () => PublicApi.get(
                "/pandemia/v1/info_locations?loc_path=/global,/Indonesia,$provPath,$locPath&with_history=true"),
            force: event.force)
        .asyncExpand((d) async* {
      if (d != null && d.data != null) {
        final entries = (d.data as List<dynamic>)
            .map((a) => InfoLocation.fromMap(a))
            .toList();

        entries.sort((a, b) =>
            -a.latestRecord.totalCases.compareTo(b.latestRecord.totalCases));

        if (d.isLocal) {
          yield StatsLoaded(entries);
        } else {
          yield StatsUpdated(entries);
        }
      } else {
        yield StatsFailure(error: "Cannot get Record data from server");
      }
    });
  }
}
