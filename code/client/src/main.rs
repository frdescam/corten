use std::{error::Error, str::FromStr};

use solana_client::rpc_client::RpcClient;
use solana_sdk::{instruction::Instruction, native_token::LAMPORTS_PER_SOL, pubkey::Pubkey, signature::{Keypair, Signature}, signer::Signer, transaction::Transaction};

pub fn create_keypair() -> Keypair {
    Keypair::new()
}

pub fn request_airdrop(rpc_client: &RpcClient, pubkey: &Pubkey, amount_sol: f64) -> Result<Signature, Box<dyn Error>> {
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

pub fn check_balance(rpc_client: &RpcClient, pubkey: &Pubkey) -> Result<f64, Box<dyn Error>> {
    Ok(rpc_client.get_balance(&pubkey)? as f64 / LAMPORTS_PER_SOL as f64)
}

fn main() {
    let rpc_client = RpcClient::new("http://localhost:8899");
    let keypair = create_keypair();

    request_airdrop(&rpc_client, &keypair.pubkey(), 1.0).unwrap();
    let balance = check_balance(&rpc_client, &keypair.pubkey()).unwrap();

    println!("Balance : {:.2}", balance);

    let program_id = Pubkey::from_str("FWEKEyaEyxeceM8Tr8YLMRmJsL9UwYejyeDMp7e8aJTn").unwrap();

    let instruction = Instruction::new_with_borsh(program_id, &(), vec![]);

    let mut transaction = Transaction::new_with_payer(&[instruction], Some(&keypair.pubkey()));
    transaction.sign(&[keypair], rpc_client.get_latest_blockhash().unwrap());

    match rpc_client.send_and_confirm_transaction(&transaction) {
        Ok(signature) => println!("Transaction Signature: {}", signature),
        Err(err) => eprintln!("Error sending transaction: {}", err)
    }

}
