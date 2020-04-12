import 'dart:async';

import 'package:flutter/material.dart';
import 'package:pandemia_mobile/blocs/profile/profile.dart';
import 'package:pandemia_mobile/user_repository/user_repository.dart';

class ProfileEditPage extends StatefulWidget {
  final ProfileBloc profileBloc;

  ProfileEditPage({this.profileBloc, Key key}) : super(key: key);

  @override
  _ProfileEditPageState createState() => _ProfileEditPageState(profileBloc);
}

class _ProfileEditPageState extends State<ProfileEditPage> {
  final ProfileBloc profileBloc;
  final UserRepository userRepository = UserRepository();
  StreamSubscription subs;
  final _formKey = GlobalKey<FormState>();
  final _fullNameCtl = TextEditingController();
  final _emailCtl = TextEditingController();
  final _scaffoldKey = GlobalKey<ScaffoldState>();

  _ProfileEditPageState(this.profileBloc) {
    _fullNameCtl.text = userRepository.currentUser.fullName;
    _emailCtl.text = userRepository.currentUser.email;

    subs = profileBloc.state.listen((ProfileState state) {
      if (state is ProfileUpdated) {
        Navigator.pop(_scaffoldKey.currentContext);
      } else if (state is ProfileFailure) {
        Scaffold.of(_scaffoldKey.currentContext).showSnackBar(
            SnackBar(content: Text(state.error), backgroundColor: Colors.red));
      }
    });
  }

  @override
  void dispose() {
    super.dispose();
    subs.cancel();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      key: _scaffoldKey,
      appBar: AppBar(title: Text("Edit Profil")),
      body: Builder(builder: (context) {
        return Center(
          child: ListView(children: <Widget>[
            Padding(
              padding: EdgeInsets.all(15.0),
              child: Form(
                key: _formKey,
                child: Column(
                  children: <Widget>[
                    TextFormField(
                      decoration: InputDecoration(labelText: 'Full Name'),
                      controller: _fullNameCtl,
                    ),
                    TextFormField(
                      decoration: InputDecoration(labelText: 'Email'),
                      controller: _emailCtl,
                    ),
                    Container(
                      margin: EdgeInsets.only(top: 20.0),
                      child: MaterialButton(
                        child: Text("Simpan", style: TextStyle(color: Colors.white)),
                        minWidth: double.infinity,
                        height: 40.0,
                        color: Theme.of(context).buttonColor,
                        onPressed: () {},
                      ),
                    ),
                  ],
                ),
              ),
            )
          ]),
        );
      }),
    );
  }
}
