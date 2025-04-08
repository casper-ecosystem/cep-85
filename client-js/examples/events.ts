import {
  type MintArgs,
  type TransactionParams,
  CEP85Client,
  type BurnArgs,
  CEP85_EVENTS,
  CEP85EventResult,
  InfoGetTransactionResult,
} from 'dist';
import {
  PRIVATE_KEY_FAUCET,
  SSE_URL,
  PRIVATE_KEY_USER_1,
  CHAIN_NAME,
  RPC_URL,
} from '../config';
import {
  findKeyFromAccountNamedKeys,
  getAccountInfo,
  getSigningKey,
} from '../tests/utils';

// Here you can check examples how to mint and burn tokens and listen to event stream

if (!PRIVATE_KEY_FAUCET) {
  throw new Error('FAUCET_SECRET_KEY environment variable is not set.');
}
if (!PRIVATE_KEY_USER_1) {
  throw new Error('PRIVATE_KEY_USER_1 environment variable is not set.');
}

const name = 'TEST_CEP85',
  owner = getSigningKey(PRIVATE_KEY_FAUCET),
  ali = getSigningKey(PRIVATE_KEY_USER_1);

let id: string;

const usage = async () => {
  const account = await getAccountInfo(RPC_URL, owner.publicKey),
    contractHash = findKeyFromAccountNamedKeys(
      account,
      `cep85_contract_hash_${name}`
    );

  const cep85 = new CEP85Client(RPC_URL, SSE_URL, CHAIN_NAME)
    .setContractHash(contractHash)
    .startEventStream();

  console.info(`Contract Hash: ${cep85.contractHash.toPrefixedString()}`);

  id = `${+(await cep85.numOfMintedTokens())}`;

  // Mint token
  let params: TransactionParams = {
    sender: owner.publicKey,
    paymentAmount: String(5_000_000_000),
    signingKeys: [owner],
  };

  const mintArgs: MintArgs = {
    recipient: ali.publicKey,
    id,
    amount: String(20),
  };

  await cep85.mint({
    params,
    args: mintArgs,
  });

  const mintEvent = CEP85_EVENTS.Mint;
  await new Promise<void>((resolve, reject) => {
    cep85.on(mintEvent, async (eventResult) => {
      try {
        await eventListener(cep85, mintEvent, eventResult);
        resolve();
      } catch (error) {
        reject(error);
      }
    });
  });

  const aliBalance = await cep85.balanceOf(ali.publicKey, id);
  console.info(`Token minted successfully, Ali's balance: ${aliBalance}`);

  const isBurnEnabled = await cep85.burnMode();

  if (!isBurnEnabled) {
    console.warn(`Burn is disabled.`);
    return;
  }

  // Burn token
  params = {
    sender: ali.publicKey,
    paymentAmount: String(5_000_000_000),
    signingKeys: [ali],
  };

  const burnArgs: BurnArgs = {
    owner: ali.publicKey,
    id,
    amount: String(20),
  };

  await cep85.burn({
    params,
    args: burnArgs,
  });

  const burnEvent = CEP85_EVENTS.Burn;
  await new Promise<void>((resolve, reject) => {
    cep85.on(burnEvent, async (eventResult) => {
      try {
        await eventListener(cep85, burnEvent, eventResult);
        resolve();
      } catch (error) {
        reject(error);
      }
    });
  });

  const newBalance = await cep85.balanceOf(ali.publicKey, id);
  console.info(
    `Token burnt successfully, Ali's balance: ${newBalance.toString()}`
  );

  cep85.stopEventStream();
};

const eventListener = async (
  cep85: CEP85Client,
  eventType: keyof typeof CEP85_EVENTS,
  eventResult: CEP85EventResult
) => {
  const { transactionInfo, executionResult } = await cep85
    .getTransactionResult(eventResult.transactionInfo.transactionHash)
    .then((transactionResult: InfoGetTransactionResult) => ({
      transactionInfo: eventResult.transactionInfo,
      executionResult: transactionResult.executionInfo?.executionResult,
    }));

  console.info(
    `Contract ${eventType} transaction hash: ${transactionInfo.transactionHash}`
  );

  if (executionResult) {
    if (executionResult?.errorMessage) {
      throw new Error(
        `Error during ${eventType}.\n${executionResult?.errorMessage.toString()}`
      );
    } else {
      console.info(
        `Contract ${eventType} cost consumed: ${executionResult?.consumed}`
      );
    }
  }
  cep85.removeListenersForEvent(CEP85_EVENTS[eventType]);
};

usage()
  .then(() => {
    console.info('Events usage completed.');
  })
  .catch((error) => {
    console.error('Usage failed:', error);
  });
