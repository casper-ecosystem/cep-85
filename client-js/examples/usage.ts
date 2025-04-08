import { CLValue, Key } from 'casper-js-sdk';
import { TextEncoder } from 'util';
import {
  PRIVATE_KEY_FAUCET,
  SSE_URL,
  PRIVATE_KEY_USER_1,
  PRIVATE_KEY_USER_2,
  CHAIN_NAME,
  RPC_URL,
} from '../config';
import {
  CEP85Client,
  TransferArgs,
  TransactionParams,
  Entity,
  MintArgs,
} from '../dist';
import {
  findKeyFromAccountNamedKeys,
  getAccountInfo,
  getSigningKey,
} from '../tests/utils';

// Here you can check examples how to check balance, approve tokens, transfer tokens, and transfer tokens by allowance

if (!PRIVATE_KEY_FAUCET) {
  throw new Error('FAUCET_SECRET_KEY environment variable is not set.');
}
if (!PRIVATE_KEY_USER_1) {
  throw new Error('PRIVATE_KEY_USER_1 environment variable is not set.');
}
if (!PRIVATE_KEY_USER_2) {
  throw new Error('PRIVATE_KEY_USER_2 environment variable is not set.');
}

const name = 'TEST_CEP85',
  owner = getSigningKey(PRIVATE_KEY_FAUCET),
  ali = getSigningKey(PRIVATE_KEY_USER_1),
  bob = getSigningKey(PRIVATE_KEY_USER_2),
  waitForTransactionProcessed = true;

let id: string;
const mintAmount = '20';
const transferAmount = '10';
const burnAmount = '1';
const totalSupply = '40';
const text = 'Casper free bytes';
const encoder = new TextEncoder();
const bytes = encoder.encode(text);
const data = new Uint8Array(bytes);

