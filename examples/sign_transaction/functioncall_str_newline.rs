use near_ledger::NEARLedgerError;
use near_primitives::transaction::FunctionCallAction;

#[path = "../common/lib.rs"]
mod common;

fn tx(ledger_pub_key: ed25519_dalek::PublicKey) -> near_primitives::transaction::Transaction {
    let mut tx = common::tx_template(ledger_pub_key.clone());

    let args = "{\"test_key\": \"value\nhidden part of value 1 2 3 4 5 6 7 8 9\"}";

    let f_call = FunctionCallAction {
        method_name: "test_payload_with_newline".to_string(),
        args: args.as_bytes().to_vec(),
        gas: 127127122121,
        deposit: 150000000000000000000000, // 0.15 NEAR,
    };

    println!("{:?}", args);

    tx.actions = vec![near_primitives::transaction::Action::FunctionCall(
        Box::new(f_call),
    )];
    tx
}

#[cfg(not(feature = "speculos"))]
fn main() -> Result<(), NEARLedgerError> {
    common::get_key_sign_and_verify_flow(tx)
}

#[cfg(feature = "speculos")]
fn main() -> Result<(), NEARLedgerError> {
    let expected = hex::decode("094a9c494bece0a9a06317b6e6e094f65f3ef4c564047aceea4078ecc4b897a2d2b3e0e7da8a8db408952e4ad02a4a93dcde89067521d6a295a5d76533fc6b0c").unwrap();
    common::get_key_sign_and_verify_flow(tx, expected)
}
