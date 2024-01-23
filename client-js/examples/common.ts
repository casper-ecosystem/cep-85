/* eslint-disable eslint-comments/disable-enable-pair */
/* eslint-disable no-console */

import { config as dotenvConfig } from "dotenv";
import {
  Keys,
  CLPublicKey,
  CasperServiceByJsonRPC,
} from "casper-js-sdk";

import * as fs from "fs";

dotenvConfig();

const { MASTER_KEY_PAIR_PATH, USER1_KEY_PAIR_PATH } =
  process.env;

const DEPLOY_TIMEOUT = parseInt(
  process.env.DEPLOY_TIMEOUT || '1200000',
  10
);

export const FAUCET_KEYS = Keys.Ed25519.parseKeyFiles(
  `${MASTER_KEY_PAIR_PATH}/public_key.pem`,
  `${MASTER_KEY_PAIR_PATH}/secret_key.pem`
);

export const USER1_KEYS = Keys.Ed25519.parseKeyFiles(
  `${USER1_KEY_PAIR_PATH}/public_key.pem`,
  `${USER1_KEY_PAIR_PATH}/secret_key.pem`
);

export const NODE_URL = process.env.NODE_URL || 'http://localhost:11101/rpc';
export const EVENT_STREAM_ADDRESS =
  process.env.EVENT_STREAM_ADDRESS || 'http://localhost:18101/events/main';

export const NETWORK_NAME = process.env.NETWORK_NAME || 'casper-net-1';

export const name = "casper_test";
export const uri = "https://test-cdn-domain/{id}.json";

export const getBinary = (pathToBinary: string) => new Uint8Array(fs.readFileSync(pathToBinary, null).buffer);

export const sleep = (ms: number) => new Promise((resolve) => { setTimeout(resolve, ms); });

export const getDeploy = async (nodeURL: string, deployHash: string) => {
  const client = new CasperServiceByJsonRPC(nodeURL);
  await client.waitForDeploy(deployHash, DEPLOY_TIMEOUT);
};

export const getAccountInfo = async (
  nodeAddress: string,
  publicKey: CLPublicKey
): Promise<unknown> => {
  const client = new CasperServiceByJsonRPC(nodeAddress);
  const stateRootHash = await client.getStateRootHash();
  const accountHash = publicKey.toAccountHashStr();
  const blockState = await client.getBlockState(stateRootHash, accountHash, []);
  return blockState.Account;
};

/**
 * Returns a value under an on-chain account's storage.
 * @param accountInfo - On-chain account's info.
 * @param namedKey - A named key associated with an on-chain account.
 */
export const getAccountNamedKeyValue = (
  accountInfo: unknown,
  namedKey: string
): string | undefined => {
  const found = (
    (accountInfo as { namedKeys?: { name: string; key: string; }[]; })?.namedKeys || []
  ).find((item) => item.name === namedKey);

  return found ? found.key : undefined;
};

export const printHeader = (text: string) => {
  console.log(`******************************************`);
  console.log(`* ${text} *`);
  console.log(`******************************************`);
};
