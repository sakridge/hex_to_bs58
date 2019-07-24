use std::env;
use bs58;
use hex;
use std::process::exit;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("usage: hex_to_bs58 <hex>");
        exit(1);
    }
    let hex_str = &args[1];
    let x = hex::decode(hex_str).unwrap();
    println!("{}", bs58::encode(x).into_string());
}
