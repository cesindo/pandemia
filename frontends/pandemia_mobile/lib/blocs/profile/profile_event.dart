
import 'package:equatable/equatable.dart';
import 'package:meta/meta.dart';
import 'package:pandemia_mobile/models/user.dart';

@immutable
abstract class ProfileEvent extends Equatable {
  ProfileEvent([List props = const []]) : super(props);
}

class LoadProfile extends ProfileEvent {
  final bool force;
  LoadProfile({this.force=false});

  @override
  String toString() => "LoadProfile";
}

class CreateProfile extends ProfileEvent {
  final int id;
  final String text;
  CreateProfile(this.id, this.text);
  @override
  String toString() => "CreateProfile";
}

/// Event to delete Profile
class DeleteProfile extends ProfileEvent {
  final User profile;
  DeleteProfile(this.profile);
  @override
  String toString() => "DeleteProfile";
}
  
