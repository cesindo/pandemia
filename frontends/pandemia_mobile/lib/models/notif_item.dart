import 'package:equatable/equatable.dart';
import 'package:meta/meta.dart';

@immutable
class NotifItem extends Equatable {
  final int id;
  final String kind;
  final String text;
  final int initiatorId;
  final int projectId;
  final List<String> keywords;
  final String ts;

  NotifItem(this.id, this.kind, this.text, this.initiatorId, this.projectId, this.keywords, this.ts): 
    super([id, kind, text, initiatorId, projectId, keywords, ts]);
  
}

