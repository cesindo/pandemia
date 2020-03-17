import 'package:flutter/material.dart';
import 'package:flutter/widgets.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:pandemia_mobile/core/core.dart';
import 'package:pandemia_mobile/blocs/notif/notif.dart';
import 'package:pandemia_mobile/widgets/notif_item_view.dart';
import 'package:pandemia_mobile/widgets/widgets.dart';
import 'package:pandemia_mobile/widgets/loading_indicator.dart';

class NotifList extends StatelessWidget {

  NotifList(BuildContext context){
    // final notifBloc = BlocProvider.of<NotifBloc>(context);
    // notifBloc.dispatch(LoadNotif());
  }

  @override
  Widget build(BuildContext context) {
    // final pandemiaBloc = BlocProvider.of<PandemiaBloc>(context);

    return BlocBuilder<NotifBloc, NotifState>(
      builder: (context, state) {
        if (state is NotifListLoading) {
          return LoadingIndicator(key: PandemiaKeys.loading);
        } else if (state is NotifListLoaded) {
          // return Text("satu");
          final notifs = state.notifs;
          return ListView.builder(
            key: PandemiaKeys.notifList,
            itemCount: notifs.length,
            itemBuilder: (BuildContext context, int index) {
              final item = notifs[index];
              return new NotifItemView(item: item);
            },
          );
        } else {
          return Text("Unknown state");
        }
      },
    );
  }
}

