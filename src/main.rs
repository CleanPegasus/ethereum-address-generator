use secp256k1::{Secp256k1, SecretKey, PublicKey};
use sha3::{Digest, Keccak256};
use rand::rngs::OsRng;
use rand::RngCore;

fn main() {
    let mut index = 0;
    loop {
        println!("Generating Ethereum Address at {}", index);
        let address = generate_ethereum_address();
        if address.starts_with("00000") {
            println!("0x{}", address);
            break;
        }
        index += 1;
    }
}

fn generate_ethereum_address() -> String {
    let secp = Secp256k1::new();
    let mut rng = OsRng;
    let mut private_key_bytes = [0u8; 32];
    rng.fill_bytes(&mut private_key_bytes);
    let secret_key = SecretKey::from_slice(&private_key_bytes).expect("32 bytes, within curve order");
    let public_key = PublicKey::from_secret_key(&secp, &secret_key);
    let public_key_serialized = public_key.serialize_uncompressed();
    let public_key_bytes = &public_key_serialized[1..];
    let mut hasher = Keccak256::new();
    hasher.update(public_key_bytes);
    let hash = hasher.finalize();
    let address_bytes = &hash[12..32];
    let address = hex::encode(address_bytes);

    println!("Private Key: {}", hex::encode(&private_key_bytes));
    println!("Public Key: {}", hex::encode(public_key_bytes));
    println!("Ethereum Address: 0x{}", address);

    return address;
}