import 'package:flutter/material.dart';
import 'package:flutter/widgets.dart';
import 'package:pandemia_mobile/models/task.dart';

class TaskItemView extends StatelessWidget {
  final Task item;
  final GestureTapCallback onTap;

  TaskItemView({Key key, @required this.item, @required this.onTap})
      : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Card(
        elevation: 8.0,
        margin: new EdgeInsets.symmetric(horizontal: 10.0, vertical: 10.0),
        child: Container(
          decoration: BoxDecoration(color: Color.fromRGBO(64, 75, 96, .9)),
          child: new ListTile(
            contentPadding:
                EdgeInsets.symmetric(horizontal: 20.0, vertical: 10.0),
            leading: Container(
              padding: EdgeInsets.only(right: 12.0),
              decoration: new BoxDecoration(
                  border: new Border(
                      right:
                          new BorderSide(width: 1.0, color: Colors.white24))),
              child: Icon(Icons.autorenew, color: Colors.white),
            ),
            title: Text(
              item.text,
              style:
                  TextStyle(color: Colors.white, fontWeight: FontWeight.bold),
            ),
            subtitle: Column(
              children: <Widget>[
                Row(
                  children: <Widget>[
                    Icon(Icons.person, color: Colors.yellowAccent),
                    Text(item.assigneeName,
                        style: TextStyle(color: Colors.white))
                  ],
                ),
                Container(
                  alignment: Alignment.topLeft,
                  child: Text(item.expireTime),
                )
              ],
            ),
          ),
        ));
  }
}

