import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Vault } from "../target/types/vault";
import crypto from "crypto";

describe("crypto-task-capstone", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.CryptoTaskCapstone as Program<Vault>;

  const seed = new anchor.BN(123);
  const amount = new anchor.BN(2_000_000_0);
  const withdrawAmount = new anchor.BN(1_000_000_0);
  const keyword = "secret"; // The keyword to be hashed

  // Function to hash the keyword
  const hashKeyword = (keyword: string): number[] => {
    const hash = crypto.createHash("sha256").update(keyword).digest();
    return Array.from(hash); // Convert Buffer to array of numbers
  };

  const hash = hashKeyword(keyword);

  const maker = anchor.web3.Keypair.generate();
  const taker = anchor.web3.Keypair.generate();



  const escrow = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("escrow"), maker.publicKey.toBytes(), seed.toArrayLike(Buffer, "le", 8)], program.programId)[0];
  const vault = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("escrow_vault"), escrow.toBytes()], program.programId)[0];


  console.log(escrow.toBase58());
  console.log(vault.toBase58());
  
  it("Aidrop Sol to maker", async () => {
    const tx = await provider.connection.requestAirdrop(maker.publicKey, 2000000000);
    await provider.connection.confirmTransaction(tx);
    console.log("Maker balance: ", await provider.connection.getBalance(maker.publicKey));
  });

  it("Initializes the escrow", async () => {
    const tx = await program.methods.initialize(seed, hash, amount).accountsPartial({
      user: maker.publicKey,
      escrowVault: vault,
      escrow: escrow,
      systemProgram: anchor.web3.SystemProgram.programId,
    }).signers([maker]).rpc();
    console.log("Initialization signature:", tx);
  });

  it("Makes a deposit!", async () => {
    // Capture the initial balance of the maker and the escrow vault
    const initialMakerBalance = await provider.connection.getBalance(maker.publicKey);
    const initialVaultBalance = await provider.connection.getBalance(vault);
  
    // Perform the deposit transaction
    const tx = await program.methods.deposit(amount).accountsPartial({
      user: maker.publicKey,
      escrow: escrow,
      escrowVault: vault,
      systemProgram: anchor.web3.SystemProgram.programId,
    }).signers([maker]).rpc();
  
    console.log("Deposit transaction signature:", tx);
  
    // Capture the final balance of the maker and the escrow vault
    const finalMakerBalance = await provider.connection.getBalance(maker.publicKey);
    const finalVaultBalance = await provider.connection.getBalance(vault);
  
    // Output the balances for debugging
    console.log("Initial maker balance: ", initialMakerBalance);
    console.log("Final maker balance: ", finalMakerBalance);
    console.log("Initial vault balance: ", initialVaultBalance);
    console.log("Final vault balance: ", finalVaultBalance);
  });

  it("Makes a withdraw!", async () => {
    console.log("Taker balance before: ", await provider.connection.getBalance(taker.publicKey));
    const tx = await program.methods.withdraw(withdrawAmount, keyword).accountsPartial({
      user: taker.publicKey,
      escrow: escrow,
      escrowVault: vault,
      systemProgram: anchor.web3.SystemProgram.programId,
    }).signers([taker]).rpc();
    console.log("Withdraw transaction signature:", tx);
    console.log("Taker balance after: ", await provider.connection.getBalance(taker.publicKey));
  });

  it("Closes the account!", async () => {
    console.log("Maker balance before: ", await provider.connection.getBalance(maker.publicKey));
    const tx = await program.methods.close(keyword).accountsPartial({
      user: maker.publicKey,
      escrow: escrow,
      escrowVault: vault,
      systemProgram: anchor.web3.SystemProgram.programId,
    }).signers([maker]).rpc();
    console.log("Close transaction signature:", tx);
    console.log("Maker balance after: ", await provider.connection.getBalance(maker.publicKey));
  });
});
