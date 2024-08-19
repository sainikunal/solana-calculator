import * as anchor from "@coral-xyz/anchor";
import { Program, Wallet } from "@coral-xyz/anchor";
import { Connection, Keypair } from "@solana/web3.js";
import { SolanaCalculator } from "../target/types/solana_calculator";
import { readFileSync } from "fs";
import assert from "assert";

describe("solana-calculator", () => {
  // Configure the client to use the local cluster.
  // anchor.setProvider(anchor.AnchorProvider.env());

  const keyFilePath = "/home/kunal.saini/.config/solana/id.json";
  const keypair = Keypair.fromSecretKey(
    new Uint8Array(JSON.parse(readFileSync(keyFilePath, 'utf-8')))
  );

  const wallet = new Wallet(keypair);
  const connection = new Connection("http://localhost:8899", "confirmed")
  const provider = new anchor.AnchorProvider(connection, wallet, {
    preflightCommitment: "confirmed",
  })
  anchor.setProvider(provider);


  const program = anchor.workspace.SolanaCalculator as Program<SolanaCalculator>;
  const calculator = anchor.web3.Keypair.generate();

  it("Is initialized!", async () => {
    // Add your test here.

    const greeting = "Calculator ON";
    await program.methods.initializeCalculator(greeting)
    .accounts({
      calculator: calculator.publicKey,
      user: provider.wallet.publicKey,
    })
    .signers([calculator])
    .rpc();

    const account = await program.account.calculator.fetch(calculator.publicKey);
    assert.equal(account.greeting, greeting);
  });

  it("Addition of numbers", async () => {
    await program.methods.addition(2, 3)
    .accounts({
      calculator: calculator.publicKey,
    })
    .rpc();

    const account = await program.account.calculator.fetch(calculator.publicKey);
    assert.equal(5, account.result);
  });

  it("Addition with args struct", async () => {
    const args = {
      num1: 5,
      num2: 3,
    }
    await program.methods.additionWithArgs(args)
      .accounts({
        calculator: calculator.publicKey,
      })
      .rpc();

    const account = await program.account.calculator.fetch(calculator.publicKey);
    assert.equal(8, account.result)
  })

});
