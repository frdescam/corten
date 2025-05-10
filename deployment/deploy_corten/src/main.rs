use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    program_pack::Pack,
    signature::{read_keypair_file, Keypair},
    signer::Signer,
    system_instruction,
    transaction::Transaction,
};
use spl_associated_token_account::instruction::create_associated_token_account;
use spl_token::{instruction::initialize_mint, state::Mint, ID as TOKEN_PROGRAM_ID};

fn main() {
    let rpc_client = RpcClient::new("http://localhost:8899");
    let corten_wallet = read_keypair_file("/home/fdec/.config/solana/id.json").unwrap();
    let corten_mint = Keypair::new();
    let mint_rent = rpc_client
        .get_minimum_balance_for_rent_exemption(Mint::LEN)
        .unwrap();

    let create_mint_ix = system_instruction::create_account(
        &corten_wallet.pubkey(),
        &corten_mint.pubkey(),
        mint_rent,
        Mint::LEN as u64,
        &TOKEN_PROGRAM_ID,
    );

    let init_mint_ix = initialize_mint(
        &TOKEN_PROGRAM_ID,
        &corten_mint.pubkey(),
        &corten_wallet.pubkey(),
        None,
        6,
    )
    .unwrap();

    let create_ata_ix = create_associated_token_account(
        &corten_wallet.pubkey(),
        &corten_wallet.pubkey(),
        &corten_mint.pubkey(),
        &TOKEN_PROGRAM_ID,
    );

    let blockhash = rpc_client.get_latest_blockhash().unwrap();
    let tx = Transaction::new_signed_with_payer(
        &[create_mint_ix, init_mint_ix, create_ata_ix],
        Some(&corten_wallet.pubkey()),
        &[&corten_wallet, &corten_mint],
        blockhash,
    );

    rpc_client.send_and_confirm_transaction(&tx).unwrap();
    println!("\nMint account pubkey : {}\n", corten_mint.pubkey().to_string());
}
