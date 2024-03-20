use anchor_lang::prelude::*;

declare_id!("6qNJgnGrx2tnvmXEgNCpCCe7yfe4tNRcUUQSZZnV9WhC");

pub mod state;

#[program]
pub mod metadata {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
