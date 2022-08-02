import * as anchor from '@project-serum/anchor';
import {Program, LangErrorCode} from '@project-serum/anchor';
import {Dejavu} from '../target/types/dejavu';
import {assert} from 'chai';

describe('dejavu', () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Dejavu as Program<Dejavu>;

  it('InitializeIdea', async () => {
    const test_idea = anchor.web3.Keypair.generate();
    // Add your test here.
    const initializeIdeaParams = {
      createdTs: new anchor.BN(Date.now()),
      uri: 'hello world',
    };
    const tx = await program.methods
      .initializeIdea(initializeIdeaParams)
      .accounts({
        idea: test_idea.publicKey,
        creator: program.provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      })
      .signers([test_idea])
      .rpc();
    console.log('Your transaction signature', tx);

    const account = await program.account.idea.fetch(test_idea.publicKey);
    assert.strictEqual(
      account.creator.toString(),
      program.provider.wallet.publicKey.toString(),
    );
    assert.strictEqual(
      account.createdTs.toNumber(),
      initializeIdeaParams.createdTs.toNumber(),
    );
    assert.strictEqual(
      account.lastUpdatedTs.toNumber(),
      initializeIdeaParams.createdTs.toNumber(),
    );
    assert.strictEqual(account.version, 0);
    assert.strictEqual(account.uri.len, 11);
    assert.strictEqual(
      Buffer.from(account.uri.uri)
        .subarray(0, account.uri.len)
        .toString('utf8'),
      initializeIdeaParams.uri,
    );
  });

  // it('InitializeIdea - bad uri length', async () => {
  //   const test_idea = anchor.web3.Keypair.generate();
  //   // Add your test here.
  //   const initializeIdeaParams = {
  //     createdTs: new anchor.BN(Date.now()),
  //     uri: '0'.repeat(257),
  //   };
  //   const tx = await program.methods
  //     .initializeIdea(initializeIdeaParams)
  //     .accounts({
  //       idea: test_idea.publicKey,
  //       creator: program.provider.wallet.publicKey,
  //       systemProgram: anchor.web3.SystemProgram.programId,
  //       rent: anchor.web3.SYSVAR_RENT_PUBKEY,
  //     })
  //     .signers([test_idea])
  //     .rpc();
  //   console.log('Your transaction signature', tx);
  //   console.log(program.idl.errors);
  // });
});
