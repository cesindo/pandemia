import 'package:flutter/material.dart';
import 'package:flutter/widgets.dart';
import 'package:pandemia_mobile/core/feed_kind.dart';
import 'package:pandemia_mobile/models/feed.dart';
import 'package:pandemia_mobile/time_helper.dart';
import 'package:pandemia_mobile/util/dialog.dart';
import 'package:timeago/timeago.dart' as timeago;

const Map<int, IconData> IconsByKind = {
  FeedKind.systemFeed: Icons.info,
  FeedKind.info: Icons.info,
  FeedKind.announcement: Icons.warning,
  FeedKind.newCases: Icons.group,
  FeedKind.newDeaths: Icons.hotel,
  FeedKind.newRecovered: Icons.assignment_turned_in
};

const Map<int, Color> ColorsByKind = {
  FeedKind.systemFeed: Colors.grey,
  FeedKind.info: Colors.grey,
  FeedKind.announcement: Colors.orange,
  FeedKind.newCases: Colors.orange,
  FeedKind.newDeaths: Colors.red,
  FeedKind.newRecovered: Colors.green
};

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
            title: Text(
              item.loc,
              style: TextStyle(
                  color: Colors.grey[800],
                  fontWeight: FontWeight.bold,
                  fontSize: 20),
            ),
            subtitle: Column(
              children: <Widget>[
                Row(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: <Widget>[
                    Padding(
                      padding: EdgeInsets.only(right: 5),
                      child: Icon(IconsByKind[item.kind],
                          color: ColorsByKind[item.kind]),
                    ),
                    Container(
                      width: MediaQuery.of(context).size.width - 100,
                      child: Text(
                        item.text,
                        style: TextStyle(color: Colors.grey[800]),
                        overflow: TextOverflow.ellipsis,
                        maxLines: 3,
                        textWidthBasis: TextWidthBasis.longestLine,
                      ),
                    )
                  ],
                ),
                Container(
                  alignment: Alignment.topLeft,
                  child: Text(
                    timeago.format(TimeHelper.parseAsUtc(item.ts)),
                    style: TextStyle(fontSize: 15, color: Colors.grey),
                  ),
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
