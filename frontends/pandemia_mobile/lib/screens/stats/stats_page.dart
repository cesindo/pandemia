import 'package:flutter/material.dart';

class StatsPage extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return _getBody(context);
  }

  Widget _getBody(BuildContext context) {
    return Center(
      child: ListView(
        children: <Widget>[
          Section(
            title: "Global",
          ),
          Section(
            title: "Indonesia")
        ],
      ),
    );
  }
}

class Section extends StatelessWidget {
  final String title;
  const Section({Key key, this.title}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Container(
      child: Text(this.title, style: TextStyle(fontWeight: FontWeight.bold, fontSize: 20.0),),
      padding: EdgeInsets.all(5),
      decoration:
          BoxDecoration(border: Border(bottom: BorderSide(color: Colors.grey, width: 2))),
    );
  }
}
