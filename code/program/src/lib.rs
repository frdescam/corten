use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let fees_payer = next_account_info(accounts_iter)?;

    // 1 → mint
    if instruction_data[0] == 1 {
        msg!("Minting 200 tokens to {}", fees_payer.key);
    }

    Ok(())
}
