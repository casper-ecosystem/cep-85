import { CHAIN_NAME, PRIVATE_KEY_FAUCET, RPC_URL, SSE_URL } from '../config';
import {
  CEP85Client,
  ContractWASM as wasm,
  type TransactionParams,
  type TransactionResult,
  type UpgradeArgs,
} from '../dist';
import {
  findKeyFromAccountNamedKeys,
  getAccountInfo,
  getSigningKey,
} from '../tests/utils';

if (!PRIVATE_KEY_FAUCET) {
  throw new Error('FAUCET_SECRET_KEY environment variable is not set.');
}

const name = 'TEST_CEP85',
  waitForTransactionProcessed = true,
  sender = getSigningKey(PRIVATE_KEY_FAUCET),
  paymentAmount = String(550_000_000_000);

const upgrade = async () => {
  const cep85 = new CEP85Client(RPC_URL, SSE_URL, CHAIN_NAME),
    params: TransactionParams = {
      wasm,
      sender: sender.publicKey,
      paymentAmount,
      signingKeys: [sender],
    },
    args: UpgradeArgs = {
      name,
    },
    transactionResult: TransactionResult = await cep85.upgrade({
      params,
      args,
      waitForTransactionProcessed,
    });

  if (!transactionResult.transactionInfo.transactionHash) {
    throw Error('Invalid transaction hash');
  }
  return transactionResult;
};

upgrade()
  .then(async (transactionResult) => {
    const { transactionInfo, executionResult } = transactionResult;
    console.info(
      `Contract upgrade transaction hash: ${transactionInfo.transactionHash.toHex()}`
    );

    if (executionResult) {
      if (executionResult?.errorMessage) {
        throw new Error(
          `Error during upgrade.\n${executionResult?.errorMessage.toString()}`
        );
      } else {
        console.info(
          `Contract upgrade cost consumed: ${executionResult?.consumed}`
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
