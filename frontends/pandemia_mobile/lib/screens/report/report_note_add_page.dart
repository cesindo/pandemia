import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:pandemia_mobile/blocs/report_note/report_note_bloc.dart';
import 'package:pandemia_mobile/blocs/report_note/report_note_event.dart';
import 'package:pandemia_mobile/blocs/report_note/report_note_state.dart';

class ReportNoteAddPage extends StatefulWidget {
  ReportNoteAddPage({Key key}) : super(key: key);

  @override
  State<ReportNoteAddPage> createState() => _ReportNoteState();
}

class _ReportNoteState extends State<ReportNoteAddPage> {
  final _notesController = TextEditingController();
  ReportNoteBloc bloc;

  @override
  Widget build(BuildContext context) {
    bloc =  BlocProvider.of<ReportNoteBloc>(context);

    _onAddButtonPressed() {
      bloc.dispatch(CreateReportNote(_notesController.text));
    }

    return BlocBuilder<ReportNoteBloc, ReportNoteState>(
        builder: (BuildContext context, ReportNoteState state) {
      return Scaffold(
        appBar: AppBar(title: Text("Buat Laporan")),
        body: BlocListener<ReportNoteBloc, ReportNoteState>(
            listener: (context, state) {
          if (state is ReportNoteCreated) {
            Navigator.pop(context, state.item);
          } else if (state is ReportNoteFailure) {
            Scaffold.of(context).showSnackBar(SnackBar(
              content: Text(
                state.error,
                style: TextStyle(color: Colors.white),
              ),
              backgroundColor: Colors.red,
              duration: Duration(seconds: 3),
            ));
          }
        }, child: BlocBuilder<ReportNoteBloc, ReportNoteState>(
          builder: (context, state) {
            return Center(
              child: ListView(
                children: <Widget>[
                  Padding(
                      padding: const EdgeInsets.all(10.0),
                      child: Form(
                          child: Column(
                        children: <Widget>[
                          TextFormField(
                            decoration:
                                InputDecoration(labelText: "Laporan:"),
                            autofocus: true,
                            controller: _notesController,
                            maxLines: 10,
                            onChanged: (_){
                              setState(() {
                                
                              });
                            },
                          ),
                          Row(mainAxisAlignment: MainAxisAlignment.end,
                          crossAxisAlignment: CrossAxisAlignment.end,
                            children: <Widget>[
                              RaisedButton(
                                onPressed: _notesController.text.trim().length > 0
                                    ? _onAddButtonPressed
                                    : null,
                                child: Text("LAPOR"),
                              )
                            ],
                          )
                        ],
                      ))),
                ],
              ),
            );
          },
        )),
      );
    });
  }
}
