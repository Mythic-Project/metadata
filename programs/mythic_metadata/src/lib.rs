#![allow(ambiguous_glob_reexports)]
use anchor_lang::prelude::*;

declare_id!("myThHf7Ec8WEFiVFeUEiuq1KPPmx3udRF7hehPQBaa3");

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;
pub mod utils;

pub use instructions::*;

#[program]
pub mod mythic_metadata {
    use super::*;

    pub fn create_metadata_key(
        ctx: Context<CreateMetadataKey>,
        args: CreateMetadataKeyArgs,
    ) -> Result<()> {
        create::metadata_key::handler(ctx, args)
    }

    pub fn create_metadata(ctx: Context<CreateMetadata>, args: CreateMetadataArgs) -> Result<()> {
        create::metadata::handler(ctx, args)
    }

    pub fn append_metadata_collection(
        ctx: Context<AppendMetadataCollection>,
        args: AppendMetadataCollectionArgs,
    ) -> Result<()> {
        append::collection::handler(ctx, args)
    }

    pub fn remove_metadata_collection(ctx: Context<RemoveMetadataCollection>) -> Result<()> {
        remove::collection::handler(ctx)
    }

    pub fn set_collection_update_authority(
        ctx: Context<SetCollectionUpdateAuthority>,
        args: SetCollectionUpdateAuthorityArgs,
    ) -> Result<()> {
        auth::set::handler(ctx, args)
    }

    pub fn revoke_collection_update_authority(
        ctx: Context<RevokeCollectionUpdateAuthority>,
    ) -> Result<()> {
        auth::revoke::handler(ctx)
    }

    pub fn append_metadata_item(
        ctx: Context<AppendMetadataItem>,
        args: AppendMetadataItemArgs,
    ) -> Result<()> {
        append::item::handler(ctx, args)
    }

    pub fn append_metadata_items(
        ctx: Context<AppendMetadataItems>,
        args: AppendMetadataItemsArgs,
    ) -> Result<()> {
        append::items::handler(ctx, args)
    }

    pub fn update_metadata_item(
        ctx: Context<UpdateMetadataItem>,
        args: UpdateMetadataItemArgs,
    ) -> Result<()> {
        update::item::handler(ctx, args)
    }

    pub fn remove_metadata_item(ctx: Context<RemoveMetadataItem>) -> Result<()> {
        remove::item::handler(ctx)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
