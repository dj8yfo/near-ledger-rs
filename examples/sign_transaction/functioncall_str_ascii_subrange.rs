
use near_ledger::NEARLedgerError;
use near_primitives::transaction::FunctionCallAction;

#[path = "../common/lib.rs"]
mod common;

fn tx(ledger_pub_key: ed25519_dalek::PublicKey) -> near_primitives::transaction::Transaction {
    let mut tx = common::tx_template(ledger_pub_key.clone());

    let mut args: String = String::new();
    args.push('{');
    args.push('"');
    for char_code in 0x20u8..=127 {
        let c = char::from(char_code);
        args.push(c);
        
    }
    args.push('"');
    args.push('}');

    let f_call = FunctionCallAction {
        method_name: "test_payload_str_with_ascii_subrange".to_string(),
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
    let expected = hex::decode("3e952a449a50a655c25e4ccad66e29f0e3f361805402e6687de19583eca37f5440570796c6efc9ccaf29261faf766e920f94f63491e480ddcf3994ebf5e2b706").unwrap();
    common::get_key_sign_and_verify_flow(tx, expected)
}
