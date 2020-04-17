import 'package:flutter/material.dart';
import 'package:flutter/widgets.dart';
import 'package:pandemia_mobile/api/pandemia_api.dart';
import 'package:pandemia_mobile/user_repository/user_repository.dart';
import 'package:pandemia_mobile/widgets/loading_indicator.dart';

class WebTokenPage extends StatefulWidget {
  WebTokenPage({Key key}) : super(key: key);

  _WebTokenPageState createState() => _WebTokenPageState();
}

class _WebTokenPageState extends State<WebTokenPage> {
  String token = "";
  bool _loading = true;
  String _error = "";

  _WebTokenPageState();

  @override
  void initState() {
    super.initState();
    setState(() {
      this._loading = true;
    });
    generateToken();
  }

  void generateToken() {
    setState(() {
      this._loading = true;
    });
    PublicApi.post("/auth/v1/satgas/get_web_token",
        {'id': UserRepository().currentUser.id}).then((data) {
      if (data != null) {
        setState(() {
          this.token = data['result'] as String;
          this._loading = false;
        });
      } else {
        setState(() {
          this._loading = false;
          this._error = "Gagal, mohon periksa kembali koneksi internet Anda";
        });
      }
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text("Web Token"),
      ),
      body: _getBody(context),
    );
  }

  Widget _getBody(BuildContext context) {
    return Center(
        child: Column(
      children: <Widget>[
        Padding(
            padding: const EdgeInsets.all(10.0),
            child: Center(
                child: Column(
              mainAxisAlignment: MainAxisAlignment.center,
              crossAxisAlignment: CrossAxisAlignment.center,
              children: <Widget>[
                Column(
                  mainAxisAlignment: MainAxisAlignment.center,
                  crossAxisAlignment: CrossAxisAlignment.center,
                  children: <Widget>[
                    Text("TOKEN:"),
                    _loading
                        ? Column(
                            mainAxisAlignment: MainAxisAlignment.center,
                            crossAxisAlignment: CrossAxisAlignment.center,
                            children: <Widget>[
                              Padding(
                                  padding: EdgeInsets.all(20),
                                  child: LoadingIndicator()),
                              Text(
                                "Menggenerasikan token,\nmohon tunggu...",
                                textAlign: TextAlign.center,
                              )
                            ],
                          )
                        : Padding(
                            padding: EdgeInsets.all(20),
                            child: Text(
                              this.token != null ? this.token : "-",
                              style: TextStyle(
                                  fontSize: 30, fontWeight: FontWeight.bold),
                            )),
                    this.token != null && this.token.length > 0
                        ? (!_loading ? Text(
                            "Masukkan kode token di atas ke Web untuk login",
                            textAlign: TextAlign.center,
                          ) : Container())
                        : Container(),
                  ],
                )
              ],
            ))),
        RaisedButton(
            onPressed: this._loading
                ? null
                : () {
                    generateToken();
                  },
            child: Text(this._loading
                ? "Generating..."
                : "TAP INI UNTUK MENDAPATKAN TOKEN BARU"))
      ],
    ));
  }
}
