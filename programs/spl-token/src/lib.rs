use anchor_lang::prelude::*;

declare_id!("5iXkkKneqKpEBrdaTbD7MKBr6WhoJqeoR55ohCLp3qTc");

#[program]
pub mod spl_token {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
