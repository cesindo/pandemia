import 'package:flutter/material.dart';
import 'package:bloc/bloc.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:flutter_dotenv/flutter_dotenv.dart';
import 'package:pandemia_mobile/api/api_client.dart';
import 'package:pandemia_mobile/api/pandemia_api.dart';
import 'package:pandemia_mobile/blocs/fcm/fcm_bloc.dart';
import 'package:pandemia_mobile/blocs/feed/feed.dart';
import 'package:pandemia_mobile/blocs/issue/issue_bloc.dart';
import 'package:pandemia_mobile/blocs/map/map_bloc.dart';
import 'package:pandemia_mobile/blocs/pandemia/pandemia_bloc.dart';
import 'package:pandemia_mobile/blocs/pandemia/pandemia_event.dart';
import 'package:pandemia_mobile/blocs/pandemia/pandemia_state.dart';
import 'package:pandemia_mobile/blocs/settings/settings_bloc.dart';
import 'package:pandemia_mobile/blocs/simple_bloc_delegate.dart';
import 'package:pandemia_mobile/blocs/stats/stats.dart';
import 'package:pandemia_mobile/blocs/sub_report/sub_report.dart';
import 'package:pandemia_mobile/blocs/tab/tab_bloc.dart';
import 'package:pandemia_mobile/core/error.dart';
import 'package:pandemia_mobile/core/smart_repo.dart';
import 'package:pandemia_mobile/screens/about/about_page.dart';
import 'package:pandemia_mobile/screens/home.dart';
import 'package:pandemia_mobile/screens/splash/splash_page.dart';
import 'package:pandemia_mobile/time_helper.dart';
import 'package:pandemia_mobile/user_repository/user_repository.dart';
import 'blocs/notif/notif.dart';
import 'blocs/profile/profile.dart';
import 'core/core.dart';

void main() async {
  await DotEnv().load('.env');
  BlocSupervisor.delegate = SimpleBlocDelegate();

  final UserRepository userRepository = UserRepository();

  ApiClient.userRepository = userRepository;

  WidgetsFlutterBinding.ensureInitialized();

  TimeHelper.setup();

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
  final appRepo = PersistentSmartRepo("pandemia");

  PandemiaApp({Key key, @required this.userRepository}) : super(key: key);

  // This widget is the root of your application.
  @override
  Widget build(BuildContext context) {
    final pandemiaBloc = BlocProvider.of<PandemiaBloc>(context);

    return MaterialApp(title: 'Pandemia', theme: PandemiaTheme.theme, routes: {
      "/": (context) {
        return BlocListener<PandemiaBloc, PandemiaState>(
          listener: (BuildContext context, PandemiaState state) {
            if (state is PandemiaReady) {
              Navigator.of(context).pushReplacementNamed('/inner');
              getVillageSuggestions("");
            }
          },
          child: SplashPage(),
        );
      },
      "/inner": (context) {
        return MultiBlocProvider(
          providers: [
            BlocProvider<PandemiaBloc>(
              builder: (context) =>
                  PandemiaBloc(userRepository: userRepository),
            ),
            BlocProvider<TabBloc>(
              builder: (context) => TabBloc(),
            ),
            BlocProvider<StatsBloc>(
              builder: (context) => StatsBloc(),
            ),
            BlocProvider<IssueBloc>(
              builder: (context) => IssueBloc(),
            ),
            BlocProvider<FeedBloc>(
              builder: (context) => FeedBloc(),
            ),
            BlocProvider<FcmBloc>(
              builder: (context) => FcmBloc(),
            ),
            BlocProvider<MapBloc>(
              builder: (context) => MapBloc(),
            ),
            BlocProvider<SettingsBloc>(
              builder: (context) => SettingsBloc(),
            ),
            BlocProvider<NotifBloc>(
              builder: (context) => NotifBloc(pandemiaBloc: pandemiaBloc),
            ),
            BlocProvider<ProfileBloc>(
              builder: (context) => ProfileBloc(),
            ),
            BlocProvider<SubReportBloc>(
              builder: (context) => SubReportBloc(),
            ),
          ],
          child: HomeScreen(
              title: "PANDEMIA",
              pandemiaBloc: pandemiaBloc),
        );
      },
      PandemiaRoutes.about: (context) => AboutPage(),
    });
  }


  Future<List<dynamic>> getVillageSuggestions(String query) async {
    final geoLoc = await appRepo.getData("latest_loc_full");
    final locPath = geoLoc["loc_path"];
    return appRepo
        .fetchApi("village_suggestions",
            "/village/v1/search?query=$query&scope=$locPath&offset=0&limit=10",
            force: false)
        .then((data) async {
      if (data != null) {
        List<dynamic> entries = data["entries"] as List;
        if (entries.length == 0) {
          // kalau kosong listing semuanya saja
          return await PublicApi.get(
                  "/village/v1/search?query=$query&offset=0&limit=10")
              .then((data2) {
            if (data2 != null) {
              List<dynamic> entries = data2["result"]["entries"] as List;
              return entries
                  .map((d) =>
                      "${d["name"]}, ${d["district_name"]}, ${d["city"]}, ${d["province"]}")
                  .toList();
            }
            return [];
          });
        } else {
          return entries
              .map((d) =>
                  "${d["name"]}, ${d["district_name"]}, ${d["city"]}, ${d["province"]}")
              .toList();
        }
      } else {
        throw PandemiaException(
            "Cannot contact API server for getting suggestions");
      }
    });
  }
}

class PandemiaTheme {
  static get theme {
    final originalTextTheme = ThemeData.light().textTheme.copyWith(
        caption: TextStyle(color: Colors.black),
        headline: TextStyle(color: Colors.red),
        title: TextStyle(color: Colors.white, fontWeight: FontWeight.bold),
        subhead: TextStyle(color: Colors.black),
        overline: TextStyle(color: Colors.black),
        subtitle: TextStyle(color: Colors.black),
        body1: TextStyle(color: Colors.black),
        body2: TextStyle(color: Colors.black),
        display1: TextStyle(color: Colors.black),
        display2: TextStyle(color: Colors.black),
        display3: TextStyle(color: Colors.black),
        button: TextStyle(color: Colors.black));
    final originalBody1 = originalTextTheme.body1;

    return ThemeData.light().copyWith(
        primaryColor: Color(0xFF7A58FF),
        accentColor: Colors.cyan[300],
        buttonColor: Color(0xFF7A58FF),
        textSelectionColor: Colors.cyan[100],
        backgroundColor: Colors.grey[900],
        toggleableActiveColor: Colors.cyan[300],
        primaryTextTheme: originalTextTheme,
        scaffoldBackgroundColor: Color(0xFFF1F6FB),
        textTheme: originalTextTheme.copyWith(
            body1: originalBody1.copyWith(
                decorationColor: Colors.transparent,
                fontSize: 20,
                fontFamily: "Roboto, Times new roman")));
  }
}
