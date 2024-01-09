import {
  CEP85Client,
  CESEventParserFactory,
  EventsMode,
} from "../src/index";

import {
  FAUCET_KEYS,
  USER1_KEYS,
  getDeploy,
  getAccountInfo,
  getAccountNamedKeyValue,
  printHeader,
  name,
  uri
} from "./common";

import {
  DeployUtil,
  CLPublicKey,
  EventStream,
  EventName,
  CasperServiceByJsonRPC,
} from "casper-js-sdk";
import { install } from "./install";
import { utf8ToBytes } from "@noble/hashes/utils";
import { assert } from "console";

const { NODE_URL, EVENT_STREAM_ADDRESS } = process.env;

const id = '1';
const ids = ['3', '4'];
const mintAmount = '20';
const transferAmount = '10';
const burnAmount = '1';
const totalSupply = '40';
const text = "hello Casper";
const data = utf8ToBytes(text);

const runDeployFlow = async (deploy: DeployUtil.Deploy) => {
  const deployHash = await deploy.send(NODE_URL!);
  console.log("...... Deploy hash: ", deployHash);
  console.log("...... Waiting for the deploy to be processed...");
  await getDeploy(NODE_URL!, deployHash);
  console.log(`...... Deploy ${deployHash} succedeed`);
};

