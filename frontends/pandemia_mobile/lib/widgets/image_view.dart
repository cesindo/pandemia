import 'package:cached_network_image/cached_network_image.dart';
import 'package:flutter/material.dart';
import 'package:photo_view/photo_view.dart';

class ViewImage extends PageRouteBuilder {
  final String imageUrl;

  ViewImage({@required this.imageUrl})
      : super(
            pageBuilder: (BuildContext context, Animation<double> animation,
                Animation<double> secondaryAnimation) {
              return new GenerateContent(
                imageUrl: imageUrl,
              );
            },
            transitionsBuilder: (BuildContext context,
                Animation<double> animation,
                Animation<double> secondaryAnimation,
                Widget child) {
              return ScaleTransition(
                scale: Tween<double>(
                  begin: 0.0,
                  end: 1.0,
                ).animate(
                  CurvedAnimation(
                    parent: animation,
                    curve: Curves.easeInOutCirc,
                  ),
                ),
                child: child,
              );
            },
            fullscreenDialog: true);
}

class GenerateContent extends StatefulWidget {
  final String imageUrl;
  GenerateContent({Key key, this.imageUrl}) : super(key: key);

  @override
  _GenerateContentState createState() => _GenerateContentState(this.imageUrl);
}

class _GenerateContentState extends State<GenerateContent> {
  final String imageUrl;

  _GenerateContentState(this.imageUrl);

  @override
  Widget build(BuildContext context) {
    return SafeArea(
      child: Scaffold(
          backgroundColor: Colors.black,
          body: Stack(
            children: <Widget>[
              Positioned(
                child: PhotoView(
                  imageProvider: CachedNetworkImageProvider(imageUrl),
                  minScale: PhotoViewComputedScale.contained * 0.8,
                  maxScale: PhotoViewComputedScale.covered * 1,
                  enableRotation: false,
                  backgroundDecoration: BoxDecoration(
                    color: Colors.black,
                  ),
                ),
              ),
              Positioned(
                top: 10,
                right: 10,
                child: InkWell(
                    onTap: () => Navigator.pop(context),
                    child: Material(
                        color: Colors.transparent,
                        child: Container(
                            padding: EdgeInsets.all(4),
                            decoration: BoxDecoration(
                                color: Color.fromRGBO(45, 45, 45, 0.6),
                                borderRadius: BorderRadius.circular(40)),
                            child: Icon(Icons.close, size: 30, color: Colors.white)))),
              ),
            ],
          )),
    );
  }
}
