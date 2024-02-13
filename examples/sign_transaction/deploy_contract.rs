use near_ledger::NEARLedgerError;
use near_primitives::transaction::DeployContractAction;
use near_primitives_core::hash::CryptoHash;

#[path = "../common/lib.rs"]
mod common;

fn tx(ledger_pub_key: ed25519_dalek::PublicKey) -> near_primitives::transaction::Transaction {
    let mut tx = common::tx_template(ledger_pub_key);

    let code = core::iter::repeat(42u8).take(2000).collect::<Vec<_>>();

    let code_hash = CryptoHash::hash_bytes(&code);
    log::info!("Contract code hash: {:?}", code_hash);
    tx.actions = vec![near_primitives::transaction::Action::DeployContract(
        DeployContractAction { code },
    )];
    tx
}

#[cfg(not(feature = "speculos"))]
fn main() -> Result<(), NEARLedgerError> {
    common::get_key_sign_and_verify_flow(tx)
}

#[cfg(feature = "speculos")]
fn main() -> Result<(), NEARLedgerError> {
    let expected = hex::decode("d48d750cfc84fff62801dbd1e4899df3471b379dbba41decf38854c2c99971bba2256d77d6318f704a4c3351f692f85f78214f5e871500523de8698a3a7d9806").unwrap();
    common::get_key_sign_and_verify_flow(tx, expected)
}
