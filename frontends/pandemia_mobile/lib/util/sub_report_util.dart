


class SubReportUtil {
  static final SubReportUtil _instance = SubReportUtil._internal();

  Map<String, String> statusLabelToIdName = {
    "ODP": "odp",
    "ODP Selesai Pemantauan": "odpsp",
    "PDP": "pdp",
    "PDP Sembuh": "pdps",
    "PDP Meninggal": "pdpm",
    "Orang Tidak Bergejala (OTG)": "otg",
    "Positif": "positive",
    "Positif Sembuh": "recovered",
    "Positif Meninggal": "death",
  };
  Map<String, String> statusIdNameToLabel = {};

  factory SubReportUtil() {
    return _instance;
  }

  SubReportUtil._internal(){
    for (final ma in statusLabelToIdName.entries) {
      statusIdNameToLabel[ma.value] = ma.key;
    }
  }
}
