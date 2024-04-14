use secp256k1::{Secp256k1, SecretKey, PublicKey};
use sha3::{Digest, Keccak256};
use rand::{rngs::OsRng, RngCore, thread_rng};
use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::atomic::{AtomicBool, Ordering};

fn main() {
    let found = Arc::new(AtomicBool::new(false));
    let attempts = Arc::new(Mutex::new(0));
    let num_threads = 10;

    let mut handles = vec![];

    for _ in 0..num_threads {
        let found_clone = found.clone();
        let attempts_clone = attempts.clone();
        let handle = thread::spawn(move || {
            let secp = Secp256k1::new();
            let mut rng = OsRng;
            let mut local_attempts = 0u64;
            while !found_clone.load(Ordering::Relaxed) {
                local_attempts += 1;
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

                if address.starts_with("000") && address.ends_with("69420") {
                    found_clone.store(true, Ordering::Relaxed);
                    {
                        let mut attempts_lock = attempts_clone.lock().unwrap();
                        *attempts_lock += local_attempts;  // Add local counter to global counter
                    }
                    println!("Found matching Ethereum address: 0x{}", address);
                    println!("Private Key: {}", hex::encode(&private_key_bytes));
                    println!("Public Key: {}", hex::encode(public_key_bytes));
                    println!("Local Attempts: {}", local_attempts);
                    break;
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let total_attempts = attempts.lock().unwrap();
    println!("Total Attempts: {}", *total_attempts);
}
