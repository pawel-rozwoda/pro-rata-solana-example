import * as anchor from '@project-serum/anchor';
const assert = require("assert");

describe('pro-rata-solana-example', () => {

	const program = anchor.workspace.ProRataSolanaExample;
  const provider = anchor.Provider.local();

  anchor.setProvider(provider);


  it('initialize contract', async () => {

    await program.state.rpc.new({
      accounts: {
        authority: provider.wallet.publicKey,
      }
    });
  });

  it('deposition', async () => {

		await program.state.rpc.deposit(new anchor.BN(0),{
			accounts: {
				authority: provider.wallet.publicKey,
			},
		}, );
		const state = await program.state.fetch();
    
		console.log(state)
		await new Promise(resolve => setTimeout(resolve, 600)); // in ms  

		await program.state.rpc.deposit(new anchor.BN(1), {
			accounts: {
				authority: provider.wallet.publicKey,
			},
		});
		const state2nd = await program.state.fetch();
		
		console.log(state2nd)
		await new Promise(resolve => setTimeout(resolve, 600)); // in ms  

		await program.state.rpc.deposit(new anchor.BN(2), {
			accounts: {
				authority: provider.wallet.publicKey,
			},
		});
		const state3nd = await program.state.fetch();
		
		console.log(state3nd)

		await new Promise(resolve => setTimeout(resolve, 600)); // in ms  

		await program.state.rpc.deposit(new anchor.BN(3), {
			accounts: {
				authority: provider.wallet.publicKey,
			},
		});
		const state4th = await program.state.fetch();
		
		console.log(state4th)

		await new Promise(resolve => setTimeout(resolve, 600)); // in ms  

		await program.state.rpc.deposit(new anchor.BN(4), {
			accounts: {
				authority: provider.wallet.publicKey,
			},
		});
		const state5th = await program.state.fetch();
		
		console.log(state5th)
		//console.log(typeof(state2nd.share))
		//console.log(state2nd.share)
		////console.log(Object.keys(state2nd.share))
		//console.log(state2nd.share.BN)
  });


});
