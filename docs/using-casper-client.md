# Installing and Interacting with the Contract using the Rust Casper Client

This documentation will guide you through installing and interacting with an instance of the CEP-85 multi-token standard contract through Casper's Rust CLI client. The contract code installs an instance of CEP-85 as per session arguments provided at the time of installation. It requires a minimum Rust version of `1.63.0`.

## Installing the Contract

Installing the multi-token contract to global state requires the use of a [Deploy](https://docs.casper.network/developers/cli/sending-deploys/). In this case, the session code can be compiled to Wasm by running the `make test` command provided in the Makefile at the top level. The Wasm will be found in the `target/wasm32-unknown-unknown/release` directory as `cep85.wasm`.

Below is an example of a `casper-client` command that provides all required session arguments to install a valid instance of the CEP-85 contract on global state.

- `casper-client put-deploy -n http://localhost:11101/rpc --chain-name "casper-net-1" --payment-amount 200000000000 -k ~/casper/casper-node/utils/nctl/assets/net-1/nodes/node-1/keys/secret_key.pem --session-path ~/casper/cep-85/target/wasm32-unknown-unknown/release/cep85.wasm`

1. `--session-arg "name:string='multi-token-1'"`

   The name of the token as a string. In this instance, "multi-token-1".

2. `--session-arg "uri:string='https://docs.casper.network/{id}.json'"`

   A string URI for an off-chain resource associated with the token. In this instance, "https://docs.casper.network/{id}.json".

3. `--session-arg "events_mode:u8='0'"`

   The events mode for this contract. In this instance the 0 represents the `NoEvents` version of `EventsMode`.

4. `--session-arg "enable_burn:bool='true'"`

   The burn mode for this contract. In this instance, true represents that burning is `Enabled` for those with the proper access.

<details>
<summary><b>Casper client command without comments</b></summary>

```bash
casper-client put-deploy -n http://localhost:11101/rpc --chain-name "casper-net-1" --payment-amount 200000000000 -k ~/casper/casper-node/utils/nctl/assets/net-1/nodes/node-1/keys/secret_key.pem --session-path ~/casper/cep-85/target/wasm32-unknown-unknown/release/cep85.wasm \
--session-arg "name:string='multi-token-1'" \
--session-arg "uri:string='https://docs.casper.network/{id}.json'" \
--session-arg "events_mode:u8='0'" \
--session-arg "enable_burn:bool='true'"
```

</details>

## Available Actions

1. [Minting a Token](#minting-a-token)

2. [Batch Minting Tokens](#batch-minting-tokens)

3. [Burning a Token](#burning-a-token)

4. [Batch Burning Tokens](#batch-burning-tokens)

5. [Checking the Supply of a Token](#checking-the-supply-of-a-token)

6. [Checking the Total Supply of a Token](#checking-the-total-supply-of-a-token)

7. [Setting the Total Supply of a Token](#setting-the-total-supply-of-a-token)

8. [Setting the Total Supply of a Batch of Tokens](#setting-the-total-supply-of-a-batch-of-tokens)

9. [Checking the Balance of a Single Token ID](#checking-the-balance-of-a-single-token-id)

10. [Checking the Balance of Multiple Token IDs](#checking-the-balance-of-multiple-token-ids)

11. [Approving An Operator to Transfer Tokens](#approving-an-operator-to-transfer-tokens)

12. [Checking Approval Status of an Account](#checking-approval-status-of-an-account)

13. [Transferring a Token](#transferring-a-token)

14. [Transferring a Batch of Tokens](#transferring-a-batch-of-tokens)

15. [Checking the URI for a Token](#checking-the-uri-for-a-token)

16. [Setting the URI of a Token](#setting-the-uri-of-a-token)

17. [Checking a Token's Fungibility](#checking-a-tokens-fungibility)

18. [Checking a Token's Total Fungible Supply](#checking-a-tokens-total-fungible-supply)

19. [Changing Account Security Permissions](#changing-account-security-permissions)

20. [Setting Modalities](#setting-modalities)

21. [Upgrading Collection Contract](#upgrading-collection-contract)

## Minting a Token

The following command will invoke the `mint` entrypoint on your instance of CEP-85, directing it to mint the given amount of a specified token ID to the recipient address. The account sending this deploy must be on the `minter_list` or `admin_list`.

```json

casper-client put-deploy -n http://<node IP>:<PORT> \
// The chain name of the Casper network on which your CEP-85 instance was installed.
--chain-name <CHAIN NAME> \
// The local path to your account's secret key.
--secret-key ~/casper/demo/user_a/secret_key.pem \
// The contract hash of your CEP-85 contract instance.
--session-hash hash-b568f50a64acc8bbe43462ffe243849a88111060b228dacb8f08d42e26985180 \
// The name of the entrypoint you are invoking.
--session-entry-point "mint" \
// The account hash of the account to which you are minting CEP-85 tokens.
--session-arg "recipient:key='account-hash-9f81014b9c7406c531ebf0477132283f4eb59143d7903a2fae54358b26cea44b'" \
// The ID of the CEP-85 token you are sending to the receiving account.
--session-arg "id:u256='2'" \
// The amount of the specified CEP-85 token you are sending to the receiving account.
--session-arg "amount:u256='10'" \
// An optional URI for the token if different from global URI set during installation.
--session-arg "uri:string='https://test-cdn-domain/{id}.json'" \
// The gas payment you are allotting, in motes.
--payment-amount "1000000000"

```

<details>
<summary><b>Casper client command without comments</b></summary>

```json

casper-client put-deploy -n http://<node IP>:<PORT> \
--chain-name <CHAIN NAME> \
--secret-key ~/casper/demo/user_a/secret_key.pem \
--session-hash hash-b568f50a64acc8bbe43462ffe243849a88111060b228dacb8f08d42e26985180 \
--session-entry-point "mint" \
--session-arg "recipient:key='account-hash-9f81014b9c7406c531ebf0477132283f4eb59143d7903a2fae54358b26cea44b'" \
--session-arg "id:u256='2'" \
--session-arg "amount:u256='10'" \
--session-arg "uri:string='https://test-cdn-domain/{id}.json'" \
--payment-amount "1000000000"

```

</details>

## Batch Minting Tokens

The following command will invoke the `batch_mint` entrypoint on your instance of CEP-85, directing it to mint the given amount of several token IDs to the recipient address. The account sending this deploy must be on the `minter_list` or `admin_list`.

```json

casper-client put-deploy -n http://<node IP>:<PORT> \
// The chain name of the Casper network on which your CEP-85 instance was installed.
--chain-name <CHAIN NAME> \
// The local path to your account's secret key.
--secret-key ~/casper/demo/user_a/secret_key.pem \
// The contract hash of your CEP-85 contract instance.
--session-hash hash-b568f50a64acc8bbe43462ffe243849a88111060b228dacb8f08d42e26985180 \
// The name of the entrypoint you are invoking.
--session-entry-point "batch_mint" \
--session-args-json '[
// The account hash of the account to which you are minting CEP-85 tokens.
{"name":"recipient","type":"Key","value":"account-hash-9f81014b9c7406c531ebf0477132283f4eb59143d7903a2fae54358b26cea44b"},
// The IDs of the CEP-85 tokens you are sending to the receiving account.
{"name":"ids","type":{"List":"U256"},"value":[3,5]},
// The amounts of the specified CEP-85 tokens you are sending to the receiving account.
{"name":"amounts","type":{"List":"U256"},"value":[10,25]},
// An optional URI for the tokens if different from global URI set during installation.
{"name":"uri","type":"String","value":"https://test-cdn-domain/{id}.json"}
]' \
// The gas payment you are allotting, in motes.
--payment-amount "1500000000"

```

<details>
<summary><b>Casper client command without comments</b></summary>

```json

casper-client put-deploy -n http://<node IP>:<PORT> \
--chain-name <CHAIN NAME> \
--secret-key ~/casper/demo/user_a/secret_key.pem \
--session-hash hash-b568f50a64acc8bbe43462ffe243849a88111060b228dacb8f08d42e26985180 \
--session-entry-point "batch_mint" \
--session-args-json '[
{"name":"recipient","type":"Key","value":"account-hash-9f81014b9c7406c531ebf0477132283f4eb59143d7903a2fae54358b26cea44b"},
{"name":"ids","type":{"List":"U256"},"value":[3,5]},
{"name":"amounts","type":{"List":"U256"},"value":[10,25]},
{"name":"uri","type":"String","value":"https://test-cdn-domain/{id}.json"}
]' \
--payment-amount "1500000000"

```

</details>

## Burning a Token

The following command will invoke the `burn` entrypoint on your instance of CEP-85, directing it to burn the given amount of tokens at the owner address. The account sending this deploy must be on the `burner_list` or `admin_list`.

```json

casper-client put-deploy -n http://<node IP>:<PORT> \
// The chain name of the Casper network on which your CEP-85 instance was installed.
--chain-name <CHAIN NAME> \
// The local path to your account's secret key.
--secret-key ~/casper/demo/user_a/secret_key.pem \
// The contract hash of your CEP-85 contract instance.
--session-hash hash-b568f50a64acc8bbe43462ffe243849a88111060b228dacb8f08d42e26985180 \
// The name of the entrypoint you are invoking.
--session-entry-point "burn" \
// The account hash of the account from which you are burning CEP-85 tokens.
--session-arg "owner:key='account-hash-9f81014b9c7406c531ebf0477132283f4eb59143d7903a2fae54358b26cea44b'" \
// The IDs of the CEP-85 tokens you are burning from the owner account.
--session-arg "id:U256='2'" \
// The amounts of the specified CEP-85 tokens you are removing from the owner account.
--session-arg "amount:U256='10'" \
// The gas payment you are allotting, in motes.
--payment-amount "10000000000"

```

<details>
<summary><b>Casper client command without comments</b></summary>

```json

casper-client put-deploy -n http://<node IP>:<PORT> \
--chain-name <CHAIN NAME> \
--secret-key ~/casper/demo/user_a/secret_key.pem \
--session-hash hash-b568f50a64acc8bbe43462ffe243849a88111060b228dacb8f08d42e26985180 \
--session-entry-point "burn" \
--session-arg "owner:key='account-hash-9f81014b9c7406c531ebf0477132283f4eb59143d7903a2fae54358b26cea44b'" \
--session-arg "id:U256='2'" \
--session-arg "amount:U256='10'" \
--payment-amount "10000000000"

```

</details>

## Batch Burning Tokens

The following command will invoke the `batch_burn` entrypoint on your instance of CEP-85, directing it to burn the given amount of several token IDs at the owner address. The account sending this deploy must be on the `burner_list` or `admin_list.

```json

casper-client put-deploy -n http://<node IP>:<PORT> \
// The chain name of the Casper network on which your CEP-85 instance was installed.
--chain-name <CHAIN NAME> \
// The local path to your account's secret key.
--secret-key ~/casper/demo/user_a/secret_key.pem \
// The contract hash of your CEP-85 contract instance.
--session-hash hash-b568f50a64acc8bbe43462ffe243849a88111060b228dacb8f08d42e26985180 \
// The name of the entrypoint you are invoking.
--session-entry-point "batch_burn" \
--session-args-json '[
// The account hash of the account from which you are burning CEP-85 tokens.
{"name":"owner","type":"Key","value":"account-hash-9f81014b9c7406c531ebf0477132283f4eb59143d7903a2fae54358b26cea44b"},
// The IDs of the CEP-85 tokens you are removing from the owning account.
{"name":"ids","type":{"List":"U256"},"value":[3,5]},
// The amounts of the specified CEP-85 tokens you are removing from the owning account.
{"name":"amounts","type":{"List":"U256"},"value":[10,25]}
]' \
// The gas payment you are allotting, in motes.
--payment-amount "1500000000"

```

<details>
<summary><b>Casper client command without comments</b></summary>

```json

casper-client put-deploy -n http://<node IP>:<PORT> \
--chain-name <CHAIN NAME> \
--secret-key ~/casper/demo/user_a/secret_key.pem \
--session-hash hash-b568f50a64acc8bbe43462ffe243849a88111060b228dacb8f08d42e26985180 \
--session-args-json '[
{"name":"owner","type":"Key","value":"account-hash-9f81014b9c7406c531ebf0477132283f4eb59143d7903a2fae54358b26cea44b"},
{"name":"ids","type":{"List":"U256"},"value":[3,5]},
{"name":"amounts","type":{"List":"U256"},"value":[10,25]}
]' \
--payment-amount "10000000000"

```

</details>

## Checking the Current Supply of a Token

The following command will query the `supply` dictionary of your instance of CEP-85, verifying the current supply of the provided token ID.

```json

casper-client get-dictionary-item -n http://<node IP>:<PORT> \
// The current state root hash.
--state-root-hash 107a33f19093b8a17cea32fd53595507e8843a30cbb5e7160d9b276b4bec3538 \
// The contract hash of your CEP-85 contract instance.
--contract-hash hash-b568f50a64acc8bbe43462ffe243849a88111060b228dacb8f08d42e26985180 \
// The name of the dictionary you are invoking.
--dictionary-name "supply" \
// The ID of the CEP-85 token for which you are checking the supply.
--dictionary-item-key "2"

```

<details>
<summary><b>Casper client command without comments</b></summary>

```json

casper-client get-state-root-hash -n http://<node IP>:<PORT>

casper-client get-dictionary-item -n http://<node IP>:<PORT> \
--state-root-hash 107a33f19093b8a17cea32fd53595507e8843a30cbb5e7160d9b276b4bec3538 \
--contract-hash hash-4ea5839b8c7e6cd1fbd67fce05cea2c5cb6097ede90aeb580f3ebf251646ad01 \
--dictionary-name "supply" \
--dictionary-item-key "2"

```

</details>

## Checking the Total Supply of a Token

The following command will query the `total_supply` dictionary of your instance of CEP-85, verifying the total supply of the provided token ID.

```json

casper-client get-dictionary-item -n http://<node IP>:<PORT> \
// The state root hash
--state-root-hash 107a33f19093b8a17cea32fd53595507e8843a30cbb5e7160d9b276b4bec3538 \
// The contract hash of your CEP-85 contract instance.
--contract-hash hash-b568f50a64acc8bbe43462ffe243849a88111060b228dacb8f08d42e26985180 \
// The name of the dictionary you are invoking.
--dictionary-name "total_supply" \
// The ID of the CEP-85 token for which you are checking the supply.
--dictionary-item-key "2"

```

<details>
<summary><b>Casper client command without comments</b></summary>

```json

casper-client get-state-root-hash -n http://<node IP>:<PORT>

casper-client get-dictionary-item -n http://<node IP>:<PORT> \
--state-root-hash 107a33f19093b8a17cea32fd53595507e8843a30cbb5e7160d9b276b4bec3538 \
--contract-hash hash-4ea5839b8c7e6cd1fbd67fce05cea2c5cb6097ede90aeb580f3ebf251646ad01 \
--dictionary-name "total_supply" \
--dictionary-item-key "2"

```

</details>

## Setting the Total Supply of a Token

The following command will invoke the `set_total_supply_of` entrypoint of your instance of CEP-85, setting the total supply of the provided token ID. The new total supply provided must be larger than the previous total supply. The account sending this deploy must be on the `admin_list`.

```json

casper-client put-deploy -n http://<node IP>:<PORT> \
// The chain name of the Casper network on which your CEP-85 instance was installed.
--chain-name <CHAIN NAME> \
// The local path to your account's secret key.
--secret-key ~/casper/demo/user_a/secret_key.pem \
// The contract hash of your CEP-85 contract instance.
--session-hash hash-b568f50a64acc8bbe43462ffe243849a88111060b228dacb8f08d42e26985180 \
// The name of the entrypoint you are invoking.
--session-entry-point "set_total_supply_of" \
// The ID of the CEP-85 token you are setting the total supply of.
--session-arg "id:u256='2'" \
// The new total supply number.
--session-arg "total_supply:u256='200'" \
// The gas payment you are allotting, in motes.
--payment-amount "500000000"

```

<details>
<summary><b>Casper client command without comments</b></summary>

```json

casper-client put-deploy -n http://<node IP>:<PORT> \
--chain-name <CHAIN NAME> \
--secret-key ~/casper/demo/user_a/secret_key.pem \
--session-hash hash-b568f50a64acc8bbe43462ffe243849a88111060b228dacb8f08d42e26985180 \
--session-entry-point "set_total_supply_of" \
--session-arg "id:u256='2'" \
--session-arg "total_supply:u256='200'" \
--payment-amount "500000000"

```

</details>

## Setting the Total Supply of a Batch of Tokens

The following command will invoke the `set_total_supply_of_batch` entrypoint of your instance of CEP-85, setting the total supplies of the provided token IDs. The new total supplies provided must be larger than the previous total supplies. The account sending this deploy must be on the `admin_list`.

```json

casper-client put-deploy -n http://<node IP>:<PORT> \
// The chain name of the Casper network on which your CEP-85 instance was installed.
--chain-name <CHAIN NAME> \
// The local path to your account's secret key.
--secret-key ~/casper/demo/user_a/secret_key.pem \
// The contract hash of your CEP-85 contract instance.
--session-hash hash-b568f50a64acc8bbe43462ffe243849a88111060b228dacb8f08d42e26985180 \
// The name of the entrypoint you are invoking.
--session-entry-point "set_total_supply_of_batch" \
--session-args-json '[
// The IDs of the CEP-85 tokens you are setting the total supply of.
{"name":"ids","type":{"List":"U256"},"value":[3,5]},
// The new total supply numbers.
{"name":"total_supplies","type":{"List":"U256"},"value":[10,25]}
// The gas payment you are allotting, in motes.
]' \
--payment-amount "500000000"

```

<details>
<summary><b>Casper client command without comments</b></summary>

```json

casper-client put-deploy -n http://<node IP>:<PORT> \
--chain-name <CHAIN NAME> \
--secret-key ~/casper/demo/user_a/secret_key.pem \
--session-hash hash-b568f50a64acc8bbe43462ffe243849a88111060b228dacb8f08d42e26985180 \
--session-entry-point "set_total_supply_of_batch" \
--session-args-json '[
{"name":"ids","type":{"List":"U256"},"value":[3,5]},
{"name":"total_supplies","type":{"List":"U256"},"value":[10,25]}
]' \
--payment-amount "500000000"

```

</details>

## Checking the Balance of a Single Token ID

Checking an owner's token balance requires two pieces of information combined: the key identifying the owner and the token ID in the form of a hash key to a dictionary item.

The following command will invoke the `make_dictionary_item_key` session entrypoint on your instance of CEP-85 and write the hash key to your account context.

```json

casper-client put-deploy -n http://<node IP>:<PORT> \
// The chain name of the Casper network on which your CEP-85 instance was installed.
--chain-name <CHAIN NAME> \
// The local path to your account's secret key.
--secret-key ~/casper/demo/user_a/secret_key.pem \
// The contract hash of your CEP-85 contract instance.
--session-hash hash-b568f50a64acc8bbe43462ffe243849a88111060b228dacb8f08d42e26985180 \
// The name of the entrypoint you are invoking.
--session-entry-point "make_dictionary_item_key" \
// The account hash of the account that you are querying as an owner.
--session-arg "owner:key='account-hash-9f81014b9c7406c531ebf0477132283f4eb59143d7903a2fae54358b26cea44b'" \
// The ID of the CEP-85 token you are querying.
--session-arg "id:u256='2'" \
// An optional name argument can be set to specify the name of the key in your account
// --session-arg "name:string='my_custom_result_key'" \
// The gas payment you are allotting, in motes.
--payment-amount "500000000"

```

<details>
<summary><b>Casper client command without comments</b></summary>

```json

casper-client put-deploy -n http://<node IP>:<PORT> \
--chain-name <CHAIN NAME> \
--secret-key ~/casper/demo/user_a/secret_key.pem \
--session-hash hash-b568f50a64acc8bbe43462ffe243849a88111060b228dacb8f08d42e26985180 \
--session-entry-point "make_dictionary_item_key" \
--session-arg "owner:key='account-hash-9f81014b9c7406c531ebf0477132283f4eb59143d7903a2fae54358b26cea44b'" \
--session-arg "id:u256='2'" \
--payment-amount "500000000"

```

</details><br>

After sending this command to the contract entrypoint, you must query the value of the dictionary item key `cep85_dictionary_item_key` within your account's `NamedKeys`. If you provided a custom key in the previous command, you will need to use it instead. If you provided a custom key in the previous command, you will need to use it instead.

Query the new named key using the following command to retrieve the dictionary item key:

```json
casper-client query-global-state -n http://<NODE IP>:<PORT> \
// This is your `account` hash location from your `NamedKeys`.
--key account-hash-9f81014b9c7406c531ebf0477132283f4eb59143d7903a2fae54358b26cea44b \
// This is the name of the dictionary item from your `NamedKeys`.
--query-path "cep85_dictionary_item_key" \
// This is the current state root hash for the Casper network where your contract is installed.
--state-root-hash 3aecd0e4b6ec29ee7c1eed701132eabfe6e66a1e0f1595c9c65bfed447e474f7
```

<details>
<summary><b>Casper client command without comments</b></summary>

```bash
casper-client query-global-state -n http://<NODE IP>:<PORT> \
--key account-hash-9f81014b9c7406c531ebf0477132283f4eb59143d7903a2fae54358b26cea44b \
--query-path "cep85_dictionary_item_key" \
--state-root-hash 3aecd0e4b6ec29ee7c1eed701132eabfe6e66a1e0f1595c9c65bfed447e474f7
```

The result of this query will be as a CLValue. Copy the "parsed" value and use it in the following command to get the token balance.

```json
{
  "jsonrpc": "2.0",
  "id": -8867683794280721261,
  "result": {
    "api_version": "1.0.0",
    "block_header": null,
    "stored_value": {
      "CLValue": {
        "cl_type": "String",
        "bytes": "4000000065643932633330346238333230393835653261623936626434353535313261643534353939306639386164623065353639626136333363313637393164356564",
        "parsed": "ed92c304b8320985e2ab96bd455512ad545990f98adb0e569ba633c16791d5ed"
      }
    },
    "merkle_proof": "[7356 hex chars]"
  }
}
```

</details><br>

> You can use the JS client to get a dictionary item key without sending a deploy. Use `makeDictionaryItemKey` in the [CEP85Client.ts](../client-js/src/CEP85Client.ts).

The following command will query the `balances` dictionary of your instance of CEP-85, verifying the balance of the provided owners key and token ID with its corresponding dictionary item key. As this dictionary item key combining an entity key and an id will never change, for further balances checks you can reuse this dictionary item key again in your queries.

```json

casper-client get-dictionary-item -n http://<node IP>:<PORT> \
// The current state root hash.
--state-root-hash 107a33f19093b8a17cea32fd53595507e8843a30cbb5e7160d9b276b4bec3538 \
// The contract hash of your CEP-85 contract instance.
--contract-hash hash-b568f50a64acc8bbe43462ffe243849a88111060b228dacb8f08d42e26985180 \
// The name of the dictionary you are invoking.
--dictionary-name "balances" \
// The Key/ID hash of the CEP-85 token for which you are checking the supply.
--dictionary-item-key "ed92c304b8320985e2ab96bd455512ad545990f98adb0e569ba633c16791d5ed"

```

<details>
<summary><b>Casper client command without comments</b></summary>

```json

casper-client get-dictionary-item -n http://<node IP>:<PORT> \
--state-root-hash 107a33f19093b8a17cea32fd53595507e8843a30cbb5e7160d9b276b4bec3538
--contract-hash hash-b568f50a64acc8bbe43462ffe243849a88111060b228dacb8f08d42e26985180 \
--dictionary-name "balances" \
--dictionary-item-key "ed92c304b8320985e2ab96bd455512ad545990f98adb0e569ba633c16791d5ed"

```

</details><br>

> You can also use the [JS client](../client-js/src/CEP85Client.ts) and calling `getBalanceOf`.

## Checking the Balance of Multiple Token IDs

For checking the balance of multiple token IDs, we recommend using the [JS client](../client-js/src/CEP85Client.ts) and calling `getBalanceOfBatch`.

## Approving an Operator to Transfer Tokens

The following command will invoke the `set_approval_for_all` entrypoint on your instance of CEP-85, directing it to approve or remove a given operator's ability to transfer (or burn) the calling account's tokens.

```json

casper-client put-deploy -n http://<node IP>:<PORT> \
// The chain name of the Casper network on which your CEP-85 instance was installed.
--chain-name <CHAIN NAME> \
// The local path to your account's secret key.
--secret-key ~/casper/demo/user_a/secret_key.pem \
// The contract hash of your CEP-85 contract instance.
--session-hash hash-b568f50a64acc8bbe43462ffe243849a88111060b228dacb8f08d42e26985180 \
// The name of the entrypoint you are invoking.
--session-entry-point "set_approval_for_all" \
// The account hash of the account that you are approving/removing as an operator.
--session-arg "operator:key='account-hash-9f81014b9c7406c531ebf0477132283f4eb59143d7903a2fae54358b26cea44b'" \
// A boolean representing approval (True) or removal (False).
--session-arg "approved:bool='true'" \
// The gas payment you are allotting, in motes.
--payment-amount "500000000"

```

<details>
<summary><b>Casper client command without comments</b></summary>

```json

casper-client put-deploy -n http://<node IP>:<PORT> \
--chain-name <CHAIN NAME> \
--secret-key ~/casper/demo/user_a/secret_key.pem \
--session-hash hash-b568f50a64acc8bbe43462ffe243849a88111060b228dacb8f08d42e26985180 \
--session-entry-point "set_approval_for_all" \
--session-arg "operator:key='account-hash-9f81014b9c7406c531ebf0477132283f4eb59143d7903a2fae54358b26cea44b'" \
--session-arg "approved:bool='true'" \
--payment-amount "500000000"

```

</details>

## Checking the Approval Status of an Operator

Checking an operator's approval status requires two pieces of information: the key identifying the owner and the key identifying the operator, which is a hash key to a dictionary item.

The following command will invoke the `make_dictionary_item_key` session entrypoint on your instance of CEP-85 and write the hash key to your account context.

```json

casper-client put-deploy -n http://<node IP>:<PORT> \
// The chain name of the Casper network on which your CEP-85 instance was installed.
--chain-name <CHAIN NAME> \
// The local path to your account's secret key.
--secret-key ~/casper/demo/user_a/secret_key.pem \
// The contract hash of your CEP-85 contract instance.
--session-hash hash-b568f50a64acc8bbe43462ffe243849a88111060b228dacb8f08d42e26985180 \
// The name of the entrypoint you are invoking.
--session-entry-point "make_dictionary_item_key" \
// The account hash of the account that you are querying as an owner.
--session-arg "owner:key='account-hash-303c0f8208220fe9a4de40e1ada1d35fdd6c678877908f01fddb2a56502d67fd'" \
// The account hash of the account that you are querying as an operator.
--session-arg "operator:key='account-hash-9f81014b9c7406c531ebf0477132283f4eb59143d7903a2fae54358b26cea44b'" \
// An optional name argument can be set to specify the name of the key in your account.
// --session-arg "name:string='my_custom_result_key'" \
// The gas payment you are allotting, in motes.
--payment-amount "500000000"

```

<details>
<summary><b>Casper client command without comments</b></summary>

```json

casper-client put-deploy -n http://<node IP>:<PORT> \
--chain-name <CHAIN NAME> \
--secret-key ~/casper/demo/user_a/secret_key.pem \
--session-hash hash-b568f50a64acc8bbe43462ffe243849a88111060b228dacb8f08d42e26985180 \
--session-entry-point "make_dictionary_item_key" \
--session-arg "owner:key='account-hash-9f81014b9c7406c531ebf0477132283f4eb59143d7903a2fae54358b26cea44b'" \
--session-arg "operator:key='account-hash-9f81014b9c7406c531ebf0477132283f4eb59143d7903a2fae54358b26cea44b'" \
--payment-amount "500000000"

```

</details><br>

After sending this command to the contract entrypoint, you must query the value of the dictionary item key `cep85_dictionary_item_key` within your account's `NamedKeys`. If you provided a custom key in the previous command, you will need to use it instead.

Query the new named key using the following command to retrieve the dictionary item key:

```json
casper-client query-global-state -n http://<NODE IP>:<PORT> \
// This is your `account` hash location from your `NamedKeys`.
--key account-hash-9f81014b9c7406c531ebf0477132283f4eb59143d7903a2fae54358b26cea44b \
// This is the current state root hash for the Casper network where your contract is installed.
--query-path "cep85_dictionary_item_key" \
// This is the current state root hash for the Casper network where your contract is installed.
--state-root-hash 3aecd0e4b6ec29ee7c1eed701132eabfe6e66a1e0f1595c9c65bfed447e474f7
```

<details>
<summary><b>Casper client command without comments</b></summary>

```bash
casper-client query-global-state -n http://<NODE IP>:<PORT> \
--key account-hash-9f81014b9c7406c531ebf0477132283f4eb59143d7903a2fae54358b26cea44b \
--query-path "cep85_dictionary_item_key" \
--state-root-hash 3aecd0e4b6ec29ee7c1eed701132eabfe6e66a1e0f1595c9c65bfed447e474f7
```

The result of this query will be as a CLValue. Copy the "parsed" value and use it in the following command to get the operator's approval status.

```json
{
  "jsonrpc": "2.0",
  "id": -1688620587857799264,
  "result": {
    "api_version": "1.0.0",
    "block_header": null,
    "stored_value": {
      "CLValue": {
        "cl_type": "String",
        "bytes": "4000000032656234333365343834633163366161396262356539643938626436636461373966386537623037363964616630656434353938393133646634356430643763",
        "parsed": "2eb433e484c1c6aa9bb5e9d98bd6cda79f8e7b0769daf0ed4598913df45d0d7c"
      }
    },
    "merkle_proof": "[7560 hex chars]"
  }
}
```

</details><br>

> You can use the JS client to get a dictionary item key without sending a deploy. Use `makeDictionaryItemKey` in the [CEP85Client.ts](../client-js/src/CEP85Client.ts).

The following command will query the `operators` dictionary in your instance of CEP-85, verifying the owner and operator's approval. Since the dictionary item key combines the two corresponding keys and will not change, you can reuse it in your queries for further approval checks.

```json

casper-client get-dictionary-item -n http://<node IP>:<PORT> \
// The current state root hash.
--state-root-hash 107a33f19093b8a17cea32fd53595507e8843a30cbb5e7160d9b276b4bec3538 \
// The contract hash of your CEP-85 contract instance.
--contract-hash hash-b568f50a64acc8bbe43462ffe243849a88111060b228dacb8f08d42e26985180 \
// The name of the dictionary you are invoking.
--dictionary-name "operators" \
// The Key/Key hash of the CEP-85 approval you are checking.
--dictionary-item-key "2eb433e484c1c6aa9bb5e9d98bd6cda79f8e7b0769daf0ed4598913df45d0d7c"

```

## Transferring a Token

The following command will invoke the `safe_transfer_from` entrypoint on your instance of CEP-85, directing it to transfer the given amount of a specified token ID from one account to another.

```json

casper-client put-deploy -n http://<node IP>:<PORT> \
// The chain name of the Casper network on which your CEP-85 instance was installed.
--chain-name <CHAIN NAME> \
// The local path to your account's secret key.
--secret-key ~/casper/demo/user_a/secret_key.pem \
// The contract hash of your CEP-85 contract instance.
--session-hash hash-b568f50a64acc8bbe43462ffe243849a88111060b228dacb8f08d42e26985180 \
// The name of the entrypoint you are invoking.
--session-entry-point "safe_transfer_from" \
// The account hash of the account sending the tokens.
--session-arg "from:key='account-hash-9f81014b9c7406c531ebf0477132283f4eb59143d7903a2fae54358b26cea44b'" \
// The account hash of the account receiving the tokens.
--session-arg "to:key='account-hash-303c0f8208220fe9a4de40e1ada1d35fdd6c678877908f01fddb2a56502d67fd'" \
// The ID of the CEP-85 token you are sending to the receiving account.
--session-arg "id:u256='2'" \
// The amount of the specified CEP-85 token you are sending to the receiving account.
--session-arg "amount:u256='10'" \
// "DATA is an optional argument passed as a byte array that allows the inclusion of custom bytes. These are sent to `before_token_transfer` and can be used to pass values to a transfer filter contract."
--session-arg "data:byte_list=''" \
// The gas payment you are allotting, in motes.
--payment-amount "10000000000"

```

<details>
<summary><b>Casper client command without comments</b></summary>

```json

casper-client put-deploy -n http://<node IP>:<PORT> \
--chain-name <CHAIN NAME> \
--secret-key ~/casper/demo/user_a/secret_key.pem \
--session-hash hash-b568f50a64acc8bbe43462ffe243849a88111060b228dacb8f08d42e26985180 \
--session-entry-point "safe_transfer_from" \
--session-arg "from:key='account-hash-9f81014b9c7406c531ebf0477132283f4eb59143d7903a2fae54358b26cea44b'" \
--session-arg "to:key='account-hash-303c0f8208220fe9a4de40e1ada1d35fdd6c678877908f01fddb2a56502d67fd'" \
--session-arg "id:u256='2'" \
--session-arg "amount:u256='10'" \
--session-arg "data:byte_list=''" \
--payment-amount "10000000000"

```

</details>

## Transferring a Batch of Tokens

The following command will invoke the `safe_batch_transfer_from` entrypoint on your instance of CEP-85, directing it to transfer the given amounts of the specified token IDs from one account to another.

```json

casper-client put-deploy -n http://<node IP>:<PORT> \
// The chain name of the Casper network on which your CEP-85 instance was installed.
--chain-name <CHAIN NAME> \
// The local path to your account's secret key.
--secret-key ~/casper/demo/user_a/secret_key.pem \
// The contract hash of your CEP-85 contract instance.
--session-hash hash-b568f50a64acc8bbe43462ffe243849a88111060b228dacb8f08d42e26985180 \
// The name of the entrypoint you are invoking.
--session-entry-point "safe_batch_transfer_from" \
--session-args-json '[
// The account hash of the account sending the tokens.
{"name":"from","type":"Key","value":"account-hash-9f81014b9c7406c531ebf0477132283f4eb59143d7903a2fae54358b26cea44b"},
// The account hash of the account receiving the tokens.
{"name":"to","type":"Key","value":"account-hash-303c0f8208220fe9a4de40e1ada1d35fdd6c678877908f01fddb2a56502d67fd"},
// The IDs of the CEP-85 tokens you are sending to the receiving account.
{"name":"ids","type":{"List":"U256"},"value":[3,5]},
// The amounts of the specified CEP-85 tokens you are sending to the receiving account.
{"name":"amounts","type":{"List":"U256"},"value":[10,25]},
// "DATA is an optional argument passed as a byte array that allows the inclusion of custom bytes. These are sent to `before_token_transfer` and can be used to pass values to a transfer filter contract."
{"name":"data","type":{"List":"U8"},"value":"0102ff"}
]' \
// The gas payment you are allotting, in motes.
--payment-amount "10000000000"

```

<details>
<summary><b>Casper client command without comments</b></summary>

```json

casper-client put-deploy -n http://<node IP>:<PORT> \
--chain-name <CHAIN NAME> \
--secret-key ~/casper/demo/user_a/secret_key.pem \
--session-hash hash-b568f50a64acc8bbe43462ffe243849a88111060b228dacb8f08d42e26985180 \
--session-entry-point "safe_batch_transfer_from" \
--session-args-json '[
{"name":"from","type":"Key","value":"account-hash-9f81014b9c7406c531ebf0477132283f4eb59143d7903a2fae54358b26cea44b"},
{"name":"to","type":"Key","value":"account-hash-303c0f8208220fe9a4de40e1ada1d35fdd6c678877908f01fddb2a56502d67fd"},
{"name":"ids","type":{"List":"U256"},"value":[3,5]},
{"name":"amounts","type":{"List":"U256"},"value":[10,25]},
{"name":"data","type":{"List":"U8"},"value":"0102ff"}
]' \
--payment-amount "10000000000"

```

</details>

## Checking the URI of a Token

The following command will invoke the `uri` entrypoint of your instance of CEP-85, returning the associated URI for the provided token ID.

```json

casper-client get-dictionary-item -n http://<node IP>:<PORT> \
// The state root hash
--state-root-hash 107a33f19093b8a17cea32fd53595507e8843a30cbb5e7160d9b276b4bec3538 \
// The contract hash of your CEP-85 contract instance.
--contract-hash hash-b568f50a64acc8bbe43462ffe243849a88111060b228dacb8f08d42e26985180 \
// The name of the dictionary you are invoking.
--dictionary-name "token_uri" \
// The ID of the CEP-85 token whose URI you are checking.
--dictionary-item-key "2"

```

<details>
<summary><b>Casper client command without comments</b></summary>

```json

casper-client get-dictionary-item -n http://<node IP>:<PORT> \
--state-root-hash 107a33f19093b8a17cea32fd53595507e8843a30cbb5e7160d9b276b4bec3538
--contract-hash hash-b568f50a64acc8bbe43462ffe243849a88111060b228dacb8f08d42e26985180 \
--dictionary-name "token_uri" \
--dictionary-item-key "2"

```

</details>

## Setting the URI of a Token

The following command will invoke the `set_uri` entrypoint of your instance of CEP-85, setting the URI of the provided token ID. The account sending this deploy must be on the `admin_list`.

```json

casper-client put-deploy -n http://<node IP>:<PORT> \
// The chain name of the Casper network on which your CEP-85 instance was installed.
--chain-name <CHAIN NAME> \
// The local path to your account's secret key.
--secret-key ~/casper/demo/user_a/secret_key.pem \
// The contract hash of your CEP-85 contract instance.
--session-hash hash-b568f50a64acc8bbe43462ffe243849a88111060b228dacb8f08d42e26985180 \
// The name of the entrypoint you are invoking.
--session-entry-point "set_uri" \
// The ID of the CEP-85 token whose URI you are setting.
--session-arg "id:u256='2'" \
// The new URI for the token.
--session-arg "uri:string='https://docs.casper.network/test-{id}.json'" \
// The gas payment you are allotting, in motes.
--payment-amount "500000000"

```

<details>
<summary><b>Casper client command without comments</b></summary>

```json

casper-client put-deploy -n http://<node IP>:<PORT> \
--chain-name <CHAIN NAME> \
--secret-key ~/casper/demo/user_a/secret_key.pem \
--session-hash hash-b568f50a64acc8bbe43462ffe243849a88111060b228dacb8f08d42e26985180 \
--session-entry-point "set_uri" \
--session-arg "id:u256='2'" \
--session-arg "uri:string='https://docs.casper.network/test-{id}.json'" \
--payment-amount "500000000"

```

</details>

<!-- TODO check these 2 sections re. supply and total_supply. What is the difference? -->

## Checking if a Token is Fungibile

Check if the [total_supply](#checking-the-total-supply-of-a-token) equals 1. Or, use the [JS client](../client-js/src/CEP85Client.ts) and calling `getIsNonFungible`.

## Checking a Token's Total Fungible Supply

Subtract the token [supply](#checking-the-supply-of-a-token) from the [total_supply](#checking-the-total-supply-of-a-token) and check if the result is positive. Or, use the [JS client](../client-js/src/CEP85Client.ts) and calling `getTotalFungibleSupply`.

## Changing Account Security Permissions

The `change_security` entrypoint can be used by an account with `admin` access to alter the security level of other accounts.

There are five security levels, with the strongest level taking precedence over other assigned levels. In order of highest strength to lowest:

1. `None` - `None` overrides other security levels and removes all admin, minting, and burning access to an account.

2. `Admin` - Allows the account full access and control over the CEP-85 contract.

3. `Minter` - The account can mint new tokens.

4. `Burner` - The account can burn tokens.

Here is an example of a `session-arg` that provides a list of account hashes to be included on the `minter_list`:

```bash
--session-args-json '[{"name":"minter_list","type":{"List":"Key"},"value":["account-hash-303c0f8208220fe9a4de40e1ada1d35fdd6c678877908f01fddb2a56502d67fd"]}]'
```

> **Be aware that removing all admin accounts will lock out all admin functionality.**

The following command can be supplied with any of the optional arguments above:

```json
casper-client put-deploy -n http://<NODE IP>:<PORT> \
--chain-name <CHAIN NAME> \
--secret-key ~/casper/demo/user_a/secret_key.pem \
--session-hash hash-b568f50a64acc8bbe43462ffe243849a88111060b228dacb8f08d42e26985180 \
--session-entry-point "change_security" \
/// The following arguments are all optional and each consists of a string of the account hashes to be added to the list specified, separated by commas.
--session-args-json '[
{"name":"minter_list","type":{"List":"Key"},"value":["account-hash-303c0f8208220fe9a4de40e1ada1d35fdd6c678877908f01fddb2a56502d67fd"]},
{"name":"burner_list","type":{"List":"Key"},"value":["account-hash-303c0f8208220fe9a4de40e1ada1d35fdd6c678877908f01fddb2a56502d67fd"]},
{"name":"admin_list","type":{"List":"Key"},"value":["account-hash-303c0f8208220fe9a4de40e1ada1d35fdd6c678877908f01fddb2a56502d67fd"]},
{"name":"none_list","type":{"List":"Key"},"value":["account-hash-303c0f8208220fe9a4de40e1ada1d35fdd6c678877908f01fddb2a56502d67fd"]}
]' \
--payment-amount 500000000
```

## Setting Modalities

The following command will invoke the `set_approval_for_all` entrypoint on your instance of CEP-85, directing it to set a modality of the contract instance. The account sending this deploy must be on the `admin_list`.

```json

casper-client put-deploy -n http://<node IP>:<PORT> \
// The chain name of the Casper network on which your CEP-85 instance was installed.
--chain-name <CHAIN NAME> \
// The local path to your account's secret key.
--secret-key ~/casper/demo/user_a/secret_key.pem \
// The contract hash of your CEP-85 contract instance.
--session-hash hash-b568f50a64acc8bbe43462ffe243849a88111060b228dacb8f08d42e26985180 \
// The name of the entrypoint you are invoking.
--session-entry-point "set_modalities" \
// The events mode for this contract. In this instance the 1 represents the `CES` version of `EventsMode`.
--session-arg "events_mode:u8='1'" \"operator:key='account-hash-9f81014b9c7406c531ebf0477132283f4eb59143d7903a2fae54358b26cea44b'" \
// A boolean representing enabling (true) or disabling (false) the burn mode.
--session-arg "enable_burn:bool='true'"
// The gas payment you are allotting, in motes.
--payment-amount "500000000"

```

<details>
<summary><b>Casper client command without comments</b></summary>

```json

casper-client put-deploy -n http://<node IP>:<PORT> \
--chain-name <CHAIN NAME> \
--secret-key ~/casper/demo/user_a/secret_key.pem \
--session-hash hash-b568f50a64acc8bbe43462ffe243849a88111060b228dacb8f08d42e26985180 \
--session-entry-point "set_modalities" \
--session-arg "events_mode:u8='1'" \
--session-arg "enable_burn:bool='true'" \
--payment-amount "1300000000"

```

</details>

## Upgrading the Contract

The following command will invoke the `call` entrypoint on your instance of CEP-85, directing it to upgrade the instance to a new version.

```json

casper-client put-deploy -n http://<node IP>:<PORT> \
// The chain name of the Casper network on which your CEP-85 instance was installed.
--chain-name <CHAIN NAME> \
// The local path to the cep85.wasm.
--session-path ~/casper/cep-85/target/wasm32-unknown-unknown/release/cep85.wasm \
// The local path to your account's secret key.
--secret-key ~/casper/demo/user_a/secret_key.pem \
// A boolean representing enabling (true) or disabling (false) the upgrade.
--session-arg "upgrade:bool='true'" \
// The name of the token as a string. In this instance, "multi-token-1".
--session-arg "name:string='multi-token-1'"  \
// The gas payment you are allotting, in motes.
--payment-amount "175000000000"

```

<details>
<summary><b>Casper client command without comments</b></summary>

```json

casper-client put-deploy -n http://<node IP>:<PORT> \
--chain-name <CHAIN NAME> \
--session-path ~/casper/cep-85/target/wasm32-unknown-unknown/release/cep85.wasm \
--secret-key ~/casper/demo/user_a/secret_key.pem \
--session-arg "upgrade:bool='true'" \
--session-arg "name:string='multi-token-1'" \
--payment-amount "175000000000"

```

</details>
