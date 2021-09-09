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
      },
    });
  });

  it('deposition', async () => {

		await program.state.rpc.deposit({
			accounts: {
				authority: provider.wallet.publicKey,
			},
		});
		const state = await program.state.fetch();
    
		console.log(state.count)

		await program.state.rpc.deposit({
			accounts: {
				authority: provider.wallet.publicKey,
			},
		});
		const state2nd = await program.state.fetch();
    
		console.log(state2nd.count)
  });

  //it('2nd deposition', async () => {
		//await program.state.rpc.deposit({
			//accounts: {
				//authority: provider.wallet.publicKey,
			//},
		//});
		//const state = await program.state.fetch();
    
		//console.log(state.count)
  //});

});
