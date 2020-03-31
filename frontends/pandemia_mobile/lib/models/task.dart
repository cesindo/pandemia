import 'package:equatable/equatable.dart';
import 'package:meta/meta.dart';

@immutable
class Task extends Equatable {
  final int id;
  final String assigneeName;
  final String text;
  final String expireTime;

  Task(this.id, this.assigneeName, this.text, this.expireTime):
   super();

  @override
  // TODO: implement props
  List<Object> get props => [id, assigneeName, text, expireTime];
  
}


