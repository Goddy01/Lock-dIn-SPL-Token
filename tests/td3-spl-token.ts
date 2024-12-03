import * as anchor from "@coral-xyz/anchor";
import assert from "assert";
import { Program } from "@coral-xyz/anchor";
import { Td3SplToken } from "../target/types/td3_spl_token";
import * as web3 from "@solana/web3.js";
import BN from "bn.js";
import { TOKEN_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";

describe("SPL Token Test", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Td3SplToken as anchor.Program<Td3SplToken>;

  const METADATA = "metadata";
  const TOKEN_METADATA_PROGRAM_ID = new web3.PublicKey(
    "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
  )
  const MINT_SEED = "mint";
  const payer = program.provider.publicKey;
  const metadata = {
    name: "Lock Da Fuck In",
    symbol: "LDFI",
    uri: "https://github.com/Goddy01/toshDa3rd-Token-Metadata/blob/main/metadata.json",
    decimal: 9,
  }
  const mintAmount = 10;
  const [mint] = web3.PublicKey.findProgramAddressSync(
    [Buffer.from(MINT_SEED)],
    program.programId
  )

  const [metadataAddress] = web3.PublicKey.findProgramAddressSync(
    [Buffer.from(METADATA),
    TOKEN_METADATA_PROGRAM_ID.toBuffer(),
    mint.toBuffer()],
    TOKEN_METADATA_PROGRAM_ID
  );

  it("Initialize", async () => {
    const info = await program.provider.connection.getAccountInfo(mint);
    if (info) {
      return; // Do not attempt to initialize ifalready initialized
    }
    console.log("  Mint not found. Initializing program....")

    const context = {
      metadata: metadataAddress,
      mint,
      payer,
      rent: web3.SYSVAR_RENT_PUBKEY,
      systemProgram: web3.SystemProgram.programId,
      tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID
    };

    const txHash = await program.methods
      .initiateToken(metadata).accounts(context).rpc();

    await program.provider.connection.confirmTransaction(txHash, "finalized") {
      console.log(`https://explorer.solana.com/tx/${txHash}?cluster=devnet`);
      const newInfo = await program.provider.connection.getAccountInfo(mint);
      assert(newInfo, " Mint should be initialized");
    }
  });

  it('Mint Token', async () => {
    const destination = await anchor.utils.token.associatedAddress({
      mint: mint,
      owner: payer
    })

    let initialBalance: number;

    try {
      const balance = await program.provider.connection.getTokenAccountBalance(destination)
      initialBalance = balance.value.uiAmount;
    }
    catch {
      // Token balance not yet initiated, has 0 balance
      initialBalance = 0;
    }

    const context = {
      mint,
      destination,
      payer,
      rent: web3.SYSVAR_RENT_PUBKEY,
      systemProgram: web3.SystemProgram.programId,
      tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
    };

    const txHash = await program.methods
      .mintTokens(new BN(mintAmount * 10 ** metadata.decimal))
      .accounts(context)
      .rpc();

    await program.provider.connection.confirmTransaction(txHash);
    console.log(`  https://explorer.solana.com/tx/${txHash}?cluster=devnet`)

    const postBalance = (
      (await program.provider.connection.getTokenAccountBalance(destination))
    ).value.uiAmount;
    assert.equal(
      initialBalance + mintAmount,
      postBalance,
      "Post balance should equal intial plus mint amount"
    )
  });

});
