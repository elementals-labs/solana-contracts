import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Elementals } from "../target/types/elementals";
import { PublicKey, SystemProgram } from "@solana/web3.js";
import * as web3 from "@solana/web3.js";

describe("elementals", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  // 2 accounts

  const program = anchor.workspace.Elementals as Program<Elementals>;
  const payer = (program.provider as anchor.AnchorProvider).wallet;
  const player2 = new PublicKey("6hjxohPktokzTrTENiF1hJ6tnV7V81uS9cHktpMosgcq");

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
      .registerToPlay()
      .accounts({
        payer: payer.publicKey,
      })
      .rpc();
    console.log("Your transaction signature", tx);
  });
});
