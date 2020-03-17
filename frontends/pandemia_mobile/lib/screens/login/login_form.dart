import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:pandemia_mobile/core/core.dart';
import 'package:pandemia_mobile/screens/login/login.dart';

class LoginForm extends StatefulWidget {
  @override
  State<LoginForm> createState() => _LoginFormState();
}

class _LoginFormState extends State<LoginForm> {
  final _emailController = TextEditingController(text: "dummy1@nowhere.net");
  final _passwordController = TextEditingController(text: "123123");

  @override
  Widget build(BuildContext context) {
    final loginBloc = BlocProvider.of<LoginBloc>(context);

    _onLoginButtonPressed() {
      loginBloc.dispatch(LoginButtonPressed(
        email: _emailController.text,
        password: _passwordController.text,
      ));
    }

    return BlocListener<LoginBloc, LoginState>(
      listener: (context, state) {
        if (state is LoginFailure) {
          Scaffold.of(context).showSnackBar(
            SnackBar(
              content: Text('${state.error}'),
              backgroundColor: Colors.red,
            ),
          );
        }
      },
      child: BlocBuilder<LoginBloc, LoginState>(
        builder: (context, state) {
          return Scaffold(
            body: Center(
              child: ListView(
                children: [
                  Hero(
                      tag: PandemiaKeys.logo,
                      child: Image.asset("assets/img/ansvia-logo.png")),
                  Center(
                      child: Text(
                    "PANDEMIA",
                    style: TextStyle(
                        color: Colors.white,
                        fontWeight: FontWeight.bold,
                        fontSize: 30),
                  )),
                  Padding(
                    padding: const EdgeInsets.all(36.0),
                    child: Form(
                      child: Column(
                        children: [
                          TextFormField(
                            decoration: InputDecoration(labelText: 'email'),
                            controller: _emailController,
                          ),
                          TextFormField(
                            decoration: InputDecoration(labelText: 'password'),
                            controller: _passwordController,
                            obscureText: true,
                          ),
                          RaisedButton(
                            onPressed: state is! LoginLoading
                                ? _onLoginButtonPressed
                                : null,
                            child: Text('Login'),
                          ),
                          Container(
                            child: state is LoginLoading
                                ? CircularProgressIndicator()
                                : null,
                          ),
                        ],
                      ),
                    ),
                  )
                ],
              ),
            ),
          );
        },
      ),
    );
  }
}

