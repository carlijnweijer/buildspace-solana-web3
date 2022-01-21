const anchor = require("@project-serum/anchor");

const { SystemProgram } = anchor.web3;

const main = async () => {
  console.log("🚀 starting test...");

  // create and set the provider.
  //We set it before but we needed to update it,
  //so that it can communicate with our frontend
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Myepicproject;

  // create an account keypair for our program to use
  const baseAcccount = anchor.web3.Keypair.generate();

  //call start_stuff_off and pass it the params it needs
  const tx = await program.rpc.startStuffOff({
    accounts: {
      baseAcccount: baseAcccount.publicKey,
      user: provider.wallet.publicKey,
      SystemProgram: SystemProgram.programId,
    },
    signers: [baseAcccount],
  });

  console.log("📝 your transaction signature ", tx);

  // fetch data from the account
  let account = await program.account.baseAccount.fetch(baseAcccount.publicKey);
  console.log("👀 goal count:", account.totalGoals.toString());
};

const runMain = async () => {
  try {
    await main();
    process.exit(0);
  } catch (error) {
    console.error(error);
    process.exit(1);
  }
};

runMain();
