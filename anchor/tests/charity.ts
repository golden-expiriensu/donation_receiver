import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Charity } from "../target/types/charity";

describe("charity", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.Charity as Program<Charity>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.rpc.setData({});
    console.log("Your transaction signature", tx);
  });
});
