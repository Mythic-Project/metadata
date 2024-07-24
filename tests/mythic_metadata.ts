import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import {
  ComputeBudgetProgram,
  ConfirmOptions,
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  SystemProgram,
} from "@solana/web3.js";
import { expect } from "chai";
import { MythicMetadata } from "../target/types/mythic_metadata";

const PREFIX = new TextEncoder().encode("mythic_metadata");
const METADATA_KEY = new TextEncoder().encode("metadata_key");
const METADATA = new TextEncoder().encode("metadata");

function getMetadataKey(id: number, programId: PublicKey) {
  return PublicKey.findProgramAddressSync(
    [PREFIX, METADATA_KEY, new anchor.BN(id).toArrayLike(Buffer, "le", 8)],
    programId
  );
}

function getMetadata(
  metadataKey: PublicKey,
  issuingAuth: PublicKey,
  subject: PublicKey,
  programId: PublicKey
) {
  return PublicKey.findProgramAddressSync(
    [
      PREFIX,
      METADATA,
      metadataKey.toBuffer(),
      issuingAuth.toBuffer(),
      subject.toBuffer(),
    ],
    programId
  );
}

async function airdrop(connection: Connection, to: PublicKey, amount: number) {
  await connection.confirmTransaction({
    ...(await connection.getLatestBlockhash("confirmed")),
    signature: await connection.requestAirdrop(to, amount),
  });
}

