use near_crypto::SecretKey;
use near_ledger::NEARLedgerError;

#[path = "../common/lib.rs"]
mod common;

fn tx(ledger_pub_key: ed25519_dalek::PublicKey) -> near_primitives::transaction::Transaction {
    let mut tx = common::tx_template(ledger_pub_key.clone());
    let sk = SecretKey::from_seed(
        near_crypto::KeyType::SECP256K1,
        &format!("{:?}", ledger_pub_key),
    );
    let public_key = sk.public_key();
    tx.actions = vec![near_primitives::transaction::Action::Stake(Box::new(
        near_primitives::transaction::StakeAction {
            stake: 1157130000000000000000000, // 1.15713 NEAR,
            public_key,
        },
    ))];
    tx
}

#[cfg(not(feature = "speculos"))]
fn main() -> Result<(), NEARLedgerError> {
    common::get_key_sign_and_verify_flow(tx)
}

#[cfg(feature = "speculos")]
fn main() -> Result<(), NEARLedgerError> {
    let expected = hex::decode("01832293252eb74d7a8a856f19b5a9087620292dd8a6ba8ee3104a1dc54618cbc05c0669079f1d27b8544724bd893f314288833583384419d0bece462e044003").unwrap();
    common::get_key_sign_and_verify_flow(tx, expected)
}
