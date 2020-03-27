import 'package:flutter/material.dart';
import 'package:package_info/package_info.dart';
import 'package:pandemia_mobile/main.dart';

class AboutPage extends StatefulWidget {
  final PackageInfo packageInfo;

  AboutPage({Key key, this.packageInfo}) : super(key: key);

  @override
  _AboutPageState createState() => _AboutPageState();
}

class _AboutPageState extends State<AboutPage> {
  String version;

  @override
  void initState() {
    super.initState();
    getPackageInfo();
  }

  getPackageInfo() async {
    PackageInfo pInfo = await PackageInfo.fromPlatform();
    setState(() => version = pInfo.version);
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text("About"),
      ),
      body: Container(
        height: MediaQuery.of(context).size.height,
        color: Colors.white,
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.center,
          mainAxisAlignment: MainAxisAlignment.spaceBetween,
          children: <Widget>[
            _getBody(context),
            Padding(
              padding: EdgeInsets.all(MediaQuery.of(context).padding.top),
              child: Text(
                "Version : ${PandemiaApp.appVersion}",
                style: TextStyle(
                  fontWeight: FontWeight.normal,
                  color: Colors.black,
                  fontSize: 16,
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }

  Widget _getBody(BuildContext context) {
    return Container(
      padding: EdgeInsets.only(
        left: MediaQuery.of(context).size.width / 7,
        right: MediaQuery.of(context).size.width / 5,
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.center,
        mainAxisAlignment: MainAxisAlignment.center,
        children: <Widget>[
          _sizedBox(context),
          Container(
            alignment: Alignment.center,
            child: Image.asset(
              "assets/img/pandemia-logo.png",
              fit: BoxFit.cover,
              width: MediaQuery.of(context).size.width / 3,
            ),
          ),
          Padding(
            padding: EdgeInsets.only(top: 10),
            child: Text(
              "Pandemia",
              style: TextStyle(
                fontSize: 20,
                fontWeight: FontWeight.w600,
              ),
            ),
          ),
          Padding(
              padding: EdgeInsets.only(top: 10),
              child: Text(
                "Version : ${PandemiaApp.appVersion}",
                style: TextStyle(
                  fontWeight: FontWeight.normal,
                  color: Colors.black,
                  fontSize: 16,
                ),
              ),
            ),
          _sizedBox(context),
          Container(
            color: Colors.black,
            height: 1,
            margin: EdgeInsets.only(
              left: MediaQuery.of(context).size.width / 8,
              right: MediaQuery.of(context).size.width / 8,
              top: MediaQuery.of(context).padding.top / 3,
              bottom: MediaQuery.of(context).padding.top,
            ),
          ),
          _row(
            label: "Data by",
            midText: ":",
            value: "www.kawalcorona.com",
          ),
          _row(
            label: " ",
            midText: " ",
            value: "www.worldmeters.info",
          ),
          SizedBox(
            height: MediaQuery.of(context).padding.top / 2,
          ),
          _row(
            label: "Server by",
            midText: ":",
            value: "Delameta",
          ),
          SizedBox(
            height: MediaQuery.of(context).padding.top / 2,
          ),
          _row(
            label: "Icon by",
            midText: ":",
            value: "photo3idea-studio",
          ),
        ],
      ),
    );
  }

  _sizedBox(BuildContext parentcontext) {
    return SizedBox(
      height: MediaQuery.of(parentcontext).size.width / 10,
    );
  }

  _textDefault(String text) {
    return Text(
      text,
      style: TextStyle(
        fontWeight: FontWeight.normal,
        color: Colors.black,
        fontSize: 16,
        
      ),
      textAlign: TextAlign.left,
    overflow: TextOverflow.ellipsis,);
  }

  _row({String label, String midText, String value}) {
    return Container(
      child: Row(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: <Widget>[
          Container(width: 80, child: _textDefault(label),),
          _textDefault(midText + "   "),
          Expanded(
            flex: 3,
            child: _textDefault(value),
          ),
        ],
      ),
    );
  }
}
