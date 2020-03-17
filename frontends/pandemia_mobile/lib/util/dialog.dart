import 'package:flutter/material.dart';
import 'package:flutter/widgets.dart';

void confirmDialog(BuildContext context, String actionText, {Function onOk, Function onCancel}) {
  showDialog(
      context: context,
      builder: (context) {
        return AlertDialog(
          title: new Text("Confirmation"),
          content: new Text("Are you sure to ${actionText}?"),
          actions: <Widget>[
            FlatButton(
              child: Text("Cancel"),
              onPressed: (){
                if (onCancel != null){
                  onCancel();
                }
                Navigator.pop(context);
              },
            ),
            FlatButton(
              child: Text("Yes"),
              onPressed: (){
                onOk();
                Navigator.pop(context);
              },
            )
          ],
        );
      });
}

