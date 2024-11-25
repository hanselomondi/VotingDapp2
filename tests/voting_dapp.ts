import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { VotingDapp } from "../target/types/voting_dapp";
import { PublicKey, SystemProgram, Transaction } from "@solana/web3.js";
import { expect } from "chai";

describe("voting_dapp", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.VotingDapp as Program<VotingDapp>;

  // Initialise the platform
  it("Initialise", async () => {
    let counter;
    let registrations;

    const [counterAddress] = PublicKey.findProgramAddressSync(
      [Buffer.from("counter")],
      program.programId
    );
    
    const [registrationsAddress] = PublicKey.findProgramAddressSync(
      [Buffer.from("registrations")],
      program.programId
    )

    try {
      counter = await program.account.counter.fetch(counterAddress);
      registrations = await program.account.registrations.fetch(registrationsAddress);
      console.log("Platform already initialised");
      console.log(counter);
      console.log(registrations);
    } catch(err) {
      console.log("Platform not initialised yet. Initialising...");
      await program.methods.initialiseCounters().rpc();

    }

    expect(counter.count.toNumber()).equal(0);
    expect(registrations.count.toNumber()).equal(0);
  })
});
