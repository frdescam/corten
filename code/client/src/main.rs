use std::{error::Error, str::FromStr};

use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    native_token::LAMPORTS_PER_SOL,
    pubkey::Pubkey,
    signature::{Keypair, Signature, Signer},
    system_instruction,
    transaction::Transaction,
};

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
    let rpc_client = RpcClient::new("http://localhost:8899"); // interagir avec le noeud solana
    let fees_payer = create_keypair(); // celui qui va payer les frais
    let mint_account = create_keypair(); // le compte de mint

    // Program id
    let program_id = Pubkey::from_str("FWEKEyaEyxeceM8Tr8YLMRmJsL9UwYejyeDMp7e8aJTn")?;

    // Fees Airdrop
    request_airdrop(&rpc_client, &fees_payer.pubkey(), 3.0)?; // 3 SOL
    println!("Fees Payer balance: {:.2}", check_balance(&rpc_client, &fees_payer.pubkey())?); // affiche le solde du fees payer

    // eviter que le compte soit supprimer pcq y a pas assez de SOL
    let mint_account_space = 8; // taille minimale (en octet) pour un compte solana
    let mint_account_lamports = rpc_client.get_minimum_balance_for_rent_exemption(mint_account_space)?; // calcul le nombre minimal de Lamports pour eviter le loyer

    // instruction pour le mint
    let create_account_ix = system_instruction::create_account(
        &fees_payer.pubkey(),
        &mint_account.pubkey(),
        mint_account_lamports,
        mint_account_space as u64,
        &Pubkey::from_str("FWEKEyaEyxeceM8Tr8YLMRmJsL9UwYejyeDMp7e8aJTn")?, // ID du programme
    );

    // construction du mint_instruction
    let mint_instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(fees_payer.pubkey(), true), // fees_payer = signer
            AccountMeta::new(mint_account.pubkey(), false), // mint_account = cible du mint
        ],
        data: vec![1], // on envoie juste 1 byte: 1 → qui correspond à l’instruction mint
    };
    
    // creation de l'instruction
    let mut transaction = Transaction::new_with_payer(
        &[create_account_ix, mint_instruction],
        Some(&fees_payer.pubkey()),
    );
    
    // signature par celui qui paye les frais
    let recent_blockhash = rpc_client.get_latest_blockhash()?;
    transaction.sign(&[&fees_payer, &mint_account], recent_blockhash);
    
    // confirmation de la signature par le reseau
    let signature = rpc_client.send_and_confirm_transaction(&transaction)?;
    println!("✅ Transaction sent! Signature: {}", signature);
    
    Ok(())

}
