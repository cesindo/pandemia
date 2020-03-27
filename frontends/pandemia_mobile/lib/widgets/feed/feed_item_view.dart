import 'package:flutter/material.dart';
import 'package:flutter/widgets.dart';
import 'package:pandemia_mobile/feed_attributes.dart';
import 'package:pandemia_mobile/models/feed.dart';
import 'package:pandemia_mobile/time_helper.dart';
import 'package:timeago/timeago.dart' as timeago;
import 'package:pandemia_mobile/util/string_extension.dart';

class FeedItemView extends StatelessWidget {
  final Feed item;
  // final GestureTapCallback onTap;
  final bool editMode;
  // final Function(Feed) onUpdated;

  FeedItemView({Key key, @required this.item, this.editMode}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    var text = item.text.split(", ");
    var leftNum = RegExp(r"\+[0-9]*").firstMatch(text[0]).group(0);
    var rightNum = RegExp(r'(\d+)+')
        .allMatches(text[1])
        .map((f) => f.group(0))
        .toList()
        .first;

    var size = MediaQuery.of(context).size;
    final double itemHeight = (size.height - kToolbarHeight - 24) / 3.5;
    final double itemWidth = size.width / 1;

    return Card(
        elevation: 2.0,
        margin: new EdgeInsets.symmetric(horizontal: 16.0, vertical: 5.0),
        child: Container(
          // decoration: BoxDecoration(color: Color.fromRGBO(64, 75, 96, .9)),
          child: new ListTile(
            isThreeLine: true,
            contentPadding:
                EdgeInsets.symmetric(horizontal: 0.0, vertical: 10.0),
            title: Padding(
              padding: const EdgeInsets.symmetric(horizontal: 20.0),
              child: Row(
                children: <Widget>[
                  Icon(
                    IconsByKind[item.kind],
                    color: ColorsByKind[item.kind],
                    size: 32,
                  ),
                  SizedBox(width: 16),
                  Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: <Widget>[
                      Text(
                        item.loc.capitalize(),
                        style: TextStyle(
                            color: Colors.grey[700],
                            fontWeight: FontWeight.w500,
                            fontSize: 19),
                      ),
                      Divider(height: 5),
                      Text(
                        timeago.format(TimeHelper.parseAsUtc(item.ts)),
                        style: TextStyle(fontSize: 13, color: Colors.grey),
                      ),
                    ],
                  ),
                ],
              ),
            ),
            subtitle: Column(
              children: <Widget>[
                Divider(color: Colors.grey[300], height: 22),
                GridView.count(
                  shrinkWrap: true,
                  primary: false,
                  crossAxisCount: 2,
                  childAspectRatio: (itemWidth / itemHeight),
                  children: <Widget>[
                    Padding(
                      padding: const EdgeInsets.symmetric(horizontal: 12.0),
                      child: Column(children: [
                        Text("+${leftNum.toNumberFormat()}",
                            style: TextStyle(
                                fontSize: 26, color: Color(0xFF987FFF))),
                        SizedBox(height: 10),
                        Text(
                          text[0].replaceAll(leftNum + " ", "").capitalize(),
                          textAlign: TextAlign.center,
                          style: TextStyle(
                              fontSize: 13, fontWeight: FontWeight.w300, ),
                        ),
                      ]),
                    ),
                    Padding(
                      padding: const EdgeInsets.symmetric(horizontal: 12.0),
                      child: Column(children: [
                        Text(rightNum.toNumberFormat(),
                            style: TextStyle(
                                fontSize: 26, color: Color(0xFF987FFF))),
                        SizedBox(height: 10),
                        Text(
                          text[1].replaceAll(" " + rightNum, "").capitalize(),
                          textAlign: TextAlign.center,
                          style: TextStyle(
                              fontSize: 13, fontWeight: FontWeight.w300),
                        ),
                      ]),
                    ),
                  ],
                ),
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
