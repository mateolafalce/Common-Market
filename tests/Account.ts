import * as anchor from "@project-serum/anchor";
const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);
export const wallet = provider.wallet as anchor.Wallet;

export const MainPDA = new anchor.web3.PublicKey(
    "3C5kGxu33aJmr1NW7bqAuyLTde1jjnWBfnCYMHiqDqtb"
);
export const Offer = new anchor.web3.PublicKey(
    "23RPaRHz6B1nkPMpzsxtbvXGr6jACgpGZAodtsGFu6wa"
);