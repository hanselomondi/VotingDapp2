import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { VotingDapp } from "../target/types/voting_dapp";
import { PublicKey, SystemProgram, Transaction } from "@solana/web3.js";
import { expect } from "chai";

describe("voting_dapp", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.VotingDapp as Program<VotingDapp>;

  const state = {
    counterAddress: null as PublicKey | null,
    counterAccount: null as any,
    regAddress: null as PublicKey | null,
    regAccount: null as any
  }

  before(async () => {
    const [counterAddress] = PublicKey.findProgramAddressSync(
      [Buffer.from("counter")],
      program.programId
    );
    const [regAddress] = PublicKey.findProgramAddressSync(
      [Buffer.from("registrations")],
      program.programId
    );

    state.counterAddress = counterAddress;
    state.regAddress = regAddress;

    try {
      state.counterAccount = await program.account.counter.fetch(counterAddress);
      state.regAccount = await program.account.registrations.fetch(regAddress);
    } catch(err) {
      console.log("Platform has not yet been initialised");
    }
  })

  // Initialise the platform
  it("Initialise", async () => {
    if (state.counterAccount == null) {
      console.log("Platform not initialised yet. Initialising...");
      await program.methods.initialiseCounters().rpc();

      console.log(state.counterAccount);
      console.log(state.regAccount);
  
      expect(state.counterAccount.count.toNumber()).equal(0);
      expect(state.regAccount.count.toNumber()).equal(0);
    } else {
      console.log(state.counterAccount);
      console.log(state.regAccount);
      expect(state.counterAccount.count.toNumber()).greaterThan(0);
      expect(state.regAccount.count.toNumber()).equal(0);
    }
    
  })

  // Initialise a poll
  it("Initialise Poll", async () => {
    const [pollAddress, pollBump] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("poll"),
        state.counterAccount.count.add(new anchor.BN(1)).toArrayLike(Buffer, 'le', 8)
      ],
      program.programId
    );
    await program.methods.initialisePoll(
      "What is your favourite type of peanut butter?",
      new anchor.BN(Date.UTC(2024, 11, 20, 12, 0, 0, 0) / 1000),
      new anchor.BN(Date.UTC(2024, 11, 30, 23, 59, 59, 0) / 1000)
    )
    .accounts({
      signer: program.provider.publicKey,
      poll: pollAddress,
      counter: state.counterAddress,
      system_program: SystemProgram.programId
    })
    .rpc();

    const poll = await program.account.poll.fetch(pollAddress);
    console.log(poll);
    
    state.counterAccount = await program.account.counter.fetch(state.counterAddress);
    state.regAccount = await program.account.registrations.fetch(state.regAddress);
    console.log(state.counterAccount);
    console.log(state.regAccount);

    expect(poll.pollDescription).equal("What is your favourite type of peanut butter?");
    expect(poll.candidateCount.toNumber()).equal(0);
    expect(poll.pollStart.toNumber()).greaterThan(new anchor.BN(Date.now() / 1000).toNumber())
  })
});
