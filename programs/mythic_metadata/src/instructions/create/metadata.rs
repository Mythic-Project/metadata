use std::vec;

use anchor_lang::prelude::*;

use crate::constants::*;
use crate::state::*;

#[derive(Accounts)]
#[instruction(args: CreateMetadataArgs)]
pub struct CreateMetadata<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account()]
    pub issuing_authority: Signer<'info>,
    #[account(
        init,
        payer = payer,
        space = Metadata::size(&MetadataRootCollection {
            collections: vec![],
            items: vec![],
            metadata_key_id: root_collection_metadata_key.id,
            update_slot: Clock::get()?.slot,
            update_authority: args.update_authority
        }),
        seeds = [
            PREFIX,
            METADATA,
            root_collection_metadata_key.key().as_ref(),
            issuing_authority.key().as_ref(),
            args.subject.as_ref()
        ],
        bump,
    )]
    pub metadata: Account<'info, Metadata>,
    #[account(
        seeds = [
            PREFIX,
            METADATA_KEY,
            &root_collection_metadata_key.id.to_le_bytes()
        ],
        bump = root_collection_metadata_key.bump,
    )]
    pub root_collection_metadata_key: Account<'info, MetadataKey>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateMetadata>, args: CreateMetadataArgs) -> Result<()> {
    let CreateMetadataArgs {
        subject,
        update_authority,
    } = args;

    let metadata = &mut ctx.accounts.metadata;
    metadata.set_inner(Metadata {
        bump: ctx.bumps.metadata,
        collection: MetadataRootCollection {
            collections: vec![],
            items: vec![],
            metadata_key_id: ctx.accounts.root_collection_metadata_key.id,
            update_authority,
            update_slot: Clock::get()?.slot,
        },
        issuing_authority: ctx.accounts.issuing_authority.key(),
        subject,
    });

    Ok(())
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct CreateMetadataArgs {
    pub subject: Pubkey,
    pub update_authority: Option<Pubkey>,
}
