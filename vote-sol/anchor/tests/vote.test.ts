import * as anchor from '@coral-xyz/anchor'
import { Program } from '@coral-xyz/anchor'
import { PublicKey, SystemProgram } from '@solana/web3.js'
import { Vote } from '../target/types/vote'

describe('vote', () => {
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider)

  const program = anchor.workspace.Vote as Program<Vote>

  jest.setTimeout(30000)

  const findCounterPda = () =>
    PublicKey.findProgramAddressSync([Buffer.from('counter')], program.programId)[0]

  const findRegistrationsPda = () =>
    PublicKey.findProgramAddressSync([Buffer.from('registrations')], program.programId)[0]

  const findPollPda = (pollId: anchor.BN) =>
    PublicKey.findProgramAddressSync([pollId.toArrayLike(Buffer, 'le', 8)], program.programId)[0]

  const findCandidatePda = (pollId: anchor.BN, candidateId: anchor.BN) =>
    PublicKey.findProgramAddressSync(
      [pollId.toArrayLike(Buffer, 'le', 8), candidateId.toArrayLike(Buffer, 'le', 8)],
      program.programId,
    )[0]

  const findVoterPda = (pollId: anchor.BN, voter: PublicKey) =>
    PublicKey.findProgramAddressSync(
      [Buffer.from('voter'), pollId.toArrayLike(Buffer, 'le', 8), voter.toBuffer()],
      program.programId,
    )[0]

  const ensureInitialized = async () => {
    const counterPda = findCounterPda()
    const registrationsPda = findRegistrationsPda()

    try {
      const counter = await program.account.counter.fetch(counterPda)
      const registrations = await program.account.registrations.fetch(registrationsPda)
      return { counterPda, registrationsPda, counter, registrations }
    } catch {
      await program.rpc.initialize({
        accounts: {
          user: provider.wallet.publicKey,
          counter: counterPda,
          registrations: registrationsPda,
          systemProgram: SystemProgram.programId,
        },
      })

      const counter = await program.account.counter.fetch(counterPda)
      const registrations = await program.account.registrations.fetch(registrationsPda)
      return { counterPda, registrationsPda, counter, registrations }
    }
  }

  const createPoll = async (label: string) => {
    const { counterPda, registrationsPda, counter } = await ensureInitialized()
    const pollId = counter.count.add(new anchor.BN(1))
    const pollPda = findPollPda(pollId)

    const now = Math.floor(Date.now() / 1000)
    const start = new anchor.BN(now)
    const end = new anchor.BN(now + 86400)

    await program.rpc.createPoll(label, start, end, {
      accounts: {
        user: provider.wallet.publicKey,
        poll: pollPda,
        counter: counterPda,
        systemProgram: SystemProgram.programId,
      },
    })

    const poll = await program.account.poll.fetch(pollPda)
    return { pollId, pollPda, poll, counterPda, registrationsPda }
  }

  const registerCandidate = async (pollId: anchor.BN, pollPda: PublicKey, name: string) => {
    const registrationsPda = findRegistrationsPda()
    const registrations = await program.account.registrations.fetch(registrationsPda)
    const candidateId = registrations.count.add(new anchor.BN(1))
    const candidatePda = findCandidatePda(pollId, candidateId)

    await program.rpc.registerCandidate(pollId, name, {
      accounts: {
        user: provider.wallet.publicKey,
        poll: pollPda,
        candidate: candidatePda,
        registrations: registrationsPda,
        systemProgram: SystemProgram.programId,
      },
    })

    const candidate = await program.account.candidate.fetch(candidatePda)
    return { candidateId, candidatePda, candidate, registrationsPda }
  }

  it('initializes and creates a poll', async () => {
    const { pollId, poll } = await createPoll(`Poll #${Date.now()}`)

    expect(poll.id.toNumber()).toEqual(pollId.toNumber())
    expect(poll.description).toContain('Poll #')
    expect(poll.start.toNumber()).toBeLessThan(poll.end.toNumber())
    expect(poll.candidates.toNumber()).toEqual(0)
  })

  it('Registers a candidate', async () => {
    const { pollId, pollPda } = await createPoll(`Candidate poll #${Date.now()}`)
    const { candidateId, candidate } = await registerCandidate(pollId, pollPda, 'Alice')

    expect(candidate.cid.toNumber()).toEqual(candidateId.toNumber())
    expect(candidate.name).toEqual('Alice')
    expect(candidate.pollId.toNumber()).toEqual(pollId.toNumber())
    expect(candidate.hasRegistered).toBe(true)
  })

  it('Votes for a candidate', async () => {
    const { pollId, pollPda } = await createPoll(`Vote poll #${Date.now()}`)
    const { candidateId, candidatePda } = await registerCandidate(pollId, pollPda, 'Bob')
    const voterPda = findVoterPda(pollId, provider.wallet.publicKey)

    await program.rpc.vote(pollId, candidateId, {
      accounts: {
        user: provider.wallet.publicKey,
        poll: pollPda,
        candidate: candidatePda,
        voter: voterPda,
        registrations: findRegistrationsPda(),
        systemProgram: SystemProgram.programId,
      },
    })

    const candidate = await program.account.candidate.fetch(candidatePda)
    const voter = await program.account.voter.fetch(voterPda)

    expect(candidate.votes.toNumber()).toEqual(1)
    expect(voter.pollId.toNumber()).toEqual(pollId.toNumber())
    expect(voter.cid.toNumber()).toEqual(candidateId.toNumber())
    expect(voter.hasVoted).toBe(true)
  })
})
