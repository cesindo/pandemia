import 'dart:async';
import 'package:bloc/bloc.dart';
import 'package:pandemia_mobile/blocs/tab/tab_event.dart';
import 'package:pandemia_mobile/models/app_tab.dart';

class TabBloc extends Bloc<TabEvent, AppTab> {
  @override
  AppTab get initialState => AppTab.updates;

  @override
  Stream<AppTab> mapEventToState(TabEvent event) async* {
    if (event is UpdateTab) {
      yield event.tab;
    }
  }
}

