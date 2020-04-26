import 'package:flutter_dotenv/flutter_dotenv.dart';
import 'package:google_maps_flutter/google_maps_flutter.dart';
import 'package:http/http.dart' as http;
import 'package:location/location.dart';
import 'package:pandemia_mobile/models/suggestion.dart';
import 'package:pandemia_mobile/util/json_helper.dart';

final apiKey = DotEnv().env['GEOLOCATOR_API_KEY'];

class GeoLocation {
  final String subdistrict;
  final String district;
  final String city;
  final String province;
  final String country;

  GeoLocation(
      {this.subdistrict,
      this.district,
      this.city,
      this.province,
      this.country});

  @override
  String toString() {
    return "$country/$province/$city/$district/$subdistrict";
  }
}

Future<dynamic> getLocationName(LocationData locationData) async {
  final resp = await http.get(Uri.parse(
      "https://reverse.geocoder.ls.hereapi.com/6.2/reversegeocode.json?prox=${locationData.latitude},${locationData.longitude}&mode=retrieveAddresses&maxResults=1&gen=1&apiKey=$apiKey"));
  if (resp != null) {
    final result = tryDecode(resp.body);
    final addr =
        result["Response"]["View"].first["Result"].first["Location"]["Address"];

    String countryName = addr["Country"];
    String provinceName = addr["County"];

    if (addr["AdditionalData"] != null) {
      final dAdd = addr["AdditionalData"];

      final String _countryName = dAdd.where((a) => a["key"] == "CountryName").map((a) => a["value"]).first;
      final String _provinceName = dAdd.where((a) => a["key"] == "CountyName").map((a) => a["value"]).first;

      countryName = _countryName != null ? _countryName : countryName;
      provinceName = _provinceName != null ? _provinceName : provinceName;
    }

    return GeoLocation(
      country: countryName,
      city: addr["City"],
      province: addr["State"] != null ? addr["State"] : provinceName,
      district: addr["District"],
      subdistrict: addr["Subdistrict"] != null ? addr["Subdistrict"] : "",
    );
  }
}

Future<List<Suggestion>> searchLocation(String query) async {
  final resp = await http.get(
      "https://autocomplete.geocoder.ls.hereapi.com/6.2/suggest.json\?apiKey\=$apiKey\&query\=$query");

  if (resp != null && query.isNotEmpty) {
    final result = tryDecode(resp.body);
    return (result["suggestions"] as List<dynamic>)
        .map((addr) => Suggestion.fromMap(addr))
        .toList();
  } else {
    return null;
  }
}

Future<LatLng> getLatLongPosition(String locationId) async {
  final resp = await http.get(
      "https://geocoder.ls.hereapi.com/6.2/geocode.json?locationid=$locationId&jsonattributes=1&gen=9&apiKey=$apiKey");

  if (resp != null && locationId.isNotEmpty) {
    final result = tryDecode(resp.body);
    final data = result["response"]["view"].first["result"].first["location"]
        ["displayPosition"];
    return LatLng(data["latitude"], data["longitude"]);
  } else {
    return null;
  }
}
