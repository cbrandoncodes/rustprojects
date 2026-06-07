import { PublicKey } from "@solana/web3.js";
import type { Wallet as AnchorWallet } from "@coral-xyz/anchor/dist/esm/provider";

export type BrowserWallet = {
  isPhantom?: boolean;
  isConnected?: boolean;
  publicKey?: PublicKey | null;
  connect: (options?: {
    onlyIfTrusted?: boolean;
  }) => Promise<{ publicKey: PublicKey }>;
  disconnect: () => Promise<void>;
  on?: (
    event: "connect" | "disconnect" | "accountChanged",
    callback: () => void,
  ) => void;
  removeListener?: (
    event: "connect" | "disconnect" | "accountChanged",
    callback: () => void,
  ) => void;
  signTransaction: AnchorWallet["signTransaction"];
  signAllTransactions?: AnchorWallet["signAllTransactions"];
};
