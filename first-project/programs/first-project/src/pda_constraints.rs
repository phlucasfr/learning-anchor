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
        bump,
    )]
    pub pda_account: SystemAccount<'info>,
}