use near_ledger::NEARLedgerError;
use near_primitives::transaction::FunctionCallAction;

#[path = "../common/lib.rs"]
mod common;

fn tx(ledger_pub_key: ed25519_dalek::PublicKey) -> near_primitives::transaction::Transaction {
    let mut tx = common::tx_template(ledger_pub_key.clone());

    let args = r#"{"previous_vesting_schedule_with_salt":{"vesting_schedule":{"start_timestamp":"1577919600000000000","cliff_timestamp":"1609455600000000000","end_timestamp":"1704150000000000000"},"salt":"7bc709c22801118b743fae3866edb4dea1630a97ab9cd67e993428b94a0f397a"}, "vesting_schedule_with_salt":{"vesting_schedule":{"start_timestamp":"1577919600000000000","cliff_timestamp":"1609455600000000000","end_timestamp":"1704150000000000000"},"salt":"7bc709c22801118b743fae3866edb4dea1630a97ab9cd67e993428b94a0f397a"}}"#;

    let f_call = FunctionCallAction {
        method_name: "saturating_add_signed".to_string(),
        args: args.as_bytes().to_vec(),
        gas: 127127122121,
        deposit: 150000000000000000000000, // 0.15 NEAR,
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
    let expected = hex::decode("e53a9694b09b0470fe72eb0531793d70ac2d8f0bd54e12d353a91a70d1413b534bfc28feb5bb78ec57a7e13600442d3ef55ee9d0fc72de1519f3e7edc0eb5306").unwrap();
    common::get_key_sign_and_verify_flow(tx, expected)
}
