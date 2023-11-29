import {
  CEP85Client,
  CESEventParserFactory,
} from "../src/index";

import {
  FAUCET_KEYS,
  USER1_KEYS,
  getDeploy,
  getAccountInfo,
  getAccountNamedKeyValue,
  printHeader,
} from "./common";

import {
  DeployUtil,
  CLPublicKey,
  EventStream,
  EventName,
  CasperServiceByJsonRPC,
} from "casper-js-sdk";
import { name } from "./install";

const { NODE_URL, EVENT_STREAM_ADDRESS } = process.env;

const runDeployFlow = async (deploy: DeployUtil.Deploy) => {
  const deployHash = await deploy.send(NODE_URL!);

  console.log("...... Deploy hash: ", deployHash);
  console.log("...... Waiting for the deploy...");

  await getDeploy(NODE_URL!, deployHash);

  console.log(`...... Deploy ${deployHash} succedeed`);
};

const run = async () => {
  const cc = new CEP85Client(process.env.NODE_URL!, process.env.NETWORK_NAME!);

  const printTokenDetails = async (id: string, pk: CLPublicKey) => {
    const ownerBalance = await cc.getBalanceOf(pk);
    console.log(`> Account ${pk.toAccountHashStr()} balance ${ownerBalance} for id ${id}`);

    // const metadataOfZero = cc.getMetadataOf(id);
    // console.log(`> Token ${id} metadata`, metadataOfZero);
  };

  let accountInfo = await getAccountInfo(NODE_URL!, FAUCET_KEYS.publicKey);

  console.log(`\n=====================================\n`);

  console.log(`... Account Info: `);
  console.log(JSON.stringify(accountInfo, null, 2));

  const contractHash = await getAccountNamedKeyValue(
    accountInfo,
    `cep85_contract_hash_${name}`
  );

  const contractPackageHash = await getAccountNamedKeyValue(
    accountInfo,
    `cep85_contract_package_hash_${name}`
  );

  console.log(`... Contract Hash: ${contractHash}`);
  console.log(`... Contract Package Hash: ${contractPackageHash}`);

  cc.setContractHash(contractHash, contractPackageHash);

  console.log(`\n=====================================\n`);

  // const allowMintingSetting = await cc.getAllowMintingConfig();
  // console.log(`AllowMintingSetting: ${allowMintingSetting}`);

  const useSessionCode = false;
  const casperClient = new CasperServiceByJsonRPC(NODE_URL);
  const cesEventParser = CESEventParserFactory({
    contractHashes: [contractHash],
    casperClient,
  });

  const es = new EventStream(EVENT_STREAM_ADDRESS!);

  es.subscribe(EventName.DeployProcessed, async (event) => {

    const parsedEvents = await cesEventParser(event);

    if (parsedEvents?.success) {
      console.log("*** EVENT ***");
      console.log(parsedEvents.data);
      console.log("*** ***");
    } else {
      console.log("*** EVENT NOT RELATED TO WATCHED CONTRACT ***");
    }
  });

  // es.start();

  /* Mint */
  printHeader("Mint");

  const id = "1";

  const mintDeploy = cc.mint(
    {
      recipient: FAUCET_KEYS.publicKey,
      id,
      amount: "1",
    },
    { useSessionCode },
    "3000000000",
    FAUCET_KEYS.publicKey,
    [FAUCET_KEYS]
  );

  await runDeployFlow(mintDeploy);

  /* Token details */
  //await printTokenDetails(id, FAUCET_KEYS.publicKey);

  //   /* Transfer */
  printHeader("Transfer");

  const transferDeploy = cc.transfer(
    {
      from: FAUCET_KEYS.publicKey,
      to: USER1_KEYS.publicKey,
      id,
      amount: "1",
    },
    { useSessionCode },
    "13000000000",
    FAUCET_KEYS.publicKey,
    [FAUCET_KEYS]
  );

  await runDeployFlow(transferDeploy);

  //   /* Token details */
  //   await printTokenDetails(id, USER1_KEYS.publicKey);

  //   // Getting new account info to update namedKeys
  accountInfo = await getAccountInfo(NODE_URL!, FAUCET_KEYS.publicKey);

  //   /* Burn */
  printHeader("Burn");

  const burnDeploy = cc.burn(
    {
      owner: USER1_KEYS.publicKey,
      id,
      amount: "1",
    },
    "13000000000",
    USER1_KEYS.publicKey,
    [USER1_KEYS]
  );

  await runDeployFlow(burnDeploy);
  es.stop();
};

run();
