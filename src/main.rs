use rand::Rng;
use secp256k1::{All, PublicKey, Secp256k1, SecretKey};
use sha3::{Digest, Keccak256};
use rand::{rngs::OsRng, RngCore};
use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::atomic::{AtomicBool, Ordering};

fn generate_key_pair(secp: &Secp256k1<All>, rng: &mut OsRng) -> (SecretKey, PublicKey, String) {
    let mut private_key_bytes = [0_u8; 32];
    rng.fill(&mut private_key_bytes);
    let secret_key = SecretKey::from_slice(&private_key_bytes).unwrap();
    let public_key = PublicKey::from_secret_key(secp, &secret_key);
    let private_key = hex::encode(private_key_bytes);
    (secret_key, public_key, private_key)
}

fn calculate_address(public_key: &PublicKey) -> String {
    let public_key_serialized = public_key.serialize_uncompressed();
    let public_key_bytes = &public_key_serialized[1..];
    // hash public key
    let mut hasher = Keccak256::new();
    hasher.update(public_key_bytes);
    let hash = hasher.finalize();
    let address_bytes = &hash[&hash.len() - 20..];
    hex::encode(address_bytes)
}


fn worker_thread(found: Arc<AtomicBool>, attempts: Arc<Mutex<u64>>, starting_string: &str) {
    let secp = Secp256k1::new();
    let mut rng = OsRng;
    let mut local_attempts = 0_u64;

    while !found.load(Ordering::Relaxed) {
        local_attempts += 1;
        let (secret_key, public_key, private_key) = generate_key_pair(&secp, &mut rng);
        let address = calculate_address(&public_key);
        let public_key_serialized = &public_key.serialize_uncompressed();
        let public_key_bytes = &public_key_serialized[1..];
        // update global pointer
        {
            let mut attempts_lock = attempts.lock().unwrap();
            *attempts_lock += local_attempts;
        }
        if address.starts_with(starting_string) {
            found.store(true, Ordering::Relaxed);
            println!("Found matching Ethereum address: 0x{}", address);
            println!("Private Key: {}", private_key);
            println!("Public Key: {}", hex::encode(public_key_bytes));
            println!("Local Attempts: {}", local_attempts);
            break;
        }

    }

}

fn main() {
    let starting_string = "000000";
    let found = Arc::new(AtomicBool::new(false));
    let attempts = Arc::new(Mutex::new(0));
    let num_threads = 12;

    let mut handles = vec![];

    for _ in 0..num_threads {
        let found_clone = found.clone();
        let attempts_clone = attempts.clone();
        let handle = thread::spawn(move || {
            worker_thread(found_clone, attempts_clone, starting_string);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let total_attempts = attempts.lock().unwrap();
    println!("Total Attempts: {}", *total_attempts);
}
