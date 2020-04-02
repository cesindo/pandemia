
export const obMapToNativeMap = (data) => {
  var nativeMap = {};
  for (var d in data) {
    nativeMap[d] = data[d];
  }
  return nativeMap;
}


export const obMapToArrayValues = (data) => {
  var rv = [];
  for (var d in data) {
    rv.push(data[d]);
  }
  return rv;
}

