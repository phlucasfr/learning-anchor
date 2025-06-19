use anchor_lang::prelude::*;

// No Optional Seeds
#[derive(Accounts)]
pub struct InstructionAccounts<'info> {
    #[account(
        seeds = [], // Use an empty array [] to define a PDA without optional seeds.
        bump,
    )]
    pub pda_account: SystemAccount<'info>,
}

// Single Static Seed
#[derive(Accounts)]
pub struct InstructionAccountsSingleStaticSeed<'info> {
    #[account(
        seeds = [b"hello_world"], // Specify optional seeds in the seeds constraint.
        bump,
    )]
    pub pda_account: SystemAccount<'info>,
}

// Multiple Seeds an Account References
#[derive(Accounts)]
pub struct InstructionAccountsMultiple<'info> {
    pub signer: Signer<'info>,
    #[account(
        seeds = [b"hello world", signer.key().as_ref()], // Multiple seeds can be specified in the seeds constraint. The seeds constraint can also reference other account addresses or account data.
        bump, //When using the bump constraint without a value, the bump is automatically calculated each time the instruction is invoked.
    )]
    pub pda_account: SystemAccount<'info>,
}

// By storing the bump value in the account's data, the program doesn't need to recalculate it, saving compute units.
// The saved bump value can be stored on the account itself or another account.
#[derive(Accounts)]
pub struct InstructionAccountsWithSpecifiedBump<'info> {
    #[account(
        seeds = [b"hello world"],
        bump = pda_account.bump_seed
    )]
    pub pda_account: Account<'info, CustomAccount>, // This assumes that the PDA account has been created and the bump seed is stored as a field on an existing account.
}

#[account]
pub struct CustomAccount {
    pub bump_seed: u8,
}


// Use this constraint when your instruction needs to interact with PDA accounts created by another program.
// #[derive(Accounts)]
// pub struct InstructionAccountsWithSeedsProgram<'info> {
//     #[account(
//         seeds = [b"hello world"],
//         bump,
//         seeds::program = other_program.key(),
//     )]
//     pub pda_account: SystemAccount<'info>,
//     pub other_program: Program<'info, OtherProgram>
// }