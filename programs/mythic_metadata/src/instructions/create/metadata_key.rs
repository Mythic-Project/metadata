use anchor_lang::prelude::*;

use crate::constants::*;
use crate::state::*;

#[derive(Accounts)]
#[instruction(args: CreateMetadataKeyArgs)]
pub struct CreateMetadataKey<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account()]
    pub namespace_authority: Signer<'info>,
    #[account(
        init,
        payer = payer,
        space = MetadataKey::size(),
        seeds = [
            PREFIX,
            METADATA_KEY,
            &args.id.to_le_bytes()
        ],
        bump,
    )]
    pub metadata_key: Account<'info, MetadataKey>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateMetadataKey>, args: CreateMetadataKeyArgs) -> Result<()> {
    let CreateMetadataKeyArgs {
        id,
        name,
        label,
        description,
        content_type,
    } = args;
    MetadataKey::validate(&name, &label, &description, &content_type)?;

    let metadata_key = &mut ctx.accounts.metadata_key;

    metadata_key.set_inner(MetadataKey {
        bump: ctx.bumps.metadata_key,
        id,
        namespace_authority: ctx.accounts.namespace_authority.key(),
        name,
        description,
        label,
        content_type,
    });

    Ok(())
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct CreateMetadataKeyArgs {
    pub id: u64,
    pub name: String,
    pub label: String,
    pub description: String,
    pub content_type: String,
}
