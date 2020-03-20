import 'package:flutter/material.dart';
import 'package:flutter/widgets.dart';
import 'package:pandemia_mobile/models/feed.dart';
import 'package:pandemia_mobile/time_helper.dart';
import 'package:pandemia_mobile/util/dialog.dart';
import 'package:timeago/timeago.dart' as timeago;

class FeedItemView extends StatelessWidget {
  final Feed item;
  // final GestureTapCallback onTap;
  final bool editMode;
  // final Function(Feed) onUpdated;

  FeedItemView({Key key, @required this.item, this.editMode}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Card(
        elevation: 8.0,
        margin: new EdgeInsets.symmetric(horizontal: 10.0, vertical: 10.0),
        child: Container(
          // decoration: BoxDecoration(color: Color.fromRGBO(64, 75, 96, .9)),
          child: new ListTile(
            contentPadding:
                EdgeInsets.symmetric(horizontal: 20.0, vertical: 10.0),
            leading: editMode == true
                ? Container(
                    padding: EdgeInsets.only(right: 12.0),
                    // decoration: new BoxDecoration(
                    //     border: new Border(
                    //         right:
                    //             new BorderSide(width: 1.0, color: Colors.white24))),
                    child: IconButton(
                      icon: Icon(
                        Icons.cancel,
                        color: Colors.red,
                      ),
                      onPressed: () {
                        confirmDialog(context, "Delete Feed ${item.id}",
                            onOk: () {
                          // @TODO(*): code here for delete operation
                          Navigator.pop(context);
                        });
                      },
                    ),
                  )
                : null,
            title: Text(
              item.loc,
              style: TextStyle(
                  color: Colors.grey[800], fontWeight: FontWeight.bold),
            ),
            subtitle: Column(
              children: <Widget>[
                Row(
                  children: <Widget>[
                    Icon(Icons.person, color: Colors.grey),
                    Text(item.text,
                        style: TextStyle(color: Colors.grey[800]))
                  ],
                ),
                Container(
                  alignment: Alignment.topLeft,
                  child: Text(timeago.format(TimeHelper.parseAsUtc(item.ts))),
                )
              ],
            ),
            // onTap: () {
            //   Navigator.of(context)
            //       .push(MaterialPageRoute(
            //           builder: (context) => FeedDetailPage(item: item)))
            //       .then((result) {
            //     // @TODO(*): code here after view item
            //     // this.onUpdated(result);
            //   });
            // },
          ),
        ));
  }
}
