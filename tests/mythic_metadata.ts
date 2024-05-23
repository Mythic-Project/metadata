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
const COUNTER = new TextEncoder().encode("counter");
const METADATA_KEY = new TextEncoder().encode("metadata_key");
const METADATA = new TextEncoder().encode("metadata");

function getCounter(programId: PublicKey) {
  return PublicKey.findProgramAddressSync([PREFIX, COUNTER], programId);
}

function getMetadataKey(
  nameSpaceAuth: PublicKey,
  name: string,
  programId: PublicKey
) {
  return PublicKey.findProgramAddressSync(
    [PREFIX, METADATA_KEY, nameSpaceAuth.toBuffer(), Buffer.from(name)],
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

  const metadataKeyAuthKeypair = new Keypair();
  const metadataAuthKeypair = new Keypair();
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
      1 * LAMPORTS_PER_SOL
    );
    await airdrop(
      connection,
      metadataAuthKeypair.publicKey,
      1 * LAMPORTS_PER_SOL
    );
    await airdrop(
      connection,
      metadataCollectionUpdateAuthKeypair.publicKey,
      1 * LAMPORTS_PER_SOL
    );
  });

  describe("Metadata", () => {
    const [counter, counterBump] = getCounter(programId);

    describe("after initializing counter account", () => {
      let counterData;
      before(async () => {
        await mythicMetadataProgram.methods
          .initializeCounter()
          .accountsStrict({
            counter,
            payer: wallet.publicKey,
            systemProgram: SystemProgram.programId,
          })
          .rpc(confirmOptions);

        counterData = await mythicMetadataProgram.account.counter.fetch(
          counter
        );
      });

      it("should have the right bump", () => {
        expect(counterData.bump).to.eql(counterBump);
      });

      it("should set the id to 1", () => {
        expect(counterData.id.toString()).to.eql("1");
      });
    });

    describe("after creating metadata key", () => {
      const { name, contentType, description, label } = {
        contentType: "metadata",
        description: "Deans List DAO Metadata Key",
        label: "Deans List DAO",
        name: "dao-metadata",
      };

      const metadataKeyBatch = getMetadataKey(
        metadataKeyAuthKeypair.publicKey,
        name,
        programId
      );

      metadataMetadataKey = metadataKeyBatch[0];
      const bump = metadataKeyBatch[1];

      let metadataMetadataKeyData;
      let counterData;
      before(async () => {
        await mythicMetadataProgram.methods
          .createMetadataKey({ name, contentType, description, label })
          .accountsStrict({
            counter,
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

        counterData = await mythicMetadataProgram.account.counter.fetch(
          counter
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
        expect(metadataMetadataKeyData.id.toString()).to.eql("1");
      });

      it("should have incremented counter id by 1", () => {
        expect(counterData.id.toString()).to.eql("2");
      });
    });

    describe("after creating metadata", () => {
      const metadataBatch = getMetadata(
        metadataMetadataKey,
        metadataAuthKeypair.publicKey,
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
            updateAuthority: metadataAuthKeypair.publicKey,
          })
          .accountsStrict({
            issuingAuthority: metadataAuthKeypair.publicKey,
            metadata: metadataKey,
            metadataKey: metadataMetadataKey,
            payer: wallet.publicKey,
            systemProgram: SystemProgram.programId,
          })
          .signers([metadataAuthKeypair])
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
          metadataAuthKeypair.publicKey.toString()
        );
      });

      it("should have right updateAuthority", () => {
        expect(metadataData.updateAuthority.toString()).to.eql(
          metadataAuthKeypair.publicKey.toString()
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
        metadataKeyAuthKeypair.publicKey,
        name,
        programId
      );

      metadataCollectionMetadataKey = metadataCollectionKeyBatch[0];
      const bump = metadataCollectionKeyBatch[1];

      let metadataCollectionMetadataKeyData;
      let counterData;
      before(async () => {
        await mythicMetadataProgram.methods
          .createMetadataKey({ name, contentType, description, label })
          .accountsStrict({
            counter,
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

        counterData = await mythicMetadataProgram.account.counter.fetch(
          counter
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
        expect(metadataCollectionMetadataKeyData.id.toString()).to.eql("2");
      });

      it("should have incremented counter id by 1", () => {
        expect(counterData.id.toString()).to.eql("3");
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
            metadataKey: metadataMetadataKey,
            updateAuthority: metadataAuthKeypair.publicKey,
          })
          .signers([metadataAuthKeypair])
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
        expect(collection.metadataKeyId.toString()).to.eql("2");
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
            metadataKey: metadataMetadataKey,
            updateAuthority: metadataAuthKeypair.publicKey,
          })
          .signers([metadataAuthKeypair])
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

    describe("after creating metadata item key", () => {
      const { name, contentType, description, label } = {
        contentType: "string",
        description: "Favorite Color for DAO",
        label: "Deans List Favorite Color",
        name: "favorite-color",
      };

      const metadataItemKeyBatch = getMetadataKey(
        metadataKeyAuthKeypair.publicKey,
        name,
        programId
      );

      metadataItemMetadataKey = metadataItemKeyBatch[0];
      const bump = metadataItemKeyBatch[1];

      let metadataItemMetadataKeyData;
      let counterData;
      before(async () => {
        await mythicMetadataProgram.methods
          .createMetadataKey({ name, contentType, description, label })
          .accountsStrict({
            counter,
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

        counterData = await mythicMetadataProgram.account.counter.fetch(
          counter
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
        expect(metadataItemMetadataKeyData.id.toString()).to.eql("3");
      });

      it("should have incremented counter id by 1", () => {
        expect(counterData.id.toString()).to.eql("4");
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
            metadataKey: metadataMetadataKey,
            updateAuthority: metadataCollectionUpdateAuthKeypair.publicKey,
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
        expect(item.metadataKeyId.toString()).to.eql("3");
      });

      it("should have right value", () => {
        expect(item.value).to.not.be.null;
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
            metadataKey: metadataMetadataKey,
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
  });
});
