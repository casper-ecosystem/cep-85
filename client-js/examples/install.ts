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
  uri
} from "./common";

const install = async () => {
  const cc = new CEP85Client(process.env.NODE_URL!, process.env.NETWORK_NAME!);

  const installDeploy = cc.install(
    {
      name,
      uri,
      eventsMode: EventsMode.CES,
      burner_list: [USER1_KEYS.publicKey]
    },
    "250000000000",
    FAUCET_KEYS.publicKey,
    [FAUCET_KEYS]
  );

  const hash = await installDeploy.send(process.env.NODE_URL!);

  console.log(`... Contract installation deployHash: ${hash}`);

  await getDeploy(process.env.NODE_URL!, hash);

  console.log(`... Contract installed successfully.`);

  const accountInfo = await getAccountInfo(
    process.env.NODE_URL!,
    FAUCET_KEYS.publicKey
  );

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
};

if (require.main === module) {
  install().catch((error) => console.error(error));
}

export { install };
