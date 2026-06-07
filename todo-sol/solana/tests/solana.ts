import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey, SystemProgram } from "@solana/web3.js";

describe("solana", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.solana as Program;
  const author = provider.wallet;

  const deriveTaskPda = (taskId: anchor.BN) =>
    PublicKey.findProgramAddressSync(
      [
        Buffer.from("task"),
        author.publicKey.toBuffer(),
        taskId.toArrayLike(Buffer, "le", 8),
      ],
      program.programId,
    )[0];

  it("creates, updates, toggles, and deletes a task", async () => {
    const taskId = new anchor.BN(Date.now());
    const task = deriveTaskPda(taskId);

    await program.methods
      .createTask(taskId, "Ship the refactor")
      .accounts({
        author: author.publicKey,
        task,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    const createdTask = await program.account.task.fetch(task);
    if (createdTask.content !== "Ship the refactor") {
      throw new Error("task content was not created");
    }
    if (createdTask.completed !== false) {
      throw new Error("task should start incomplete");
    }

    await program.methods
      .updateTask("Ship the modular refactor")
      .accounts({ author: author.publicKey, task })
      .rpc();

    const updatedTask = await program.account.task.fetch(task);
    if (updatedTask.content !== "Ship the modular refactor") {
      throw new Error("task content was not updated");
    }

    await program.methods
      .toggleComplete()
      .accounts({ author: author.publicKey, task })
      .rpc();

    const completedTask = await program.account.task.fetch(task);
    if (completedTask.completed !== true) {
      throw new Error("task was not toggled");
    }

    await program.methods
      .deleteTask()
      .accounts({ author: author.publicKey, task })
      .rpc();

    const deletedTask = await program.account.task.fetchNullable(task);
    if (deletedTask !== null) {
      throw new Error("task account was not closed");
    }
  });
});
