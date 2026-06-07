import { BN } from "@coral-xyz/anchor";
import type { Wallet as AnchorWallet } from "@coral-xyz/anchor/dist/esm/provider";
import { PublicKey } from "@solana/web3.js";
import { TODO_PROGRAM_ID } from "@/lib/todo-idl";
import { TASK_SEED_PREFIX } from "@/lib/constants";
import { BrowserWallet } from "@/types";

export function toNumber(value: BN | bigint | number) {
  if (typeof value === "number") {
    return value;
  }

  if (typeof value === "bigint") {
    return Number(value);
  }

  return value.toNumber();
}

export function formatTimestamp(timestamp: number) {
  return new Intl.DateTimeFormat("en", {
    dateStyle: "medium",
    timeStyle: "short",
  }).format(new Date(timestamp * 1000));
}

export function getWalletAdapter(wallet: BrowserWallet): AnchorWallet {
  const publicKey = wallet?.publicKey ?? null;
  if (!publicKey) {
    throw new Error("No public key on wallet");
  }

  return {
    publicKey,
    signTransaction: wallet.signTransaction.bind(wallet),
    signAllTransactions:
      wallet.signAllTransactions?.bind(wallet) ??
      (async (transactions) =>
        Promise.all(
          transactions.map((transaction) =>
            wallet.signTransaction(transaction),
          ),
        )),
  };
}

export function deriveTaskPda(author: PublicKey, taskId: number) {
  return PublicKey.findProgramAddressSync(
    [
      TASK_SEED_PREFIX,
      author.toBytes(),
      Uint8Array.from(new BN(taskId).toArray("le", 8)),
    ],
    new PublicKey(TODO_PROGRAM_ID),
  )[0];
}
