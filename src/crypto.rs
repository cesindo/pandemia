//! Utilitas berkaitan dengan kriptografi.
//! Misalnya untuk mendapatkan passhash dari plain password,
//! menggenerasikan pasangan kunci (keypair) asimetris,
//! melakukan signing pada data, dll.

use bcrypt;
use ed25519_dalek::Keypair;
use hex;
use rand::thread_rng;
use sha2::{Digest, Sha256, Sha512};

/// Number of bytes in a public key.
pub const PUBLIC_KEY_LENGTH: usize = ed25519_dalek::PUBLIC_KEY_LENGTH;
/// Number of bytes in a secret key.
pub const SECRET_KEY_LENGTH: usize = ed25519_dalek::EXPANDED_SECRET_KEY_LENGTH; //ds::SECRETKEYBYTES;
/// Number of bytes in a `Hash`.
pub const HASH_SIZE: usize = 32; //sha256::DIGESTBYTES;
/// Number of bytes in a signature.
pub const SIGNATURE_LENGTH: usize = ed25519_dalek::SIGNATURE_LENGTH;

const DEFAULT_BCRYPT_COST: u32 = 5;

// Buatkan wrapper untuk object-object internal dari crypto_impl
// agar lebih flexibel kita bisa menambahkan implementasi sendiri.

implement_crypto_wrapper!(
    struct PublicKey, ed25519_dalek::PublicKey, PublicKey, PUBLIC_KEY_LENGTH
);
implement_crypto_wrapper!(
    struct SecretKey, ed25519_dalek::ExpandedSecretKey, ExpandedSecretKey, SECRET_KEY_LENGTH
);
implement_crypto_wrapper!(
    struct Signature, ed25519_dalek::Signature, Signature, SIGNATURE_LENGTH
);

/// Hash
pub struct Hash([u8; HASH_SIZE]);

impl Hash {
    /// Encode to hex string
    pub fn to_hex(&self) -> String {
        hex::encode(self.0)
    }
}

impl AsRef<[u8; HASH_SIZE]> for Hash {
    fn as_ref(&self) -> &[u8; HASH_SIZE] {
        &self.0
    }
}

impl PublicKey {
    /// Memastikan signature valid untuk message dengan cara memverifikasi
    /// digital signature menggunakan public-key ini.
    pub fn valid(&self, message: &[u8], signature: &Signature) -> bool {
        let raw_pubkey =
            ed25519_dalek::PublicKey::from_bytes(&self.0).expect("Cannot parse bytes for public key");
        raw_pubkey.verify::<Sha512>(message, &signature.into()).is_ok()
    }
}

impl<'a> std::convert::Into<ed25519_dalek::Signature> for &'a Signature {
    fn into(self) -> ed25519_dalek::Signature {
        ed25519_dalek::Signature::from_bytes(&self.0).unwrap()
    }
}

/// Mendapatkan passhash dari sebuah password.
/// Algo menggunakan Bcrypt.
pub fn get_passhash(password: &str) -> String {
    bcrypt::hash(password, DEFAULT_BCRYPT_COST)
        .unwrap_or_else(|_| panic!("Cannot bcrypt password `{}`", password))
}

/// Memverifikasi apakah password match (verified) dengan hash-nya?
pub fn password_match(password: &str, hashed: &str) -> bool {
    bcrypt::verify(password, hashed).expect("Cannot verify bcrypt hash")
}

/// Generate key pair
pub fn gen_keypair() -> (PublicKey, SecretKey) {
    let mut csprng = thread_rng();
    let keypair = Keypair::generate::<Sha512, _>(&mut csprng);

    (
        PublicKey::new(keypair.public.to_bytes()),
        SecretKey::new(keypair.secret.expand::<Sha512>().to_bytes()),
    )
}

/// Hash some str text
pub fn hash_str(text: &str) -> Hash {
    sha256_hash(&text.as_bytes())
}

/// Get hash sha256 from bytes
pub fn sha256_hash(bytes: &[u8]) -> Hash {
    let mut hasher = Sha256::new();
    hasher.input(bytes);
    let hash = hasher.result().to_vec();

    let mut fixed: [u8; HASH_SIZE] = Default::default();
    fixed.copy_from_slice(hash.as_slice());
    Hash(fixed)
}

