import * as anchor from "@project-serum/anchor";
const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);
export const wallet = provider.wallet as anchor.Wallet;

export const MainPDA = new anchor.web3.PublicKey(
    "7vDEz34wRcwk9Uh19LvtxtcdtqBeg9EwqB6oMDTt9oSt"
);
export const Offer = new anchor.web3.PublicKey(
    "3xXhj3Te7T2T469unjx9J2EgZT4Azhnb2BnTcKbvxtp5"
);