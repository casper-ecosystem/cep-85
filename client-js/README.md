## casper-cep85-js-client

This package was created to help JavaScript/TypeScript users with the [cep-85](https://github.com/casper-ecosystem/cep-85) contract and is published in `npm` as the [casper-cep85-js-client](https://www.npmjs.com/package/casper-cep85-js-client). It was built on top of the [casper-js-sdk](https://github.com/casper-ecosystem/casper-js-sdk).

`CEP85Client` is a TypeScript class that provides a high-level interface for interacting with a Casper blockchain smart contract implementing the CEP-85 standard. The standard defines methods for managing fungible and non-fungible tokens (NFTs) and setting various parameters for token contracts. The distinguishing factor between NFTs and fungible tokens lies in their uniqueness and divisibility. NFTs represent unique assets with a total supply of 1, while fungible tokens are interchangeable, for instance in a game.

This JavaScript client gives you an easy way to install and interact with the Casper CEP-85 contract.
Users can treat this package as a deploy builder for all of these possible interactions:

- Contract installation including different configurations
- Token minting
- Token transfers
- Token burning
- Batch actions
- Approval of operators
- Changing security configurations after installation
- Setting the URI for a collection or tokens
- Querying some of the contract-related data

## Usage

1. To install the client, run the following command:

   `npm i -S casper-cep85-js-client`

2. Import the CEP-85 contract in the code for your project:

   `import { CEP85Client } from 'casper-cep85-js-client'`

3. If you want to install an instance of CEP-85 on a Casper network, familiarize yourself with the `install` method and all possible configuration options (`InstallArgs`).

4. If you want to work with a previously installed instance of CEP-85, use the `setContractHash(contractHash)` method and include the existing `Contract Hash` for that instance of CEP-85.

> NOTE: Since version `1.3`, both `casper-js-sdk` and `@make-software/ces-js-parser` are peer dependencies. If you use npm version `<7`, you may need to install both dependencies manually.

## Prepare

Run the following command to copy the CEP-85 contract Wasm file to the `/wasm/` folder:

`npm run wasms:convert`

## Examples

As a starting point, look into the [TUTORIAL](https://github.com/casper-ecosystem/TUTORIAL.md) and the `examples/` directory to see the most common usage scenarios (`install.ts` and `usage.ts`). Install the contract and run the example scenario by running the `npm run example:install` and `npm run example:usage` commands. You must specify environment variables, preferably in a `client-js/.env` file. Here are some example values for the environment variables that need to be specified, pointing to a local NCTL network:

```
NODE_URL=http://localhost:11101/rpc
EVENT_STREAM_ADDRESS='http://localhost:18101/events/main'
NETWORK_NAME=casper-net-1
MASTER_KEY_PAIR_PATH=/Users/someuser/.casper/casper-node/utils/nctl/assets/net-1/faucet
USER1_KEY_PAIR_PATH=/Users/someuser/.casper/casper-node/utils/nctl/assets/net-1/users/user-1
```

## Development

Before installing the node modules, run the following command in the root folder of the `cep-85` repository to generate the contract Wasm:

```bash
make build-contract
```

After generating the Wasm file, install the node modules and the Wasm will be automatically bundled.

```bash
cd client-js
npm install && npm run wasms:convert
```

## Testing

The following command runs the integration tests:

```bash
npm run test
```

## Event Handling

CEP-85 tokens support the [Casper Event Standard (CES)](https://github.com/make-software/casper-event-standard), and the tokens can be installed with or without event logging as described [here](../cep85/README.md#eventsmode). If you install a token with the `EventsMode` set to CES, you can listen to token events using the `EventStream` from the `casper-js-sdk`. To consume token events, you should also install the `@make-software/ces-js-parser` by running this command:

```bash
npm install @make-software/ces-js-parser
```

Here is how to set up the `EventStream`:

```ts
import { EventStream } from 'casper-js-sdk';
import { CEP85Client } from 'casper-cep85-js-client';

const cep85 = new CEP85Client(
  'http://localhost:11101/rpc', // Node address
  'casper-net-1' // Network name
);
CEP85Client.setContractHash(
  `hash-0885c63f5f25ec5b6f3b57338fae5849aea5f1a2c96fc61411f2bfc5e432de5a`
);
await CEP85Client.setupEventStream(
  new EventStream('http://localhost:18101/events/main')
);
```

Here is how you can consume events using event listeners:

- Add an event listener:

  ```ts
  const listener = (event) => {
    console.log(event.name); // 'Burn'
    console.log(event.data); // Burn event info
  };

  cep85.on('Burn', listener);
  ```

- Remove an event listener:

  ```ts
  cep85.off('Burn', listener);
  ```
