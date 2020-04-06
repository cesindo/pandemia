import 'package:pandemia_mobile/models/user_settings.dart';

UserSettings toUserSettings(List<dynamic> data) {
  UserSettings settings =
      UserSettings(0, true, false, false, false, false, false);
  for (final s in data) {
    if (s["s_key"] == "enable_push_notif") {
      settings = settings.copy(enablePushNotif: s["s_value"] == "true");
    } else if (s["s_key"] == "complaint_map") {
      settings = settings.copy(complaintMap: s["s_value"] == "true");
    } else if (s["s_key"] == "has_cough") {
      settings = settings.copy(hasCough: s["s_value"] == "true");
    } else if (s["s_key"] == "has_fever") {
      settings = settings.copy(hasFever: s["s_value"] == "true");
    } else if (s["s_key"] == "has_cold") {
      settings = settings.copy(hasCold: s["s_value"] == "true");
    } else if (s["s_key"] == "has_headache") {
      settings = settings.copy(hasHeadache: s["s_value"] == "true");
    }
  }
  return settings;
}