const setEventsSubscription = (contractHash: string) => {
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
  try {
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

    const collectionName = await cc.collectionName();
    console.log(`... Contract collection name: ${collectionName}`);
    let collectionUri = await cc.collectionUri();
    console.log(`... Contract collection global uri: ${collectionUri}`);
    assert(collectionName === name);
    console.log(`\n=====================================\n`);

    const es = setEventsSubscription(contractHash);
    // es.start();

    /* Mint */
    printHeader("Mint");

    const mintDeploy = cc.mint(
      {
        recipient: FAUCET_KEYS.publicKey,
        id,
        amount: mintAmount,
      },
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
        data
      },
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

    /* Set URI */
    printHeader("URI");

    const uriDeploy = cc.setUri(
      {
        id,
        uri: uri.replace('test', 'usage')
      },
      "3000000000",
      FAUCET_KEYS.publicKey,
      [FAUCET_KEYS]
    );

    await runDeployFlow(uriDeploy);

    const result_uri = await cc.getURI(id);
    console.log(`> URI for token ${id} ${result_uri}`);

    /* Set Supply */
    printHeader("Supply");

    const setTotalSupplyDeploy = cc.setTotalSupplyOf(
      {
        id,
        total_supply: totalSupply
      },
      "3000000000",
      FAUCET_KEYS.publicKey,
      [FAUCET_KEYS]
    );

    await runDeployFlow(setTotalSupplyDeploy);

    const result_total_supply = await cc.getTotalSupplyOf(id);
    console.log(`> Total supply for token ${id} ${result_total_supply}`);
    const result_supply = await cc.getSupplyOf(id);
    console.log(`> Current supply for token ${id} ${result_supply}`);

    /* Fungible */
    printHeader("Fungible");

    const is_nft = await cc.getIsNonFungible(id);
    console.log(`> is token ${id} NFT ${is_nft}`);
    const result_total_fungible_supply = await cc.getTotalFungibleSupply(id);
    console.log(`> Total fungible supply for token ${id} ${result_total_fungible_supply}`);

    /* Approval */
    printHeader("Approval");

    const setApprovallForAll = cc.setApprovalForAll(
      {
        operator: FAUCET_KEYS.publicKey,
        approved: true
      },
      "3000000000",
      USER1_KEYS.publicKey,
      [USER1_KEYS]
    );

    await runDeployFlow(setApprovallForAll);

    const is_operator = await cc.getIsApprovedForAll(USER1_KEYS.publicKey, FAUCET_KEYS.publicKey);
    console.log(`> is account ${FAUCET_KEYS.publicKey.toAccountHashStr()} operator of ${USER1_KEYS.publicKey.toAccountHashStr()} ${is_operator}`);

    printHeader("Change security");

    const changeSecurity = cc.changeSecurity(
      {
        minter_list: [USER1_KEYS.publicKey],
      },
      "3000000000",
      FAUCET_KEYS.publicKey,
      [FAUCET_KEYS]
    );

    await runDeployFlow(changeSecurity);

    const minterDeploy = cc.mint(
      {
        recipient: USER1_KEYS.publicKey,
        id: '2',
        amount: mintAmount,
      },
      "3000000000",
      USER1_KEYS.publicKey,
      [USER1_KEYS]
    );

    await runDeployFlow(minterDeploy);
    await printTokenDetails(USER1_KEYS.publicKey, '2');

    /* Batch actions */
    printHeader("Batch actions");

    const batchMintDeploy = cc.batchMint(
      {
        recipient: USER1_KEYS.publicKey,
        ids,
        amounts: [mintAmount, mintAmount],
      },
      "3000000000",
      USER1_KEYS.publicKey,
      [USER1_KEYS]
    );

    await runDeployFlow(batchMintDeploy);
    let ownerBalance = await cc.getBalanceOfBatch(USER1_KEYS.publicKey, ids);
    console.log(`> Account ${USER1_KEYS.publicKey.toAccountHashStr()} balance [${ownerBalance}] for ids [${ids}]`);

    const batchTransferDeploy = cc.batchTransfer(
      {
        from: USER1_KEYS.publicKey,
        to: FAUCET_KEYS.publicKey,
        ids,
        amounts: [transferAmount, transferAmount],
        data
      },
      "3000000000",
      USER1_KEYS.publicKey,
      [USER1_KEYS]
    );

    await runDeployFlow(batchTransferDeploy);

    ownerBalance = await cc.getBalanceOfBatch(FAUCET_KEYS.publicKey, ids);
    console.log(`> Account ${FAUCET_KEYS.publicKey.toAccountHashStr()} balance [${ownerBalance}] for ids [${ids}]`);

    const batchBurnDeploy = cc.batchBurn(
      {
        owner: FAUCET_KEYS.publicKey,
        ids,
        amounts: [burnAmount, burnAmount],
      },
      "3000000000",
      FAUCET_KEYS.publicKey,
      [FAUCET_KEYS]
    );

    await runDeployFlow(batchBurnDeploy);

    ownerBalance = await cc.getBalanceOfBatch(FAUCET_KEYS.publicKey, ids);
    console.log(`> Account ${FAUCET_KEYS.publicKey.toAccountHashStr()} balance [${ownerBalance}] for ids [${ids}]`);

    const setTotalSupplyBatchDeploy = cc.setTotalSupplyOfBatch(
      {
        ids,
        total_supplies: [totalSupply, totalSupply],
      },
      "3000000000",
      FAUCET_KEYS.publicKey,
      [FAUCET_KEYS]
    );

    await runDeployFlow(setTotalSupplyBatchDeploy);

    const result_total_supply_batch = await cc.getTotalSupplyOfBatch(ids);
    console.log(`> Total supply for tokens [${ids}] = [${result_total_supply_batch}]`);
    const result_supply_batch = await cc.getSupplyOfBatch(ids);
    console.log(`> Current supply for tokens [${ids}] = [${result_supply_batch}]`);

    /* Events mode */
    printHeader("Events mode");

    let events_mode = await cc.getEventsMode();
    console.log(`> Events mode ${events_mode}`);

    /* Set Modalities */
    printHeader("Set modalities");

    const set_modalitiesDeploy = cc.setModalities(
      {
        enable_burn: false,
        events_mode: EventsMode.NoEvents,
      },
      "3000000000",
      FAUCET_KEYS.publicKey,
      [FAUCET_KEYS]
    );

    await runDeployFlow(set_modalitiesDeploy);

    events_mode = await cc.getEventsMode();
    console.log(`> Events mode ${events_mode}`);


    /* Set Modalities */
    printHeader("Upgrade");
    accountInfo = await getAccountInfo(NODE_URL!, FAUCET_KEYS.publicKey);
    const currentContractHash = await getAccountNamedKeyValue(
      accountInfo,
      `cep85_contract_hash_${name}`
    );
    const currentContractVersionUref = await getAccountNamedKeyValue(
      accountInfo,
      `cep85_contract_version_${name}`
    );

    const casperClientRPC = new CasperServiceByJsonRPC(NODE_URL as string);
    let stateRootHash = await casperClientRPC.getStateRootHash();
    const currentContractVersionStoredValue = await casperClientRPC.getBlockState(stateRootHash, currentContractVersionUref, []);

    console.log(`... Contract Hash before Upgrade: ${currentContractHash}`);
    console.log(`... Contract Version before Upgrade: ${currentContractVersionStoredValue.CLValue?.toJSON() as string}`);

    const upgradeDeploy = cc.upgrade(
      {
        name
      },
      "250000000000",
      FAUCET_KEYS.publicKey,
      [FAUCET_KEYS]
    );

    await runDeployFlow(upgradeDeploy);

    accountInfo = await getAccountInfo(NODE_URL!, FAUCET_KEYS.publicKey);
    const upgradedContractHash = await getAccountNamedKeyValue(
      accountInfo,
      `cep85_contract_hash_${name}`
    );
    const upgradedContractVersionUref = await getAccountNamedKeyValue(
      accountInfo,
      `cep85_contract_version_${name}`
    );

    cc.setContractHash(upgradedContractHash, contractPackageHash);

    stateRootHash = await casperClientRPC.getStateRootHash();
    const upgradedContractVersionStoredValue = await casperClientRPC.getBlockState(stateRootHash, upgradedContractVersionUref, []);

    console.log(`... Contract Hash after Upgrade: ${upgradedContractHash}`);
    console.log(`... Contract Version after Upgrade: ${upgradedContractVersionStoredValue.CLValue?.toJSON() as string}`);

    es.stop();

  } catch (error) {
    console.error("Error in usage:", error);
  }
};

if (require.main === module) {
  usage().catch((error) => console.error(error));
}
