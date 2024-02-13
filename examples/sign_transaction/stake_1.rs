use near_crypto::SecretKey;
use near_ledger::NEARLedgerError;

#[path = "../common/lib.rs"]
mod common;

const ONE_NEAR: u128 = 10_u128.pow(24);

fn tx(ledger_pub_key: ed25519_dalek::PublicKey) -> near_primitives::transaction::Transaction {
    let mut tx = common::tx_template(ledger_pub_key.clone());
    let sk = SecretKey::from_seed(
        near_crypto::KeyType::SECP256K1,
        &format!("{:?}", ledger_pub_key),
    );
    let public_key = sk.public_key();
    tx.actions = vec![near_primitives::transaction::Action::Stake(Box::new(
        near_primitives::transaction::StakeAction {
            stake: 1_000_000 * ONE_NEAR, // 1M NEAR,
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
    let expected = hex::decode("02f7cdb55e55b8a7efe9d87b6a7971b4fc816c2a1e17262a544a12a599789d1e457fb40bef286f9dcaf709f751e7869a19fcccec9b2156e293ee1f6595a23406").unwrap();
    common::get_key_sign_and_verify_flow(tx, expected)
}
