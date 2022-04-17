import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Ido } from "../target/types/ido";

describe("ido", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.Ido as Program<Ido>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
