import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { GainGame } from '../target/types/gain_game';

describe('gain-game', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.GainGame as Program<GainGame>;

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
