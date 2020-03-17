import 'package:equatable/equatable.dart';
import 'package:meta/meta.dart';

@immutable
class User extends Equatable {
  final int id;
  final String fullName;
  final String email;

  User(this.id, this.fullName, this.email)
      : super([id, fullName, email]);

  Map<String,dynamic> toMap(){
    Map<String,dynamic> data;
    data['id'] = this.id;
    data['full_name'] = this.fullName;
    data['email'] = this.email;
    return data;
  }

  static User fromMap(Map<String, dynamic> data){
    return User(data['id'], data['fullName'], data['email']);
  }
}

