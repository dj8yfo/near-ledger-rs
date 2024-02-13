use near_crypto::SecretKey;
use near_ledger::NEARLedgerError;

#[path = "../common/lib.rs"]
mod common;

fn tx(ledger_pub_key: ed25519_dalek::PublicKey) -> near_primitives::transaction::Transaction {
    let mut tx = common::tx_template(ledger_pub_key.clone());
    let sk = SecretKey::from_seed(
        near_crypto::KeyType::ED25519,
        &format!("{:?}", ledger_pub_key),
    );
    let public_key = sk.public_key();
    tx.actions = vec![near_primitives::transaction::Action::DeleteKey(Box::new(
        near_primitives::transaction::DeleteKeyAction { public_key },
    ))];
    tx
}

#[cfg(not(feature = "speculos"))]
fn main() -> Result<(), NEARLedgerError> {
    common::get_key_sign_and_verify_flow(tx)
}

#[cfg(feature = "speculos")]
fn main() -> Result<(), NEARLedgerError> {
    let expected = hex::decode("62ae46a48cbd3ac193cc6e5fed7a33fb877f4c6a775eb283eb8813b1a0168ee768252cd2d2208073ddde78974c0e84ceca7cde95254629ee2e3eed8671be170f").unwrap();
    common::get_key_sign_and_verify_flow(tx, expected)
}
