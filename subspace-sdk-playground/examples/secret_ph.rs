//! Generate secret phrase for an account on Subspace network

use sp_core::Pair;
use zeroize::Zeroizing;

#[tokio::main]
async fn main() {
    // generate new mnemonic
    let (_pair, phrase, seed): (
        sp_core::sr25519::Pair,
        String,
        <sp_core::sr25519::Pair as Pair>::Seed,
    ) = Pair::generate_with_phrase(None);

    let _seed = Zeroizing::new(seed);
    let phrase = Zeroizing::new(phrase);
    let _words: Vec<&str> = phrase.split_whitespace().collect();
    println!("Secret phrase:       {}", phrase.as_str());
}
