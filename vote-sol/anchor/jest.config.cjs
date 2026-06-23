module.exports = {
  preset: 'ts-jest',
  testEnvironment: 'node',
  transform: {
    '^.+\\.(ts|tsx)$': 'ts-jest',
    '^.+\\.(js|mjs)$': 'babel-jest',
  },
  transformIgnorePatterns: [
    '/node_modules/(?!rpc-websockets|uuid|@coral-xyz|@solana/web3.js)'
  ],
  testMatch: ['**/tests/**/*.test.ts', '**/tests/**/*.spec.ts', '**/tests/**/*.ts'],
};
