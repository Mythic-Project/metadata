use anchor_lang::prelude::*;

use crate::constants::*;
use crate::errors::*;
use crate::state::*;
use crate::utils::*;

#[derive(Accounts)]
pub struct SetCollectionUpdateAuthority<'info> {
    #[account()]
    pub update_authority: Signer<'info>,
    #[account(
        mut,
        constraint = metadata.collection.metadata_key_id.eq(&root_collection_metadata_key.id) @ MythicMetadataError::InvalidMetadataKey,
        seeds = [
            PREFIX,
            METADATA,
            root_collection_metadata_key.key().as_ref(),
            metadata.issuing_authority.as_ref(),
            metadata.subject.as_ref()
        ],
        bump = metadata.bump,
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
    #[account(
        seeds = [
            PREFIX,
            METADATA_KEY,
            &collection_metadata_key.id.to_le_bytes()
        ],
        bump = collection_metadata_key.bump,
    )]
    pub collection_metadata_key: Account<'info, MetadataKey>,
}

pub fn handler(
    ctx: Context<SetCollectionUpdateAuthority>,
    args: SetCollectionUpdateAuthorityArgs,
) -> Result<()> {
    let metadata = &mut ctx.accounts.metadata;
    let root_collection_metadata_key = &ctx.accounts.root_collection_metadata_key;
    let collection_metadata_key = &ctx.accounts.collection_metadata_key;
    let update_authority = &ctx.accounts.update_authority;

    // Check if root collection and collection is same to update root_collection.update_authority
    if check_collection_root_collection_equality(
        root_collection_metadata_key,
        collection_metadata_key,
    ) {
        verify_root_collection_update_authority(&metadata.collection, update_authority.key)?;
        metadata.collection.update_authority = Some(args.new_update_authority);
    } else {
        let (collection_index, mut collection) = verify_collection_update_authority(
            &metadata.collection,
            collection_metadata_key.id,
            &update_authority.key(),
        )?;

        collection.update_authority = Some(args.new_update_authority);
        metadata.collection.collections.remove(collection_index);
        metadata
            .collection
            .collections
            .insert(collection_index, collection);
    }

    metadata.validate()?;

    Ok(())
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct SetCollectionUpdateAuthorityArgs {
    pub new_update_authority: Pubkey,
}
