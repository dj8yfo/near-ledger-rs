use std::{convert::TryInto, str::FromStr};

use near_account_id::AccountId;
use near_crypto::Signature;
use near_ledger::NEARLedgerError;
use near_primitives::action::delegate::{DelegateAction, SignedDelegateAction};
use slip10::BIP32Path;

use crate::common::display_pub_key;
#[cfg(feature = "speculos")]
use crate::common::static_speculos_public_key;

#[path = "./common/lib.rs"]
mod common;

fn main() -> Result<(), NEARLedgerError> {
    env_logger::builder().init();

    let hd_path = BIP32Path::from_str("44'/397'/0'/0'/1'").unwrap();
    #[cfg(not(feature = "speculos"))]
    let ledger_pub_key = near_ledger::get_public_key_with_display_flag(hd_path.clone(), false)?;
    #[cfg(feature = "speculos")]
    let ledger_pub_key = static_speculos_public_key();
    display_pub_key(ledger_pub_key);

    let sender_id = AccountId::from_str("bob.near").unwrap();

    let actions = common::batch_of_all_types_of_actions(ledger_pub_key)
        .into_iter()
        .map(|action| action.try_into().unwrap())
        .collect::<Vec<_>>();

    let ledger_pub_key = near_crypto::PublicKey::ED25519(near_crypto::ED25519PublicKey::from(
        ledger_pub_key.to_bytes(),
    ));

    let delegate_action = DelegateAction {
        sender_id,
        receiver_id: AccountId::from_str("alice.near").unwrap(),
        actions,
        nonce: 127127122121,
        max_block_height: 100500,
        public_key: ledger_pub_key,
    };
    log::info!("{:#?}", delegate_action);

    #[allow(unused)]
    let signature_bytes =
        near_ledger::sign_message_nep366_delegate_action(&delegate_action, hd_path)?;

    #[cfg(feature = "speculos")]
    let signature_bytes = hex::decode("3f671b1d2ba42132e78c39dad35848ef0ee67858d85bd63cb1ce9e03d629c74cec9529add083aa0de6dbd45d372baa67bd8f8e49e76297a87cec1dc7084ae80d").unwrap();

    let signature = Signature::from_parts(near_crypto::KeyType::ED25519, &signature_bytes).unwrap();

    let signed_delegate = SignedDelegateAction {
        delegate_action,
        signature,
    };
    assert!(signed_delegate.verify());

    common::display_signature(signature_bytes);
    Ok(())
}
