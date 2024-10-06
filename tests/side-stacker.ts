import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Elementals } from "../target/types/elementals";
import { PublicKey, SystemProgram, Keypair } from "@solana/web3.js";
import * as web3 from "@solana/web3.js";
import { hash as sha256_hash } from "@coral-xyz/anchor/dist/cjs/utils/sha256";
import * as borsh from "borsh";

describe("elementals", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Elementals as Program<Elementals>;

  // Create two separate keypairs
  const payer = Keypair.generate();
  const payer2 = Keypair.generate();

  const system_program = SystemProgram.programId;

  const name = "elementals2";

  const [queue, bump1] = web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from(anchor.utils.bytes.utf8.encode("queue")),
      Buffer.from(anchor.utils.bytes.utf8.encode(name)),
    ],
    program.programId
  );

  const [game, _] = web3.PublicKey.findProgramAddressSync(
    [Buffer.from("game"), queue.toBuffer()],
    program.programId
  );

  async function airdropSol(address: PublicKey) {
    const connection = program.provider.connection;
    const signature = await connection.requestAirdrop(address, 1000000000); // 1 SOL
    await connection.confirmTransaction(signature);
  }

  program.addEventListener("playerRegistered", (event) => {
    console.log(
      "\x1b[32m%s\x1b[0m",
      "PlayerRegistered",
      event.player.toBase58()
    );
  });

  program.addEventListener("gameCreated", (event) => {
    console.log(
      "\x1b[32m%s\x1b[0m",
      "GameCreated",
      event.roomId,
      event.players[0].pubkey,
      event.players[1].pubkey
    );
  });

  program.addEventListener("userPlayRegistered", (event) => {
    console.log(
      "\x1b[32m%s\x1b[0m",
      "UserPlayRegistered",
      event.roomId,
      event.player,
      event.play
    );
  });

  before(async () => {
    // Airdrop SOL to both keypairs before running tests
    await airdropSol(payer.publicKey);
    await airdropSol(payer2.publicKey);
  });

  it("Is initialized!", async () => {
    const tx = await program.methods
      .initializeQueue(name)
      .accounts({ payer: payer.publicKey, queue })
      .signers([payer])
      .rpc();

    console.log("InitializeQueue txid", tx);
  });

  it("Register Players!", async () => {
    console.log("\x1b[36m%s\x1b[0m", payer.publicKey.toBase58());
    console.log("\x1b[36m%s\x1b[0m", payer2.publicKey.toBase58());

    const tx1 = await program.methods
      .registerToPlay({
        elementals: [
          {
            name: "Elemental1",
            stats: {
              hp: 100,
              atk: 50,
              def: 50,
              spa: 50,
              spd: 50,
              spe: 50,
            },
            movements: [
              { mindflare: {} },
              { revitalize: {} },
              { sparkShackle: {} },
              { groundbreaker: {} },
            ],
            isAlive: true,
            status: { normal: {} },
          },
          {
            name: "Elemental2",
            stats: {
              hp: 90,
              atk: 60,
              def: 40,
              spa: 60,
              spd: 40,
              spe: 60,
            },
            movements: [
              { dreamDust: {} },
              { naturesHold: {} },
              { verdantSlash: {} },
              { bindingVines: {} },
            ],
            isAlive: true,
            status: { normal: {} },
          },
          {
            name: "Elemental3",
            stats: {
              hp: 110,
              atk: 40,
              def: 60,
              spa: 40,
              spd: 60,
              spe: 40,
            },
            movements: [
              { mindShield: {} },
              { tidalWave: {} },
              { tranquilSlumber: {} },
              { aquaBind: {} },
            ],
            isAlive: true,
            status: { normal: {} },
          },
        ],
      })
      .accountsStrict({
        player: payer.publicKey,
        queue,
        game,
        systemProgram: system_program,
      })
      .signers([payer])
      .rpc();
    console.log("Your transaction signature", tx1);

    const tx2 = await program.methods
      .registerToPlay({
        elementals: [
          {
            name: "Elemental1",
            stats: {
              hp: 100,
              atk: 50,
              def: 50,
              spa: 50,
              spd: 50,
              spe: 50,
            },
            movements: [
              { mindflare: {} },
              { revitalize: {} },
              { sparkShackle: {} },
              { groundbreaker: {} },
            ],
            isAlive: true,
            status: { normal: {} },
          },
          {
            name: "Elemental2",
            stats: {
              hp: 90,
              atk: 60,
              def: 40,
              spa: 60,
              spd: 40,
              spe: 60,
            },
            movements: [
              { dreamDust: {} },
              { naturesHold: {} },
              { verdantSlash: {} },
              { bindingVines: {} },
            ],
            isAlive: true,
            status: { normal: {} },
          },
          {
            name: "Elemental3",
            stats: {
              hp: 110,
              atk: 40,
              def: 60,
              spa: 40,
              spd: 60,
              spe: 40,
            },
            movements: [
              { mindShield: {} },
              { tidalWave: {} },
              { tranquilSlumber: {} },
              { aquaBind: {} },
            ],
            isAlive: true,
            status: { normal: {} },
          },
        ],
      })
      .accountsStrict({
        player: payer2.publicKey,
        queue,
        game,
        systemProgram: system_program,
      })
      .signers([payer2])
      .rpc();

    console.log("Your transaction signature", tx2);
  });

  it("Commits both plays!", async () => {
    console.log("\x1b[36m%s\x1b[0m", payer.publicKey.toBase58());
    console.log("\x1b[36m%s\x1b[0m", payer2.publicKey.toBase58());
    console.log("\x1b[36m%s\x1b[0m", game.toBase58());
    console.log("\x1b[36m%s\x1b[0m", queue.toBase58());

    const struct1 = {
      player: payer.publicKey,
      elemental: 0,
      movement: 2,
    };

    const struct2 = {
      player: payer.publicKey,
      elemental: 1,
      movement: 3,
    };

    const tx1 = await program.methods
      .playGame(struct1)
      .accounts({ payer: payer.publicKey, game })
      .rpc();

    console.log("play 1 txid", tx1);

    const tx2 = await program.methods
      .playGame(struct2)
      .accounts({ payer: payer2.publicKey, game })
      .rpc();

    console.log("play 2 txid", tx2);
  });
});
