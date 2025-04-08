import {
  PRIVATE_KEY_FAUCET,
  PRIVATE_KEY_USER_1,
  PRIVATE_KEY_USER_2,
} from '../../config';
import {
  CEP85Client,
  EVENTS_MODE,
  TransactionParams,
  TransactionResult,
} from '../../src';
import wasm from '../../src/wasm/cep85';
import { getSigningKey } from '../utils';

if (!PRIVATE_KEY_FAUCET) {
  throw new Error('FAUCET_SECRET_KEY environment variable is not set.');
}
if (!PRIVATE_KEY_USER_1) {
  throw new Error('PRIVATE_KEY_USER_1 environment variable is not set.');
}
if (!PRIVATE_KEY_USER_2) {
  throw new Error('PRIVATE_KEY_USER_2 environment variable is not set.');
}

export const uri = 'https://test-cdn-domain/{id}.json';
export const eventsMode = EVENTS_MODE.CES;
export const enableBurn = true;
export const paymentAmount = String(550_000_000_000);
export const owner = getSigningKey(PRIVATE_KEY_FAUCET);
export const ali = getSigningKey(PRIVATE_KEY_USER_1);
export const bob = getSigningKey(PRIVATE_KEY_USER_2);

export const install = async (
  client: CEP85Client,
  name: string
): Promise<TransactionResult> => {
  const params: TransactionParams = {
      wasm,
      sender: owner.publicKey,
      paymentAmount,
      signingKeys: [owner],
    },
    args = {
      name,
      uri,
      eventsMode,
      enableBurn,
      burnerList: [ali.publicKey],
    };

  return client.install({
    params,
    args,
    waitForTransactionProcessed: true,
  });
};

export const mint = async (
  client: CEP85Client,
  id: string,
  mintAmount: bigint,
  waitForTransactionProcessed: boolean = true
): Promise<TransactionResult> => {
  return client.mint({
    params: {
      sender: owner.publicKey,
      paymentAmount: String(5_000_000_000),
      signingKeys: [owner],
    },
    args: {
      id,
      recipient: ali.publicKey,
      amount: String(mintAmount),
    },
    waitForTransactionProcessed,
  });
};
