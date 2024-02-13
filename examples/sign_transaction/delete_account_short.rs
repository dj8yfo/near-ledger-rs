use std::str::FromStr;

use near_account_id::AccountId;
use near_ledger::NEARLedgerError;

#[path = "../common/lib.rs"]
mod common;

fn tx(ledger_pub_key: ed25519_dalek::PublicKey) -> near_primitives::transaction::Transaction {
    let mut tx = common::tx_template(ledger_pub_key);
    tx.actions = vec![near_primitives::transaction::Action::DeleteAccount(
        near_primitives::transaction::DeleteAccountAction {
            beneficiary_id: AccountId::from_str("bob.near").unwrap(),
        },
    )];
    tx
}

#[cfg(not(feature = "speculos"))]
fn main() -> Result<(), NEARLedgerError> {
    common::get_key_sign_and_verify_flow(tx)
}

#[cfg(feature = "speculos")]
fn main() -> Result<(), NEARLedgerError> {
    let expected = hex::decode("4abe2be79a2120d66e2bd17b92e843cd94e1a661e115e728d5f0d317272590c2872b40b323d74642862ff4f0cd584e2d874c3a440e68dcb829cf60788c510905").unwrap();
    common::get_key_sign_and_verify_flow(tx, expected)
}
