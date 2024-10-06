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
  const system_program = SystemProgram.programId;

  const stats = {
    hp: 5,
    atk: 5,
    def: 5,
    spa: 5, // Special Attack
    spd: 5, // Special Defense
    spe: 5, // Speed
  };

  const elemental = {
    name: "JUANCITO",
    stats: stats,
    movements: ["Mindflare", "Mindflare", "Mindflare", "Mindflare"],
    isAlive: true,
    status: { Normal: {} },
  };

  const teamInput = { elementals: [elemental, elemental, elemental] };
  const [queue, _] = web3.PublicKey.findProgramAddressSync(
    [Buffer.from(anchor.utils.bytes.utf8.encode("elementals"))],
    program.programId
  );

  //

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
      .testRegisterToPlay("JUANCITO_PEREZ_EL_JUEGO2")
      .accounts({
        payer: payer.publicKey,
      })
      .rpc();
    console.log("Your transaction signature", tx);

    /*     const tx2 = await program.methods
      .registerToPlay([elemental, elemental, elemental])
      .accounts({
        payer: payer.publicKey,
      })
      .rpc();
    console.log("Your transaction signature", tx); */
  });
});
