use std::{error::Error, str::FromStr};

use solana_client::rpc_client::RpcClient;

use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    native_token::LAMPORTS_PER_SOL,
    pubkey::Pubkey,
    signature::{Keypair, Signature, Signer},
    system_program,
    transaction::Transaction,
};

pub fn create_keypair() -> Keypair {
    Keypair::new()
}

pub fn request_airdrop(
    rpc_client: &RpcClient,
    pubkey: &Pubkey,
    amount_sol: f64,
) -> Result<Signature, Box<dyn Error>> {
    let lamports = (amount_sol * LAMPORTS_PER_SOL as f64) as u64;
    let sig = rpc_client.request_airdrop(&pubkey, lamports)?;

    loop {
        let confirmed = rpc_client.confirm_transaction(&sig)?;

        if confirmed {
            break;
        }
    }
    Ok(sig)
}

pub fn check_balance(
    rpc_client: &RpcClient,
    pubkey: &Pubkey,
) -> Result<f64, Box<dyn Error>> {
    Ok(rpc_client.get_balance(&pubkey)? as f64 / LAMPORTS_PER_SOL as f64)
}

fn main() -> Result<(), Box<dyn Error>> {
    let rpc_client = RpcClient::new("http://localhost:8899");

    let sender_keypair = create_keypair(); //sender
    let receiver_keypair = create_keypair(); //receiver
    let receiver_pubkey = receiver_keypair.pubkey();

    // Requesting airdrop for the sender
    request_airdrop(&rpc_client, &sender_keypair.pubkey(), 1.0)?;

    // Requesting airdrop for the receiver
    request_airdrop(&rpc_client, &receiver_pubkey, 1.0)?;

    // Printing the balances
    let sender_balance = check_balance(&rpc_client, &sender_keypair.pubkey())?;
    let receiver_balance = check_balance(&rpc_client, &receiver_pubkey)?;
    println!("Sender Balance : {:.2}", sender_balance);
    println!("Receiver Balance : {:.2}", receiver_balance);

    // Buffer for the data to be passed to the transfer instruction
    let mut instruction_data = Vec::with_capacity(4 + 8); // u32 + u64

    // SOL to transfer
    let lamports_to_send: u64 = (0.1 * LAMPORTS_PER_SOL as f64) as u64;

    // Instruction Index
    let transfer_instruction_index: [u8; 4] = 2u32.to_le_bytes(); // Convertion in Little Endian

    // Add to the buffer
    instruction_data.extend_from_slice(&transfer_instruction_index); // Add index to buffer
    instruction_data.extend_from_slice(&lamports_to_send.to_le_bytes()); // Convert lamports in Little Endian then add it to buffer
    println!("Data for transfer: {:?}", instruction_data);

    // Create manually the instruction to transfer SOL
    let instruction = Instruction {
        program_id: system_program::id(),
        accounts: vec![
            AccountMeta::new(sender_keypair.pubkey(), true),
            AccountMeta::new(receiver_pubkey, false),
        ],
        data: instruction_data,
    };

    // Create manually the transaction
    let recent_blockhash = rpc_client.get_latest_blockhash()?;
    let mut transaction =
        Transaction::new_with_payer(&[instruction], Some(&sender_keypair.pubkey()));
    transaction.sign(&[&sender_keypair], recent_blockhash);

    let signature = rpc_client.send_and_confirm_transaction(&transaction)?;
    println!("✅ Transaction envoyée ! Signature : {}", signature);

    Ok(())
}
