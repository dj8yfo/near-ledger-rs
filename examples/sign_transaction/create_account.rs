use near_ledger::NEARLedgerError;

#[path = "../common/lib.rs"]
mod common;

fn tx(ledger_pub_key: ed25519_dalek::PublicKey) -> near_primitives::transaction::Transaction {
    let mut tx = common::tx_template(ledger_pub_key);
    tx.actions = vec![near_primitives::transaction::Action::CreateAccount(
        near_primitives::transaction::CreateAccountAction {},
    )];
    tx
}

#[cfg(not(feature = "speculos"))]
fn main() -> Result<(), NEARLedgerError> {
    common::get_key_sign_and_verify_flow(tx)
}

#[cfg(feature = "speculos")]
fn main() -> Result<(), NEARLedgerError> {
    let expected = hex::decode("a2c1efe9c020858eb7429b8430b126059d6a6a0f1f2ec56b2364250d999aefa4b5c7ca957e652078cc94d2e02dce95de2d95b5d7867261d77ca2c8987d7ae209").unwrap();
    common::get_key_sign_and_verify_flow(tx, expected)
}