describe("metadata", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const mythicMetadataProgram = anchor.workspace
    .MythicMetadata as Program<MythicMetadata>;
  const { programId } = mythicMetadataProgram;

  const { connection, wallet } = provider;

  const confirmOptions: ConfirmOptions = {
    skipPreflight: true,
    commitment: "confirmed",
    preflightCommitment: "confirmed",
  };

  const demoSubject = new PublicKey(
    "DEan8xJtNMkEy4CJDRdx7qq67mWexjiHV4uEb19wK8i"
  );

  const metadataRootCollectionMetadataId = 123;
  const metadataCollectionMetadataKeyId = 456;
  const metadataItemMetadataKeyId = 789;

  const metadataKeyAuthKeypair = new Keypair();
  const metadataRootCollectionAuthKeypair = new Keypair();
  const metadataCollectionUpdateAuthKeypair = new Keypair();

  let metadataMetadataKey: PublicKey;
  let metadataCollectionMetadataKey: PublicKey;
  let metadataItemMetadataKey: PublicKey;

  let metadataKey: PublicKey;

  // Airdropping all addresses
  before(async () => {
    await airdrop(
      connection,
      metadataKeyAuthKeypair.publicKey,
      2 * LAMPORTS_PER_SOL
    );
    await airdrop(
      connection,
      metadataRootCollectionAuthKeypair.publicKey,
      2 * LAMPORTS_PER_SOL
    );
    await airdrop(
      connection,
      metadataCollectionUpdateAuthKeypair.publicKey,
      2 * LAMPORTS_PER_SOL
    );
  });

  describe("Metadata", () => {
    describe("after creating metadata root collection key", () => {
      const { name, contentType, description, label } = {
        contentType: "metadata-root-collection",
        description: "Deans List DAO Metadata Key",
        label: "Deans List DAO",
        name: "dao-metadata",
      };

      const metadataKeyBatch = getMetadataKey(
        metadataRootCollectionMetadataId,
        programId
      );

      metadataMetadataKey = metadataKeyBatch[0];
      const bump = metadataKeyBatch[1];

      let metadataMetadataKeyData;
      before(async () => {
        await mythicMetadataProgram.methods
          .createMetadataKey({
            name,
            contentType,
            description,
            label,
            id: new anchor.BN(metadataRootCollectionMetadataId),
          })
          .accountsStrict({
            metadataKey: metadataMetadataKey,
            namespaceAuthority: metadataKeyAuthKeypair.publicKey,
            payer: wallet.publicKey,
            systemProgram: SystemProgram.programId,
          })
          .preInstructions([
            ComputeBudgetProgram.setComputeUnitLimit({ units: 25000 }),
          ])
          .signers([metadataKeyAuthKeypair])
          .rpc(confirmOptions);

        metadataMetadataKeyData =
          await mythicMetadataProgram.account.metadataKey.fetch(
            metadataMetadataKey
          );
      });

      it("should have right bump", () => {
        expect(metadataMetadataKeyData.bump).to.eql(bump);
      });

      it("should have right namespaceAuthority", () => {
        expect(metadataMetadataKeyData.namespaceAuthority.toString()).to.eql(
          metadataKeyAuthKeypair.publicKey.toString()
        );
      });

      it("should have right name", () => {
        expect(metadataMetadataKeyData.name).to.eql(name);
      });

      it("should have right label", () => {
        expect(metadataMetadataKeyData.label).to.eql(label);
      });

      it("should have right description", () => {
        expect(metadataMetadataKeyData.description).to.eql(description);
      });

      it("should have right contentType", () => {
        expect(metadataMetadataKeyData.contentType).to.eql(contentType);
      });

      it("should have right id", () => {
        expect(metadataMetadataKeyData.id.toString()).to.eql(
          metadataRootCollectionMetadataId.toString()
        );
      });
    });

    describe("after creating metadata", () => {
      const metadataBatch = getMetadata(
        metadataMetadataKey,
        metadataRootCollectionAuthKeypair.publicKey,
        demoSubject,
        programId
      );

      metadataKey = metadataBatch[0];
      const bump = metadataBatch[1];

      let metadataData;
      before(async () => {
        await mythicMetadataProgram.methods
          .createMetadata({
            subject: demoSubject,
            updateAuthority: metadataRootCollectionAuthKeypair.publicKey,
          })
          .accountsStrict({
            issuingAuthority: metadataRootCollectionAuthKeypair.publicKey,
            metadata: metadataKey,
            metadataMetadataKey: metadataMetadataKey,
            payer: wallet.publicKey,
            systemProgram: SystemProgram.programId,
          })
          .signers([metadataRootCollectionAuthKeypair])
          .rpc(confirmOptions);

        metadataData = await mythicMetadataProgram.account.metadata.fetch(
          metadataKey
        );
      });

      it("should have right bump", () => {
        expect(metadataData.bump).to.eql(bump);
      });

      it("should have right issuingAuthority", () => {
        expect(metadataData.issuingAuthority.toString()).to.eql(
          metadataRootCollectionAuthKeypair.publicKey.toString()
        );
      });

      it("should have right updateAuthority", () => {
        expect(metadataData.updateAuthority.toString()).to.eql(
          metadataRootCollectionAuthKeypair.publicKey.toString()
        );
      });

      it("should have right subject", () => {
        expect(metadataData.subject.toString()).to.eql(demoSubject.toString());
      });
    });

    describe("after creating metadata collection key", () => {
      const { name, contentType, description, label } = {
        contentType: "metadata-collection",
        description: "Deans List DAO Metadata Collection Key",
        label: "Deans List DAO Collection 1",
        name: "dao-metadata-collection",
      };

      const metadataCollectionKeyBatch = getMetadataKey(
        metadataCollectionMetadataKeyId,
        programId
      );

      metadataCollectionMetadataKey = metadataCollectionKeyBatch[0];
      const bump = metadataCollectionKeyBatch[1];

      let metadataCollectionMetadataKeyData;
      before(async () => {
        await mythicMetadataProgram.methods
          .createMetadataKey({
            name,
            contentType,
            description,
            label,
            id: new anchor.BN(metadataCollectionMetadataKeyId),
          })
          .accountsStrict({
            metadataKey: metadataCollectionMetadataKey,
            namespaceAuthority: metadataKeyAuthKeypair.publicKey,
            payer: wallet.publicKey,
            systemProgram: SystemProgram.programId,
          })
          .preInstructions([
            ComputeBudgetProgram.setComputeUnitLimit({ units: 25000 }),
          ])
          .signers([metadataKeyAuthKeypair])
          .rpc(confirmOptions);

        metadataCollectionMetadataKeyData =
          await mythicMetadataProgram.account.metadataKey.fetch(
            metadataCollectionMetadataKey
          );
      });

      it("should have right bump", () => {
        expect(metadataCollectionMetadataKeyData.bump).to.eql(bump);
      });

      it("should have right namespaceAuthority", () => {
        expect(metadataCollectionMetadataKeyData.namespaceAuthority).to.eql(
          metadataKeyAuthKeypair.publicKey
        );
      });

      it("should have right name", () => {
        expect(metadataCollectionMetadataKeyData.name).to.eql(name);
      });

      it("should have right label", () => {
        expect(metadataCollectionMetadataKeyData.label).to.eql(label);
      });

      it("should have right description", () => {
        expect(metadataCollectionMetadataKeyData.description).to.eql(
          description
        );
      });

      it("should have right contentType", () => {
        expect(metadataCollectionMetadataKeyData.contentType).to.eql(
          contentType
        );
      });

      it("should have right id", () => {
        expect(metadataCollectionMetadataKeyData.id.toString()).to.eql(
          metadataCollectionMetadataKeyId.toString()
        );
      });
    });

    describe("after appending collection to metadata", () => {
      let metadataData;
      let collection;
      before(async () => {
        await mythicMetadataProgram.methods
          .appendMetadataCollection({
            updateAuthority: null,
          })
          .accountsStrict({
            collectionMetadataKey: metadataCollectionMetadataKey,
            metadata: metadataKey,
            metadataMetadataKey: metadataMetadataKey,
            payer: metadataRootCollectionAuthKeypair.publicKey,
            updateAuthority: metadataRootCollectionAuthKeypair.publicKey,
            systemProgram: SystemProgram.programId,
          })
          .signers([metadataRootCollectionAuthKeypair])
          .rpc(confirmOptions);

        metadataData = await mythicMetadataProgram.account.metadata.fetch(
          metadataKey
        );
        collection = metadataData.collections[0];
      });

      it("should have 1 collection in metadata", () => {
        expect(metadataData.collections.length).to.eql(1);
      });

      it("should have right metadata key id", () => {
        expect(collection.metadataKeyId.toString()).to.eql(
          metadataCollectionMetadataKeyId.toString()
        );
      });

      it("collection update authority should be null", () => {
        expect(collection.updateAuthority).to.be.null;
      });
    });

    describe("after setting collection update authority", () => {
      let metadataData;
      before(async () => {
        await mythicMetadataProgram.methods
          .setCollectionUpdateAuthority({
            newUpdateAuthority: metadataCollectionUpdateAuthKeypair.publicKey,
          })
          .accountsStrict({
            collectionMetadataKey: metadataCollectionMetadataKey,
            metadata: metadataKey,
            metadataMetadataKey: metadataMetadataKey,
            updateAuthority: metadataRootCollectionAuthKeypair.publicKey,
          })
          .signers([metadataRootCollectionAuthKeypair])
          .rpc(confirmOptions);

        metadataData = await mythicMetadataProgram.account.metadata.fetch(
          metadataKey
        );
      });

      it("should have updated collection update authority", () => {
        const collection = metadataData.collections[0];
        expect(collection.updateAuthority.toString()).to.eql(
          metadataCollectionUpdateAuthKeypair.publicKey.toString()
        );
      });
    });

    describe("after creating collection metadata item key", () => {
      const { name, contentType, description, label } = {
        contentType: "string",
        description: "Favorite Color for DAO",
        label: "Deans List Favorite Color",
        name: "favorite-color",
      };

      const metadataItemKeyBatch = getMetadataKey(
        metadataItemMetadataKeyId,
        programId
      );

      metadataItemMetadataKey = metadataItemKeyBatch[0];
      const bump = metadataItemKeyBatch[1];

      let metadataItemMetadataKeyData;
      before(async () => {
        await mythicMetadataProgram.methods
          .createMetadataKey({
            name,
            contentType,
            description,
            label,
            id: new anchor.BN(metadataItemMetadataKeyId),
          })
          .accountsStrict({
            metadataKey: metadataItemMetadataKey,
            namespaceAuthority: metadataKeyAuthKeypair.publicKey,
            payer: wallet.publicKey,
            systemProgram: SystemProgram.programId,
          })
          .preInstructions([
            ComputeBudgetProgram.setComputeUnitLimit({ units: 25000 }),
          ])
          .signers([metadataKeyAuthKeypair])
          .rpc(confirmOptions);

        metadataItemMetadataKeyData =
          await mythicMetadataProgram.account.metadataKey.fetch(
            metadataItemMetadataKey
          );
      });

      it("should have right bump", () => {
        expect(metadataItemMetadataKeyData.bump).to.eql(bump);
      });

      it("should have right namespaceAuthority", () => {
        expect(metadataItemMetadataKeyData.namespaceAuthority).to.eql(
          metadataKeyAuthKeypair.publicKey
        );
      });

      it("should have right name", () => {
        expect(metadataItemMetadataKeyData.name).to.eql(name);
      });

      it("should have right label", () => {
        expect(metadataItemMetadataKeyData.label).to.eql(label);
      });

      it("should have right description", () => {
        expect(metadataItemMetadataKeyData.description).to.eql(description);
      });

      it("should have right contentType", () => {
        expect(metadataItemMetadataKeyData.contentType).to.eql(contentType);
      });

      it("should have right id", () => {
        expect(metadataItemMetadataKeyData.id.toString()).to.eql(
          metadataItemMetadataKeyId.toString()
        );
      });
    });

    describe("after appending metadata item to metadata collection", () => {
      const favoriteColor = "red";

      let metadataData;
      let collection;
      let item;

      before(async () => {
        await mythicMetadataProgram.methods
          .appendMetadataItem({
            value: Buffer.from(favoriteColor),
          })
          .accountsStrict({
            collectionMetadataKey: metadataCollectionMetadataKey,
            itemMetadataKey: metadataItemMetadataKey,
            metadata: metadataKey,
            metadataMetadataKey: metadataMetadataKey,
            updateAuthority: metadataCollectionUpdateAuthKeypair.publicKey,
            systemProgram: SystemProgram.programId,
          })
          .signers([metadataCollectionUpdateAuthKeypair])
          .rpc(confirmOptions);

        metadataData = await mythicMetadataProgram.account.metadata.fetch(
          metadataKey
        );
        collection = metadataData.collections[0];
        item = collection.items[0];
      });

      it("should update collections to have a new item", () => {
        expect(collection.items.length).to.eql(1);
      });

      it("should have right metadata key id", () => {
        expect(item.metadataKeyId.toString()).to.eql(
          metadataItemMetadataKeyId.toString()
        );
      });

      it("should have right value", () => {
        expect(item.value).to.not.be.null;
      });
    });

    describe("after updating metadata item of metadata collection", () => {
      const newFavoriteColor = "blue";

      let metadataData;
      let collection;
      let item;

      before(async () => {
        await mythicMetadataProgram.methods
          .updateMetadataItem({
            newValue: Buffer.from(newFavoriteColor),
          })
          .accountsStrict({
            collectionMetadataKey: metadataCollectionMetadataKey,
            itemMetadataKey: metadataItemMetadataKey,
            metadata: metadataKey,
            metadataMetadataKey: metadataMetadataKey,
            updateAuthority: metadataCollectionUpdateAuthKeypair.publicKey,
            systemProgram: SystemProgram.programId,
          })
          .signers([metadataCollectionUpdateAuthKeypair])
          .rpc(confirmOptions);

        metadataData = await mythicMetadataProgram.account.metadata.fetch(
          metadataKey
        );
        collection = metadataData.collections[0];
        item = collection.items[0];
      });

      it("should have right value", () => {
        expect(item.value.toString()).to.eql(newFavoriteColor);
      });
    });

    describe("after removing item from collection", () => {
      let metadataData;
      before(async () => {
        await mythicMetadataProgram.methods
          .removeMetadataItem()
          .accountsStrict({
            collectionMetadataKey: metadataCollectionMetadataKey,
            metadata: metadataKey,
            itemMetadataKey: metadataItemMetadataKey,
            metadataMetadataKey: metadataMetadataKey,
            updateAuthority: metadataCollectionUpdateAuthKeypair.publicKey,
          })
          .signers([metadataCollectionUpdateAuthKeypair])
          .rpc(confirmOptions);

        metadataData = await mythicMetadataProgram.account.metadata.fetch(
          metadataKey
        );
      });

      it("should have removed from collection", () => {
        const collection = metadataData.collections[0];
        expect(collection.items.length).to.eql(0);
      });
    });

    describe("after revoking collection update authority", () => {
      let metadataData;
      before(async () => {
        await mythicMetadataProgram.methods
          .revokeCollectionUpdateAuthority()
          .accountsStrict({
            collectionMetadataKey: metadataCollectionMetadataKey,
            metadata: metadataKey,
            metadataMetadataKey: metadataMetadataKey,
            updateAuthority: metadataCollectionUpdateAuthKeypair.publicKey,
          })
          .signers([metadataCollectionUpdateAuthKeypair])
          .rpc(confirmOptions);

        metadataData = await mythicMetadataProgram.account.metadata.fetch(
          metadataKey
        );
      });

      it("should have nullified collection update authority", () => {
        const collection = metadataData.collections[0];
        expect(collection.updateAuthority).to.be.null;
      });
    });

    describe("after removing collection from root collection", () => {
      let metadataData;
      before(async () => {
        await mythicMetadataProgram.methods
          .removeMetadataCollection()
          .accountsStrict({
            collectionMetadataKey: metadataCollectionMetadataKey,
            metadata: metadataKey,
            metadataMetadataKey: metadataMetadataKey,
            updateAuthority: metadataRootCollectionAuthKeypair.publicKey,
          })
          .signers([metadataRootCollectionAuthKeypair])
          .rpc(confirmOptions);

        metadataData = await mythicMetadataProgram.account.metadata.fetch(
          metadataKey
        );
      });

      it("should have removed from root collection", () => {
        expect(metadataData.collections.length).to.eql(0);
      });
    });
  });
});
