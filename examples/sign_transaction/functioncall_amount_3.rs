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
        deposit: (0.00001f64 * ONE_NEAR as f64).round() as u128, // 0.00001 NEAR,
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
    let expected = hex::decode("6c3bdf78ca23ea3c0c8797d90fb384293c4396487f2de5094b20e69232bf67a5a5da142e953ef4b7306aac53498befac0afa7b7877589651a711960712ea560b").unwrap();
    common::get_key_sign_and_verify_flow(tx, expected)
}