/// Get hash sha256 from bytes
pub fn sha256_hash_raw(bytes: &[u8]) -> [u8; HASH_SIZE] {
    let mut hasher = Sha256::new();
    hasher.input(bytes);
    let hash = hasher.result().to_vec();

    let mut fixed: [u8; HASH_SIZE] = Default::default();
    fixed.copy_from_slice(hash.as_slice());
    fixed
}

/// Get hash sha512 from bytes
pub fn sha512_hash_raw(bytes: &[u8]) -> [u8; 64] {
    let mut hasher = Sha512::new();
    hasher.input(bytes);
    let hash = hasher.result().to_vec();

    let mut fixed: [u8; 64] = [0u8; 64];
    fixed.copy_from_slice(hash.as_slice());
    fixed
}

/// Sign a data in bytes, return Signature.
pub fn sign(bytes: &[u8], secret_key: &SecretKey) -> Signature {
    let (pub_key, secret_key) = get_raw_keypair_from_secret(secret_key);
    // let raw_signature = keypair.sign::<Sha512>(bytes);
    let raw_signature = secret_key.sign::<Sha512>(bytes, &pub_key);

    Signature(raw_signature.to_bytes())
}

fn get_raw_keypair_from_secret(
    secret_key: &SecretKey,
) -> (ed25519_dalek::PublicKey, ed25519_dalek::ExpandedSecretKey) {
    let raw_secret_key =
        ed25519_dalek::ExpandedSecretKey::from_bytes(&secret_key.0).expect("to raw secret key");
    let public_key = ed25519_dalek::PublicKey::from_expanded_secret(&raw_secret_key);
    (public_key, raw_secret_key)
}

/// Memverifikasi digital signature apakah cocok dengan data dan public key-nya.
pub fn is_verified(message: &[u8], signature: &Signature, pub_key: &PublicKey) -> bool {
    pub_key.valid(message, signature)
}

#[cfg(test)]
mod tests {
    use super::{PublicKey, SecretKey, Signature};

    #[test]
    fn test_get_passhash() {
        let passhash = super::get_passhash("123");
        assert_ne!(passhash, "");
        assert!(passhash.len() > 10);
    }

    #[test]
    fn test_verify_passhash() {
        let passhash = super::get_passhash("123");
        assert_eq!(super::password_match("123", &passhash), true);
        assert_eq!(super::password_match("1234", &passhash), false);
        assert_eq!(super::password_match(" 123 ", &passhash), false);
    }

    #[test]
    fn test_gen_keyppair() {
        let (p, s) = super::gen_keypair();
        println!("{} -> {}", p.to_hex(), s.to_hex());
        assert_ne!(p.to_hex(), s.to_hex());
        assert_ne!(p.to_hex(), "".to_string());
        assert_ne!(s.to_hex(), "".to_string());
    }

    #[test]
    fn test_hash() {
        let h = super::hash_str("Zufar");
        assert_eq!(
            h.to_hex(),
            "96d301802cf09936d0aa746c5de12b2f2085bd878f2c1e43ebad6074650f218a".to_string()
        );
    }

    const DATA: &'static [u8] = b"Zufar";

    fn get_preset_keypair() -> (PublicKey, SecretKey) {
        (
            "7eb68ec11925cb0ac8b1d1e142492b2e496cdafa06e09eca72dd1846a47d2985"
                .parse::<PublicKey>()
                .unwrap(),
            "20041a200036f4b24fd7fe49f809f4dcd90e37fbea3a46bf8524d06c46c66b6b\
             4c3ddc41b2573731f130d5d29d27b609d505ac97902af952ae74fc97b996bbb7"
                .parse::<SecretKey>()
                .unwrap(),
        )
    }

    fn create_signature() -> Signature {
        let (p, s) = get_preset_keypair();
        super::sign(DATA, &s)
    }

    #[test]
    fn test_create_signature() {
        let signature = create_signature();

        println!("signature: {}", signature.to_hex());

        assert_eq!(
            signature.to_hex(),
            "8d136e468e3c24ac61518641c01b6dffc79bf4951c3a02c22cab855119bc7f34\
             d0765a3922af7f423390befe2e7e1e0b6e8b6e066d893c352a0b0b5f0f39e902"
        );
    }

    #[test]
    fn test_verify_signature() {
        let (p, _) = get_preset_keypair();
        let signature = create_signature();

        assert!(super::is_verified(DATA, &signature, &p));
    }
}
