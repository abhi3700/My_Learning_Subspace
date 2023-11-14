//! Generate keypair from randomly generated secret phrase for Subspace network

use sp_core::Pair;
use zeroize::Zeroizing;

#[tokio::main]
async fn main() {
    // generate new mnemonic and key pair
    let (pair, phrase, seed): (
        sp_core::sr25519::Pair,
        String,
        <sp_core::sr25519::Pair as Pair>::Seed,
    ) = Pair::generate_with_phrase(None);

    // Zeroize the seed and phrase for security
    let _seed = Zeroizing::new(seed);
    let phrase = Zeroizing::new(phrase);
    let _words: Vec<&str> = phrase.split_whitespace().collect();

    println!("Secret phrase:       {}", phrase.as_str());
    println!("  Network ID:        substrate");

    // NOTE: private/secret key is not public for security. But, can be implemented
    // for requirement using different lib. like 'subkey'
    // let priv_key_hex = format!("0x{}", hex::encode(pair.0.secret));
    // println!("  Secret seed:       {}", priv_key_hex);

    let pub_key_ss58 = pair.public();
    let pub_key_hex = format!("0x{}", hex::encode(pub_key_ss58.to_vec()));

    println!("  Public key (hex):  {}", pub_key_hex);
    println!("  Account ID:        {}", pub_key_hex);

    println!("  Public key (SS58): {}", pub_key_ss58);
    println!("  SS58 Address:      {}", pub_key_ss58);
}
