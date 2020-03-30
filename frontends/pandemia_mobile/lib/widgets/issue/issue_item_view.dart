import 'package:cached_network_image/cached_network_image.dart';
import 'package:flutter/material.dart';
import 'package:flutter/widgets.dart';
import 'package:pandemia_mobile/blocs/issue/issue_bloc.dart';
import 'package:pandemia_mobile/blocs/issue/issue_event.dart';
import 'package:pandemia_mobile/models/issue.dart';
import 'package:pandemia_mobile/screens/issue/issue_detail_page.dart';
// import 'package:pandemia_mobile/util/dialog.dart';

class IssueItemView extends StatelessWidget {
  final Issue item;
  // final GestureTapCallback onTap;
  final IssueBloc issueBloc;
  final bool editMode;
  // final Function(Issue) onUpdated;

  IssueItemView({Key key, @required this.item, this.editMode, this.issueBloc})
      : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Container(
      decoration: BoxDecoration(
          border: Border(bottom: BorderSide(color: Colors.grey[300])),
          color: Colors.white),
      child: new ListTile(
        contentPadding: EdgeInsets.symmetric(horizontal: 16.0, vertical: 10.0),
        leading: CachedNetworkImage(
            imageUrl: item.primaryImage,
            width: 80,
            height: 80,
            fit: BoxFit.fill),
        title: Text(
          item.name,
          style: TextStyle(
              color: Colors.grey[800],
              fontWeight: FontWeight.w300,
              fontSize: 19),
        ),
        subtitle: Container(
          margin: EdgeInsets.only(top: 8),
          child: Text(
            "${item.classification.toUpperCase()}",
            style: TextStyle(fontWeight: FontWeight.w400, fontSize: 18),
          ),
        ),
        onTap: () {
          issueBloc.dispatch(LoadDetailIssue(item.id));
          Navigator.of(context)
              .push(MaterialPageRoute(
                  builder: (context) => IssueDetailPage(issueBloc: issueBloc)))
              .then((result) {
            issueBloc.dispatch(LoadIssue());
          });
        },
      ),
    );
  }
}
