use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
    program_error::ProgramError,
};

entrypoint!(process_instruction);

fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let _fees_payer = next_account_info(accounts_iter)?;
    let mint_account = next_account_info(accounts_iter)?;

    // l'instruction '1' → mint
    if instruction_data[0] == 1 {
        msg!("Minting 200 tokens to {}", mint_account.key);

        // On encode 200 comme u64 et on l’écrit dans le data du compte
        let amount: u64 = 200;
        let amount_bytes = amount.to_le_bytes();

        // Vérifie que le compte est writable
        if !mint_account.is_writable {
            return Err(ProgramError::InvalidAccountData);
        }

        // On écrit dans le compte
        let data = &mut *mint_account.try_borrow_mut_data()?;
        data[..8].copy_from_slice(&amount_bytes);
    }

    Ok(())
}
