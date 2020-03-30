import 'dart:async';

import 'package:cached_network_image/cached_network_image.dart';
import 'package:flutter/material.dart';
import 'package:flutter/widgets.dart';
import 'package:pandemia_mobile/blocs/issue/issue_bloc.dart';
import 'package:pandemia_mobile/blocs/issue/issue_state.dart';
import 'package:pandemia_mobile/core/core.dart';
import 'package:pandemia_mobile/models/issue_detail.dart';
import 'package:pandemia_mobile/time_helper.dart';
import 'package:pandemia_mobile/widgets/image_view.dart';
import 'package:pandemia_mobile/widgets/widgets.dart';
import 'package:url_launcher/url_launcher.dart';

class IssueDetailPage extends StatefulWidget {
  final IssueBloc issueBloc;

  IssueDetailPage({Key key, this.issueBloc}) : super(key: key);

  _IssueDetailPageState createState() => _IssueDetailPageState(this.issueBloc);
}

class _IssueDetailPageState extends State<IssueDetailPage> {
  final IssueBloc issueBloc;
  StreamSubscription _subs;
  IssueDetail item;

  _IssueDetailPageState(this.issueBloc);

  @override
  void initState() {
    _subs = issueBloc.state.listen((state) {
      if (state is IssueDetailLoaded) {
        setState(() => item = state.item);
      }
    });
    super.initState();
  }

  @override
  void dispose() {
    _subs.cancel();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text("Issue Detail"),
      ),
      body: _getBody(context),
    );
  }

  Widget _getBody(BuildContext context) {
    if (item == null) {
      return LoadingIndicator(key: PandemiaKeys.loading);
    }

    return ListView(
      children: <Widget>[
        Container(
          color: Colors.white,
          padding: EdgeInsets.symmetric(horizontal: 16, vertical: 18),
          child: Row(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: <Widget>[
              InkWell(
                onTap: () => Navigator.push(
                    context, ViewImage(imageUrl: item.primaryImage)),
                child: CachedNetworkImage(
                    imageUrl: item.primaryImage,
                    width: 160,
                    height: 160,
                    fit: BoxFit.fill),
              ),
              SizedBox(width: 14),
              Expanded(
                child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: <Widget>[
                      Text("${item.classification}".toUpperCase(),
                          style: TextStyle(
                              fontSize: 36, fontWeight: FontWeight.w500)),
                      Text(
                        "${item.name}",
                        style: TextStyle(
                            fontWeight: FontWeight.w500,
                            fontSize: 18,
                            color: Colors.grey[800]),
                      ),
                      Container(
                        margin: EdgeInsets.only(top: 8),
                        child: Text(
                          TimeHelper.formatSimple(
                              DateTime.parse(item.registerTime)),
                          style: TextStyle(
                              fontSize: 16, fontWeight: FontWeight.w400),
                        ),
                      )
                    ]),
              )
            ],
          ),
        ),
        Container(
          color: Colors.white,
          padding: EdgeInsets.only(bottom: 10),
          child: Card(
              color: Colors.grey[100],
              margin: EdgeInsets.symmetric(horizontal: 16, vertical: 8),
              child: Container(
                  padding: EdgeInsets.symmetric(horizontal: 16, vertical: 12),
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: <Widget>[
                      Text("Penjelasan:",
                          style: TextStyle(fontWeight: FontWeight.w500)),
                      SizedBox(height: 5),
                      Text("${item.desc}",
                          style: TextStyle(
                              fontSize: 17, fontWeight: FontWeight.w300))
                    ],
                  ))),
        ),
        Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: <Widget>[
            Container(
              color: Colors.grey[200],
              width: double.infinity,
              padding: EdgeInsets.symmetric(horizontal: 16, vertical: 10),
              child: Text("Referensi (${item.refs.length})"),
            ),
            ListView.builder(
                primary: false,
                shrinkWrap: true,
                itemCount: item.refs.length,
                itemBuilder: (context, index) {
                  var ref = item.refs[index];
                  return ListTile(
                    contentPadding:
                        EdgeInsets.symmetric(horizontal: 8, vertical: 4),
                    title: Text("${ref.caption}"),
                    leading: CachedNetworkImage(
                        imageUrl: ref.thumbnailUrl,
                        errorWidget: (context, text, object) =>
                            Image.asset("assets/img/no_image.png"),
                        width: 60,
                        height: 60,
                        fit: BoxFit.fill),
                    onTap: () => _launchURL(context, ref.urlLink),
                  );
                })
          ],
        )
      ],
    );
  }

  _launchURL(BuildContext context, String url) async {
    if (await canLaunch(url)) {
      await launch(url);
    } else {
      // throw 'Could not launch $url';
      Scaffold.of(context).showSnackBar(SnackBar(
        content:
            Text("Cannot launch $url", style: TextStyle(color: Colors.white)),
        backgroundColor: Colors.red,
      ));
    }
  }
}
