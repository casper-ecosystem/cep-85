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
  name
} from "./common";

import {
  DeployUtil,
  CLPublicKey,
  EventStream,
  EventName,
  CasperServiceByJsonRPC,
} from "casper-js-sdk";
import { install } from "./install";

const { NODE_URL, EVENT_STREAM_ADDRESS } = process.env;

const runDeployFlow = async (deploy: DeployUtil.Deploy) => {
  const deployHash = await deploy.send(NODE_URL!);

  console.log("...... Deploy hash: ", deployHash);
  console.log("...... Waiting for the deploy...");

  await getDeploy(NODE_URL!, deployHash);

  console.log(`...... Deploy ${deployHash} succedeed`);
};

const setEvenstSubscription = (contractHash) => {
  const casperClient = new CasperServiceByJsonRPC(NODE_URL as string);
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
  return es;
};

const usage = async () => {
  const cc = new CEP85Client(process.env.NODE_URL!, process.env.NETWORK_NAME!);

  await install();

  const printTokenDetails = async (pk: CLPublicKey, id: string) => {
    const ownerBalance = await cc.getBalanceOf(pk, id);
    console.log(`> Account ${pk.toAccountHashStr()} balance ${ownerBalance} for id ${id}`);
    const uri = await cc.getURI(id);
    console.log(`> Token ${id} uri`, uri);
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

  const es = setEvenstSubscription(contractHash);
  // es.start();

  const id = '1';
  const mintAmount = '20';
  const transferAmount = '10';
  const burnAmount = '1';
  let useSessionCode = false;

  /* Mint */
  printHeader("Mint");

  const mintDeploy = cc.mint(
    {
      recipient: FAUCET_KEYS.publicKey,
      id,
      amount: mintAmount,
    },
    { useSessionCode },
    "3000000000",
    FAUCET_KEYS.publicKey,
    [FAUCET_KEYS]
  );

  await runDeployFlow(mintDeploy);
  await printTokenDetails(FAUCET_KEYS.publicKey, id);

  /* Transfer */
  printHeader("Transfer");

  const transferDeploy = cc.transfer(
    {
      from: FAUCET_KEYS.publicKey,
      to: USER1_KEYS.publicKey,
      id,
      amount: transferAmount,
    },
    { useSessionCode },
    "13000000000",
    FAUCET_KEYS.publicKey,
    [FAUCET_KEYS]
  );

  await runDeployFlow(transferDeploy);
  await printTokenDetails(USER1_KEYS.publicKey, id);

  /* Burn */
  printHeader("Burn");

  const faucetBurnDeploy = cc.burn(
    {
      owner: FAUCET_KEYS.publicKey,
      id,
      amount: burnAmount,
    },
    "13000000000",
    FAUCET_KEYS.publicKey,
    [FAUCET_KEYS]
  );

  await runDeployFlow(faucetBurnDeploy);

  const burnDeploy = cc.burn(
    {
      owner: USER1_KEYS.publicKey,
      id,
      amount: burnAmount,
    },
    "13000000000",
    USER1_KEYS.publicKey,
    [USER1_KEYS]
  );

  await runDeployFlow(burnDeploy);

  console.log(`... Owners Details after burn`);
  await printTokenDetails(FAUCET_KEYS.publicKey, id);
  await printTokenDetails(USER1_KEYS.publicKey, id);

  es.stop();
};

if (require.main === module) {
  usage().catch((error) => console.error(error));
}
