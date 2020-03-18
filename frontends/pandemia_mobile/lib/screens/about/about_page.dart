import 'package:flutter/material.dart';

class AboutPage extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
        appBar: AppBar(title: Text("About")), body: _getBody(context));
  }

  Widget _getBody(BuildContext context) {
    return Center(
      child: ListView(
        children: <Widget>[
          Padding(
              padding: const EdgeInsets.all(10.0),
              child: Container(
                  child: Table(columnWidths: {1:FixedColumnWidth(5)},children: [
                    TableRow(children: [
                      Text("Icon by : ", textAlign: TextAlign.end,), Container(width: 1,), Text("photo3idea-studio")
                    ])
                  ],)
                  )),
        ],
      ),
    );
  }
}
