use mpl_token_auth_rules::{
    instruction::{
        builders::{CreateOrUpdateBuilder, WriteToBufferBuilder},
        CreateOrUpdateArgs, InstructionBuilder, WriteToBufferArgs,
    },
    state::{CompareOp, Rule, RuleSetV1},
};
use num_derive::ToPrimitive;
use rmp_serde::Serializer;
use serde::Serialize;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    native_token::LAMPORTS_PER_SOL, pubkey, pubkey::Pubkey, signature::Signer,
    signer::keypair::Keypair, transaction::Transaction,
};
use std::fmt::Display;
use std::fs;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TransferScenario {
    Holder,
    TransferDelegate,
    SaleDelegate,
    MigrationDelegate,
    WalletToWallet,
}

impl Display for TransferScenario {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Holder => write!(f, "Owner"),
            Self::TransferDelegate => write!(f, "TransferDelegate"),
            Self::SaleDelegate => write!(f, "SaleDelegate"),
            Self::MigrationDelegate => write!(f, "MigrationDelegate"),
            Self::WalletToWallet => write!(f, "WalletToWallet"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UpdateScenario {
    MetadataAuth,
    Delegate,
    Proxy,
}

impl Display for UpdateScenario {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UpdateScenario::MetadataAuth => write!(f, "MetadataAuth"),
            UpdateScenario::Delegate => write!(f, "Delegate"),
            UpdateScenario::Proxy => write!(f, "Proxy"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Operation {
    Transfer { scenario: TransferScenario },
    Update { scenario: UpdateScenario },
}

impl ToString for Operation {
    fn to_string(&self) -> String {
        match self {
            Self::Transfer { scenario } => format!("Transfer:{}", scenario),
            Self::Update { scenario } => format!("Update:{}", scenario),
        }
    }
}

fn main() {
    let url = "https://api.devnet.solana.com".to_string();
    let rpc_client = RpcClient::new(url);
    let payer = read_keypair(&("keypair/devnet-test-rule-set.json".to_string()));
    let signature = rpc_client
        .request_airdrop(&payer.pubkey(), LAMPORTS_PER_SOL)
        .unwrap();

    loop {
        let confirmed = rpc_client.confirm_transaction(&signature).unwrap();
        if confirmed {
            break;
        }
    }

    // --------------------------------
    // Create RuleSet
    // --------------------------------
    // Find RuleSet PDA.
    let rule_set_name = "Metaplex Royalty RuleSet Dev".to_string();
    let (rule_set_addr, _ruleset_bump) =
        mpl_token_auth_rules::pda::find_rule_set_address(payer.pubkey(), rule_set_name.clone());
    println!("{}: {}", rule_set_name, rule_set_addr);

    // Create a RuleSet.
    let mut royalty_rule_set = RuleSetV1::new(rule_set_name, payer.pubkey());
    let (transfer_rule, wallet_to_wallet_rule) = get_rules();

    let owner_operation = Operation::Transfer {
        scenario: TransferScenario::Holder,
    };

    let transfer_delegate_operation = Operation::Transfer {
        scenario: TransferScenario::TransferDelegate,
    };

    let sale_delegate_operation = Operation::Transfer {
        scenario: TransferScenario::SaleDelegate,
    };

    let migration_delegate_operation = Operation::Transfer {
        scenario: TransferScenario::MigrationDelegate,
    };

    let wallet_to_wallet_operation = Operation::Transfer {
        scenario: TransferScenario::WalletToWallet,
    };

    royalty_rule_set
        .add(owner_operation.to_string(), transfer_rule.clone())
        .unwrap();
    royalty_rule_set
        .add(
            transfer_delegate_operation.to_string(),
            transfer_rule.clone(),
        )
        .unwrap();
    royalty_rule_set
        .add(sale_delegate_operation.to_string(), transfer_rule.clone())
        .unwrap();
    royalty_rule_set
        .add(migration_delegate_operation.to_string(), transfer_rule)
        .unwrap();
    royalty_rule_set
        .add(
            wallet_to_wallet_operation.to_string(),
            wallet_to_wallet_rule,
        )
        .unwrap();

    println!("{:#?}", royalty_rule_set);

    // Serialize the RuleSet using RMP serde.
    let mut serialized_rule_set = Vec::new();
    royalty_rule_set
        .serialize(&mut Serializer::new(&mut serialized_rule_set))
        .unwrap();

    // We need to write this RuleSet in chunks.
    let (buffer_pda, _buffer_bump) = mpl_token_auth_rules::pda::find_buffer_address(payer.pubkey());

    let mut overwrite = true;
    for serialized_rule_set_chunk in serialized_rule_set.chunks(500) {
        // Create a `write to buffer` instruction.
        let buffer_ix = WriteToBufferBuilder::new()
            .payer(payer.pubkey())
            .buffer_pda(buffer_pda)
            .build(WriteToBufferArgs::V1 {
                serialized_rule_set: serialized_rule_set_chunk.to_vec(),
                overwrite,
            })
            .unwrap()
            .instruction();

        // Add it to a transaction.
        let latest_blockhash = rpc_client.get_latest_blockhash().unwrap();
        let buffer_tx = Transaction::new_signed_with_payer(
            &[buffer_ix],
            Some(&payer.pubkey()),
            &[&payer],
            latest_blockhash,
        );

        println!("TX Length: {:?}", buffer_tx.message.serialize().len());
        assert!(
            buffer_tx.message.serialize().len() <= 1232,
            "Transaction exceeds packet limit of 1232"
        );

        // Send and confirm transaction.
        let signature = rpc_client.send_and_confirm_transaction(&buffer_tx).unwrap();
        println!("Buffer tx signature: {}", signature);

        if overwrite {
            overwrite = false;
        }
    }

    // Create a `create` instruction.
    let create_ix = CreateOrUpdateBuilder::new()
        .payer(payer.pubkey())
        .rule_set_pda(rule_set_addr)
        .buffer_pda(buffer_pda)
        .build(CreateOrUpdateArgs::V1 {
            serialized_rule_set: vec![],
        })
        .unwrap()
        .instruction();

    // Add it to a transaction.
    let latest_blockhash = rpc_client.get_latest_blockhash().unwrap();
    let create_tx = Transaction::new_signed_with_payer(
        &[create_ix],
        Some(&payer.pubkey()),
        &[&payer],
        latest_blockhash,
    );

    assert!(
        create_tx.message.serialize().len() <= 1232,
        "Transaction exceeds packet limit of 1232"
    );

    // Send and confirm transaction.
    let signature = rpc_client.send_and_confirm_transaction(&create_tx).unwrap();
    println!("Create tx signature: {}", signature);
}

pub fn read_keypair(path: &String) -> Keypair {
    let secret_string: String = fs::read_to_string(path).expect("Could not get path from string");

    // Try to decode the secret string as a JSON array of ints first and then as a base58 encoded string to support Phantom private keys.
    let secret_bytes: Vec<u8> = match serde_json::from_str(&secret_string) {
        Ok(bytes) => bytes,
        Err(_) => panic!("Could not deserialize string"),
    };

    Keypair::from_bytes(&secret_bytes).unwrap()
}
const ROOSTER_PROGRAM_ID: Pubkey = pubkey!("Roostrnex2Z9Y2XZC49sFAdZARP8E4iFpEnZC5QJWdz");
const TOKEN_METADATA_PROGRAM_ID: Pubkey = pubkey!("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");
const PROGRAM_ALLOW_LIST: [Pubkey; 2] = [TOKEN_METADATA_PROGRAM_ID, ROOSTER_PROGRAM_ID];

#[derive(Debug, Clone, ToPrimitive)]
pub enum PayloadKey {
    Amount,
    Authority,
    AuthoritySeeds,
    Delegate,
    DelegateSeeds,
    Destination,
    DestinationSeeds,
    Holder,
    Source,
    SourceSeeds,
}

impl ToString for PayloadKey {
    fn to_string(&self) -> String {
        match self {
            PayloadKey::Amount => "Amount",
            PayloadKey::Authority => "Authority",
            PayloadKey::AuthoritySeeds => "AuthoritySeeds",
            PayloadKey::Delegate => "Delegate",
            PayloadKey::DelegateSeeds => "DelegateSeeds",
            PayloadKey::Destination => "Destination",
            PayloadKey::DestinationSeeds => "DestinationSeeds",
            PayloadKey::Holder => "Holder",
            PayloadKey::Source => "Source",
            PayloadKey::SourceSeeds => "SourceSeeds",
        }
        .to_string()
    }
}

macro_rules! get_primitive_rules {
    (
        $nft_amount:ident,
        $source_program_allow_list:ident,
        $source_pda_match:ident,
        $dest_program_allow_list:ident,
        $dest_pda_match:ident,
        $source_is_wallet:ident,
        $dest_is_wallet:ident
    ) => {
        let $nft_amount = Rule::Amount {
            field: PayloadKey::Amount.to_string(),
            amount: 1,
            operator: CompareOp::Eq,
        };

        let $source_program_allow_list = Rule::ProgramOwnedList {
            programs: PROGRAM_ALLOW_LIST.to_vec(),
            field: PayloadKey::Source.to_string(),
        };

        let $source_pda_match = Rule::PDAMatch {
            program: None,
            pda_field: PayloadKey::Source.to_string(),
            seeds_field: PayloadKey::SourceSeeds.to_string(),
        };

        let $dest_program_allow_list = Rule::ProgramOwnedList {
            programs: PROGRAM_ALLOW_LIST.to_vec(),
            field: PayloadKey::Destination.to_string(),
        };

        let $dest_pda_match = Rule::PDAMatch {
            program: None,
            pda_field: PayloadKey::Destination.to_string(),
            seeds_field: PayloadKey::DestinationSeeds.to_string(),
        };

        let $source_is_wallet = Rule::IsWallet {
            field: PayloadKey::Source.to_string(),
        };

        let $dest_is_wallet = Rule::IsWallet {
            field: PayloadKey::Destination.to_string(),
        };
    };
}

fn get_rules() -> (Rule, Rule) {
    get_primitive_rules!(
        nft_amount,
        source_program_allow_list,
        source_pda_match,
        dest_program_allow_list,
        dest_pda_match,
        source_is_wallet,
        dest_is_wallet
    );

    // --------------------------------
    // Create Rules
    // --------------------------------
    // (source is on allow list && source is a PDA && amount is 1) ||
    // (dest is on allow list && dest is a PDA && amount is 1)
    let transfer_rule = Rule::Any {
        rules: vec![
            Rule::All {
                rules: vec![
                    source_program_allow_list,
                    source_pda_match,
                    nft_amount.clone(),
                ],
            },
            Rule::All {
                rules: vec![dest_program_allow_list, dest_pda_match, nft_amount.clone()],
            },
        ],
    };

    // (source is wallet && dest is wallet && amount is 1)
    let wallet_to_wallet_rule = Rule::All {
        rules: vec![source_is_wallet, dest_is_wallet, nft_amount],
    };

    (transfer_rule, wallet_to_wallet_rule)
}
