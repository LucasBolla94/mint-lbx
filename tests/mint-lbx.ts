import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { MintLbx } from "../target/types/mint_lbx";
import { PublicKey, SystemProgram } from "@solana/web3.js";

describe("mint-lbx", () => {
  // Set the Anchor provider (default from CLI config or environment)
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.mintLbx as Program<MintLbx>;
  const wallet = provider.wallet;

  it("Initialize config!", async () => {
    // Deriva a conta PDA para o Config
    const [configPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("config")],
      program.programId
    );

    // Executa a instrução initialize_config com uma taxa de câmbio inicial de 10
    const tx = await program.methods
      .initializeConfig(new anchor.BN(10))
      .accounts({
        config: configPda,
        authority: wallet.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([]) // Nenhum signer extra além do wallet padrão
      .rpc();

    console.log("✅ Config initialized. Tx Signature:", tx);
  });
});
