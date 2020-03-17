
import 'package:flutter/material.dart';
import 'package:flutter/widgets.dart';
import 'package:pandemia_mobile/models/notif_item.dart';

class NotifItemView extends StatelessWidget {
  final NotifItem item;
  // final GestureTapCallback onTap;

  NotifItemView({Key key, @required this.item}): super(key: key);

  @override
  Widget build(BuildContext context) {
    return new Container(
      alignment: Alignment.center,
      color: Colors.purple,
      child: Text(item.text),
    );
  }
}

