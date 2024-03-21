use anchor_lang::prelude::*;

/// MetadataKey account defines a single metadata item
///
/// PDA seeds: 'metadatakey' + [namespace_authority] + [name]
///
/// Instructions:
///
/// 1) CreateMetadataKey - Creates a new MetadataKey
/// namespace_authority must sign the transaction
///
/// 2) SetCertificationAuthority - Sets the certification authority for the MetadataKey
/// Current certification_authority must sign the transaction
///
/// Note: MetadataKey can describe Metadata account, MetadataCollection and MetadataItem
pub struct MetadataKey {
    /// Unique identifier of the MetadataKey assigned by the program
    pub id: u64,

    /// Authority of the MetadataKey namespace
    /// It allows authorities to create unique namespaces for metadata keys
    pub namespace_authority: Pubkey,

    /// Name of the MetadataKey
    /// It must be unique within the namespace authority
    pub name: String,

    /// User friendly label of the MetadataKey
    pub label: String,

    /// Description of the MetadataKey
    pub description: String,

    /// The type of the metadata described by the key
    /// e.g. string, number, image, metadata, metadata-collection etc.
    pub content_type: String,
}

/// MetadataItem defines a single metadata item identified by its MetadataKey
pub struct MetadataItem {
    /// The id of the key identifying the Metadata item
    pub metadata_key_id: u64,

    /// Serialized metadata item value
    pub value: Vec<u8>,

    /// The slot when the value was last updated
    pub update_slot: u64,
}

pub struct MetadataCollection {
    /// Unique identifier of the MetadataKey describing the collection
    pub metadata_key_id: u64,

    /// The authority that can update the collection metadata items
    /// If the authority is None then the authority is inherited from parent Metadata account
    pub update_authority: Option<Pubkey>,

    /// Metadata items of the collection
    pub items: Vec<MetadataItem>,

    /// The slot when the collection was last updated
    pub update_slot: u64,
}

/// Metadata account defines a set of metadata items
///
// PDA seeds: 'metadata' + [metadata_key_id] + [subject] + [metadata_authority]
pub struct Metadata {
    /// Unique identifier of the MetadataKey describing the collection
    pub metadata_key_id: u64,

    /// The subject described by the metadata (e.g. a DAO, NFT, a program etc.)
    pub subject: Pubkey,

    /// The authority which created the Metadata account and owns it
    /// Note: The authority is embedded in the PDA seeds and cannot be changed
    /// If a new authority is required then a new Metadata account must be created
    pub metadata_authority: Pubkey,

    /// The default update authority for all the collections
    /// Note: The authority can be overridden at the collection level
    pub update_authority: Option<Pubkey>,

    /// A set of metadata collections  
    pub collections: Vec<MetadataCollection>,
}
