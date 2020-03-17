import 'package:flutter/material.dart';
import 'package:bloc/bloc.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:pandemia_mobile/api/api_client.dart';
import 'package:pandemia_mobile/blocs/pandemia/pandemia_bloc.dart';
import 'package:pandemia_mobile/blocs/pandemia/pandemia_event.dart';
import 'package:pandemia_mobile/blocs/pandemia/pandemia_state.dart';
import 'package:pandemia_mobile/blocs/simple_bloc_delegate.dart';
import 'package:pandemia_mobile/blocs/tab/tab_bloc.dart';
import 'package:pandemia_mobile/screens/home.dart';
import 'package:pandemia_mobile/screens/login/login.dart';
import 'package:pandemia_mobile/screens/splash/splash_page.dart';
import 'package:pandemia_mobile/user_repository/user_repository.dart';
import 'blocs/notif/notif.dart';

void main() {
  BlocSupervisor.delegate = SimpleBlocDelegate();
  
  final UserRepository userRepository = UserRepository();

  ApiClient.userRepository = userRepository;

  runApp(BlocProvider(
    builder: (ctx) {
      return PandemiaBloc(userRepository: userRepository)
        ..dispatch(StartupEvent());
    },
    child: PandemiaApp(userRepository: userRepository),
  ));
}

class PandemiaApp extends StatelessWidget {
  final UserRepository userRepository;

  PandemiaApp({Key key, @required this.userRepository}) : super(key: key);

  // This widget is the root of your application.
  @override
  Widget build(BuildContext context) {
    final pandemiaBloc = BlocProvider.of<PandemiaBloc>(context);

    return MaterialApp(title: 'Pandemia', theme: PandemiaTheme.theme, routes: {
      "/": (context) {
        return BlocListener<PandemiaBloc, PandemiaState>(
          listener: (BuildContext context, PandemiaState state) {
            print("main state: $state");
            if (state is AuthenticationUnauthenticated) {
              Navigator.of(context).pushReplacementNamed('/login');
            } else if (state is AuthenticationAuthenticated) {
              Navigator.of(context).pushReplacementNamed('/inner');
            }
          },
          child: SplashPage(),
        );
      },
      "/inner": (context) {
        return MultiBlocProvider(
          providers: [
            BlocProvider<PandemiaBloc>(
              builder: (context) => PandemiaBloc(userRepository: userRepository),
            ),
            BlocProvider<TabBloc>(
              builder: (context) => TabBloc(),
            ),
            BlocProvider<NotifBloc>(builder: (context) => NotifBloc(pandemiaBloc: pandemiaBloc),),
            // BlocProvider<TaskManagerBloc>(builder: (context) => TaskManagerBloc(),),
          ],
          child: HomeScreen(
            title: "Pandemia Home",
            pandemiaBloc: pandemiaBloc
          ),
        );
      },
      "/login": (context) {
        return BlocListener<PandemiaBloc, PandemiaState>(
          listener: (BuildContext context, PandemiaState state) {
            if (state is AuthenticationAuthenticated) {
              Navigator.of(context).pushReplacementNamed('/inner');
            }
          },
          child: LoginPage(
            userRepository: userRepository,
          ),
        );
      }
    });
  }
}


class PandemiaTheme {
  static get theme {
    final originalTextTheme = ThemeData.light().textTheme;
    final originalBody1 = originalTextTheme.body1;

    return ThemeData.light().copyWith(
        primaryColor: Colors.grey[100],
        accentColor: Colors.cyan[300],
        buttonColor: Colors.grey[800],
        textSelectionColor: Colors.cyan[100],
        backgroundColor: Colors.grey[900],
        toggleableActiveColor: Colors.cyan[300],
        textTheme: originalTextTheme.copyWith(
            body1:
                originalBody1.copyWith(decorationColor: Colors.transparent)));
  }
}

