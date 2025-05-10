use std::str::FromStr;

use borsh::to_vec;
use common::TokenInstructions;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::read_keypair_file,
    signer::Signer,
    transaction::Transaction,
};

use spl_associated_token_account::get_associated_token_address;
use spl_token::ID as TOKEN_PROGRAM_ID;

fn main() {
    let rpc_client = RpcClient::new("http://localhost:8899");
    let program_id = Pubkey::from_str("ArQLysRL8pKBBMVDid7wuZw4VGrkor5ir6ygi8GHUcCy").unwrap();
    let corten_wallet = read_keypair_file("/home/fdec/.config/solana/id.json").unwrap();
    let corten_mint = Pubkey::from_str("4tkP2MiygWAqJkrwXd5ENpHLXi2KiQ6ZckyQ8hYPF3j6").unwrap();

    let corten_ata = get_associated_token_address(&corten_wallet.pubkey(), &corten_mint);

    let instruction = Instruction {
        program_id,
        data: to_vec(&TokenInstructions::AskMint).unwrap(),
        accounts: vec![
            AccountMeta::new(corten_wallet.pubkey(), true),
            AccountMeta::new(corten_ata, false),
            AccountMeta::new(corten_mint, false),
            AccountMeta::new(TOKEN_PROGRAM_ID, false),
        ],
    };

    let mut transaction =
        Transaction::new_with_payer(&[instruction], Some(&corten_wallet.pubkey()));
    transaction.sign(
        &[&corten_wallet],
        rpc_client.get_latest_blockhash().unwrap(),
    );

    println!("Running transaction ...");

    match rpc_client.send_and_confirm_transaction(&transaction) {
        Ok(signature) => println!("Transaction Signature: {}", signature),
        Err(err) => eprintln!("Error sending transaction: {}", err),
    }

    println!(
        "Balance : {}",
        rpc_client
            .get_token_account_balance(&corten_ata)
            .unwrap()
            .ui_amount_string
    );
}
