use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};
use spl_token_2022::{
    instruction::{initialize_mint, initialize_non_transferable_mint, mint_to},
};

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey, // Public key of the account the hello world program was loaded into
    accounts: &[AccountInfo], // The account to say hello to
    instruction_data: &[u8], // Ignored, all helloworld instructions are hellos
) -> ProgramResult {
    let instruction = instruction_data[0];
    match instruction {
        0 => {
            msg!("Init");
            initialize_non_transferable_nft(program_id, accounts)?;
        }
        1 => {
            msg!("Mint");
            mint_non_transferable_nft(program_id, accounts, instruction_data)?;
        }
        _ => msg!("Invalid instruction"),
    }
    Ok(())
}

fn initialize_non_transferable_nft(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let mint_acc = next_account_info(accounts_iter)?;
    let rent_acc = next_account_info(accounts_iter)?;
    let freeze_authority = next_account_info(accounts_iter)?;

    // Initialize Non-Transferable Mint
    initialize_non_transferable_mint(program_id, mint_acc.key)?;

    // Initialize standard Mint
    initialize_mint(
        program_id,
        mint_acc.key,
        rent_acc.key,
        Some(freeze_authority.key),
        6,
    )?;
    msg!("Mint initialized");
    Ok(())
}

fn mint_non_transferable_nft(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let mint_acc = next_account_info(accounts_iter)?;
    let reciever_acc = next_account_info(accounts_iter)?;
    let mint_authority = next_account_info(accounts_iter)?;
    let mint_amount = u64::from_le_bytes(_instruction_data[1..9].try_into().unwrap());
    mint_to(
        program_id,
        mint_acc.key,
        reciever_acc.key,
        mint_authority.key,
        &[],
        mint_amount,
    )?;
    msg!("Minted {}", mint_amount);
    Ok(())
}
