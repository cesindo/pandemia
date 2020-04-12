
import 'package:equatable/equatable.dart';
import 'package:meta/meta.dart';
import 'package:pandemia_mobile/models/user.dart';

@immutable
abstract class ProfileState extends Equatable {
  ProfileState([List props = const []]) : super(props);
}

/// Loading state
class ProfileLoading extends ProfileState {
  /// Set true to block screen with blocking loading modal box.
  final bool block;
  ProfileLoading({this.block = false});
  @override
  String toString() => "ProfileLoading";
}

class ProfileUpdateLoading extends ProfileState {
  @override
  String toString() => "ProfileUpdateLoading";
}

class ProfileListLoaded extends ProfileState {
  final List<User> items;
  ProfileListLoaded(this.items);
  @override
  String toString() => "ProfileListLoaded";
}

/// State when error/failure occurred
class ProfileFailure extends ProfileState {
  final String error;
  ProfileFailure({this.error}) : super([error]);
  @override
  String toString() => "ProfileFailure";
}

class ProfileLoaded extends ProfileState {
  final User user;
  ProfileLoaded(this.user);
  @override
  String toString() => "ProfileLoaded";
}

/// State when Profile already deleted
class ProfileDeleted extends ProfileState {
  final User profile;
  ProfileDeleted(this.profile);
  @override
  String toString() => "ProfileDeleted";
}

class ProfileUpdated extends ProfileState {
  final User profile;
  ProfileUpdated(this.profile);
  @override
  String toString() => "ProfileUpdated";
}