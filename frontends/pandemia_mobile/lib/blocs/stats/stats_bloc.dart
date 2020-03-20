import 'package:pandemia_mobile/core/smart_repo.dart';

import 'package:bloc/bloc.dart';
import 'package:pandemia_mobile/blocs/stats/stats_event.dart';
import 'package:pandemia_mobile/blocs/stats/stats_state.dart';
import 'package:pandemia_mobile/models/record.dart';

class StatsBloc extends Bloc<StatsEvent, StatsState> {
  PersistentSmartRepo repo;

  StatsBloc() {
    repo = PersistentSmartRepo("bloc_stats");
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
    yield StatsLoading();

    // yield* repo
    //     .fetchGradually("entries",
    //         () => PublicApi.get("/pandemia/v1/info_location?loc=global"),
    //         force: event.force)
    //     .map((d) {
    //   if (d != null) {
    //     final entries = (d.data["entries"] as List<dynamic>)
    //         .map((a) => Record.fromMap(a))
    //         .toList();

    //     if (d.isLocal) {
    //       return StatsLoaded(entries);
    //     } else {
    //       return StatsUpdated(entries);
    //     }
    //   } else {
    //     return StatsFailure(error: "Cannot get Stats data from server");
    //   }
    // });

    // print("data2: $data2");
    // if (data2 != null) {
    //   dataAll.add(data2);
    // } else {
    //   yield StatsFailure(error: "Cannot get stats data from server");
    // }

    final dataAll = await getStats(event.force);

    if (dataAll.isNotEmpty) {
      yield StatsLoaded(dataAll.map((a) => Record.fromMap(a)).toList());
    }

    if (!event.force) {
      final dataAll = await getStats(true);
      yield StatsUpdated(dataAll.map((a) => Record.fromMap(a)).toList());
    }
  }

  Future<List<dynamic>> getStats(bool force) async {
    List<dynamic> dataAll = [];
    final data = await repo.fetchApi(
        "entries", "/pandemia/v1/info_location?loc=global",
        force: force);

    if (data != null) {
      dataAll.add(data);
    }
    final data2 = await repo.fetchApi(
        "entries", "/pandemia/v1/info_location?loc=Indonesia",
        force: force);

    if (data2 != null) {
      dataAll.add(data2);
    }
    return dataAll;
  }
}
