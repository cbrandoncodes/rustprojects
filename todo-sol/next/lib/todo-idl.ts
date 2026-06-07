import type { Idl } from "@coral-xyz/anchor";

export const TODO_PROGRAM_ID =
  process.env.NEXT_PUBLIC_TODO_PROGRAM_ID ??
  "9D9oRNLqRebSN6GFAweZ5a4HncoSTuAfuXSL2bUK6xSA";

export const TODO_IDL = {
  address: TODO_PROGRAM_ID,
  metadata: {
    name: "solana",
    version: "0.1.0",
    spec: "0.1.0",
    description: "Anchor todo program",
  },
  instructions: [
    {
      name: "createTask",
      discriminator: [194, 80, 6, 180, 232, 127, 48, 171],
      accounts: [
        {
          name: "author",
          writable: true,
          signer: true,
        },
        {
          name: "task",
          writable: true,
          pda: {
            seeds: [
              {
                kind: "const",
                value: [116, 97, 115, 107],
              },
              {
                kind: "account",
                path: "author",
              },
              {
                kind: "arg",
                path: "taskId",
              },
            ],
          },
        },
        {
          name: "systemProgram",
          address: "11111111111111111111111111111111",
        },
      ],
      args: [
        {
          name: "taskId",
          type: "u64",
        },
        {
          name: "content",
          type: "string",
        },
      ],
    },
    {
      name: "deleteTask",
      discriminator: [112, 220, 10, 109, 3, 168, 46, 73],
      accounts: [
        {
          name: "task",
          writable: true,
        },
        {
          name: "author",
          signer: true,
          relations: ["task"],
        },
      ],
      args: [],
    },
    {
      name: "toggleComplete",
      discriminator: [67, 212, 58, 191, 149, 34, 149, 208],
      accounts: [
        {
          name: "task",
          writable: true,
        },
        {
          name: "author",
          signer: true,
          relations: ["task"],
        },
      ],
      args: [],
    },
    {
      name: "updateTask",
      discriminator: [100, 51, 124, 168, 211, 208, 42, 228],
      accounts: [
        {
          name: "task",
          writable: true,
        },
        {
          name: "author",
          signer: true,
          relations: ["task"],
        },
      ],
      args: [
        {
          name: "content",
          type: "string",
        },
      ],
    },
  ],
  accounts: [
    {
      name: "task",
      discriminator: [79, 34, 229, 55, 88, 90, 55, 84],
    },
  ],
  errors: [
    {
      code: 6000,
      name: "invalidContent",
      msg: "Task content must be between 1 and 200 bytes.",
    },
  ],
  types: [
    {
      name: "task",
      type: {
        kind: "struct",
        fields: [
          {
            name: "taskId",
            type: "u64",
          },
          {
            name: "content",
            type: "string",
          },
          {
            name: "completed",
            type: "bool",
          },
          {
            name: "author",
            type: "pubkey",
          },
          {
            name: "createdAt",
            type: "i64",
          },
        ],
      },
    },
  ],
} as const satisfies Idl;

export type TodoIdl = typeof TODO_IDL;
