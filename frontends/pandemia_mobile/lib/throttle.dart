


class Throttle {
  static Map<String, int> _throttle = {};

  static bool isReady(String key){
    if (Throttle._throttle[key] != null && DateTime.now().millisecondsSinceEpoch < _throttle[key]){
      return false;
    }
    Throttle._throttle[key] = DateTime.now().millisecondsSinceEpoch + 3000;
    return true;
  }

  static bool isReadyWithin(String key, int within){
    if (Throttle._throttle[key] != null && DateTime.now().millisecondsSinceEpoch < _throttle[key]){
      return false;
    }
    Throttle._throttle[key] = DateTime.now().millisecondsSinceEpoch + within;
    return true;
  }
}

