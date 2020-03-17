
var util = {
  // Konversi bytes ke hexa string.
  toHexString(byteArray) {
    return Array.from(byteArray, function(byte) {
      return ('0' + (byte & 0xFF).toString(16)).slice(-2);
    }).join('')
  }
};

export default util;


