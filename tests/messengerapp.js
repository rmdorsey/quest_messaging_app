const assert = require('assert');
const anchor = require('@project-serum/anchor');
const { SystemProgram } = anchor.web3;

describe("Testing our messaging app: ", function() {
  //- The provider is the abstraction of a connection to the Solana network. 
  //In the test, the Anchor framework will create the provider for us based 
  //on the environment (anchor.Provider.env())
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);
  //Tthe program is an abstraction that combines the Provider, idl, and the 
  //programID (which is generated when the program is built) and allows us to 
  //call RPC methods against our program.
  const program = anchor.workspace.Messengerapp;

  it("An account is initialized", async function() {
    const baseAccount  = anchor.web3.Keypair.generate();
    await program.rpc.initialize("My first message", {
      accounts: {
        baseAccount: baseAccount.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [baseAccount]
    });

    const account = await program.account.baseAccount.fetch(baseAccount.publicKey);
    console.log('Data: ', account.data);
    assert.ok(account.data === "My first message");
    _baseAccount = baseAccount;
  });

  it("Update the account previously created: ", async function() {
    const baseAccount = _baseAccount;

    await program.rpc.update("My second message", {
      accounts: {
        baseAccount: baseAccount.publicKey,
      },
    });

    const account = await program.account.baseAccount.fetch(baseAccount.publicKey);
    console.log("Updated data: ", account.data);
    assert.ok(account.data === "My second message");
    console.log("All account data: ", account);
    console.log("All data: ", account.dataList);
    assert.ok(account.dataList.length === 2);
  });
});