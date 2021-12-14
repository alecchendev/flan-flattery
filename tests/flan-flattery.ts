import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { FlanFlattery } from '../target/types/flan_flattery';

describe('flan-flattery', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.FlanFlattery as Program<FlanFlattery>;

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
