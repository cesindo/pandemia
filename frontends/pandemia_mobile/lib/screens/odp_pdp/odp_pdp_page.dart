import 'package:flutter/material.dart';

class OdpPdpScreen extends StatefulWidget {
  OdpPdpScreen({Key key}) : super(key: key);

  @override
  _OdpPdpScreenState createState() => _OdpPdpScreenState();
}

class _OdpPdpScreenState extends State<OdpPdpScreen> {
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: DefaultTabController(
        length: 2,
        child: Scaffold(
          appBar: AppBar(
            leading: IconButton(
              icon: Icon(Icons.arrow_back),
              onPressed: () {
                Navigator.pop(context);
              },
            ),
            backgroundColor: Color(0xFF7A58FF),
            title: Text("Data ODP/PDP"),
            bottom: TabBar(
              tabs: <Widget>[
                Tab(
                  text: 'ODP',
                ),
                Tab(
                  text: 'PDP',
                )
              ],
            ),
          ),
          body: TabBarView(
            children: <Widget>[
              Padding(
                padding: EdgeInsets.only(top: 10, left: 10, right: 10),
                child: ListView.builder(
                  itemCount: 10,
                  itemBuilder: (BuildContext context, index) {
                    return Card(
                      child: ListTile(
                        leading: Icon(
                          Icons.person_pin,
                          size: 30,
                        ),
                        title: Text("John Doe"),
                        subtitle: Text("Wadaslintang"),
                      ),
                    );
                  },
                ),
              ),
              Padding(
                  padding: EdgeInsets.only(top: 10, left: 10, right: 10),
                  child: ListView.builder(
                    itemCount: 10,
                    itemBuilder: (BuildContext context, index) {
                      return Card(
                        child: ListTile(
                          leading: Icon(
                            Icons.person_pin,
                            size: 30,
                          ),
                          title: Text("John Doe"),
                          subtitle: Text("Wadaslintang"),
                        ),
                      );
                    },
                  )),
            ],
          ),
        ),
      ),
    );
  }
}
