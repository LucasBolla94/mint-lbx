import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { MintLbx } from "../target/types/mint_lbx";
import {
  PublicKey,
  SystemProgram,
  LAMPORTS_PER_SOL,
} from "@solana/web3.js";
import {
  TOKEN_PROGRAM_ID,
  getAccount,
} from "@solana/spl-token";

describe("mint-lbx", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.mintLbx as Program<MintLbx>;
  const wallet = provider.wallet;

  const [configPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("config")],
    program.programId
  );

  const [vaultPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("vault")],
    program.programId
  );

  const [mintAuthorityPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("mint_authority")],
    program.programId
  );

  const mint = new PublicKey("HUShvmkvCxXhdMH6A3EG4yyvDeqRbhYR7uXef5kFTw7G");
  const userTokenAccount = new PublicKey("35gJ1qNrTxDTWne8FFTmdXFcovwoz6XDWe9sciheYe8W");

  it("Initialize config", async () => {
    const configAccount = await provider.connection.getAccountInfo(configPda);

    if (configAccount !== null) {
      console.log("⚠️  Config já está inicializado, pulando...");
      return;
    }

    const tx = await program.methods
      .initializeConfig(new anchor.BN(10))
      .accounts({
        config: configPda,
        authority: wallet.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    console.log("✅ Config initialized. Tx Signature:", tx);
  });

  it("Update exchange rate", async () => {
    const tx = await program.methods
      .updateExchangeRate(new anchor.BN(20))
      .accounts({
        config: configPda,
        authority: wallet.publicKey,
      })
      .rpc();

    console.log("✅ Exchange rate updated. Tx Signature:", tx);
  });

  it("Deposit SOL and mint LBX", async () => {
    const amount = 3 * LAMPORTS_PER_SOL; // meio SOL para teste

    const tx = await program.methods
      .depositSolAndMint(new anchor.BN(amount))
      .accounts({
        user: wallet.publicKey,
        vault: vaultPda,
        config: configPda,
        mintAuthority: mintAuthorityPda,
        mint: mint,
        userTokenAccount: userTokenAccount,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    console.log("✅ Deposited SOL and minted LBX. Tx Signature:", tx);

    const accountInfo = await getAccount(provider.connection, userTokenAccount);
    console.log("💰 Token balance:", accountInfo.amount.toString());
  });
});
