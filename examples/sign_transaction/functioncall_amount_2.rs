use near_ledger::NEARLedgerError;
use near_primitives::transaction::FunctionCallAction;

#[path = "../common/lib.rs"]
mod common;

const ONE_NEAR: u128 = 10_u128.pow(24);

fn tx(ledger_pub_key: ed25519_dalek::PublicKey) -> near_primitives::transaction::Transaction {
    let mut tx = common::tx_template(ledger_pub_key.clone());

    let args = r#"{"key":"value"}"#;

    let f_call = FunctionCallAction {
        method_name: "saturating_call_signed".to_string(),
        args: args.as_bytes().to_vec(),
        gas: 127127122121,
        deposit: 1_000_000 * ONE_NEAR, // 1 M NEAR,
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
    let expected = hex::decode("f2f7fef77b3ac0371b05586a831d022783f7edbec25c40cf65894fcc53db1fdefede704ce8132980f9038ef2e5832c8b9377c68dac820e08743ff77554965a0f").unwrap();
    common::get_key_sign_and_verify_flow(tx, expected)
}
