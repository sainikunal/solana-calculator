import * as anchor from "@coral-xyz/anchor";
import { Program, Wallet } from "@coral-xyz/anchor";
import { Connection, Keypair, PublicKey } from "@solana/web3.js";
import { SolanaCalculator } from "../target/types/solana_calculator";
import { readFileSync } from "fs";
import assert from "assert";

describe("solana-calculator", () => {
  // Configure the client to use the local cluster.
  // anchor.setProvider(anchor.AnchorProvider.env());

  // const keyFilePath = "/home/kunal.saini/.config/solana/id.json";
  // const keypair = Keypair.fromSecretKey(
  //   new Uint8Array(JSON.parse(readFileSync(keyFilePath, 'utf-8')))
  // );

  // const wallet = new Wallet(keypair);
  // const connection = new Connection("http://localhost:8899", "confirmed")
  // const provider = new anchor.AnchorProvider(connection, wallet, {
  //   preflightCommitment: "confirmed",
  // })
  // const provider = anchor.getProvider();

  // anchor.setProvider(provider);
  // console.log(`Wallet public key: ${provider.wallet.publicKey.toBase58()}`)
  // console.log(`User wallet public key: ${wallet.publicKey.toBase58()}`)
  // const calculator = anchor.web3.Keypair.generate();

  const program = anchor.workspace.SolanaCalculator as Program<SolanaCalculator>;

  const [calculatorPDA] = PublicKey.findProgramAddressSync(
    [Buffer.from("calculator")],
    program.programId,
  )

  it("Is initialized!", async () => {
    // Add your test here.

    const greeting = "Calculator ON";
    await program.methods.initializeCalculator(greeting)
    .rpc();

    const account = await program.account.calculator.fetch(calculatorPDA);
    assert.equal(account.greeting, greeting);
  });

  it("Addition of numbers", async () => {
    await program.methods.addition(2, 3)
    .accounts({
      calculator: calculatorPDA,
    })
    .rpc();

    const account = await program.account.calculator.fetch(calculatorPDA);
    assert.equal(5, account.result);
  });

  it("Addition with args struct", async () => {
    const args = {
      num1: 5,
      num2: 3,
    }
    await program.methods.additionWithArgs(args)
      .accounts({
        calculator: calculatorPDA,
      })
      .rpc();

    const account = await program.account.calculator.fetch(calculatorPDA);
    assert.equal(8, account.result)
  })

});
