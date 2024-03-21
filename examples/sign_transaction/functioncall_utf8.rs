use near_ledger::NEARLedgerError;
use near_primitives::transaction::FunctionCallAction;

#[path = "../common/lib.rs"]
mod common;

fn tx(ledger_pub_key: ed25519_dalek::PublicKey) -> near_primitives::transaction::Transaction {
    let mut tx = common::tx_template(ledger_pub_key.clone());
    
    // 2, 3 and 4 bytes
    let utf_mutibyte_chars = [('©', 2), ('ଔ', 3), ('🝙', 4)];

    let mut str = String::new();

    for i in 0..100 {
        let (char, bytes) = utf_mutibyte_chars[i%3];
        str.push_str(&format!("{}", bytes));
        str.push(char);
    }

    let args = format!("{{\"test_utf8_key\": \"{}\"}}", str);

    println!("args: {}", args);
    println!("args.len() {}", args.len());

    let f_call = FunctionCallAction {
        method_name: "test_payload_with_utf8_text".to_string(),
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
    let expected = hex::decode("cf094450a6fb2d5028fadc44e540df92bbdc9144ef9716558af75fec77ee0ec05fb7f63aca69f90a8b3908365e4242905827734198bf830a60b4c7498df6d80c").unwrap();
    common::get_key_sign_and_verify_flow(tx, expected)
}