const usage = async () => {
  const accountInfo = await getAccountInfo(RPC_URL, owner.publicKey),
    contractHash = findKeyFromAccountNamedKeys(
      accountInfo,
      `cep85_contract_hash_${name}`
    );

  const cep85 = new CEP85Client(RPC_URL, SSE_URL, CHAIN_NAME).setContractHash(
    contractHash
  );
  console.info(`Contract Hash: ${cep85.contractHash.toPrefixedString()}`);

  id = `${+(await cep85.numOfMintedTokens())}`;

  const printTokenDetails = async (pk: Entity, tokenId: string) => {
    const ownerBalance = await cep85.balanceOf(pk, tokenId);
    console.info(
      `Account ${pk.toString()} balance ${ownerBalance} for id ${tokenId}`
    );
    const tokenUri = await cep85.getURI(id);
    console.info(`Token ${id} uri`, tokenUri);
  };

  // Mint token
  console.info(`Mint token ${id}`);

  let params: TransactionParams = {
    sender: owner.publicKey,
    paymentAmount: String(5_000_000_000),
    signingKeys: [owner],
  };

  let mintArgs: MintArgs = {
    recipient: ali.publicKey,
    id,
    amount: mintAmount,
    uri: 'https://example-domain/{id}.json',
  };

  await cep85.mint({
    params,
    args: mintArgs,
    waitForTransactionProcessed,
  });

  // Fetch token info
  const token_name = await cep85.collectionName(),
    uri = await cep85.collectionUri(),
    eventsMode = await cep85.eventsMode();
  // totalSupply = await cep85();

  console.info('tokenInfo: ', {
    token_name,
    uri,
    eventsMode,
  });

  await printTokenDetails(ali.publicKey, id);

  // Transfer tokens
  params = {
    sender: ali.publicKey,
    paymentAmount: String(5_000_000_000),
    signingKeys: [ali],
  };

  const transferArgs: TransferArgs = {
    from: ali.publicKey,
    to: bob.publicKey,
    id,
    amount: transferAmount,
    data,
  };

  let { transactionInfo, executionResult } = await cep85.transfer({
    params,
    args: transferArgs,
    waitForTransactionProcessed,
  });

  if (executionResult?.errorMessage) {
    throw new Error(
      `Error during transfer.\n${executionResult?.errorMessage.toString()}`
    );
  } else {
    console.info(
      `Token transfer transaction hash: ${transactionInfo.transactionHash}`
    );
    console.info(`Transfer cost consumed: ${executionResult?.consumed}`);
  }

  await printTokenDetails(bob.publicKey, id);

  // Set URI
  console.info(`Set URI for token ${id}`);

  params = {
    sender: owner.publicKey,
    paymentAmount: String(3_000_000_000),
    signingKeys: [owner],
  };

  await cep85.setUri({
    params,
    args: {
      id,
      uri: uri.replace('test', 'usage'),
    },
    waitForTransactionProcessed,
  });

  const resultUri = await cep85.getURI(id);
  console.info(`URI for token ${id}: ${resultUri}`);

  // Set Supply
  console.info(`Set total supply for token ${id}`);

  params = {
    sender: owner.publicKey,
    paymentAmount: String(3_000_000_000),
    signingKeys: [owner],
  };

  await cep85.setTotalSupplyOf({
    params,
    args: {
      id,
      totalSupply,
    },
    waitForTransactionProcessed,
  });

  let resultTotalSupply = await cep85.getTotalSupplyOf(id);
  console.info(`Total supply for token ${id}: ${resultTotalSupply}`);
  resultTotalSupply = await cep85.getSupplyOf(id);
  console.info(`Circulating supply for token ${id}: ${resultTotalSupply}`);

  /* Fungible */
  console.info('Fungible');

  const isNft = await cep85.getIsNonFungible(id);
  console.info(`Is token ${id} NFT ? ${isNft}`);
  const resultTotalFungibleSupply = await cep85.getTotalFungibleSupply(id);
  console.info(
    `Total fungible supply for token ${id}: ${resultTotalFungibleSupply}`
  );

  /* Approval */
  console.info('Approval');

  params = {
    sender: ali.publicKey,
    paymentAmount: String(3_000_000_000),
    signingKeys: [ali],
  };

  await cep85.setApprovalForAll({
    params,
    args: {
      operator: owner.publicKey,
      approved: true,
    },
    waitForTransactionProcessed,
  });

  const isOperator = await cep85.getIsApprovedForAll(
    ali.publicKey,
    owner.publicKey
  );
  console.info(
    `Is account ${owner.publicKey.accountHash().toPrefixedString()} operator of ${ali.publicKey.accountHash().toPrefixedString()} ${isOperator}`
  );

  console.info('Change security');

  params = {
    sender: owner.publicKey,
    paymentAmount: String(5_000_000_000),
    signingKeys: [owner],
  };

  await cep85.changeSecurity({
    params,
    args: {
      minterList: [bob.publicKey],
    },
    waitForTransactionProcessed,
  });

  id = `${+(await cep85.numOfMintedTokens())}`; // Mint a new id

  params = {
    sender: bob.publicKey,
    paymentAmount: String(3_000_000_000),
    signingKeys: [bob],
  };

  console.info('Mint NFT');

  await cep85.mint({
    params,
    args: {
      recipient: bob.publicKey,
      id,
      amount: String(1),
      uri: 'https://bob-domain/{id}.json',
    },
    waitForTransactionProcessed,
  });

  await printTokenDetails(bob.publicKey, id);
  console.info(`Is token ${id} NFT ? ${await cep85.getIsNonFungible(id)}`);

  /* Batch actions */
  console.info('Batch actions');

  const numMinted = await cep85.numOfMintedTokens();
  const ids = [
    Number(numMinted).toString(),
    (Number(numMinted) + 1).toString(),
  ];

  console.info('Batch mint');

  params = {
    sender: bob.publicKey,
    paymentAmount: String(3_000_000_000),
    signingKeys: [bob],
  };

  await cep85.batchMint({
    params,
    args: {
      recipient: bob.publicKey,
      ids,
      amounts: [mintAmount, mintAmount],
      uri: 'https://bob-domain/{id}.json',
    },
    waitForTransactionProcessed,
  });

  let ownerBalance = await cep85.balanceOfBatch(bob.publicKey, ids);
  console.info(
    `Account ${bob.publicKey.accountHash().toPrefixedString()} balance [${ownerBalance.toString()}] for ids [${ids.toString()}]`
  );

  console.info('Batch transfer');

  await cep85.batchTransfer({
    params,
    args: {
      from: bob.publicKey,
      to: ali.publicKey,
      ids,
      amounts: [mintAmount, mintAmount],
    },
    waitForTransactionProcessed,
  });

  console.info('Batch burn');

  params = {
    sender: ali.publicKey,
    paymentAmount: String(3_000_000_000),
    signingKeys: [ali],
  };

  await cep85.batchBurn({
    params,
    args: {
      owner: ali.publicKey,
      ids,
      amounts: [burnAmount, burnAmount],
    },
    waitForTransactionProcessed,
  });

  ownerBalance = await cep85.balanceOfBatch(ali.publicKey, ids);
  console.info(
    `Account ${ali.publicKey.accountHash().toPrefixedString()} balance [${ownerBalance.toString()}] for ids [${ids.toString()}]`
  );

  console.info('Set total supply of Batch');

  params = {
    sender: owner.publicKey,
    paymentAmount: String(3_000_000_000),
    signingKeys: [owner],
  };

  await cep85.setTotalSupplyOfBatch({
    params,
    args: {
      ids,
      totalSupplies: [totalSupply, totalSupply],
    },
    waitForTransactionProcessed,
  });

  const resultTotalSupplyBatch = await cep85.getTotalSupplyOfBatch(ids);
  console.info(
    `Total supply for tokens [${ids.toString()}] = [${resultTotalSupplyBatch.toString()}]`
  );
  const resultSupplyBatch = await cep85.getSupplyOfBatch(ids);
  console.info(
    `Circulating supply for tokens [${ids.toString()}] = [${resultSupplyBatch.toString()}]`
  );

  /* Make Dictionary Item Key */
  console.info('Make Dictionary Item Key');

  let dictionaryItemKey = CEP85Client.makeDictionaryItemKey(
    CLValue.newCLKey(
      Key.newKey(`entity-account-${bob.publicKey.accountHash().toHex()}`)
    ),
    CLValue.newCLUInt256(id)
  );

  console.info(
    `Balances Dictionary Item Key for key ${bob.publicKey.accountHash().toPrefixedString()} and value token id ${id} : ${dictionaryItemKey}`
  );

  dictionaryItemKey = CEP85Client.makeDictionaryItemKey(
    CLValue.newCLKey(
      Key.newKey(`entity-account-${ali.publicKey.accountHash().toHex()}`)
    ),
    CLValue.newCLKey(
      Key.newKey(`entity-account-${owner.publicKey.accountHash().toHex()}`)
    )
  ); // owner of token as key, operator as value

  console.info(
    `Operators Dictionary Item Key for key ${ali.publicKey.accountHash().toPrefixedString()} and value ${owner.publicKey.accountHash().toPrefixedString()} : ${dictionaryItemKey}`
  );
};

usage()
  .then(() => {
    console.info('Usage completed successfully.');
  })
  .catch((error) => {
    console.error('Usage failed:', error);
  });
