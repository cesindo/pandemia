import 'package:flutter/material.dart';
import 'package:package_info/package_info.dart';
import 'package:url_launcher/url_launcher.dart';

class AboutPage extends StatefulWidget {
  AboutPage({Key key}) : super(key: key);

  @override
  _AboutPageState createState() => _AboutPageState();
}

const PandemiaRepoLink = "https://github.com/cesindo/pandemia";

const List<String> Programmers = [
  "Robin (@anvie)",
  "Fatkhur",
  "Cak Nasrul (luffynas)",
  "Samsul",
  "Muiz",
  "Rifai"
];

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
        child: ListView(
          shrinkWrap: true,
          addAutomaticKeepAlives: true,
          children: <Widget>[
            _getBody(context),
          ],
        ),
      ),
    );
  }

  Widget _getBody(BuildContext context) {
    List<Widget> credits = [
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
          "Version : $version",
          style: TextStyle(
            fontWeight: FontWeight.normal,
            color: Colors.black,
            fontSize: 16,
          ),
        ),
      ),
      _sizedBox(context),
      _textDefault(
          "Adalah program sumber terbuka (open source) yang dikembangkan oleh komunitas " +
              "untuk memudahkan " +
              "dalam memantau persebaran wabah, sehingga dapat mengambil keputusan yang " +
              "lebih bijak dan terukur dalam melakukan kegiatan kesehariannya.",
          maxLines: 10,
          textAlign: TextAlign.center),
      Container(
        color: Colors.black,
        height: 1,
        margin: EdgeInsets.only(
          left: MediaQuery.of(context).size.width / 8,
          right: MediaQuery.of(context).size.width / 8,
          top: MediaQuery.of(context).padding.top / 3,
          bottom: MediaQuery.of(context).padding.top / 3,
        ),
      ),
    ];

    credits.addAll([
      _row(
        label: "Data dari",
        midText: ":",
        value: "www.worldmeters.info",
      ),
      _row(
        label: " ",
        midText: " ",
        value: "corona.jatengprov.go.id",
      ),
      _row(
        label: " ",
        midText: " ",
        value: "corona.jogjaprov.go.id",
      ),
      _row(
        label: "",
        midText: " ",
        value: "www.cekdiri.id",
      ),
      _row(
        label: "",
        midText: " ",
        value: "www.detax.org",
      ),
      SizedBox(
        height: MediaQuery.of(context).padding.top / 2,
      ),
    ]);

    credits.add(Text("Kontributor:"));
    credits.add(_divider());

    credits.addAll([
      _row(
        label: "Server oleh",
        midText: ":",
        value: "Delameta",
      ),
      _divider(),
      _row(
        label: "Icon oleh",
        midText: ":",
        value: "photo3idea_studio",
      ),
      _row(
        label: "",
        midText: "",
        value: " (www.flaticon.com)",
      ),
      _row(
        label: "",
        midText: ":",
        value: "freeicons.io",
      ),
      _divider(),
    ]);

    credits.add(Text("Pemrogram:"));

    credits.add(_divider());
    Programmers.forEach((p) {
      credits.add(_textDefault(p));
    });

    credits.add(_divider());

    credits.add(
      Column(
        mainAxisAlignment: MainAxisAlignment.center,
        crossAxisAlignment: CrossAxisAlignment.center,
        children: <Widget>[
          _textDefault(
              "Anda seorang pemrogram? Mari berkontribusi pada proyek Pandemia",
              maxLines: 2,
              textAlign: TextAlign.center),
          _divider(),
          Container(
            child: Row(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: <Widget>[
                Container(
                  width: 100,
                  child: _textDefault("Kode sumber"),
                ),
                _textDefault(": "),
                Expanded(
                  flex: 3,
                  child: GestureDetector(child: Text(PandemiaRepoLink, style: TextStyle(color: Colors.blue,fontSize: 15),), onTap: (){
                    launch(PandemiaRepoLink);
                  },)
                ),
              ],
            ),
          )
        ],
      ),
    );

    return Container(
      padding: EdgeInsets.only(
          left: MediaQuery.of(context).size.width / 10,
          right: MediaQuery.of(context).size.width / 10,
          bottom: MediaQuery.of(context).padding.top),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.center,
        mainAxisAlignment: MainAxisAlignment.center,
        children: credits,
      ),
    );
  }

  Widget _divider() {
    return SizedBox(
      height: MediaQuery.of(context).padding.top / 2,
    );
  }

  Widget _sizedBox(BuildContext parentcontext) {
    return SizedBox(
      height: MediaQuery.of(parentcontext).size.width / 10,
    );
  }

  Widget _textDefault(String text, {maxLines: 1, textAlign: TextAlign.left}) {
    return Text(
      text,
      style: TextStyle(
        fontWeight: FontWeight.normal,
        color: Colors.black,
        fontSize: 16,
      ),
      textAlign: textAlign,
      overflow: TextOverflow.ellipsis,
      maxLines: maxLines,
    );
  }

  _row({String label, String midText, String value}) {
    return Container(
      child: Row(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: <Widget>[
          Container(
            width: 100,
            child: _textDefault(label),
          ),
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
