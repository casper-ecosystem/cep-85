# CEP-85 JavaScript Client Tutorial

This tutorial outlines the usage of the JavaScript client available for the CEP-85 Standard.

Further information on the CEP-85 Standard can be found [here](https://github.com/casper-ecosystem/cep-85).

The client is available in _npm_ as [casper-cep85-js-client](https://www.npmjs.com/package/casper-cep85-js-client).

## Client Installation

The client can be installed in a project you have built using TypeScript / Javascript.

## Usage

1. To install, run:

   ```bash
   npm i -S casper-cep85-js-client
   ```

2. Import the contract in your code:

   `import { CEP85Client } from 'casper-cep85-js-client'`

3. If you want to install it, look at the `install` method and all of the possible configuration options (`InstallArgs`).

4. If you want to start working with a previously installed contract, use the `setContractHash(contractHash)` method.

NOTE: If you are using npm version `<7` you may need to install both peer dependencies manually.

```bash
npm install casper-js-sdk @make-software/ces-js-parser casper-cep85-js-client
```

## Using a CEP-85 Contract using the JavaScript Client

Set the contract hash (a unique identifier for the network) for instance:

```ts
cep85.setContractHash(
  'hash-c2402c3d88b13f14390ff46fde9c06b8590c9e45a9802f7fb8a2674ff9c1e5b1'
);
```

You can retrieve contract information by calling these methods:

```ts
const name = await cep85.collectionName();

const uri = await cep85.collectionUri();

const events_mode = await cep85.getEventsMode();
```

## Installing a CEP-85 Contract using the JavaScript Client

The `install` method crafts a [Deploy](https://docs.casperlabs.io/design/casper-design/#execution-semantics-deploys) using `InstallArgs`.
As with every deploy created by the SDK, you can send it using the `.send(rpcUrl)` method providing the RPC URL that you want to use. It will return deployHash.

```ts
const cc = new CEP85Client(process.env.NODE_URL!, process.env.NETWORK_NAME!);

const installDeploy = await cep85.install(
  {
    name: 'my-collection',
    uri: 'https://storage-domain/{id}.json',
    events_mode: EventsMode.CES,
    enable_burn: true,
    minter_list: [USER1_KEYS.publicKey],
    burner_list: [USER1_KEYS.publicKey],
    // CEP-82 filter contract may be set like this
    // transfer_filter_contract: 'hash-5f272c9300e51657de7ff68489285efa9c0f62b1574b7614a8486c4cc497a690',
    // transfer_filter_method: 'transfer_filter'
  },
  '250000000000',
  FAUCET_KEYS.publicKey,
  [FAUCET_KEYS]
);

const hash = await installDeploy.send(process.env.NODE_URL!);
```

`InstallArgs` are specified as follows:

- `name` - The name of the collection, passed in as a `String`. **This parameter is required and cannot be changed post installation**.

- `uri` - The uri representing a global given uri for the collection, passed in as a `String`. **This parameter is required and can be changed post installation, or set per token via set_uri entrypoint**.

- `events_mode` - The `EventsMode` modality that dictates the events behavior of the contract. This optional argument is passed in as a `u8` value and default to no events. **This parameter and can be changed post installation via set_modalities entrypoint**.

- `enable_burn` - The `BurnMode` modality dictates whether minted tokens can be burned. This optional parameter will not allow tokens to be burnt by default. **This parameter can be changed once the contract has been installed via set_modalities entrypoin**.

- `admin_list` : A list of users with `admin` access to this contract instance. Passed in as a string consisting of a list of `PublicKeys`.
- `minter_list` : A list of users that can mint tokens using this contract instance. Passed in as a string consisting of a list of `PublicKeys`.
- `burner_list` : A list of users that can burn tokens using this contract instance. Passed in as a string consisting of a list of `PublicKeys`.
- `meta_list` : A list of users that have access to the `set_uri` entrypoint. Passed in as a string consisting of a list of `PublicKeys`.
- `none_list` : A list of users without (banned of) special access to the contract instance. Passed in as a string consisting of a list of `PublicKeys`.

**These security parameters can be changed once the contract has been installed via change_security entrypoint**.

Further information on CEP-85 modality options can be found in the base [cep-85](https://github.com/casper-ecosystem/cep-85) repository on GitHub.

## Examples

This example will acquire the contract's hash and package hash, prior to sending three separate deploys to perform several function tests as follows:

- `Mint` - The example will attempt to mint a token using the installation account.

- `Transfer` - The example will transfer the previously minted token to a second account (USER1 as defined in the variables.)

- `Burn` - The example will burn the minted token.

The associated code for these deploys may be found in the `client-js/examples` directory.

## Minting a Token

The CEP-85 JS Client includes code to construct a deploy that will `Mint` a token (with minter rights), as follows:

```ts
const mintDeploy = cep85.mint(
  {
    recipient: USER1_KEYS.publicKey,
    id: '1',
    amount: '10',
  },
  { useSessionCode: true },
  '2000000000',
  USER1_KEYS.publicKey,
  [USER1_KEYS]
);

const mintDeployHash = await mintDeploy.send(process.env.NODE_URL!);
```

Minting accepts the following arguments:

- `recipient` - The account receiving the minted token.

- `id` - The sequential ID assigned to a token in mint order.

- `amount` - The amount of supply (and thus total supply) the minted token has.

## Transferring a Token

After minting one or more tokens, you can then use the following code to transfer the tokens between accounts:

```ts
const transferDeploy = cep85.transfer(
  {
    from: USER1_KEYS.publicKey,
    to: FAUCET_KEYS.publicKey,
    id: '1',
    amount: '5',
  },
  { useSessionCode: true },
  '13000000000',
  USER1_KEYS.publicKey,
  [USER1_KEYS]
);

const transferDeployHash = await transferDeploy.send(process.env.NODE_URL!);
```

Transferring accepts the following arguments:

- `id` - The sequential ID assigned to a token in mint order.

- `from` - The account sending the token in question.

- `to` - The account receiving the transferred token.

- `amount` - The amount of supply receiving the transferred token.

## Burning a Token

The following code shows how to decrease supply of a token as an owner or as operator with burner rights

```ts
const burnDeploy = await contractClient.burn(
  {
    owner: USER1_KEYS.publicKey,
    id,
    amount: '5',
  },
  '13000000000',
  USER1_KEYS.publicKey,
  [USER1_KEYS]
);

const burnDeployHash = await burnDeploy.send(process.env.NODE_URL!);
```

## More examples

### Running an Install Example

This repository includes an example script for installing a CEP-85 contract instance.

You will need to define the following variables in the `.env` file:

- `NODE_URL` - The address of a node. If you are testing using [NCTL](https://docs.casperlabs.io/dapp-dev-guide/building-dapps/setup-nctl/), this will be `http://localhost:11101/rpc`.

- `NETWORK_NAME` - The name of the Casper network you are operating on, `casper-net-1` when testing using a local network with NCTL.

- `MASTER_KEY_PAIR_PATH` - The path to the key pair of the minting account.

- `USER1_KEY_PAIR_PATH` - The path to an additional account's key pair for use in testing transfer features.

This example can be run using the following command:

```ts
npm run example:install
```

The example will then return the installation's `deployHash`, and inform you when the installation is successful.

The example will then provide the installing account's information, which will include the CEP-85 contract's hash and package hash.

### Running an Usage Example

A usage example uses the same variables as the Install example above, but tests the basic functionality of the contract after installation.

The usage example can be run using the following command:

```ts
npm run example:usage
```
