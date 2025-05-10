use borsh::BorshDeserialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use spl_token::instruction::mint_to;

use common::TokenInstructions;

entrypoint!(process_instruction);

fn process_instruction<'a>(
    program_id: &Pubkey,
    accounts: &'a [AccountInfo<'a>],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let corten_wallet = next_account_info(accounts_iter)?; // Signer, user account
    let corten_ata = next_account_info(accounts_iter)?; // User corten account
    let corten_mint = next_account_info(accounts_iter)?; // Account that stores token meta data
    let spl_token_program = next_account_info(accounts_iter)?; // Official token program

    let instruction = TokenInstructions::try_from_slice(instruction_data)?;

    match instruction {
        TokenInstructions::AskMint { amount } =>
            mint_tokens(program_id, corten_wallet, corten_ata, corten_mint, spl_token_program, amount),
        TokenInstructions::Instruction2 => {
            msg!("Instruction 2");
            Ok(())
        }
        TokenInstructions::Instruction3 => {
            msg!("Instruction 3");
            Ok(())
        }
    }
}

fn mint_tokens<'a>(
    program_id: &Pubkey,
    corten_wallet: &'a AccountInfo<'a>,
    corten_ata: &'a AccountInfo<'a>,
    corten_mint: &'a AccountInfo<'a>,
    spl_token_program: &'a AccountInfo<'a>,
    amount: u64
) -> Result<(), ProgramError> {
    let instruction = mint_to(
        spl_token_program.key,
        corten_mint.key,
        corten_ata.key,
        corten_wallet.key,
        &[],
        amount,
    )?;

    let signer_seeds: &[&[&[u8]]] = &[&[
        b"mint_authority",
        &[mint_authority_lamport_bump(program_id)?],
    ]];

    invoke_signed(
        &instruction,
        &[
            corten_wallet.clone(),
            corten_ata.clone(),
            corten_mint.clone(),
            spl_token_program.clone(),
        ],
        signer_seeds,
    )?;
    Ok(())
}

// A dummy PDA bump calculation function (replace with real one during deployment)
fn mint_authority_lamport_bump(program_id: &Pubkey) -> Result<u8, ProgramError> {
    let (_pda, bump) = Pubkey::find_program_address(&[b"mint_authority"], program_id);
    Ok(bump)
}
