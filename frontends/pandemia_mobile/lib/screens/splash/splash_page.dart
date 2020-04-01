import 'dart:io';

import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:pandemia_mobile/blocs/pandemia/pandemia_bloc.dart';
import 'package:pandemia_mobile/blocs/pandemia/pandemia_event.dart';
import 'package:pandemia_mobile/blocs/pandemia/pandemia_state.dart';
import 'package:pandemia_mobile/notification_util.dart';

class SplashPage extends StatelessWidget {

  @override
  Widget build(BuildContext context) {
    PandemiaBloc pandemiaBloc = BlocProvider.of<PandemiaBloc>(context);

    pandemiaBloc.dispatch(StartupEvent());

    if (Platform.isIOS) {
      NotificationUtil().getIOSPermission();
    }

    return BlocBuilder<PandemiaBloc, PandemiaState>(
        builder: (BuildContext context, PandemiaState state) {
      String statusText = "memuat...";

      if (state is ValidateToken) {
        statusText = "validasi...";
      } else if (state is AuthorizeToken) {
        statusText = "otorisasi...";
      }

      return Scaffold(
        body: Center(
          child: ListView(
            children: <Widget>[
              Image.asset("assets/img/pandemia-logo-256.png"),
              Center(child: Text(statusText))
            ],
          ),
        ),
      );
    });
  }
}
