/* eslint-disable eslint-comments/disable-enable-pair */
/* eslint-disable no-console */

import {
  DeployUtil,
  CLPublicKey,
  EventStream,
  CasperServiceByJsonRPC,
  CLU256,
  CLValueBuilder,
} from "casper-js-sdk";
import { utf8ToBytes } from "@noble/hashes/utils";
import { assert } from "console";
import { BigNumber } from "@ethersproject/bignumber";
import {
  FAUCET_KEYS,
  USER1_KEYS,
  getDeploy,
  getAccountInfo,
  getAccountNamedKeyValue,
  printHeader,
  name,
  uri,
  NODE_URL,
  NETWORK_NAME,
  EVENT_STREAM_ADDRESS
} from "./common";
import { install } from "./install";
import { CEP85Client, EventsMode } from "../src";

const id = '1';
const ids = ['3', '4'];
const mintAmount = '20';
const transferAmount = '10';
const burnAmount = '1';
const totalSupply = '40';
const text = "hello Casper";
const data = utf8ToBytes(text);

const runDeployFlow = async (deploy: DeployUtil.Deploy) => {
  const deployHash = await deploy.send(NODE_URL);
  console.log("...... Deploy hash: ", deployHash);
  console.log("...... Waiting for the deploy to be processed...");
  await getDeploy(NODE_URL, deployHash);
  console.info(`...... Deploy ${deployHash} succedeed`);
};

