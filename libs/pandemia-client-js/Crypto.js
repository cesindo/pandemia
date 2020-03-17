
import cryptojs from "crypto";
import ds_impl from "supercop.js";

export class Crypto {
  // Mendapatkan passhash dari plain password.
  static getPasshash(plainPassword){
    var bytes = Buffer.from(plainPassword, 'utf-8');
    var hash = this.sha256(bytes);
    Array(9).fill().map((_, _i) => {
      hash = this.sha256(hash);
    });
    // return util.toHexString(hash);
    return hash.toString('hex');
  }
  // Kalkulasikan hash sha256 pada data.
  static sha256(data, output){
    return cryptojs.createHash("sha256").update(data).digest(output);
  }
  static genSeed(){
    return ds_impl.createSeed();
  }
  static genKeypair(){
    return ds_impl.createKeyPair(this.genSeed());
  }
  static sign(data, pubKey, secretKey){
    var sig = ds_impl.sign(new Buffer(data), pubKey, secretKey);
    return new Buffer(sig).toString("hex");
  }
  static verify(sig, data, pubKey){
    return ds_impl.verify(sig, data, pubKey);
  }
}


