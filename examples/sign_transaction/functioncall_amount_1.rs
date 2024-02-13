use near_ledger::NEARLedgerError;
use near_primitives::transaction::FunctionCallAction;

#[path = "../common/lib.rs"]
mod common;

fn tx(ledger_pub_key: ed25519_dalek::PublicKey) -> near_primitives::transaction::Transaction {
    let mut tx = common::tx_template(ledger_pub_key.clone());

    let args = r#"{"key":"value"}"#;

    let f_call = FunctionCallAction {
        method_name: "saturating_call_signed".to_string(),
        args: args.as_bytes().to_vec(),
        gas: 127127122121,
        deposit: 1, // 0.15 NEAR,
    };

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
    let expected = hex::decode("19dfc1496e17b979a79b5442eed52ab79112eaf35d98638fdd56c6bbdf3c327a8cf1f1a6702e93db757200dfcef18ac2b62db1bf3f6f6c1772be60b6a44d8004").unwrap();
    common::get_key_sign_and_verify_flow(tx, expected)
}
