import { expect, describe, it, beforeEach } from 'vitest';
import { owner, paymentAmount, install } from './helpers';
import { RPC_URL, SSE_URL, CHAIN_NAME, PRIVATE_KEY_FAUCET } from '../../config';
import {
  CEP85Client,
  EVENTS_MODE,
  TransactionParams,
  TransactionResult,
} from '../../src';
import wasm from '../../src/wasm/cep85';
import { getAccountInfo, findKeyFromAccountNamedKeys } from '../utils';

if (!PRIVATE_KEY_FAUCET) {
  throw new Error('FAUCET_SECRET_KEY environment variable is not set.');
}

let client: CEP85Client;
const name = `TEST_CEP85_E2E_${Math.floor(Math.random() * 1000000)}`;

describe('CEP85Client - E2E Upgrade', () => {
  beforeEach(async () => {
    client = new CEP85Client(RPC_URL, SSE_URL, CHAIN_NAME);
    await install(client, name);
  }, 60000);

  it('should upgrade the CEP85 contract and return valid transaction info', async () => {
    const params: TransactionParams = {
        wasm,
        sender: owner.publicKey,
        paymentAmount,
        signingKeys: [owner],
      },
      args = {
        name,
      },
      transactionResult: TransactionResult = await client.upgrade({
        params,
        args,
        waitForTransactionProcessed: false,
      });

    expect(
      transactionResult.transactionInfo.transactionHash.toString()
    ).toBeTruthy();
  });

  it('should upgrade the CEP85 contract and return valid transaction result', async () => {
    const params: TransactionParams = {
        wasm,
        sender: owner.publicKey,
        paymentAmount,
        signingKeys: [owner],
      },
      args = {
        name,
        eventsMode: EVENTS_MODE.Native,
      },
      transactionResult: TransactionResult = await client.upgrade({
        params,
        args,
        waitForTransactionProcessed: true,
      });

    expect(
      transactionResult.transactionInfo.transactionHash.toString()
    ).toBeTruthy();
    expect(transactionResult.executionResult?.consumed).toBeTruthy();
    expect(transactionResult.executionResult?.errorMessage).toBeFalsy();

    const account = await getAccountInfo(RPC_URL, owner.publicKey);
    const contractHash = findKeyFromAccountNamedKeys(
      account,
      `cep85_contract_hash_${name}`
    );
    expect(contractHash).toBeDefined();
    const contractPackageHash = findKeyFromAccountNamedKeys(
      account,
      `cep85_contract_package_${name}`
    );
    expect(contractPackageHash).toBeDefined();
  }, 60000);
});
