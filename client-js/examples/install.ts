import {
  CHAIN_NAME,
  PRIVATE_KEY_FAUCET,
  PRIVATE_KEY_USER_1,
  RPC_URL,
  SSE_URL,
} from '../config';
import {
  CEP85Client,
  ContractWASM as wasm,
  EVENTS_MODE,
  type InstallArgs,
  type TransactionParams,
  type TransactionResult,
} from '../dist';
import {
  findKeyFromAccountNamedKeys,
  getAccountInfo,
  getSigningKey,
} from '../tests/utils';

if (!PRIVATE_KEY_FAUCET) {
  throw new Error('FAUCET_SECRET_KEY environment variable is not set.');
}

if (!PRIVATE_KEY_USER_1) {
  throw new Error('PRIVATE_KEY_USER_1 environment variable is not set.');
}

const name = 'TEST_CEP85',
  uri = 'https://test-cdn-domain/{id}.json',
  eventsMode = EVENTS_MODE.CES,
  enableBurn = true,
  waitForTransactionProcessed = true,
  sender = getSigningKey(PRIVATE_KEY_FAUCET),
  ali = getSigningKey(PRIVATE_KEY_USER_1),
  paymentAmount = String(550_000_000_000);

const install = async () => {
  const cep85 = new CEP85Client(RPC_URL, SSE_URL, CHAIN_NAME);

  const params: TransactionParams = {
    wasm,
    sender: sender.publicKey,
    paymentAmount,
    signingKeys: [sender],
  };

  const args: InstallArgs = {
    name,
    uri,
    eventsMode,
    enableBurn,
    burnerList: [ali.publicKey],
  };

  const transactionResult: TransactionResult = await cep85.install({
    params,
    args,
    waitForTransactionProcessed,
  });

  if (!transactionResult.transactionInfo.transactionHash) {
    throw Error('Invalid transaction hash');
  }
  return transactionResult;
};

install()
  .then(async (transactionResult) => {
    const { transactionInfo, executionResult } = transactionResult;
    console.info(
      `Contract installation transaction hash: ${transactionInfo.transactionHash.toHex()}`
    );

    if (executionResult) {
      if (executionResult?.errorMessage) {
        throw new Error(
          `Error during installation.\n${executionResult?.errorMessage.toString()}`
        );
      } else {
        console.info(
          `Contract installation cost consumed: ${executionResult?.consumed}`
        );
      }
    }

    const account = await getAccountInfo(RPC_URL, sender.publicKey),
      contractHash = findKeyFromAccountNamedKeys(
        account,
        `cep85_contract_hash_${name}`
      ),
      contractPackageHash = findKeyFromAccountNamedKeys(
        account,
        `cep85_contract_package_${name}`
      );

    console.info(`Contract Hash: ${contractHash}`);
    console.info(`Contract Package Hash: ${contractPackageHash}`);
  })
  .catch((error) => {
    console.error(error);
  });
