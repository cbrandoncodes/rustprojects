"use client";

import { AnchorProvider, BN, Program } from "@coral-xyz/anchor";
import { PublicKey, Connection } from "@solana/web3.js";
import { useState, useTransition } from "react";

import { TODO_IDL, TODO_PROGRAM_ID, type TodoIdl } from "@/lib/todo-idl";
import { RPC_ENDPOINT } from "@/lib/constants";
import { BrowserWallet } from "@/types";
import {
  formatTimestamp,
  getWalletAdapter,
  toNumber,
} from "@/lib/utils";

const connection = new Connection(RPC_ENDPOINT, "confirmed");

type TaskAccount = {
  taskId: BN | bigint | number;
  content: string;
  completed: boolean;
  author: PublicKey;
  createdAt: BN | bigint | number;
};

type TaskRecord = {
  publicKey: PublicKey;
  taskId: number;
  content: string;
  completed: boolean;
  createdAt: number;
};

declare global {
  interface Window {
    solana?: BrowserWallet;
  }
}

export function Todo() {
  const [wallet] = useState<BrowserWallet | null>(() => {
    if (typeof window === "undefined") {
      return null;
    }

    return window.solana ?? null;
  });
  const [tasks, setTasks] = useState<TaskRecord[]>([]);
  const [draft, setDraft] = useState("");
  const [editingTaskId, setEditingTaskId] = useState<number | null>(null);
  const [editingContent, setEditingContent] = useState("");
  const [statusMessage, setStatusMessage] = useState(() => {
    if (typeof window === "undefined" || !window.solana) {
      return "Install Phantom or another injected wallet to begin.";
    }

    return window.solana.isConnected && window.solana.publicKey
      ? "Wallet detected. Refresh to load your tasks."
      : "Wallet available. Connect to load your tasks.";
  });
  const [errorMessage, setErrorMessage] = useState<string | null>(null);
  const [isPending, startTransition] = useTransition();

  const program: Program<TodoIdl> | null = wallet?.publicKey
    ? new Program<TodoIdl>(
        TODO_IDL,
        new AnchorProvider(connection, getWalletAdapter(wallet), {
          commitment: "confirmed",
          preflightCommitment: "confirmed",
        }),
      )
    : null;

  const loadTasks = async (
    activeProgram: Program<TodoIdl>,
    author: PublicKey,
  ) => {
    const accounts = await activeProgram.account.task.all();
    const nextTasks = accounts
      .filter((account: { account: TaskAccount }) =>
        account.account.author.equals(author),
      )
      .map(
        (account: { publicKey: PublicKey; account: TaskAccount }) =>
          ({
            publicKey: account.publicKey,
            taskId: toNumber(account.account.taskId),
            content: account.account.content,
            completed: account.account.completed,
            createdAt: toNumber(account.account.createdAt),
          }) satisfies TaskRecord,
      )
      .sort((left: TaskRecord, right: TaskRecord) => right.createdAt - left.createdAt);

    setTasks(nextTasks);
    setStatusMessage(
      nextTasks.length
        ? "Tasks loaded from the Anchor program."
        : "No on-chain tasks yet. Create the first one.",
    );
  };

  const refreshTasks = () => {
    if (!program || !wallet?.publicKey) {
      setErrorMessage("Connect your wallet before loading tasks.");
      return;
    }
    const publicKey = wallet.publicKey;

    startTransition(() => {
      void loadTasks(program, publicKey).catch((error: unknown) => {
        setErrorMessage(
          error instanceof Error ? error.message : "Failed to load tasks.",
        );
      });
    });
  };

  const connectWallet = () => {
    if (!wallet) {
      setErrorMessage("No injected wallet was found in this browser.");
      return;
    }

    startTransition(() => {
      void wallet
        .connect()
        .then(() => {
          setErrorMessage(null);
          setStatusMessage("Wallet connected. Refresh to sync tasks.");
        })
        .catch((error: unknown) => {
          setErrorMessage(
            error instanceof Error
              ? error.message
              : "Wallet connection failed.",
          );
        });
    });
  };

  const disconnectWallet = () => {
    if (!wallet) {
      return;
    }

    startTransition(() => {
      void wallet.disconnect().finally(() => {
        setTasks([]);
        setStatusMessage("Wallet disconnected.");
      });
    });
  };

  const createTask = () => {
    if (!program || !wallet?.publicKey) {
      setErrorMessage("Connect your wallet before creating a task.");
      return;
    }
    const publicKey = wallet.publicKey;

    const content = draft.trim();
    if (!content || content.length > 200) {
      setErrorMessage("Task content must be between 1 and 200 characters.");
      return;
    }

    const nextTaskId =
      tasks.reduce((max, task) => Math.max(max, task.taskId), -1) + 1;

    startTransition(() => {
      void program.methods
        .createTask(new BN(nextTaskId), content)
        .accounts({
          author: publicKey,
        })
        .rpc()
        .then(async () => {
          setDraft("");
          setErrorMessage(null);
          await loadTasks(program, publicKey);
        })
        .catch((error: unknown) => {
          setErrorMessage(
            error instanceof Error
              ? error.message
              : "Failed to create the task.",
          );
        });
    });
  };

  const updateTask = (task: TaskRecord) => {
    if (!program || !wallet?.publicKey) {
      return;
    }
    const publicKey = wallet.publicKey;

    const content = editingContent.trim();
    if (!content || content.length > 200) {
      setErrorMessage("Task content must be between 1 and 200 characters.");
      return;
    }

    startTransition(() => {
      void program.methods
        .updateTask(content)
        .accounts({
          task: task.publicKey,
        })
        .rpc()
        .then(async () => {
          setEditingTaskId(null);
          setEditingContent("");
          setErrorMessage(null);
          await loadTasks(program, publicKey);
        })
        .catch((error: unknown) => {
          setErrorMessage(
            error instanceof Error
              ? error.message
              : "Failed to update the task.",
          );
        });
    });
  };

  const toggleTask = (task: TaskRecord) => {
    if (!program || !wallet?.publicKey) {
      return;
    }
    const publicKey = wallet.publicKey;

    startTransition(() => {
      void program.methods
        .toggleComplete()
        .accounts({
          task: task.publicKey,
        })
        .rpc()
        .then(async () => {
          setErrorMessage(null);
          await loadTasks(program, publicKey);
        })
        .catch((error: unknown) => {
          setErrorMessage(
            error instanceof Error
              ? error.message
              : "Failed to toggle the task.",
          );
        });
    });
  };

  const deleteTask = (task: TaskRecord) => {
    if (!program || !wallet?.publicKey) {
      return;
    }
    const publicKey = wallet.publicKey;

    startTransition(() => {
      void program.methods
        .deleteTask()
        .accounts({
          task: task.publicKey,
        })
        .rpc()
        .then(async () => {
          setErrorMessage(null);
          await loadTasks(program, publicKey);
        })
        .catch((error: unknown) => {
          setErrorMessage(
            error instanceof Error
              ? error.message
              : "Failed to delete the task.",
          );
        });
    });
  };

  return (
    <main className="min-h-screen px-5 py-8 sm:px-8 lg:px-12">
      <div className="mx-auto flex w-full max-w-6xl flex-col gap-8">
        <section className="grid gap-6 rounded-4xl border border-(--border) bg-(--surface) p-6 shadow-[0_30px_80px_rgba(89,52,34,0.12)] backdrop-blur md:grid-cols-[1.35fr_0.95fr] md:p-10">
          <div className="space-y-5">
            <p className="text-(--accent-strong) text-sm uppercase tracking-[0.3em]">
              Anchor x Next.js
            </p>
            <h1 className="max-w-2xl font-(--font-fraunces) text-5xl leading-none text-foreground sm:text-6xl">
              On-chain tasks, shaped for actual use.
            </h1>
            <p className="max-w-2xl text-(--muted) text-lg leading-8">
              The UI talks directly to your Anchor todo program, derives each
              task PDA from the connected wallet and task id, and keeps edits,
              toggles, and deletes on-chain.
            </p>
            <div className="flex flex-wrap gap-3">
              <button
                className="bg-(--accent) hover:bg-(--accent-strong) rounded-full px-5 py-3 font-semibold text-white transition disabled:cursor-not-allowed disabled:opacity-60"
                onClick={wallet?.publicKey ? disconnectWallet : connectWallet}
                disabled={isPending}
              >
                {wallet?.publicKey ? "Disconnect wallet" : "Connect wallet"}
              </button>
              <button
                className="rounded-full border border-(--border) bg-white/70 px-5 py-3 font-semibold text-foreground transition hover:bg-white disabled:cursor-not-allowed disabled:opacity-60"
                onClick={refreshTasks}
                disabled={!wallet?.publicKey || isPending}
              >
                Refresh tasks
              </button>
            </div>
          </div>

          <div className="rounded-3xl border border-(--border) bg-(--surface-strong) p-5 shadow-[inset_0_1px_0_rgba(255,255,255,0.6)]">
            <dl className="space-y-4 text-(--muted) text-sm">
              <div>
                <dt className="text-(--accent-strong) text-[11px] uppercase tracking-[0.24em]">
                  Program
                </dt>
                <dd className="mt-1 break-all text-[13px] leading-6 text-foreground">
                  {TODO_PROGRAM_ID}
                </dd>
              </div>
              <div>
                <dt className="text-(--accent-strong) text-[11px] uppercase tracking-[0.24em]">
                  RPC
                </dt>
                <dd className="mt-1 break-all text-[13px] leading-6 text-foreground">
                  {RPC_ENDPOINT}
                </dd>
              </div>
              <div>
                <dt className="text-(--accent-strong) text-[11px] uppercase tracking-[0.24em]">
                  Wallet
                </dt>
                <dd className="mt-1 break-all text-[13px] leading-6 text-foreground">
                  {wallet?.publicKey?.toBase58() ?? "No wallet connected"}
                </dd>
              </div>
              <div>
                <dt className="text-(--accent-strong) text-[11px] uppercase tracking-[0.24em]">
                  Status
                </dt>
                <dd className="mt-1 text-[13px] leading-6 text-foreground">
                  {statusMessage}
                </dd>
              </div>
              {errorMessage ? (
                <div className="rounded-2xl border border-red-200 bg-red-50 px-4 py-3 text-red-700">
                  {errorMessage}
                </div>
              ) : null}
            </dl>
          </div>
        </section>

        <section className="grid gap-6 lg:grid-cols-[0.85fr_1.15fr]">
          <div className="rounded-4xl border border-(--border) bg-(--surface) p-6 shadow-[0_20px_60px_rgba(89,52,34,0.08)]">
            <div className="space-y-3">
              <p className="text-(--accent-strong) text-sm uppercase tracking-[0.24em]">
                Create task
              </p>
              <h2 className="font-(--font-fraunces) text-3xl text-foreground">
                Write once, send once.
              </h2>
              <p className="text-(--muted) text-sm leading-7">
                Content is validated against the program limit of 200 characters
                before the transaction is sent.
              </p>
            </div>
            <div className="mt-6 space-y-4">
              <textarea
                className="min-h-40 w-full rounded-3xl border border-(--border) bg-white/80 px-4 py-4 text-base text-foreground outline-none ring-0 placeholder:text-(--muted) focus:border-(--accent)"
                placeholder="Plan migration, close audits, ship feature flags..."
                value={draft}
                onChange={(event) => setDraft(event.target.value)}
                maxLength={200}
              />
              <div className="flex items-center justify-between text-(--muted) text-sm">
                <span>{draft.trim().length}/200</span>
                <button
                  className="bg-foreground hover:bg-(--accent-strong) rounded-full px-5 py-3 font-semibold text-white transition disabled:cursor-not-allowed disabled:opacity-60"
                  onClick={createTask}
                  disabled={!wallet?.publicKey || isPending}
                >
                  Create on-chain task
                </button>
              </div>
            </div>
          </div>

          <div className="rounded-4xl border border-(--border) bg-(--surface) p-6 shadow-[0_20px_60px_rgba(89,52,34,0.08)]">
            <div className="flex items-end justify-between gap-4">
              <div>
                <p className="text-(--accent-strong) text-sm uppercase tracking-[0.24em]">
                  Task ledger
                </p>
                <h2 className="font-(--font-fraunces) text-3xl text-foreground">
                  Your PDA-backed queue.
                </h2>
              </div>
              <span className="rounded-full border border-(--border) bg-white/70 px-4 py-2 text-(--muted) text-sm">
                {tasks.length} task{tasks.length === 1 ? "" : "s"}
              </span>
            </div>

            <div className="mt-6 space-y-4">
              {tasks.length ? (
                tasks.map((task) => {
                  const isEditing = editingTaskId === task.taskId;

                  return (
                    <article
                      key={task.publicKey.toBase58()}
                      className="rounded-3xl border border-(--border) bg-white/80 p-5 shadow-[inset_0_1px_0_rgba(255,255,255,0.8)]"
                    >
                      <div className="flex flex-wrap items-start justify-between gap-3">
                        <div>
                          <p className="text-(--accent-strong) text-xs uppercase tracking-[0.24em]">
                            Task #{task.taskId}
                          </p>
                          <p className="mt-2 text-(--muted) text-sm">
                            Created {formatTimestamp(task.createdAt)}
                          </p>
                        </div>
                        <span
                          className={`rounded-full px-3 py-1 text-xs font-semibold uppercase tracking-[0.18em] ${
                            task.completed
                              ? "bg-emerald-100 text-emerald-700"
                              : "bg-amber-100 text-amber-700"
                          }`}
                        >
                          {task.completed ? "done" : "open"}
                        </span>
                      </div>

                      {isEditing ? (
                        <div className="mt-4 space-y-3">
                          <textarea
                            className="min-h-28 w-full rounded-2xl border border-(--border) bg-[#fffdf9] px-4 py-3 outline-none focus:border-(--accent)"
                            value={editingContent}
                            maxLength={200}
                            onChange={(event) =>
                              setEditingContent(event.target.value)
                            }
                          />
                          <div className="flex flex-wrap gap-3">
                            <button
                              className="bg-(--accent) hover:bg-(--accent-strong) rounded-full px-4 py-2 text-sm font-semibold text-white transition disabled:opacity-60"
                              onClick={() => updateTask(task)}
                              disabled={isPending}
                            >
                              Save update
                            </button>
                            <button
                              className="rounded-full border border-(--border) px-4 py-2 text-sm font-semibold text-foreground transition hover:bg-white"
                              onClick={() => {
                                setEditingTaskId(null);
                                setEditingContent("");
                              }}
                              disabled={isPending}
                            >
                              Cancel
                            </button>
                          </div>
                        </div>
                      ) : (
                        <p className="mt-4 text-base leading-8 text-foreground">
                          {task.content}
                        </p>
                      )}

                      <div className="mt-5 flex flex-wrap gap-3">
                        <button
                          className="rounded-full border border-(--border) px-4 py-2 text-sm font-semibold text-foreground transition hover:bg-white disabled:opacity-60"
                          onClick={() => toggleTask(task)}
                          disabled={isPending}
                        >
                          {task.completed ? "Mark open" : "Mark complete"}
                        </button>
                        <button
                          className="rounded-full border border-(--border) px-4 py-2 text-sm font-semibold text-foreground transition hover:bg-white disabled:opacity-60"
                          onClick={() => {
                            setEditingTaskId(task.taskId);
                            setEditingContent(task.content);
                          }}
                          disabled={isPending}
                        >
                          Edit
                        </button>
                        <button
                          className="rounded-full border border-red-200 px-4 py-2 text-sm font-semibold text-red-700 transition hover:bg-red-50 disabled:opacity-60"
                          onClick={() => deleteTask(task)}
                          disabled={isPending}
                        >
                          Delete
                        </button>
                      </div>
                    </article>
                  );
                })
              ) : (
                <div className="rounded-3xl border border-(--border) border-dashed bg-white/60 px-5 py-10 text-center text-(--muted)">
                  Refresh after connecting your wallet, then create a task to
                  seed the list.
                </div>
              )}
            </div>
          </div>
        </section>
      </div>
    </main>
  );
}
