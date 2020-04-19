import 'package:equatable/equatable.dart';
import 'package:google_maps_flutter/google_maps_flutter.dart';
import 'package:meta/meta.dart';
import 'package:pandemia_mobile/models/user.dart';

@immutable
abstract class ProfileEvent extends Equatable {
  ProfileEvent([List props = const []]) : super(props);
}

class LoadProfile extends ProfileEvent {
  final bool force;
  LoadProfile({this.force = false});

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

/// Event to Register as Satgas
class RegisterAsSatgas extends ProfileEvent {
  final User user;
  // final LatLng location;
  final String areaCode;
  final bool isMedic;
  RegisterAsSatgas(this.user, this.areaCode, this.isMedic);
  @override
  String toString() => "RegisterAsSatgas";
}
