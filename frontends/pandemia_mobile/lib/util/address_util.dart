import 'package:flutter_dotenv/flutter_dotenv.dart';
import 'package:http/http.dart' as http;
import 'package:location/location.dart';
import 'package:pandemia_mobile/util/json_helper.dart';

Future<dynamic> getLocationName(LocationData locationData) async {
  final apiKey = DotEnv().env['GEOLOCATOR_API_KEY'];
  final resp = await http.get(Uri.parse(
      "https://reverse.geocoder.ls.hereapi.com/6.2/reversegeocode.json?prox=${locationData.latitude},${locationData.longitude}&mode=retrieveAddresses&maxResults=1&gen=1&apiKey=$apiKey"));
  if (resp != null) {
    final result = tryDecode(resp.body);
    final addr =
        result["Response"]["View"].first["Result"].first["Location"]["Address"];
    return "${addr["City"]}, ${addr["County"]}, ${addr["AdditionalData"].first["value"]}";
  }
}
