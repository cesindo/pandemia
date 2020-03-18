import 'package:flutter/material.dart';

class SplashPage extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Center(
        child: ListView(children: <Widget>[
          Image.asset("assets/img/pandemia-logo-256.png"),
          Center(child: Text("loading..."))
        ],),
      ),
    );
  }
}

