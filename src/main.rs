use rand::Rng;
use secp256k1::{All, PublicKey, Secp256k1, SecretKey};
use sha3::{Digest, Keccak256};
use rand::rngs::OsRng;
use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::atomic::{AtomicBool, Ordering};
use structopt::StructOpt;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, StructOpt)]
#[structopt(name = "ethereum-address-generator", about = "Generate Vanity Ethereum addresses")]
struct Args {
    #[structopt(short = "s", long = "start_string", default_value = "0000")]
    start_string: String,

    #[structopt(short = "n", long = "num_threads", default_value = "4")]
    num_threads: i32,

    #[structopt(short = "e", long = "end_string", default_value = "")]
    end_string: String,
}

fn args_checker(args: &Args) -> (String, String) {
    let start_string = args.start_string.clone();
    let end_string: String = args.end_string.clone();

    // Check if start_string and end_string are hex values
    if u64::from_str_radix(&start_string, 16).is_err() && !start_string.is_empty() {
       panic!("Invalid Starting String")
    };
    if u64::from_str_radix(&end_string, 16).is_err() && !end_string.is_empty() {
        panic!("Invalid Ending String");
    };

    return (start_string, end_string);
}

fn calculate_fifty_percent_probability(start_string: &String, end_string: &String) {
    let total_length = start_string.len() + end_string.len();
    let total_permutations = total_length.pow(16); // 16 hex values for each byte
    println!("It will take {} attempts to get a 50% chance of getting the desired address", total_permutations/2);
}

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
    let address_bytes = &hash[&hash.len() - 20..]; // last 20 bytes of the hash of public key
    hex::encode(address_bytes)
}

fn worker_thread(found: Arc<AtomicBool>, attempts: Arc<Mutex<u64>>, starting_string: &String, end_string: &String) {
    let secp = Secp256k1::new();
    let mut rng = OsRng;
    let mut local_attempts = 0_u64;

    while !found.load(Ordering::Relaxed) {
        local_attempts += 1;
        let (_, public_key, private_key) = generate_key_pair(&secp, &mut rng);
        let address = calculate_address(&public_key);
        let public_key_serialized = &public_key.serialize_uncompressed();
        let public_key_bytes = &public_key_serialized[1..];
        // update global pointer
        {
            let mut attempts_lock = attempts.lock().unwrap();
            *attempts_lock += local_attempts;
        }
        if address.starts_with(starting_string) && address.ends_with(end_string) {
            found.store(true, Ordering::Relaxed);
            println!("Found matching Ethereum address: 0x{}", address);
            let data = format!(
                "Address   : 0x{address}\n\
                 PrivateKey: {private_key}\n\
                 PublicKey : {public_key}",
                address = address,
                private_key = private_key,
                public_key = hex::encode(public_key_bytes)
            );
            let file_path = "vanity_address.txt";
            let mut file = File::create(file_path).expect("Unable to create file");
            file.write_all(data.as_bytes()).expect("Unable to write data");
            println!("Private Key stored at {}", file_path);
            break;
        }
    }
}

fn main() {
    let args = Args::from_args();
    let (start_string, end_string) = args_checker(&args);
    calculate_fifty_percent_probability(&start_string, &end_string);
    let found = Arc::new(AtomicBool::new(false));
    let attempts = Arc::new(Mutex::new(0));
    let num_threads = args.num_threads;

    let mut handles = vec![];
    println!("Starting to calculate address........");

    for _ in 0..num_threads {
        let found_clone = found.clone();
        let attempts_clone = attempts.clone();
        let starting_string_clone = start_string.clone();
        let ending_string_clone = end_string.clone();
        let handle = thread::spawn(move || {
            worker_thread(found_clone, attempts_clone, &starting_string_clone, &ending_string_clone);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let total_attempts = attempts.lock().unwrap();
    println!("Total Attempts: {}", *total_attempts);
}
