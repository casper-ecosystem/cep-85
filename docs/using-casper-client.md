# Installing and Interacting with the Contract using the Rust Casper Client

This documentation will guide you through installing and interacting with an instance of the CEP-85 multi-token standard contract through Casper's Rust CLI client. The contract code installs an instance of CEP-85 as per session arguments provided at the time of installation. It requires a minimum Rust version of `1.63.0`.

## Installing the Contract

Installing the multi-token contract to global state requires the use of a [Deploy](https://docs.casper.network/developers/dapps/sending-deploys/). In this case, the session code can be compiled to Wasm by running the `make test` command provided in the Makefile at the top level. The Wasm will be found in the `target/wasm32-unknown-unknown/release` directory as `cep-85.wasm`.

Below is an example of a `casper-client` command that provides all required session arguments to install a valid instance of the CEP-85 contract on global state.

- `casper-client put-deploy -n http://localhost:11101/rpc --chain-name "casper-net-1" --payment-amount 500000000000 -k ~/casper/casper-node/utils/nctl/assets/net-1/nodes/node-1/keys/secret_key.pem --session-path ~/casper/cep-85/target/wasm32-unknown-unknown/release/cep-85.wasm`

1. `--session-arg "name:string='multi-token-1'"`

   The name of the token as a string. In this instance, "multi-token-1".

2. `--session-arg "uri:string='https://docs.casper.network/'"`

   A string URI for an off-chain resource associated with the token. In this instance, "https://docs.casper.network/".

3. `--session-arg "events_mode:u8='0'"`

   The events mode for this contract. In this instance the 0 represents the `NoEvents` version of `EventsMode`.

4. `--session-arg "enable_burn:u8='1'"`

   The burn mode for this contract. In this instance, the 1 represents that burning is `Enabled` for those with the proper access.

<details>
<summary><b>Casper client command without comments</b></summary>

```bash
`casper-client put-deploy -n http://localhost:11101/rpc --chain-name "casper-net-1" --payment-amount 500000000000 -k ~/casper/casper-node/utils/nctl/assets/net-1/nodes/node-1/keys/secret_key.pem --session-path ~/casper/cep-85/target/wasm32-unknown-unknown/release/cep-85.wasm` \
--session-arg "name:string='multi-token-1'" \
--session-arg "uri:string='https://docs.casper.network/'" \
--session-arg "events_mode:u8='0'" \
--session-arg "enable_burn:u8='1'" \
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

12. [Checking the Supply of a Batch of Tokens](#checking-the-supply-of-a-batch-of-tokens)

13. [Checking the Total Supply of a Token](#checking-the-total-supply-of-a-token)

14. [Checking the Total Supply of a Batch of Tokens](#checking-the-total-supply-of-a-batch-of-tokens)

15. [Checking the URI for a Token](#checking-the-uri-for-a-token)

16. [Setting the URI of a Token](#setting-the-uri-of-a-token)

17. [Checking a Token's Fungibility](#checking-a-tokens-fungibility)

18. [Checking a Token's Total Fungible Supply](#checking-a-tokens-total-fungible-supply)

19. [Changing Account Security Permissions](#changing-account-security-permissions)

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
--session-entry-point "mint" \
--session-arg "recipient:key='account-hash-9f81014b9c7406c531ebf0477132283f4eb59143d7903a2fae54358b26cea44b'" \
--session-arg "id:u256='2'" \
--session-arg "amount:u256='10'" \
--payment-amount "10000000000"

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
// The account hash of the account to which you are minting CEP-85 tokens.
--session-arg "recipient:key='account-hash-9f81014b9c7406c531ebf0477132283f4eb59143d7903a2fae54358b26cea44b'" \
// The IDs of the CEP-85 tokens you are sending to the receiving account.
--session-arg "ids:string='2, 5'" \
// The amounts of the specified CEP-85 tokens you are sending to the receiving account.
--session-arg "amounts:string='10, 25'" \
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
--session-entry-point "mint" \
--session-arg "recipient:key='account-hash-9f81014b9c7406c531ebf0477132283f4eb59143d7903a2fae54358b26cea44b'" \
--session-arg "ids:string='2, 5'" \
--session-arg "amounts:string='10, 25'" \
--payment-amount "10000000000"

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
// The account hash of the account that you are burning CEP-85 tokens from.
--session-arg "owner:key='account-hash-9f81014b9c7406c531ebf0477132283f4eb59143d7903a2fae54358b26cea44b'" \
// The IDs of the CEP-85 tokens you are removing from the owning account.
--session-arg "ids:string='2, 5'" \
// The amounts of the specified CEP-85 tokens you are removing from the owning account.
--session-arg "amounts:string='10, 25'" \
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
--session-entry-point "batch_burn" \
--session-arg "owner:key='account-hash-9f81014b9c7406c531ebf0477132283f4eb59143d7903a2fae54358b26cea44b'" \
--session-arg "ids:string='2, 5'" \
--session-arg "amounts:string='10, 25'" \
--payment-amount "10000000000"

```

</details>

## Checking the Balance of a Single Token ID

The following command will invoke the `balance_of` entrypoint on your instance of CEP-85. This will store the balance of the specified token ID for the given owning account as a URef `balance` in the `NamedKeys` of the CEP-85 contract instance.

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

The following command will invoke the `balance_of_batch` entrypoint on your instance of CEP-85. This will store the balance of the specified token IDs for the given owning account as a URef `batch_balances` in the `NamedKeys` of the CEP-85 contract instance.

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
--payment-amount "10000000000"

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
--payment-amount "10000000000"

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
--session-arg "from:key='account-hash-303c0f8208220fe9a4de40e1ada1d35fdd6c678877908f01fddb2a56502d67fd'" \
// The ID of the CEP-85 token you are sending to the receiving account.
--session-arg "id:u256='2'" \
// The amount of the specified CEP-85 token you are sending to the receiving account.
--session-arg "amount:u256='10'" \
// "DATA is an optional argument passed as a string that allows the inclusion of custom bytes. These are sent to `before_token_transfer` and can be used to pass values to a transfer filter contract."
--session-arg "data:string=''" \
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
--session-arg "from:key='account-hash-303c0f8208220fe9a4de40e1ada1d35fdd6c678877908f01fddb2a56502d67fd'" \
--session-arg "id:u256='2'" \
--session-arg "amount:u256='10'" \
--session-arg "data:string=''" \
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
// The account hash of the account sending the tokens.
--session-arg "from:key='account-hash-9f81014b9c7406c531ebf0477132283f4eb59143d7903a2fae54358b26cea44b'" \
// The account hash of the account receiving the tokens.
--session-arg "from:key='account-hash-303c0f8208220fe9a4de40e1ada1d35fdd6c678877908f01fddb2a56502d67fd'" \
// The IDs of the CEP-85 tokens you are sending to the receiving account.
--session-arg "ids:string='2, 5'" \
// The amounts of the specified CEP-85 tokens you are sending to the receiving account.
--session-arg "amounts:string='10, 25'" \
// "DATA is an optional argument passed as a string that allows the inclusion of custom bytes. These are sent to `before_token_transfer` and can be used to pass values to a transfer filter contract."
--session-arg "data:string=''" \
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
--session-arg "from:key='account-hash-9f81014b9c7406c531ebf0477132283f4eb59143d7903a2fae54358b26cea44b'" \
--session-arg "from:key='account-hash-303c0f8208220fe9a4de40e1ada1d35fdd6c678877908f01fddb2a56502d67fd'" \
--session-arg "ids:string='2, 5'" \
--session-arg "amounts:string='10, 25'" \
--session-arg "data:string=''" \
--payment-amount "10000000000"

```

</details>

## Checking the Supply of a Token

The following command will invoke the `supply_of` entrypoint of your instance of CEP-85, verifying the supply of the provided token ID.

```json

casper-client put-deploy -n http://<node IP>:<PORT> \
// The chain name of the Casper network on which your CEP-85 instance was installed.
--chain-name <CHAIN NAME> \
// The local path to your account's secret key.
--secret-key ~/casper/demo/user_a/secret_key.pem \
// The contract hash of your CEP-85 contract instance.
--session-hash hash-b568f50a64acc8bbe43462ffe243849a88111060b228dacb8f08d42e26985180 \
// The name of the entrypoint you are invoking.
--session-entry-point "supply_of" \
// The ID of the CEP-85 token you are checking the supply of.
--session-arg "id:u256='2'" \
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
--session-entry-point "supply_of" \
--session-arg "id:u256='2'" \
--payment-amount "10000000000"

```

</details>

After sending this command, you will need to query the `supply` URef within the `NamedKeys` of your CEP-85 contract instance.

You can use the following command to query global state for the `supply` URef.

```json
casper-client query-global-state -n http://<NODE IP>:<PORT> \
// This is the `supply` URef location from your CEP-85 contract's `NamedKeys`.
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

## Checking the Supply of a Batch of Tokens

The following command will invoke the `supply_of_batch` entrypoint of your instance of CEP-85, verifying the supply of the provided token IDs.

```json

casper-client put-deploy -n http://<node IP>:<PORT> \
// The chain name of the Casper network on which your CEP-85 instance was installed.
--chain-name <CHAIN NAME> \
// The local path to your account's secret key.
--secret-key ~/casper/demo/user_a/secret_key.pem \
// The contract hash of your CEP-85 contract instance.
--session-hash hash-b568f50a64acc8bbe43462ffe243849a88111060b228dacb8f08d42e26985180 \
// The name of the entrypoint you are invoking.
--session-entry-point "supply_of" \
// The IDs of the CEP-85 tokens you are checking the supply of.
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
--session-entry-point "supply_of" \
--session-arg "ids:string='2, 5'" \
--payment-amount "10000000000"

```

</details>

After sending this command, you will need to query the `batch_supplies` URef within the `NamedKeys` of your CEP-85 contract instance.

You can use the following command to query global state for the `batch_supplies` URef.

```json
casper-client query-global-state -n http://<NODE IP>:<PORT> \
// This is the `batch_supplies` URef location from your CEP-85 contract's `NamedKeys`.
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

## Checking the Total Supply of a Token

The following command will invoke the `total_supply_of` entrypoint of your instance of CEP-85, verifying the total supply of the provided token ID.

```json

casper-client put-deploy -n http://<node IP>:<PORT> \
// The chain name of the Casper network on which your CEP-85 instance was installed.
--chain-name <CHAIN NAME> \
// The local path to your account's secret key.
--secret-key ~/casper/demo/user_a/secret_key.pem \
// The contract hash of your CEP-85 contract instance.
--session-hash hash-b568f50a64acc8bbe43462ffe243849a88111060b228dacb8f08d42e26985180 \
// The name of the entrypoint you are invoking.
--session-entry-point "total_supply_of" \
// The ID of the CEP-85 token you are checking the total supply of.
--session-arg "id:u256='2'" \
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
--session-entry-point "total_supply_of" \
--session-arg "id:u256='2'" \
--payment-amount "10000000000"

```

</details>

After sending this command, you will need to query the `total_supply` URef within the `NamedKeys` of your CEP-85 contract instance.

You can use the following command to query global state for the `total_supply` URef.

```json
casper-client query-global-state -n http://<NODE IP>:<PORT> \
// This is the `total_supply` URef location from your CEP-85 contract's `NamedKeys`
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

## Checking the Total Supply of a Batch of Tokens

The following command will invoke the `total_supply_of_batch` entrypoint of your instance of CEP-85, verifying the total supply of the provided token IDs.

```json

casper-client put-deploy -n http://<node IP>:<PORT> \
// The chain name of the Casper network on which your CEP-85 instance was installed.
--chain-name <CHAIN NAME> \
// The local path to your account's secret key.
--secret-key ~/casper/demo/user_a/secret_key.pem \
// The contract hash of your CEP-85 contract instance.
--session-hash hash-b568f50a64acc8bbe43462ffe243849a88111060b228dacb8f08d42e26985180 \
// The name of the entrypoint you are invoking.
--session-entry-point "total_supply_of_batch" \
// The IDs of the CEP-85 tokens you are checking the total supply of.
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
--session-entry-point "total_supply_of_batch" \
--session-arg "ids:string='2, 5'" \
--payment-amount "10000000000"

```

</details>

After sending this command, you will need to query the `batch_total_supplies` URef within the `NamedKeys` of your CEP-85 contract instance.

You can use the following command to query global state for the `batch_total_supplies` URef.

```json
casper-client query-global-state -n http://<NODE IP>:<PORT> \
// This is the `batch_total_supplies` URef location from your CEP-85 contract's `NamedKeys`.
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
--payment-amount "10000000000"

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
--payment-amount "10000000000"

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
// The IDs of the CEP-85 tokens you are setting the total supply of.
--session-arg "ids:string='2, 5'" \
// The new total supply numbers.
--session-arg "total_supplies:string='200, 350'" \
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
--session-entry-point "set_total_supply_of_batch" \
--session-arg "ids:string='2, 5'" \
--session-arg "total_supplies:string='200, 350'" \
--payment-amount "10000000000"

```

</details>

## Checking the URI for a Token

The following command will invoke the `uri` entrypoint of your instance of CEP-85, returning the associated URI for the provided token ID.

```json

casper-client put-deploy -n http://<node IP>:<PORT> \
// The chain name of the Casper network on which your CEP-85 instance was installed.
--chain-name <CHAIN NAME> \
// The local path to your account's secret key.
--secret-key ~/casper/demo/user_a/secret_key.pem \
// The contract hash of your CEP-85 contract instance.
--session-hash hash-b568f50a64acc8bbe43462ffe243849a88111060b228dacb8f08d42e26985180 \
// The name of the entrypoint you are invoking.
--session-entry-point "uri" \
// The ID of the CEP-85 token you are checking the URI of.
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
--session-entry-point "uri" \
--session-arg "id:U256='2'" \
--payment-amount "10000000000"

```

</details>

After sending this command, you will need to query the `uri` URef within the `NamedKeys` of your CEP-85 contract instance.

You can use the following command to query global state for the `uri` URef.

```json
casper-client query-global-state -n http://<NODE IP>:<PORT> \
// This is the `uri` URef location from your CEP-85 contract's `NamedKeys`
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
--session-arg "uri:string='http://docs.casper.network/'" \
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
--session-entry-point "set_uri" \
--session-arg "id:u256='2'" \
--session-arg "uri:string='http://docs.casper.network/'" \
--payment-amount "10000000000"

```

</details>

## Checking a Token's Fungibility

The following command will invoke the `is_non_fungible` entrypoint of your instance of CEP-85, returning a boolean value.

```json

casper-client put-deploy -n http://<node IP>:<PORT> \
// The chain name of the Casper network on which your CEP-85 instance was installed.
--chain-name <CHAIN NAME> \
// The local path to your account's secret key.
--secret-key ~/casper/demo/user_a/secret_key.pem \
// The contract hash of your CEP-85 contract instance.
--session-hash hash-b568f50a64acc8bbe43462ffe243849a88111060b228dacb8f08d42e26985180 \
// The name of the entrypoint you are invoking.
--session-entry-point "is_non_fungible" \
// The ID of the CEP-85 token you are checking the fungibility of.
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
--session-entry-point "is_non_fungible" \
--session-arg "id:U256='2'" \
--payment-amount "10000000000"

```

</details>

After sending this command, you will need to query the `is_non_fungible` URef within the `NamedKeys` of your CEP-85 contract instance.

You can use the following command to query global state for the `is_non_fungible` URef.

```json
casper-client query-global-state -n http://<NODE IP>:<PORT> \
// This is the `is_non_fungible` URef location from your CEP-85 contract's `NamedKeys`.
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

## Checking a Token's Total Fungible Supply

The following command will invoke the `total_fungible_supply` entrypoint of your instance of CEP-85, returning the total fungible supply of tokens for that ID.

```json

casper-client put-deploy -n http://<node IP>:<PORT> \
// The chain name of the Casper network on which your CEP-85 instance was installed.
--chain-name <CHAIN NAME> \
// The local path to your account's secret key.
--secret-key ~/casper/demo/user_a/secret_key.pem \
// The contract hash of your CEP-85 contract instance.
--session-hash hash-b568f50a64acc8bbe43462ffe243849a88111060b228dacb8f08d42e26985180 \
// The name of the entrypoint you are invoking.
--session-entry-point "total_fungible_supply" \
// The ID of the CEP-85 token you are checking the total fungible supply of.
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
--session-entry-point "total_fungible_supply" \
--session-arg "id:U256='2'" \
--payment-amount "10000000000"

```

</details>

After sending this command, you will need to query the `total_fungible_supply` URef within the `NamedKeys` of your CEP-85 contract instance.

You can use the following command to query global state for the `total_fungible_supply` URef.

```json
casper-client query-global-state -n http://<NODE IP>:<PORT> \
// This is the `total_fungible_supply` URef location from your CEP-85 contract's `NamedKeys`.
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

## Changing Account Security Permissions

The `change_security` entrypoint can be used by an account with `admin` access to alter the security level of other accounts.

There are five security levels, with the strongest level taking precedence over any other assigned levels. In order of highest strength to lowest:

1. `None` - `None` overrides other security levels and removes all admin, minting and burning access of an account.

2. `Admin` - Allows the account full access and control over the CEP-85 contract.

3. `Minter` - The account can mint new tokens.

4. `Burner` - The account can burn tokens.

5. `Meta` - The account can set URIs for a token.

Here is an example of a `session-arg` that provides a list of account hashes to be included on the `minter_list`:

```bash
--session-arg "minter_list:string='account-hash-1ed5a1c39bea93c105f2d22c965a84b205b36734a377d05dbb103b6bfaa595a7,account-hash-0ea7998b2822afe5b62b08a21d54c941ad791279b089f3f7ede0d72b477eca34,account-hash-e70dbca48c2d31bc2d754e51860ceaa8a1a49dc627b20320b0ecee1b6d9ce655'"
```

**Be aware that removing all admin accounts will lock out all admin functionality.**

The following command can be supplied with any of the optional arguments above:

```json
casper-client put-deploy -n http://<NODE IP>:<PORT> \
--secret-key ~/casper/demo/user_a/secret_key.pem \
--session-hash hash-b568f50a64acc8bbe43462ffe243849a88111060b228dacb8f08d42e26985180 \
--session-entry-point "change_security" \
/// The following arguments are all optional and each consists of a string of the account hashes to be added to the list specified, separated by commas.
--session-arg "none_list:string:'<List of account hashes>'" \
--session-arg "admin_list:string:'<List of account hashes>'" \
--session-arg "minter_list:string:'<List of account hashes>'" \
--session-arg "burner_list:string:'<List of account hashes>'" \
--session-arg "meta_list:string:'<List of account hashes>'" \
--chain-name <CHAIN NAME> \
--payment-amount 1000000000
```