const usage = async () => {
  try {
    const cep85 = new CEP85Client(NODE_URL, NETWORK_NAME);

    await install();

    const printTokenDetails = async (pk: CLPublicKey, tokenId: string) => {
      const ownerBalance = await cep85.getBalanceOf(pk, tokenId);
      console.log(`> Account ${pk.toAccountHashStr()} balance ${ownerBalance} for id ${tokenId}`);
      const tokenUri = await cep85.getURI(id);
      console.log(`> Token ${id} uri`, tokenUri);
    };

    let accountInfo: unknown = await getAccountInfo(NODE_URL, FAUCET_KEYS.publicKey);

    console.log(`\n=====================================\n`);
    console.log(`... Account Info: `);
    console.log(JSON.stringify(accountInfo, null, 2));

    const contractHash = getAccountNamedKeyValue(
      accountInfo,
      `cep85_contract_hash_${name}`
    );

    const contractPackageHash = getAccountNamedKeyValue(
      accountInfo,
      `cep85_contract_package_hash_${name}`
    );

    console.log(`... Contract Hash: ${contractHash}`);
    console.log(`... Contract Package Hash: ${contractPackageHash}`);

    cep85.setContractHash(contractHash, contractPackageHash);

    if (EVENT_STREAM_ADDRESS) {
      await cep85.setupEventStream(new EventStream(EVENT_STREAM_ADDRESS));
    }

    const listener = (eventEmitted: { name: string; data: unknown; }) => {
      console.info(`-----> Event emitted: ${eventEmitted.name}`);
      // console.info(eventEmitted.data); // Event info
    };

    cep85.on('Mint', listener);
    cep85.on('Burn', listener);

    const collectionName = await cep85.collectionName();
    console.log(`... Contract collection name: ${collectionName}`);
    const collectionUri = await cep85.collectionUri();
    console.log(`... Contract collection global uri: ${collectionUri}`);
    assert(collectionName === name);
    console.log(`\n=====================================\n`);

    /* Mint */
    printHeader("Mint");

    const mintDeploy = cep85.mint(
      {
        recipient: FAUCET_KEYS.publicKey,
        id,
        amount: mintAmount,
      },
      "1000000000",
      FAUCET_KEYS.publicKey,
      [FAUCET_KEYS]
    );

    await runDeployFlow(mintDeploy);
    await printTokenDetails(FAUCET_KEYS.publicKey, id);

    cep85.off('Mint', listener);

    /* Transfer */
    printHeader("Transfer");

    const transferDeploy = cep85.transfer(
      {
        from: FAUCET_KEYS.publicKey,
        to: USER1_KEYS.publicKey,
        id,
        amount: transferAmount,
        data
      },
      "1000000000",
      FAUCET_KEYS.publicKey,
      [FAUCET_KEYS]
    );

    await runDeployFlow(transferDeploy);
    await printTokenDetails(USER1_KEYS.publicKey, id);

    /* Burn */
    printHeader("Burn");

    const faucetBurnDeploy = cep85.burn(
      {
        owner: FAUCET_KEYS.publicKey,
        id,
        amount: burnAmount,
      },
      "1000000000",
      FAUCET_KEYS.publicKey,
      [FAUCET_KEYS]
    );

    await runDeployFlow(faucetBurnDeploy);

    const burnDeploy = cep85.burn(
      {
        owner: USER1_KEYS.publicKey,
        id,
        amount: burnAmount,
      },
      "1000000000",
      USER1_KEYS.publicKey,
      [USER1_KEYS]
    );

    await runDeployFlow(burnDeploy);

    cep85.off('Burn', listener);

    cep85.stopEventStream();

    console.log(`... Owners Details after burn`);
    await printTokenDetails(FAUCET_KEYS.publicKey, id);
    await printTokenDetails(USER1_KEYS.publicKey, id);

    /* Set URI */
    printHeader("URI");

    const uriDeploy = cep85.setUri(
      {
        id,
        uri: uri.replace('test', 'usage')
      },
      "500000000",
      FAUCET_KEYS.publicKey,
      [FAUCET_KEYS]
    );

    await runDeployFlow(uriDeploy);

    const resultUri = await cep85.getURI(id);
    console.log(`> URI for token ${id} ${resultUri}`);

    /* Set Supply */
    printHeader("Supply");

    const setTotalSupplyDeploy = cep85.setTotalSupplyOf(
      {
        id,
        total_supply: totalSupply
      },
      "500000000",
      FAUCET_KEYS.publicKey,
      [FAUCET_KEYS]
    );

    await runDeployFlow(setTotalSupplyDeploy);

    let resultTotalSupply = await cep85.getTotalSupplyOf(id);
    console.log(`> Total supply for token ${id} ${resultTotalSupply}`);
    resultTotalSupply = await cep85.getSupplyOf(id);
    console.log(`> Current supply for token ${id} ${resultTotalSupply}`);

    /* Fungible */
    printHeader("Fungible");

    const isNft = await cep85.getIsNonFungible(id);
    console.log(`> is token ${id} NFT ${isNft}`);
    const resultTotalFungibleSupply = await cep85.getTotalFungibleSupply(id);
    console.log(`> Total fungible supply for token ${id} ${resultTotalFungibleSupply}`);

    /* Approval */
    printHeader("Approval");

    const setApprovallForAll = cep85.setApprovalForAll(
      {
        operator: FAUCET_KEYS.publicKey,
        approved: true
      },
      "500000000",
      USER1_KEYS.publicKey,
      [USER1_KEYS]
    );

    await runDeployFlow(setApprovallForAll);

    const isOperator = await cep85.getIsApprovedForAll(USER1_KEYS.publicKey, FAUCET_KEYS.publicKey);
    console.log(`> is account ${FAUCET_KEYS.publicKey.toAccountHashStr()} operator of ${USER1_KEYS.publicKey.toAccountHashStr()} ${isOperator}`);

    printHeader("Change security");

    const changeSecurity = cep85.changeSecurity(
      {
        minter_list: [USER1_KEYS.publicKey],
      },
      "500000000",
      FAUCET_KEYS.publicKey,
      [FAUCET_KEYS]
    );

    await runDeployFlow(changeSecurity);

    const minterDeploy = cep85.mint(
      {
        recipient: USER1_KEYS.publicKey,
        id: '2',
        amount: mintAmount,
      },
      "1000000000",
      USER1_KEYS.publicKey,
      [USER1_KEYS]
    );

    await runDeployFlow(minterDeploy);
    await printTokenDetails(USER1_KEYS.publicKey, '2');

    /* Batch actions */
    printHeader("Batch actions");

    const batchMintDeploy = cep85.batchMint(
      {
        recipient: USER1_KEYS.publicKey,
        ids,
        amounts: [mintAmount, mintAmount],
      },
      "1500000000",
      USER1_KEYS.publicKey,
      [USER1_KEYS]
    );

    await runDeployFlow(batchMintDeploy);
    let ownerBalance = await cep85.getBalanceOfBatch(USER1_KEYS.publicKey, ids);
    console.log(`> Account ${USER1_KEYS.publicKey.toAccountHashStr()} balance [${ownerBalance.toString()}] for ids [${ids.toString()}]`);

    const batchTransferDeploy = cep85.batchTransfer(
      {
        from: USER1_KEYS.publicKey,
        to: FAUCET_KEYS.publicKey,
        ids,
        amounts: [transferAmount, transferAmount],
        data
      },
      "1000000000",
      USER1_KEYS.publicKey,
      [USER1_KEYS]
    );

    await runDeployFlow(batchTransferDeploy);

    ownerBalance = await cep85.getBalanceOfBatch(FAUCET_KEYS.publicKey, ids);
    console.log(`> Account ${FAUCET_KEYS.publicKey.toAccountHashStr()} balance [${ownerBalance.toString()}] for ids [${ids.toString()}]`);

    const batchBurnDeploy = cep85.batchBurn(
      {
        owner: FAUCET_KEYS.publicKey,
        ids,
        amounts: [burnAmount, burnAmount],
      },
      "1000000000",
      FAUCET_KEYS.publicKey,
      [FAUCET_KEYS]
    );

    await runDeployFlow(batchBurnDeploy);

    ownerBalance = await cep85.getBalanceOfBatch(FAUCET_KEYS.publicKey, ids);
    console.log(`> Account ${FAUCET_KEYS.publicKey.toAccountHashStr()} balance [${ownerBalance.toString()}] for ids [${ids.toString()}]`);

    const setTotalSupplyBatchDeploy = cep85.setTotalSupplyOfBatch(
      {
        ids,
        total_supplies: [totalSupply, totalSupply],
      },
      "500000000",
      FAUCET_KEYS.publicKey,
      [FAUCET_KEYS]
    );

    await runDeployFlow(setTotalSupplyBatchDeploy);

    const resultTotalSupplyBatch = await cep85.getTotalSupplyOfBatch(ids);
    console.log(`> Total supply for tokens [${ids.toString()}] = [${resultTotalSupplyBatch.toString()}]`);
    const resultSupplyBatch = await cep85.getSupplyOfBatch(ids);
    console.log(`> Current supply for tokens [${ids.toString()}] = [${resultSupplyBatch.toString()}]`);

    /* Events mode */
    printHeader("Events mode");

    let eventsMode = await cep85.getEventsMode();
    console.log(`> Events mode ${eventsMode}`);

    /* Set Modalities */
    printHeader("Set modalities");

    const setModalitiesDeploy = cep85.setModalities(
      {
        enable_burn: false,
        events_mode: EventsMode.NoEvents,
      },
      "500000000",
      FAUCET_KEYS.publicKey,
      [FAUCET_KEYS]
    );

    await runDeployFlow(setModalitiesDeploy);

    eventsMode = await cep85.getEventsMode();
    console.log(`> Events mode ${eventsMode}`);


    // Instantiate a Casper RPC Client to do some state queries like getting state root hash
    const casperClientRPC = new CasperServiceByJsonRPC(NODE_URL);
    let stateRootHash = await casperClientRPC.getStateRootHash();

    /* Make Dictionary Item Key */
    printHeader("Make Dictionary Item Key");

    // owner as key, id as value
    let dictionaryItemKey = CEP85Client.makeDictionaryItemKey(
      CLValueBuilder.key(FAUCET_KEYS.publicKey),
      new CLU256(id)
    );

    const dictionaryItem = await casperClientRPC.getDictionaryItemByName(stateRootHash, contractHash, "balances", dictionaryItemKey);
    const balance = dictionaryItem.CLValue?.toJSON() as BigNumber;

    console.log(`... Balances Dictionary Item Key for key ${FAUCET_KEYS.publicKey.toAccountHashStr()} and value token id ${id} : ${dictionaryItemKey}`);
    console.log(`... Balance for ${FAUCET_KEYS.publicKey.toAccountHashStr()} and token id ${id} : ${balance.toString()}`);

    dictionaryItemKey = CEP85Client.makeDictionaryItemKey(CLValueBuilder.key(USER1_KEYS.publicKey), CLValueBuilder.key(FAUCET_KEYS.publicKey)); // owner as key, operator as value

    const isApprovedForAll = await casperClientRPC.getDictionaryItemByName(stateRootHash, contractHash, "operators", dictionaryItemKey);

    console.log(`... Operators Dictionary Item Key for key ${USER1_KEYS.publicKey.toAccountHashStr()} and value ${FAUCET_KEYS.publicKey.toAccountHashStr()} : ${dictionaryItemKey}`);
    console.log(`... Is ${FAUCET_KEYS.publicKey.toAccountHashStr()} approved for all as operator of owner ${USER1_KEYS.publicKey.toAccountHashStr()} : ${isApprovedForAll.CLValue?.toJSON() as boolean}`);

    /* Upgrade */
    printHeader("Upgrade");
    accountInfo = await getAccountInfo(NODE_URL, FAUCET_KEYS.publicKey);
    const currentContractHash = getAccountNamedKeyValue(
      accountInfo,
      `cep85_contract_hash_${name}`
    );
    const currentContractVersionUref = getAccountNamedKeyValue(
      accountInfo,
      `cep85_contract_version_${name}`
    );

    const currentContractVersionStoredValue = await casperClientRPC.getBlockState(stateRootHash, currentContractVersionUref, []);

    console.log(`... Contract Hash before Upgrade: ${currentContractHash}`);
    console.log(`... Contract Version before Upgrade: ${currentContractVersionStoredValue.CLValue?.toJSON() as string}`);

    const upgradeDeploy = cep85.upgrade(
      {
        name
      },
      "200000000000",
      FAUCET_KEYS.publicKey,
      [FAUCET_KEYS]
    );

    await runDeployFlow(upgradeDeploy);

    accountInfo = await getAccountInfo(NODE_URL, FAUCET_KEYS.publicKey);
    const upgradedContractHash = getAccountNamedKeyValue(
      accountInfo,
      `cep85_contract_hash_${name}`
    );
    const upgradedContractVersionUref = getAccountNamedKeyValue(
      accountInfo,
      `cep85_contract_version_${name}`
    );

    cep85.setContractHash(upgradedContractHash, contractPackageHash);

    stateRootHash = await casperClientRPC.getStateRootHash();
    const upgradedContractVersionStoredValue = await casperClientRPC.getBlockState(stateRootHash, upgradedContractVersionUref, []);

    console.log(`... Contract Hash after Upgrade: ${upgradedContractHash}`);
    console.log(`... Contract Version after Upgrade: ${upgradedContractVersionStoredValue.CLValue?.toJSON() as string}`);

  } catch (error) {
    console.error("Error in usage:", error);
  }
};

if (require.main === module) {
  usage().catch((error) => console.error(error));
}
