class Throttle {
  static Map<String, int> _throttle = {};

  static bool isReady(String key, {int within: 2000}) {
    if (Throttle._throttle[key] != null &&
        DateTime.now().millisecondsSinceEpoch < _throttle[key]) {
      return false;
    }
    Throttle._throttle[key] = DateTime.now().millisecondsSinceEpoch + within;
    return true;
  }
}
