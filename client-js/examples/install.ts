/* eslint-disable eslint-comments/disable-enable-pair */
/* eslint-disable no-console */

import {
  CEP85Client,
  EventsMode
} from "../src/index";

import {
  FAUCET_KEYS,
  getDeploy,
  getAccountInfo,
  getAccountNamedKeyValue,
  USER1_KEYS,
  name,
  uri,
  NETWORK_NAME,
  NODE_URL
} from "./common";

const install = async () => {
  const cc = new CEP85Client(NODE_URL, NETWORK_NAME);

  const installDeploy = cc.install(
    {
      name,
      uri,
      events_mode: EventsMode.CES,
      burner_list: [USER1_KEYS.publicKey],
      enable_burn: true
    },
    "200000000000",
    FAUCET_KEYS.publicKey,
    [FAUCET_KEYS]
  );

  const hash = await installDeploy.send(NODE_URL);

  console.log(`... Contract installation deployHash: ${hash}`);

  await getDeploy(NODE_URL, hash);

  console.log(`... Contract installed successfully.`);

  const accountInfo = await getAccountInfo(
    NODE_URL,
    FAUCET_KEYS.publicKey
  );

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
};

if (require.main === module) {
  install().catch((error) => console.error(error));
}

export { install };
