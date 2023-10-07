import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Vpl } from "../target/types/vpl";
import lumina from "@lumina-dev/test";
import { assert } from "chai";

lumina();

describe("vaxchain-pl", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Vpl as Program<Vpl>;
  const connection = anchor.getProvider().connection;

  const manufacturer = anchor.web3.Keypair.generate();
  const distributor = anchor.web3.Keypair.generate();
  const doctor = anchor.web3.Keypair.generate();

  before(async () => {
    await connection.requestAirdrop(
      manufacturer.publicKey,
      1 * anchor.web3.LAMPORTS_PER_SOL
    );
    await connection.requestAirdrop(
      distributor.publicKey,
      1 * anchor.web3.LAMPORTS_PER_SOL
    );
    await connection.requestAirdrop(
      doctor.publicKey,
      1 * anchor.web3.LAMPORTS_PER_SOL
    );
  });

  it("Creates manufacturer", async () => {
    const userPda = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("user"), manufacturer.publicKey.toBuffer()],
      program.programId
    )[0];

    await program.methods
      .createUser({
        manufacturer: {},
      })
      .accounts({
        user: manufacturer.publicKey,
        userPda: userPda,
      })
      .signers([manufacturer])
      .rpc();

    const userPdaAccount = await program.account.user.fetch(userPda);

    assert.equal(
      userPdaAccount.pubkey.toBase58(),
      manufacturer.publicKey.toBase58()
    );
    assert.ok(userPdaAccount.role.manufacturer);
  });

  it("Creates distributor", async () => {
    const userPda = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("user"), distributor.publicKey.toBuffer()],
      program.programId
    )[0];

    await program.methods
      .createUser({
        distributor: {},
      })
      .accounts({
        user: distributor.publicKey,
        userPda: userPda,
      })
      .signers([distributor])
      .rpc();

    const userPdaAccount = await program.account.user.fetch(userPda);

    assert.equal(
      userPdaAccount.pubkey.toBase58(),
      distributor.publicKey.toBase58()
    );
    assert.ok(userPdaAccount.role.distributor);
  });

  it("Creates doctor", async () => {
    const userPda = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("user"), doctor.publicKey.toBuffer()],
      program.programId
    )[0];

    await program.methods
      .createUser({
        doctor: {},
      })
      .accounts({
        user: doctor.publicKey,
        userPda: userPda,
      })
      .signers([doctor])
      .rpc();

    const userPdaAccount = await program.account.user.fetch(userPda);

    assert.equal(userPdaAccount.pubkey.toBase58(), doctor.publicKey.toBase58());
    assert.ok(userPdaAccount.role.doctor);
  });
});
