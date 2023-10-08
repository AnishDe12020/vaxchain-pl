import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Vpl } from "../target/types/vpl";
import lumina from "@lumina-dev/test";
import { assert } from "chai";
import {
  getAccount,
  getOrCreateAssociatedTokenAccount,
  mintTo,
} from "@solana/spl-token";

lumina();

const tokenMint = new anchor.web3.PublicKey(
  "2JU4847ngmiGjuZ6m2pt3unq41GVt6WRw6wnJhVPe2oD"
);

describe("vaxchain-pl", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Vpl as Program<Vpl>;
  const connection = anchor.getProvider().connection;
  const userWallet = anchor.workspace.Vpl.provider.wallet;

  const manufacturer = anchor.web3.Keypair.generate();
  const distributor = anchor.web3.Keypair.generate();
  const doctor = anchor.web3.Keypair.generate();

  const batchPubkey = anchor.web3.Keypair.generate().publicKey;
  const vaccine1Pubkey = anchor.web3.Keypair.generate().publicKey;
  const vaccine2Pubkey = anchor.web3.Keypair.generate().publicKey;
  const vaccine3Pubkey = anchor.web3.Keypair.generate().publicKey;

  before(async () => {
    const sig1 = await connection.requestAirdrop(
      manufacturer.publicKey,
      1 * anchor.web3.LAMPORTS_PER_SOL
    );
    await connection.confirmTransaction(sig1, "confirmed");
    const sig2 = await connection.requestAirdrop(
      distributor.publicKey,
      1 * anchor.web3.LAMPORTS_PER_SOL
    );
    await connection.confirmTransaction(sig2, "confirmed");
    const sig3 = await connection.requestAirdrop(
      doctor.publicKey,
      1 * anchor.web3.LAMPORTS_PER_SOL
    );
    await connection.confirmTransaction(sig3, "confirmed");

    const distributorAta = await getOrCreateAssociatedTokenAccount(
      connection,
      distributor,
      tokenMint,
      distributor.publicKey
    );

    const doctorAta = await getOrCreateAssociatedTokenAccount(
      connection,
      doctor,
      tokenMint,
      doctor.publicKey
    );

    await mintTo(
      connection,
      userWallet.payer,
      tokenMint,
      distributorAta.address,
      userWallet.payer,
      1000 * 10 ** 9
    );

    await mintTo(
      connection,
      userWallet.payer,
      tokenMint,
      doctorAta.address,
      userWallet.payer,
      1000 * 10 ** 9
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

  it("Creates batch", async () => {
    const batchPda = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("batch"), batchPubkey.toBuffer()],
      program.programId
    )[0];

    const userPda = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("user"), manufacturer.publicKey.toBuffer()],
      program.programId
    )[0];

    const vaultPda = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vault"), batchPubkey.toBuffer(), tokenMint.toBuffer()],
      program.programId
    )[0];

    const far_away_expiry_ms = 1000 * 60 * 60 * 24 * 365 * 10; // 10 years

    await program.methods
      .createBatch(new anchor.BN(far_away_expiry_ms), 250, 350, 200, 3)
      .accounts({
        batch: batchPubkey,
        user: manufacturer.publicKey,
        batchPda,
        userPda,
        vault: vaultPda,
        mint: tokenMint,
      })
      .signers([manufacturer])
      .rpc();

    const batchPdaAccount = await program.account.batch.fetch(batchPda);

    assert.equal(batchPdaAccount.pubkey.toBase58(), batchPubkey.toBase58());
    assert.equal(batchPdaAccount.expiresAt.toNumber(), far_away_expiry_ms);
    assert.equal(batchPdaAccount.costPerPiece, 200);
    assert.equal(batchPdaAccount.tempMin, 250);
    assert.equal(batchPdaAccount.tempMax, 350);
    assert.equal(batchPdaAccount.quantity, 3);
    assert.equal(
      batchPdaAccount.manufacturer.toBase58(),
      manufacturer.publicKey.toBase58()
    );
    assert.ok(batchPdaAccount.status.manufactured);
    assert.ok(!batchPdaAccount.distributor);
  });

  it("can create vaccines", async () => {
    const userPda = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("user"), manufacturer.publicKey.toBuffer()],
      program.programId
    )[0];

    const batchPda = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("batch"), batchPubkey.toBuffer()],
      program.programId
    )[0];

    const vaccine1Pda = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vaccine"), vaccine1Pubkey.toBuffer()],
      program.programId
    )[0];
    const vaccine2Pda = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vaccine"), vaccine2Pubkey.toBuffer()],
      program.programId
    )[0];
    const vaccine3Pda = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vaccine"), vaccine3Pubkey.toBuffer()],
      program.programId
    )[0];

    const ix1 = await program.methods
      .createVaccine()
      .accounts({
        user: manufacturer.publicKey,
        userPda,
        batch: batchPubkey,
        batchPda,
        vaccine: vaccine1Pubkey,
        vaccinePda: vaccine1Pda,
      })
      .instruction();

    const ix2 = await program.methods
      .createVaccine()
      .accounts({
        user: manufacturer.publicKey,
        userPda,
        batch: batchPubkey,
        batchPda,
        vaccine: vaccine2Pubkey,
        vaccinePda: vaccine2Pda,
      })
      .instruction();

    const ix3 = await program.methods
      .createVaccine()
      .accounts({
        user: manufacturer.publicKey,
        userPda,
        batch: batchPubkey,
        batchPda,
        vaccine: vaccine3Pubkey,
        vaccinePda: vaccine3Pda,
      })
      .instruction();

    const tx = new anchor.web3.Transaction();
    tx.add(ix1);
    tx.add(ix2);
    tx.add(ix3);

    const sig = await connection.sendTransaction(tx, [manufacturer]);

    await connection.confirmTransaction(sig, "confirmed");

    const vaccine1PdaAccount = await program.account.vaccine.fetch(vaccine1Pda);
    const vaccine2PdaAccount = await program.account.vaccine.fetch(vaccine2Pda);
    const vaccine3PdaAccount = await program.account.vaccine.fetch(vaccine3Pda);

    assert.equal(
      vaccine1PdaAccount.pubkey.toBase58(),
      vaccine1Pubkey.toBase58()
    );
    assert.equal(
      vaccine2PdaAccount.pubkey.toBase58(),
      vaccine2Pubkey.toBase58()
    );
    assert.equal(
      vaccine3PdaAccount.pubkey.toBase58(),
      vaccine3Pubkey.toBase58()
    );
    assert.equal(vaccine1PdaAccount.batch.toBase58(), batchPubkey.toBase58());
    assert.equal(vaccine2PdaAccount.batch.toBase58(), batchPubkey.toBase58());
    assert.equal(vaccine3PdaAccount.batch.toBase58(), batchPubkey.toBase58());
    assert.equal(vaccine1PdaAccount.used, false);
    assert.equal(vaccine2PdaAccount.used, false);
    assert.equal(vaccine3PdaAccount.used, false);
    assert.ok(!vaccine1PdaAccount.usedAt);
    assert.ok(!vaccine2PdaAccount.usedAt);
    assert.ok(!vaccine3PdaAccount.usedAt);
    assert.ok(!vaccine1PdaAccount.usedBy);
    assert.ok(!vaccine2PdaAccount.usedBy);
    assert.ok(!vaccine3PdaAccount.usedBy);
  });

  it("distributor can receive consignment", async () => {
    const userPda = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("user"), distributor.publicKey.toBuffer()],
      program.programId
    )[0];

    const batchPda = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("batch"), batchPubkey.toBuffer()],
      program.programId
    )[0];

    const vaultPda = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vault"), batchPubkey.toBuffer(), tokenMint.toBuffer()],
      program.programId
    )[0];

    const distributorAta = await getOrCreateAssociatedTokenAccount(
      connection,
      distributor,
      tokenMint,
      distributor.publicKey
    );

    const manufacturerAta = await getOrCreateAssociatedTokenAccount(
      connection,
      distributor,
      tokenMint,
      manufacturer.publicKey
    );

    const sig = await program.methods
      .distributorReceive()
      .accounts({
        batch: batchPubkey,
        batchPda,
        distributorTokenAccount: distributorAta.address,
        manufacturerTokenAccount: manufacturerAta.address,
        mint: tokenMint,
        user: distributor.publicKey,
        userPda,
        vault: vaultPda,
      })
      .signers([distributor])
      .rpc();

    await connection.confirmTransaction(sig, "confirmed");

    const distributorAtaAccount = await getAccount(
      connection,
      distributorAta.address
    );

    assert.equal(
      distributorAtaAccount.amount.toString(),
      (1000 * 10 ** 9 - (200 * 3 + 100 * 3) * 10 ** 9).toString()
    );

    const manufacturerAtaAccount = await getAccount(
      connection,
      manufacturerAta.address
    );

    assert.equal(
      manufacturerAtaAccount.amount.toString(),
      (200 * 3 * 10 ** 9).toString()
    );

    const vaultAtaAccount = await getAccount(connection, vaultPda);

    assert.equal(
      vaultAtaAccount.amount.toString(),
      (100 * 3 * 10 ** 9).toString()
    );

    const batchPdaAccount = await program.account.batch.fetch(batchPda);

    assert.ok(batchPdaAccount.status.storedByDistributor);
    assert.ok(batchPdaAccount.startDate.toNumber() > 0);
  });

  it("doctor can receive", async () => {
    const userPda = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("user"), doctor.publicKey.toBuffer()],
      program.programId
    )[0];

    const batchPda = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("batch"), batchPubkey.toBuffer()],
      program.programId
    )[0];

    const vaultPda = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vault"), batchPubkey.toBuffer(), tokenMint.toBuffer()],
      program.programId
    )[0];

    const distributorAta = await getOrCreateAssociatedTokenAccount(
      connection,
      distributor,
      tokenMint,
      distributor.publicKey
    );

    const doctorAta = await getOrCreateAssociatedTokenAccount(
      connection,
      distributor,
      tokenMint,
      doctor.publicKey
    );

    const sig = await program.methods
      .doctorReceive()
      .accounts({
        batch: batchPubkey,
        batchPda,
        distributorTokenAccount: distributorAta.address,
        doctorTokenAccount: doctorAta.address,
        mint: tokenMint,
        user: doctor.publicKey,
        userPda,
        vault: vaultPda,
      })
      .signers([doctor])
      .rpc();

    await connection.confirmTransaction(sig, "confirmed");

    const distributorAtaAccount = await getAccount(
      connection,
      distributorAta.address
    );

    const doctorAtaAccount = await getAccount(connection, doctorAta.address);

    const vaultAtaAccount = await getAccount(connection, vaultPda);

    const batchPdaAccount = await program.account.batch.fetch(batchPda);

    assert.ok(batchPdaAccount.status.receivedByDoctor);
    assert.ok(batchPdaAccount.stopDate.toNumber() > 0);

    assert.equal(
      distributorAtaAccount.amount.toString(),
      ((1000 + 3 * 10) * 10 ** 9).toString()
    );

    assert.equal(
      doctorAtaAccount.amount.toString(),
      (1000 * 10 ** 9 - (200 + 10) * 3 * 10 ** 9).toString()
    );

    assert.equal(vaultAtaAccount.amount.toString(), "0");
  });

  it("can create a temp log", async () => {
    const userPda = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("user"), distributor.publicKey.toBuffer()],
      program.programId
    )[0];

    const batchPda = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("batch"), batchPubkey.toBuffer()],
      program.programId
    )[0];

    const tempLogPda = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("temp_log"), batchPubkey.toBuffer()],
      program.programId
    )[0];

    await program.methods
      .tempLog(300, "hello")
      .accounts({
        batch: batchPubkey,
        batchPda,
        tempLog: tempLogPda,
        user: distributor.publicKey,
        userPda,
      })
      .signers([distributor])
      .rpc();

    const tempLogPdaAccount = await program.account.tempLog.fetch(tempLogPda);

    assert.equal(tempLogPdaAccount.temp, 300);
    assert.equal(tempLogPdaAccount.id, "hello");
    assert.equal(tempLogPdaAccount.batch.toBase58(), batchPubkey.toBase58());
  });

  it("can use a vaccine", async () => {
    const userPda = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("user"), doctor.publicKey.toBuffer()],
      program.programId
    )[0];

    const vaccine1Pda = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vaccine"), vaccine1Pubkey.toBuffer()],
      program.programId
    )[0];

    const vaccine2Pda = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vaccine"), vaccine2Pubkey.toBuffer()],
      program.programId
    )[0];

    const vaccine3Pda = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vaccine"), vaccine3Pubkey.toBuffer()],
      program.programId
    )[0];

    await program.methods
      .useVaccine()
      .accounts({
        user: doctor.publicKey,
        userPda,
        vaccine: vaccine1Pubkey,
        vaccinePda: vaccine1Pda,
      })
      .signers([doctor])
      .rpc();

    await program.methods
      .useVaccine()
      .accounts({
        user: doctor.publicKey,
        userPda,
        vaccine: vaccine2Pubkey,
        vaccinePda: vaccine2Pda,
      })
      .signers([doctor])
      .rpc();

    await program.methods
      .useVaccine()
      .accounts({
        user: doctor.publicKey,
        userPda,
        vaccine: vaccine3Pubkey,
        vaccinePda: vaccine3Pda,
      })
      .signers([doctor])
      .rpc();

    const vaccine1PdaAccount = await program.account.vaccine.fetch(vaccine1Pda);
    const vaccine2PdaAccount = await program.account.vaccine.fetch(vaccine2Pda);
    const vaccine3PdaAccount = await program.account.vaccine.fetch(vaccine3Pda);

    assert.equal(vaccine1PdaAccount.used, true);
    assert.equal(vaccine2PdaAccount.used, true);
    assert.equal(vaccine3PdaAccount.used, true);
    assert.ok(vaccine1PdaAccount.usedAt);
    assert.ok(vaccine2PdaAccount.usedAt);
    assert.ok(vaccine3PdaAccount.usedAt);
    assert.equal(
      vaccine1PdaAccount.usedBy.toBase58(),
      doctor.publicKey.toBase58()
    );
    assert.equal(
      vaccine2PdaAccount.usedBy.toBase58(),
      doctor.publicKey.toBase58()
    );
    assert.equal(
      vaccine3PdaAccount.usedBy.toBase58(),
      doctor.publicKey.toBase58()
    );
  });
});
