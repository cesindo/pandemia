
import 'package:equatable/equatable.dart';
import 'package:meta/meta.dart';

@immutable
abstract class FcmState extends Equatable {
  FcmState([List props = const []]) : super(props);
}

/// Loading state
class FcmLoading extends FcmState {
  /// Set true to block screen with blocking loading modal box.
  final bool block;
  FcmLoading({this.block = false});
  @override
  String toString() => "FcmLoading";
}

/// State when error/failure occurred
class FcmFailure extends FcmState {
  final String error;
  FcmFailure({this.error}) : super([error]);
  @override
  String toString() => "FcmFailure";
}

class FcmCreated extends FcmState {
  FcmCreated();
  @override
  String toString() => "FcmCreated";
}

/// State when fcm already deleted
class FcmDeleted extends FcmState {
  FcmDeleted();
  @override
  String toString() => "FcmDeleted";
}
