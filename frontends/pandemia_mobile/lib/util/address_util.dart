import 'package:flutter_dotenv/flutter_dotenv.dart';
import 'package:http/http.dart' as http;
import 'package:location/location.dart';
import 'package:pandemia_mobile/util/json_helper.dart';

class GeoLocation {
  final String subdistrict;
  final String district;
  final String city;
  final String state;
  final String country;

  GeoLocation(
      {this.subdistrict, this.district, this.city, this.state, this.country});

  @override
  String toString() {
    return "$country/$state/$city/$district/$subdistrict";
  }
}

Future<dynamic> getLocationName(LocationData locationData) async {
  final apiKey = DotEnv().env['GEOLOCATOR_API_KEY'];
  final resp = await http.get(Uri.parse(
      "https://reverse.geocoder.ls.hereapi.com/6.2/reversegeocode.json?prox=${locationData.latitude},${locationData.longitude}&mode=retrieveAddresses&maxResults=1&gen=1&apiKey=$apiKey"));
  if (resp != null) {
    final result = tryDecode(resp.body);
    final addr =
        result["Response"]["View"].first["Result"].first["Location"]["Address"];
    // return "${addr["District"]}/${addr["Subdistrict"]} ${addr["City"]}, ${addr["County"]}, ${addr["AdditionalData"].first["value"]}";

    String countryName = addr["Country"];

    if (addr["AdditionalData"] != null &&
        addr["AdditionalData"].first["CountryName"] != null) {
      countryName = addr["AdditionalData"].first["CountryName"];
    }

    return GeoLocation(
      country: countryName,
      city: addr["City"],
      state: addr["State"],
      district: addr["District"],
      subdistrict: addr["Subdistrict"],
    );
  }
}
