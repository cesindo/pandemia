import 'dart:async';
import 'package:bloc/bloc.dart';
import 'package:meta/meta.dart';
import 'package:pandemia_mobile/blocs/notif/notif_event.dart';
import 'package:pandemia_mobile/blocs/notif/notif_state.dart';
import 'package:pandemia_mobile/blocs/pandemia/pandemia_bloc.dart';
import 'package:pandemia_mobile/models/notif_item.dart';

class NotifBloc extends Bloc<NotifEvent, NotifState> {
  final PandemiaBloc pandemiaBloc;
  // StreamSubscription _pandemiaSubs;

  NotifBloc({@required this.pandemiaBloc}){
    // _pandemiaSubs = pandemiaBloc.state.listen((state){
    //   // if (state is AuthenticationAuthenticated){
    //   //   dispatch(LoadNotif());
    //   // }
    // });
  }


  @override
  NotifState get initialState => NotifListLoading();

  @override
  Stream<NotifState> mapEventToState(NotifEvent event) async* {
    if (event is LoadNotif) {
      yield* _mapLoadNotifToState(event);
    }
  }

  Stream<NotifState> _mapLoadNotifToState(LoadNotif event) async* {
    // @TODO(*): fix this
    yield NotifListLoading();
    // final result = ApiClient.public().
    yield NotifListLoaded([NotifItem(1, "a", "Halo", 1, 2, [], "1 hour ago")]);
  }

}

