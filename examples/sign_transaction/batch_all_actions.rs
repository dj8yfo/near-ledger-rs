use std::str::FromStr;

use near_account_id::AccountId;
use near_crypto::SecretKey;
use near_ledger::NEARLedgerError;
use near_primitives::transaction::{DeployContractAction, FunctionCallAction};
use near_primitives_core::hash::CryptoHash;

#[path = "../common/lib.rs"]
mod common;

fn tx(ledger_pub_key: ed25519_dalek::PublicKey) -> near_primitives::transaction::Transaction {
    let mut tx = common::tx_template(ledger_pub_key);
    let cr_acc = near_primitives::transaction::Action::CreateAccount(
        near_primitives::transaction::CreateAccountAction {},
    );

    let del_acc_short = near_primitives::transaction::Action::DeleteAccount(
        near_primitives::transaction::DeleteAccountAction {
            beneficiary_id: AccountId::from_str("bob.near").unwrap(),
        },
    );
    let del_acc_long = near_primitives::transaction::Action::DeleteAccount(
        near_primitives::transaction::DeleteAccountAction {
            beneficiary_id: AccountId::new_unvalidated(
                "dc7e34eecec3096a4a661e10932834f801149c49dba9b93322f6d9de18047f9c1b11b3b31673033936ad07bddc01f9da27d974811e480fb197c799e23480a489".to_string()),
        },
    );

    let sk = SecretKey::from_seed(
        near_crypto::KeyType::ED25519,
        &format!("{:?}", ledger_pub_key),
    );
    let public_key_ed = sk.public_key();

    let del_key_ed = near_primitives::transaction::Action::DeleteKey(Box::new(
        near_primitives::transaction::DeleteKeyAction {
            public_key: public_key_ed,
        },
    ));

    let sk = SecretKey::from_seed(
        near_crypto::KeyType::SECP256K1,
        &format!("{:?}", ledger_pub_key),
    );
    let public_key_secp = sk.public_key();

    let del_key_secp = near_primitives::transaction::Action::DeleteKey(Box::new(
        near_primitives::transaction::DeleteKeyAction {
            public_key: public_key_secp.clone(),
        },
    ));

    let stake = near_primitives::transaction::Action::Stake(Box::new(
        near_primitives::transaction::StakeAction {
            stake: 1157130000000000000000000, // 1.15713 NEAR,
            public_key: public_key_secp.clone(),
        },
    ));

    let add_key_fullaccess = near_primitives::transaction::Action::AddKey(Box::new(
        near_primitives::transaction::AddKeyAction {
            public_key: public_key_secp.clone(),
            access_key: near_primitives_core::account::AccessKey {
                nonce: 127127127127,
                permission: near_primitives_core::account::AccessKeyPermission::FullAccess,
            },
        },
    ));

    let method_names = vec![
            "first_method", "saturating_add_signed", "iterator_chain_to_do_multiple_instances_of_an_operation_that_can_fail",
            "from_residual", "from_output", "unwrap_err_unchecked", "try_reserve_exact",
            "first_method", "saturating_add_signed", "iterator_chain_to_do_multiple_instances_of_an_operation_that_can_fail",
        ].into_iter().map(Into::into).collect::<Vec<_>>();

    let permission = near_primitives_core::account::FunctionCallPermission { 
        allowance: Some(150000000000000000000), 
        receiver_id: 
        "dc7e34eecec3096a4a661e10932834f801149c49dba9b93322f6d9de18047f9c1b11b3b31673033936ad07bddc01f9da27d974811e480fb197c799e23480a489".into(), 
        method_names,   
    };

    let add_key_funccall = near_primitives::transaction::Action::AddKey(Box::new(
        near_primitives::transaction::AddKeyAction {
            public_key: public_key_secp.clone(),
            access_key: near_primitives_core::account::AccessKey {
                nonce: 127127127127,
                permission: near_primitives_core::account::AccessKeyPermission::FunctionCall(permission),
            },
        },
    ));

    let transfer = near_primitives::transaction::Action::Transfer(
        near_primitives::transaction::TransferAction {
            deposit: 150000000000000000000000, // 0.15 NEAR
        },
    );

    let code = core::iter::repeat(42u8).take(3000).collect::<Vec<_>>();

    let code_hash = CryptoHash::hash_bytes(&code);
    log::info!("Contract code hash: {:?}", code_hash);

    let deploy_contract = near_primitives::transaction::Action::DeployContract(
        DeployContractAction { code },
    );

    let args = r#"{"previous_vesting_schedule_with_salt":{"vesting_schedule":{"start_timestamp":"1577919600000000000","cliff_timestamp":"1609455600000000000","end_timestamp":"1704150000000000000"},"salt":"7bc709c22801118b743fae3866edb4dea1630a97ab9cd67e993428b94a0f397a"}, "vesting_schedule_with_salt":{"vesting_schedule":{"start_timestamp":"1577919600000000000","cliff_timestamp":"1609455600000000000","end_timestamp":"1704150000000000000"},"salt":"7bc709c22801118b743fae3866edb4dea1630a97ab9cd67e993428b94a0f397a"}}"#;

    let f_call = FunctionCallAction {
        method_name: "saturating_add_signed".to_string(),
        args: args.as_bytes().to_vec(),
        gas: 127127122121,
        deposit: 150000000000000000000000, // 0.15 NEAR,
    };

    let f_call_str = near_primitives::transaction::Action::FunctionCall(
        Box::new(f_call),
    );

    let args = hex::decode("204f6e206f6c646572207465726d696e616c732c2074686520756e64657273636f726520636f646520697320646973706c617965642061732061206c6566740a202020202020206172726f772c2063616c6c6564206261636b6172726f772c2074686520636172657420697320646973706c6179656420617320616e2075702d6172726f770a20202020202020616e642074686520766572746963616c2062617220686173206120686f6c6520696e20746865206d6964646c652e0a0a2020202020202055707065726361736520616e64206c6f77657263617365206368617261637465727320646966666572206279206a757374206f6e652062697420616e64207468650a20202020202020415343494920636861726163746572203220646966666572732066726f6d2074686520646f75626c652071756f7465206279206a757374206f6e65206269742c0a20202020202020746f6f2e202054686174206d616465206974206d7563682065617369657220746f20656e636f64652063686172616374657273206d656368616e6963616c6c790a202020202020206f7220776974682061206e6f6e2d6d6963726f636f6e74726f6c6c65722d626173656420656c656374726f6e6963206b6579626f61726420616e6420746861740a2020202020202070616972696e672077617320666f756e64206f6e206f6c642074656c6574797065732e0a").unwrap();

    let f_call = FunctionCallAction {
        method_name: "saturating_add_signed".to_string(),
        args,
        gas: 127127122121,
        deposit: 150000000000000000000000, // 0.15 NEAR,
    };

    let f_call_bin = near_primitives::transaction::Action::FunctionCall(
        Box::new(f_call),
    );

    tx.actions = vec![
        cr_acc,
        del_acc_short,
        del_acc_long,
        del_key_ed,
        del_key_secp,
        stake,
        add_key_fullaccess,
        add_key_funccall,
        transfer,
        deploy_contract,
        f_call_str,
        f_call_bin,
    ];
    tx
}

fn main() -> Result<(), NEARLedgerError> {
    common::get_key_sign_and_verify_flow(tx)
}
