use mpl_token_auth_rules::{
    payload::{PayloadKey, PayloadType},
    state::{Operation, Rule, RuleSet},
};
use rmp_serde::Serializer;
use serde::Serialize;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    native_token::LAMPORTS_PER_SOL, signature::Signer, signer::keypair::Keypair,
    transaction::Transaction,
};
use std::collections::HashMap;

fn main() {
    // let url = "http://127.0.0.1:8899".to_string();
    let url = "https://api.devnet.solana.com".to_string();

    let rpc_client = RpcClient::new(url);

    let payer = Keypair::new();

    let signature = rpc_client
        .request_airdrop(&payer.pubkey(), LAMPORTS_PER_SOL)
        .unwrap();

    loop {
        let confirmed = rpc_client.confirm_transaction(&signature).unwrap();
        if confirmed {
            break;
        }
    }

    // Find RuleSet PDA.
    let (ruleset_addr, _ruleset_bump) = mpl_token_auth_rules::pda::find_rule_set_address(
        payer.pubkey(),
        "test ruleset".to_string(),
    );

    // Second signer.
    let second_signer = Keypair::new();

    // Create some rules.
    let adtl_signer = Rule::AdditionalSigner {
        account: payer.pubkey(),
    };
    let adtl_signer2 = Rule::AdditionalSigner {
        account: second_signer.pubkey(),
    };
    let amount_check = Rule::Amount { amount: 2 };

    let first_rule = Rule::All {
        rules: vec![adtl_signer, adtl_signer2],
    };

    let overall_rule = Rule::All {
        rules: vec![first_rule, amount_check],
    };

    // Create a RuleSet.
    let mut rule_set = RuleSet::new();
    rule_set.add(Operation::Transfer, overall_rule);

    println!("{:#?}", rule_set);

    // Serialize the RuleSet using RMP serde.
    let mut serialized_data = Vec::new();
    rule_set
        .serialize(&mut Serializer::new(&mut serialized_data))
        .unwrap();

    // Create a `create` instruction.
    let create_ix = mpl_token_auth_rules::instruction::create(
        mpl_token_auth_rules::id(),
        payer.pubkey(),
        ruleset_addr,
        "test ruleset".to_string(),
        serialized_data,
    );

    // Add it to a transaction.
    let latest_blockhash = rpc_client.get_latest_blockhash().unwrap();
    let create_tx = Transaction::new_signed_with_payer(
        &[create_ix],
        Some(&payer.pubkey()),
        &[&payer],
        latest_blockhash,
    );

    // Send and confirm transaction.
    let signature = rpc_client.send_and_confirm_transaction(&create_tx).unwrap();

    println!("Create tx signature: {}", signature);

    // Store the payload of data to validate against the rule definition.
    let payload = HashMap::from([(PayloadKey::Amount, PayloadType::Number(2))]);

    // Create a `validate` instruction.
    let validate_ix = mpl_token_auth_rules::instruction::validate(
        mpl_token_auth_rules::id(),
        payer.pubkey(),
        ruleset_addr,
        "test ruleset".to_string(),
        Operation::Transfer,
        payload,
        vec![second_signer.pubkey()],
        vec![],
    );

    // Add it to a transaction.
    let latest_blockhash = rpc_client.get_latest_blockhash().unwrap();
    let validate_tx = Transaction::new_signed_with_payer(
        &[validate_ix],
        Some(&payer.pubkey()),
        &[&payer, &second_signer],
        latest_blockhash,
    );

    // Send and confirm transaction.
    let signature = rpc_client
        .send_and_confirm_transaction(&validate_tx)
        .unwrap();

    println!("Validate tx signature: {}", signature);
}
