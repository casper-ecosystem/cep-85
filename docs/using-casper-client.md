# Installing and Interacting with the Contract using the Rust Casper Client

This documentation will guide you through installing and interacting with an instance of the CEP-85 multi-token standard contract through Casper's Rust CLI client. The contract code installs an instance of CEP-85 as per session arguments provided at the time of installation. It requires a minimum Rust version of `1.63.0`.

## Installing the Contract

Installing the multi-token contract to global state requires the use of a [Deploy](https://docs.casper.network/developers/dapps/sending-deploys/). In this case, the session code can be compiled to Wasm by running the `make test` command provided in the Makefile at the top level. The Wasm will be found in the `target/wasm32-unknown-unknown/release` directory as `cep85.wasm`.

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

5. [Checking the Balance of a Single Token ID](#checking-the-balance-of-a-single-token-id)

6. [Checking the Balance of Multiple Token IDs](#checking-the-balance-of-multiple-token-ids)

7. [Approving An Operator to Transfer Tokens](#approving-an-operator-to-transfer-tokens)

8. [Checking Approval Status of an Account](#checking-approval-status-of-an-account)

9. [Transferring a Token](#transferring-a-token)

10. [Transferring a Batch of Tokens](#transferring-a-batch-of-tokens)

11. [Checking the Supply of a Token](#checking-the-supply-of-a-token)

12. [Checking the Total Supply of a Token](#checking-the-total-supply-of-a-token)

13. [Checking the URI for a Token](#checking-the-uri-for-a-token)

14. [Setting the URI of a Token](#setting-the-uri-of-a-token)

15. [Checking a Token's Fungibility](#checking-a-tokens-fungibility)

16. [Checking a Token's Total Fungible Supply](#checking-a-tokens-total-fungible-supply)

17. [Changing Account Security Permissions](#changing-account-security-permissions)

18. [Setting Modalities](#setting-modalities)

19. [Upgrading Collection Contract](#upgrading-collection-contract)

## Minting a Token

The following command will invoke the `mint` entrypoint on your instance of CEP-85, directing it to mint the given amount of a specified token ID to the recipient address. The account sending this deploy must be on the `minter_list`.

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
// An optional uri for the token
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
--session-arg "uri:string='https://test-cdn-domain/{id}.json'" \ // Optional, defaults to global uri
--payment-amount "1000000000"

```

</details>

## Batch Minting Tokens

The following command will invoke the `batch_mint` entrypoint on your instance of CEP-85, directing it to mint the given amount of several token IDs to the recipient address. The account sending this deploy must be on the `minter_list`.

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
// An optional uri for the tokens
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

The following command will invoke the `burn` entrypoint on your instance of CEP-85, directing it to burn the given amount of tokens at the owner address. The account sending this deploy must be on the `burner_list`.

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
// The account hash of the account that you are burning CEP-85 tokens from.
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

The following command will invoke the `batch_burn` entrypoint on your instance of CEP-85, directing it to burn the given amount of several token IDs at the owner address. The account sending this deploy must be on the `burner_list`.

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
// The account hash of the account that you are burning CEP-85 tokens from.
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

## Checking the Balance of a Single Token ID

The following command will invoke the `balance_of` entrypoint on your instance of CEP-85.

```json

casper-client put-deploy -n http://<node IP>:<PORT> \
// The chain name of the Casper network on which your CEP-85 instance was installed.
--chain-name <CHAIN NAME> \
// The local path to your account's secret key.
--secret-key ~/casper/demo/user_a/secret_key.pem \
// The contract hash of your CEP-85 contract instance.
--session-hash hash-b568f50a64acc8bbe43462ffe243849a88111060b228dacb8f08d42e26985180 \
// The name of the entrypoint you are invoking.
--session-entry-point "balance_of" \
// The account hash of the account that you are querying the balance of.
--session-arg "account:key='account-hash-9f81014b9c7406c531ebf0477132283f4eb59143d7903a2fae54358b26cea44b'" \
// The ID of the CEP-85 token you are querying the balance of.
--session-arg "id:U256='2'" \
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
--session-entry-point "balance_of" \
--session-arg "account:key='account-hash-9f81014b9c7406c531ebf0477132283f4eb59143d7903a2fae54358b26cea44b'" \
--session-arg "id:U256='2'" \
--payment-amount "10000000000"

```

</details>

After sending this command, you will need to query the `balance` URef within the `NamedKeys` of your CEP-85 contract instance.

You can use the following command to query global state for the `balance` URef.

```json
casper-client query-global-state -n http://<NODE IP>:<PORT> \
// This is the `balance` URef location from your CEP-85 contract's `NamedKeys`.
--key uref-a46ad389b53715d9991a513c8ca48e1502facc4c563c0700a31e830c4cb8a7d4-007 \
// This is the current state-root-hash for the Casper network your contract is installed on.
--state-root-hash 3aecd0e4b6ec29ee7c1eed701132eabfe6e66a1e0f1595c9c65bfed447e474f7
```

<details>
<summary><b>Casper client command without comments</b></summary>

```bash
casper-client query-global-state -n http://<NODE IP>:<PORT> \
--key uref-a46ad389b53715d9991a513c8ca48e1502facc4c563c0700a31e830c4cb8a7d4-007 \
--state-root-hash 3aecd0e4b6ec29ee7c1eed701132eabfe6e66a1e0f1595c9c65bfed447e474f7
```

</details>

## Checking the Balance of Multiple Token IDs

The following command will invoke the `balance_of_batch` entrypoint on your instance of CEP-85.

```json

casper-client put-deploy -n http://<node IP>:<PORT> \
// The chain name of the Casper network on which your CEP-85 instance was installed.
--chain-name <CHAIN NAME> \
// The local path to your account's secret key.
--secret-key ~/casper/demo/user_a/secret_key.pem \
// The contract hash of your CEP-85 contract instance.
--session-hash hash-b568f50a64acc8bbe43462ffe243849a88111060b228dacb8f08d42e26985180 \
// The name of the entrypoint you are invoking.
--session-entry-point "balance_of_batch" \
// The account hash of the account that you are querying the balances of.
--session-arg "account:key='account-hash-9f81014b9c7406c531ebf0477132283f4eb59143d7903a2fae54358b26cea44b'" \
// The ID of the CEP-85 tokens you are querying the balance of.
--session-arg "ids:string='2, 5'" \
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
--session-entry-point "balance_of_batch" \
--session-arg "account:key='account-hash-9f81014b9c7406c531ebf0477132283f4eb59143d7903a2fae54358b26cea44b'" \
--session-arg "ids:string='2, 5'" \
--payment-amount "10000000000"

```

</details>

After sending this command, you will need to query the `batch_balances` URef within the `NamedKeys` of your CEP-85 contract instance.

You can use the following command to query global state for the `batch_balances` URef.

```json
casper-client query-global-state -n http://<NODE IP>:<PORT> \
// This is the `batch_balances` URef location from your CEP-85 contract's `NamedKeys`.
--key uref-a46ad389b53715d9991a513c8ca48e1502facc4c563c0700a31e830c4cb8a7d4-007 \
// This is the current state-root-hash for the Casper network your contract is installed on.
--state-root-hash 3aecd0e4b6ec29ee7c1eed701132eabfe6e66a1e0f1595c9c65bfed447e474f7
```

<details>
<summary><b>Casper client command without comments</b></summary>

```bash
casper-client query-global-state -n http://<NODE IP>:<PORT> \
--key uref-a46ad389b53715d9991a513c8ca48e1502facc4c563c0700a31e830c4cb8a7d4-007 \
--state-root-hash 3aecd0e4b6ec29ee7c1eed701132eabfe6e66a1e0f1595c9c65bfed447e474f7
```

</details>

## Approving An Operator to Transfer Tokens

The following command will invoke the `set_approval_for_all` entrypoint on your instance of CEP-85, directing it to approve or remove a given `operator`s ability to transfer the calling account's tokens.

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

## Checking Approval Status of an Account

The following command will invoke the `is_approved_for_all` entrypoint of your instance of CEP-85, verifying if the provided `operator` key is approved to transfer tokens from the balance of the given `account`.

```json

casper-client put-deploy -n http://<node IP>:<PORT> \
// The chain name of the Casper network on which your CEP-85 instance was installed.
--chain-name <CHAIN NAME> \
// The local path to your account's secret key.
--secret-key ~/casper/demo/user_a/secret_key.pem \
// The contract hash of your CEP-85 contract instance.
--session-hash hash-b568f50a64acc8bbe43462ffe243849a88111060b228dacb8f08d42e26985180 \
// The name of the entrypoint you are invoking.
--session-entry-point "is_approved_for_all" \
// The account hash of the account that you are checking the operator approval status of.
--session-arg "operator:key='account-hash-9f81014b9c7406c531ebf0477132283f4eb59143d7903a2fae54358b26cea44b'" \
// The account hash of the account that holds the tokens that the operator may be approved to transfer.
--session-arg "account:key='account-hash-303c0f8208220fe9a4de40e1ada1d35fdd6c678877908f01fddb2a56502d67fd'" \
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
--session-entry-point "is_approved_for_all" \
--session-arg "operator:key='account-hash-9f81014b9c7406c531ebf0477132283f4eb59143d7903a2fae54358b26cea44b'" \
--session-arg "account:key='account-hash-303c0f8208220fe9a4de40e1ada1d35fdd6c678877908f01fddb2a56502d67fd'" \
--payment-amount "10000000000"

```

</details>

After sending this command, you will need to query the `is_approved_for_all` URef within the `NamedKeys` of your CEP-85 contract instance.

You can use the following command to query global state for the `is_approved_for_all` URef.

```json
casper-client query-global-state -n http://<NODE IP>:<PORT> \
// This is the `is_approved_for_all` URef location from your CEP-85 contract's `NamedKeys`.
--key uref-a46ad389b53715d9991a513c8ca48e1502facc4c563c0700a31e830c4cb8a7d4-007 \
// This is the current state-root-hash for the Casper network your contract is installed on.
--state-root-hash 3aecd0e4b6ec29ee7c1eed701132eabfe6e66a1e0f1595c9c65bfed447e474f7
```

<details>
<summary><b>Casper client command without comments</b></summary>

```bash
casper-client query-global-state -n http://<NODE IP>:<PORT> \
--key uref-a46ad389b53715d9991a513c8ca48e1502facc4c563c0700a31e830c4cb8a7d4-007 \
--state-root-hash 3aecd0e4b6ec29ee7c1eed701132eabfe6e66a1e0f1595c9c65bfed447e474f7
```

</details>

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
// "DATA is an optional argument passed as a bytes array that allows the inclusion of custom bytes. These are sent to `before_token_transfer` and can be used to pass values to a transfer filter contract."
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
// "DATA is an optional argument passed as a string that allows the inclusion of custom bytes. These are sent to `before_token_transfer` and can be used to pass values to a transfer filter contract."
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

## Checking the Supply of a Token

The following command will query the `supply` dictionary of your instance of CEP-85, verifying the supply of the provided token ID.

```json

casper-client get-dictionary-item -n http://<node IP>:<PORT> \
// The state root hash
--state-root-hash 107a33f19093b8a17cea32fd53595507e8843a30cbb5e7160d9b276b4bec3538 \
// The contract hash of your CEP-85 contract instance.
--contract-hash hash-b568f50a64acc8bbe43462ffe243849a88111060b228dacb8f08d42e26985180 \
// The name of the entrypoint you are invoking.
--dictionary-name "supply" \
// The ID of the CEP-85 token you are checking the supply of.
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

The following command will invoke the `total_supply_of` entrypoint of your instance of CEP-85, verifying the total supply of the provided token ID.

```json

casper-client get-dictionary-item -n http://<node IP>:<PORT> \
// The state root hash
--state-root-hash 107a33f19093b8a17cea32fd53595507e8843a30cbb5e7160d9b276b4bec3538 \
// The contract hash of your CEP-85 contract instance.
--contract-hash hash-b568f50a64acc8bbe43462ffe243849a88111060b228dacb8f08d42e26985180 \
// The name of the entrypoint you are invoking.
--dictionary-name "total_supply" \
// The ID of the CEP-85 token you are checking the supply of.
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

The following command will invoke the `set_total_supply_of` entrypoint of your instance of CEP-85, setting the total supply of the provided token ID. The new total supply provided must be larger than the previous total supply.

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

The following command will invoke the `set_total_supply_of_batch` entrypoint of your instance of CEP-85, setting the total supplies of the provided token IDs. The new total supplies provided must be larger than the previous total supplies.

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

## Checking the URI for a Token

The following command will invoke the `uri` entrypoint of your instance of CEP-85, returning the associated URI for the provided token ID.

```json

casper-client get-dictionary-item -n http://<node IP>:<PORT> \
// The contract hash of your CEP-85 contract instance.
--contract-hash hash-b568f50a64acc8bbe43462ffe243849a88111060b228dacb8f08d42e26985180 \
// The name of the entrypoint you are invoking.
--dictionary-name "token_uri" \
// The ID of the CEP-85 token you are checking the URI of.
--dictionary-item-key "2"

```

<details>
<summary><b>Casper client command without comments</b></summary>

```json

casper-client get-dictionary-item -n http://<node IP>:<PORT> \
--contract-hash hash-b568f50a64acc8bbe43462ffe243849a88111060b228dacb8f08d42e26985180 \
--dictionary-name "token_uri" \
--dictionary-item-key "2"

```

</details>

## Setting the URI of a Token

The following command will invoke the `set_uri` entrypoint of your instance of CEP-85, setting the URI of the provided token ID.

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
// The ID of the CEP-85 token you are setting the URI of.
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

## Checking a Token's Fungibility

Check if total_supply is equal to 1 with [Checking the Total Supply of a Token](#checking-the-total-supply-of-a-token)

## Checking a Token's Total Fungible Supply

Check if total_supply susctracted of supply is positive with [Checking the Total Supply of a Token](#checking-the-total-supply-of-a-token) and [Checking the Supply of a Token](#checking-the-supply-of-a-token)

## Changing Account Security Permissions

The `change_security` entrypoint can be used by an account with `admin` access to alter the security level of other accounts.

There are five security levels, with the strongest level taking precedence over any other assigned levels. In order of highest strength to lowest:

1. `None` - `None` overrides other security levels and removes all admin, minting and burning access of an account.

2. `Admin` - Allows the account full access and control over the CEP-85 contract.

3. `Minter` - The account can mint new tokens.

4. `Burner` - The account can burn tokens.

Here is an example of a `session-arg` that provides a list of account hashes to be included on the `minter_list`:

```bash
--session-args-json '[{"name":"minter_list","type":{"List":"Key"},"value":["account-hash-303c0f8208220fe9a4de40e1ada1d35fdd6c678877908f01fddb2a56502d67fd"]}]'
```

**Be aware that removing all admin accounts will lock out all admin functionality.**

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

## Setting modalities

The following command will invoke the `set_approval_for_all` entrypoint on your instance of CEP-85, directing it to set a modality of the contract instance.

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

## Upgrading Collection Contract

The following command will invoke the `call` entrypoint on your instance of CEP-85, directing it to upgrade the instance to a new version.

```json

casper-client put-deploy -n http://<node IP>:<PORT> \
// The chain name of the Casper network on which your CEP-85 instance was installed.
--chain-name <CHAIN NAME> \
// The local path to cep85.wasm
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
