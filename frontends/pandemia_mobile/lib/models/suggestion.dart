class Suggestion {
  Address address;
  String countryCode;
  String label;
  String language;
  String locationId;
  String matchLevel;

  Suggestion(
      {this.address,
      this.countryCode,
      this.label,
      this.language,
      this.locationId,
      this.matchLevel});

  Suggestion.fromMap(Map<String, dynamic> json) {
    address =
        json['address'] != null ? new Address.fromMap(json['address']) : null;
    countryCode = json['countryCode'];
    label = json['label'];
    language = json['language'];
    locationId = json['locationId'];
    matchLevel = json['matchLevel'];
  }

  Map<String, dynamic> toMap() {
    final Map<String, dynamic> data = new Map<String, dynamic>();
    if (this.address != null) {
      data['address'] = this.address.toMap();
    }
    data['countryCode'] = this.countryCode;
    data['label'] = this.label;
    data['language'] = this.language;
    data['locationId'] = this.locationId;
    data['matchLevel'] = this.matchLevel;
    return data;
  }
}

class Address {
  String city;
  String country;
  String county;
  String postalCode;

  Address({this.city, this.country, this.county, this.postalCode});

  Address.fromMap(Map<String, dynamic> json) {
    city = json['city'];
    country = json['country'];
    county = json['county'];
    postalCode = json['postalCode'];
  }

  Map<String, dynamic> toMap() {
    final Map<String, dynamic> data = new Map<String, dynamic>();
    data['city'] = this.city;
    data['country'] = this.country;
    data['county'] = this.county;
    data['postalCode'] = this.postalCode;
    return data;
  }
}
