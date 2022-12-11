import * as anchor from "@project-serum/anchor";
import { Commonmarket } from "../target/types/commonmarket";
import { wallet, Offer, MainPDA } from "./Account";

describe("Update Supply", () => {
  const program = anchor.workspace.Commonmarket as anchor.Program<Commonmarket>;
    it("...", async () => {
      let amount = new anchor.BN(72);
      const tx = await program.methods.updateSupply(
        amount
      )
      .accounts({
          mainAccount: MainPDA,
          offer: Offer,
          user: wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        }).rpc();
    //const AccountData = await program.account.accountModel.fetch(Account);
    const SellAccount = await program.account.sell.fetch(Offer);
    console.log("---------------------------------------------") 
    console.log("Your transaction signature", tx);
    console.log("---------------------------------------------")
    console.log("Current Supply", SellAccount.supply.toString());
    console.log("---------------------------------------------")
    });
})