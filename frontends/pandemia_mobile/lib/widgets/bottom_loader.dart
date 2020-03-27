import 'package:flutter/material.dart';

class BottomLoader extends StatelessWidget {
  final bool isLoading;
  const BottomLoader({Key key, @required this.isLoading}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    if (isLoading) {
      return Container(
        padding: EdgeInsets.symmetric(vertical: 5),
        alignment: Alignment.center,
        child: Center(
          child: SizedBox(
            width: 33,
            height: 33,
            child: CircularProgressIndicator(
              strokeWidth: 2,
            ),
          ),
        ),
      );
    }

    return Container();
  }
}
