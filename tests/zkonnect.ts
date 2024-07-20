import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Zkonnect } from "../target/types/zkonnect";
import { randomBytes } from "crypto";

// Import the assertion library
import { assert } from "chai";
import { TOKEN_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";

describe("zkonnect", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.local();
  anchor.setProvider(provider);

  const program = anchor.workspace.zkonnect as Program<Zkonnect>;

  it('Creates an event', async () => {
    // Generate a new keypair for the event
    const eventKeypair = anchor.web3.Keypair.generate();

    // Define the parameters for the create_event function
    const seed = new anchor.BN(randomBytes(8));
    const creatorName = "Test Creator";
    const creatorDomain = "test.domain";
    const eventName = "Test Event";
    const banner = "Test Banner";
    const dateTime = new anchor.BN(Math.floor(Date.now() / 1000));
    const location = "Test Location";
    const ticketPrice = new anchor.BN(1);
    const totalTickets = new anchor.BN(100);

    // Derive the PDA for the event account
    const [eventPDA, bump] = await anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("zkonnect"), provider.wallet.publicKey.toBuffer(), seed.toArrayLike(Buffer, 'le', 8)],
        program.programId
    );

    // Call the create_event function
    // await program.methods
    //     .createEvent(
    //       seed,
    //       "manice18",
    //       "CTO",
    //       "Cholo",
    //       "bye",
    //       new anchor.BN(Date.now()),
    //       "hi",
    //       new anchor.BN(1),
    //       new anchor.BN(100),
    //     )
    //     .accountsPartial({
    //       creator: provider.wallet.publicKey,
    //       mint: new anchor.web3.PublicKey("4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU"),
    //       tokenProgram: TOKEN_PROGRAM_ID,
    //       event: eventPDA,
    //     })
    //     .rpc();

    // Fetch the event account and assert the values
    const eventAccount = await program.account.event.all();

    console.log("Event Account:", eventAccount);

    // Add assertions to verify the event account values
    // assert.equal(eventAccount[0].creatorName, creatorName);
    // assert.equal(eventAccount.creatorDomain, creatorDomain);
    // assert.equal(eventAccount.name, eventName);
    // assert.equal(eventAccount.banner, banner);
    // assert.ok(eventAccount.dateTime.eq(dateTime));
    // assert.equal(eventAccount.location, location);
    // assert.ok(eventAccount.ticketPrice.eq(ticketPrice));
    // assert.equal(eventAccount.totalTickets, totalTickets);
});
});
