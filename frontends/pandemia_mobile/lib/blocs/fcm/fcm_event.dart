
import 'package:equatable/equatable.dart';
import 'package:meta/meta.dart';

@immutable
abstract class FcmEvent extends Equatable {
  FcmEvent([List props = const []]) : super(props);
}

// class LoadFcm extends FcmEvent {
//   LoadFcm();
//   @override
//   String toString() => "LoadFcm";
// }

class CreateFcm extends FcmEvent {
  CreateFcm();
  @override
  String toString() => "CreateFcm";
}

/// Event to delete fcm
class DeleteFcm extends FcmEvent {
  DeleteFcm();
  @override
  String toString() => "DeleteFcm";
}
  
