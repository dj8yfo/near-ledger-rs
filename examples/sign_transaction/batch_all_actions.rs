use near_ledger::NEARLedgerError;

#[path = "../common/lib.rs"]
mod common;

fn tx(ledger_pub_key: ed25519_dalek::PublicKey) -> near_primitives::transaction::Transaction {
    let mut tx = common::tx_template(ledger_pub_key.clone());
    tx.actions = common::batch_of_all_types_of_actions(ledger_pub_key);
    tx
}

#[cfg(not(feature = "speculos"))]
fn main() -> Result<(), NEARLedgerError> {
    common::get_key_sign_and_verify_flow(tx)
}

#[cfg(feature = "speculos")]
fn main() -> Result<(), NEARLedgerError> {
    let expected = hex::decode("bd5420d0279f398893231b505b004403c682c8ef8e2b5181d0b007dfbc802dacfadbd20883938a236ccd78f388d2b52b522574d2a3c682c380c814cbf6ccad02").unwrap();
    common::get_key_sign_and_verify_flow(tx, expected)
}
