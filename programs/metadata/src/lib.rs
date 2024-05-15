#![allow(ambiguous_glob_reexports)]
use anchor_lang::prelude::*;

declare_id!("DAom1h29CGo2G5WRvLPeZeTqN96ft22YfZU4yt82RCh");

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;
pub mod utils;

pub use instructions::*;

#[program]
pub mod metadata {
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
}

#[derive(Accounts)]
pub struct Initialize {}
