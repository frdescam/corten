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

fn main() -> Result<(), Box<dyn Error>> {
    let rpc_client = RpcClient::new("http://localhost:8899");

    

    

    Ok(())
}
