import 'package:intl/intl.dart';
import 'package:timezone/timezone.dart';
import 'package:flutter/services.dart';

class TimeHelper {
  // static final TimeHelperService _singleton = TimeHelperService._internal();

  // factory TimeHelperService() {
  //   return _singleton;
  // }

  static void setup() async {
    var byteData = await rootBundle.load('assets/timezone/data/2019b.tzf');
    initializeDatabase(byteData.buffer.asUint8List());
  }

  static Location getLocal() {
    // @TODO(*): buat bisa configurable location-nya
    final loc = getLocation("Asia/Jakarta");
    setLocalLocation(loc);
    return loc;
  }

  // TimeHelperService._internal(){
  //   setup();
  // }

  /// Parse date in text string format as UTC DateTime
  /// example parseable date time format: 2020-01-08 17:17:11.121896
  static DateTime parseAsUtc(String text) {
    final t = DateTime.parse(text);
    return DateTime.utc(t.year, t.month, t.day, t.hour, t.minute, t.second);
  }

  static String formatSimple(DateTime dt) {
    return DateFormat('dd MMMM yyyy kk:mm').format(dt);
  }

  /// Membentuk format untuk mempermudah serialize di client <-> server.
  static String serFormat(DateTime dt) {
    return DateFormat('yyyy-MM-ddTHH:mm:ss').format(dt);
  }

  /// Get current date time.
  static DateTime now() {
    return DateTime.now();
  }

  /// Get serialized text formatted of current date time.
  static String nowFormatted() {
    return TimeHelper.serFormat(TimeHelper.now());
  }
}
