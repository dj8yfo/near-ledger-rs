use near_account_id::AccountId;
use near_ledger::NEARLedgerError;

#[path = "../common/lib.rs"]
mod common;

#[allow(deprecated)]
fn tx(ledger_pub_key: ed25519_dalek::PublicKey) -> near_primitives::transaction::Transaction {
    let mut tx = common::tx_template(ledger_pub_key);
    tx.actions = vec![near_primitives::transaction::Action::DeleteAccount(
        near_primitives::transaction::DeleteAccountAction {
            beneficiary_id: AccountId::new_unvalidated(
                "dc7e34eecec3096a4a661e10932834f801149c49dba9b93322f6d9de18047f9c1b11b3b31673033936ad07bddc01f9da27d974811e480fb197c799e23480a489".to_string()),
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
    let expected = hex::decode("318ab694a182f1793d7796f8741fa399c44ef033fcebef23a57293a37f536e53a993ecd7b480763dc154d606d39bdae8f90f698a935e856e8c0114a6d9567009").unwrap();
    common::get_key_sign_and_verify_flow(tx, expected)
}
