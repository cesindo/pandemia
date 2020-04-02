import 'package:flutter/material.dart';
import 'package:flutter/widgets.dart';
import 'package:pandemia_mobile/core/feed_kind.dart';
import 'package:pandemia_mobile/feed_attributes.dart';
import 'package:pandemia_mobile/models/feed.dart';
import 'package:pandemia_mobile/time_helper.dart';
import 'package:timeago/timeago.dart' as timeago;
import 'package:pandemia_mobile/util/string_extension.dart';

class FeedItemView extends StatelessWidget {
  final Feed item;
  final bool editMode;

  FeedItemView({Key key, @required this.item, this.editMode}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    if (item.kind == FeedKind.newCases ||
        item.kind == FeedKind.newDeaths ||
        item.kind == FeedKind.newRecovered) {
      return _buildNewUpdate(context);
    } else if (item.kind == FeedKind.info) {
      return _buildInfo(context);
    } else {
      return _buildOther(context);
    }
  }

  Widget _buildInfo(context) {
    String title = "INFO";

    if (item.loc.isNotEmpty) {
      title = title + " area " + item.loc;
    }
    return Card(
        elevation: 2.0,
        margin: new EdgeInsets.symmetric(horizontal: 16.0, vertical: 5.0),
        child: Container(
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
                      color: Colors.blue,
                      size: 32,
                    ),
                    SizedBox(
                      width: 16,
                    ),
                    Text(
                      title,
                      style: TextStyle(
                          color: Colors.grey[700],
                          fontWeight: FontWeight.w500,
                          fontSize: 19),
                    )
                  ],
                ),
              ),
              subtitle: Column(children: <Widget>[
                Divider(color: Colors.grey[300], height: 22),
                Text(item.text)
              ])),
        ));
  }

  Widget _buildOther(BuildContext context) {
    return Container();
  }

  Widget _buildNewUpdate(BuildContext context) {
    var text = item.text.split(", ");
    var leftNum = RegExp(r"\+[0-9]*").firstMatch(text[0]).group(0);
    var rightNum = RegExp(r'(\d+)+')
        .allMatches(text[1])
        .map((f) => f.group(0))
        .toList()
        .first;

    var rightText = text[1].replaceAll(rightNum, "").capitalize();

    var size = MediaQuery.of(context).size;

    return Card(
      elevation: 2.0,
      margin: new EdgeInsets.symmetric(horizontal: 16.0, vertical: 5.0),
      child: Container(
        child: new ListTile(
          isThreeLine: true,
          contentPadding:
              EdgeInsets.symmetric(horizontal: 20.0, vertical: 10.0),
          title: Row(
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
          subtitle: Column(
            children: <Widget>[
              Divider(color: Colors.grey[300], height: 22),
              Container(
                child: GridViewCustom(
                  text1: "+${leftNum.toNumberFormat()}",
                  text2: rightNum.toNumberFormat(),
                  desc1: text[0].replaceAll(leftNum + " ", "").capitalize(),
                  desc2: text[1].replaceAll(rightNum, "").capitalize(),
                  colorText: ColorsByKind[item.kind],
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}

class GridViewCustom extends StatelessWidget {
  final String text1;
  final String text2;
  final String desc1;
  final String desc2;
  final Color colorText;

  const GridViewCustom(
      {Key key, this.text1, this.text2, this.desc1, this.desc2, this.colorText})
      : super(key: key);

  @override
  Widget build(BuildContext context) {
    var size = MediaQuery.of(context).size;
    final double itemHeight = (size.height - kToolbarHeight - 180) / 3.4;
    final double itemWidth = size.width / 1.3;
    return Container(
      child: GridView.builder(
        addAutomaticKeepAlives: true,
        shrinkWrap: true,
        gridDelegate: SliverGridDelegateWithFixedCrossAxisCount(
            crossAxisCount: 2,
            childAspectRatio: itemWidth / itemHeight,
            crossAxisSpacing: 0.0),
        physics: NeverScrollableScrollPhysics(),
        itemCount: 2,
        itemBuilder: (context, index) {
          String value;
          String descValue;
          if (index == 0) {
            value = text1;
            descValue = desc1;
          } else if (index == 1) {
            value = text2;

            descValue = desc2.replaceAll(
                "Total  yang telah meninggal", "Total telah meninggal");
          }
          return _itemView(
            context: context,
            text: value,
            desc: descValue,
          );
        },
      ),
    );
  }

  Widget _itemView({context, String text, String desc}) {
    return Container(
      child: Column(
        children: <Widget>[
          Expanded(
            flex: 3,
            child: Container(
              alignment: Alignment.center,
              child: Text(
                text,
                textAlign: TextAlign.center,
                style: TextStyle(
                  fontSize: 28,
                  fontWeight: FontWeight.w700,
                  color: colorText,
                ),
              ),
            ),
          ),
          Expanded(
            child: Container(
              alignment: Alignment.topCenter,
              child: Text(
                desc,
                textAlign: TextAlign.center,
                style: TextStyle(fontSize: 15, fontWeight: FontWeight.w300),
              ),
            ),
          ),
        ],
      ),
    );
  }
}