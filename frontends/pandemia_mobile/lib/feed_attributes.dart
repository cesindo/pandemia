
import 'package:flutter/material.dart';

import 'core/feed_kind.dart';

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
