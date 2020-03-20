import 'dart:async';
import 'package:bloc/bloc.dart';
import 'package:meta/meta.dart';
import 'package:pandemia_mobile/blocs/pandemia/pandemia_event.dart';
import 'package:pandemia_mobile/blocs/pandemia/pandemia_state.dart';
import 'package:pandemia_mobile/user_repository/user_repository.dart';

class PandemiaBloc extends Bloc<PandemiaEvent, PandemiaState> {
  final UserRepository userRepository;
  PandemiaBloc({@required this.userRepository}) : assert(userRepository != null);

  @override
  PandemiaState get initialState => PandemiaLoading();

  @override
  Stream<PandemiaState> mapEventToState(PandemiaEvent event) async* {
    // if (event is LoggedIn) {
    //   yield* _mapLoginPandemiaToState(event);
    // } else if (event is StartupEvent) {
    if (event is StartupEvent) {
      print("Got startup event");
      yield* _mapStartupToState(event);
    // } else if (event is LoggedOut) {
    //   yield* _mapLoggedOutToState(event);
    }
  }

  // Stream<PandemiaState> _mapLoginPandemiaToState(LoggedIn event) async* {
  //   yield AuthenticationLoading();
  //   await userRepository.persistToken(event.token);
  //   yield AuthenticationAuthenticated();
  // }

  Stream<PandemiaState> _mapStartupToState(StartupEvent event) async* {
    // final bool hasToken = await userRepository.hasToken();

    // if (hasToken) {
    //   yield AuthenticationAuthenticated();
    // } else {
    //   yield AuthenticationUnauthenticated();
    // }
    // sleep(Duration(seconds: 5));
    yield PandemiaReady();
  }

  // Stream<PandemiaState> _mapLoggedOutToState(LoggedOut event) async* {
  //   yield AuthenticationLoading();
  //   await userRepository.deleteToken();
  //   ApiResource.accessToken = "";
  //   yield AuthenticationUnauthenticated();
  // }

}

