# CEP-85: Multi-Token Standard

## Design Goals

This repository adheres to the following design goals for the [CEP-85 multi-token standard](https://github.com/casper-network/ceps/blob/multi-token-standard/text/0085-multi-token-standard.md):

- DApp developers attempting to create a multi-token contract should be able to install the contract as is, with modalities as required for their needs.
- This reference implementation showcases modalities and options for installation and entrypoints for use after installation.
- The reference implementation is self-contained within a single repository, including all tests and documentation for the SDKs provided.
- The implementation adheres to a multi-token standard's publicly perceived expected behavior.
- Standard session code is provided to interact with the installed contract, allowing dApp developers to access normal functions without writing new Wasm-producing logic.

## Table of Contents

1. [Building the Contract](#building-the-contract)

2. [Required Runtime Arguments](#required-runtime-arguments)

   - [Modalities](#modalities)

3. [Installing and Interacting with CEP-85 Contracts using the Rust Casper Client](#installing-and-interacting-with-the-contract-using-the-rust-casper-client)

4. [Installing and Interacting with CEP-85 Contracts using the JS Casper Client](#installing-and-interacting-with-the-contract-using-the-js-casper-client)

5. [Test Suite and Specification](#test-suite-and-specification)

6. [Error Codes](#error-codes)

## Building the Contract

The `main.rs` file within the contract provides the installer for the multi-token contract. Users can compile the contract to Wasm alongside support tests using the following commands from the Makefile provided:

```sh
make prepare
make test
```

The pre-built Wasm for the contract and all other utility session code can be found in the most current release. Users wishing to build the Wasm themselves can pull the code and use the `make build-contract` command provided in the Makefile. Please note, however, that you must install `wasm-strip` to build the contract.

The `call` method will install the contract with the necessary entrypoints and call the `init()` entrypoint, which allows the contract to self-initialize and set up the necessary state variables for operation.

## Required Runtime Arguments

The following are the required runtime arguments to be passed to the installer session code to install the multi-token contract correctly.

- `"name"`: The name of the multi-token collection, passed in as a `String`. This parameter is required and cannot be changed after installation.
- `"uri"`: A string URI for any off-chain resource associated with the collection.

The following are the optional parameters that can be passed in during installation.

- `"events_mode"`: The [`EventsMode`](#eventsmode) modality that selects the event schema used to record any changes that occur to tokens issued by the contract instance. This argument is passed in as a `u8` value.
- `"enable_burn"`: The [`EnableBurn`](#enableburn) modality dictates whether the contract instance will allow approved entities to burn tokens permanently. This argument is passed in as a `bool` value.
- `"transfer_filter_contract"`: This argument dictates a secondary contract instance that will serve as a transfer filter for the installing instance of CEP-85. Passing an argument with a value of type `Key` will enable this feature.
- `"transfer_filter_method"`: This argument outlines the name of the entrypoint on the transfer filter contract that is used to process the filter. It is passed as a `String`.

In addition, the following arguments may be passed to establish their associated user lists.

- `"admin_list"` : A list of users with `admin` access to this contract instance. Passed in as a list of `Key`.
- `"minter_list"` : A list of users that can mint tokens using this contract instance. Passed in as a list of `Key`.
- `"burner_list"` : A list of users that can burn tokens using this contract instance. Passed in as a list of `Key`.
- `"meta_list"` : A list of users that have access to the `set_uri` entrypoint. Passed in as a list of `Key`.
- `"none_list"` : A list of users without (banned of) special access to the contract instance. Passed in as a list of `Key`.

### Modalities

#### EventsMode

The `EventsMode` modality determines how the installed instance of CEP-85 will handle the recording of events that occur from interacting with the contract.

The modality provides two options:

1. `NoEvents`: This modality will signal the contract not to record any events. This is the default mode.
2. `CES`: This modality will signal the contract to record events using the [Casper Event Standard](#casper-event-standard).

| EventsMode | u8  |
| ---------- | --- |
| NoEvents   | 0   |
| CES        | 1   |

#### EnableBurn

The `EnableBurn` modality determines if the installed instance of CEP-85 will allow the burning of tokens.

The modality provides two options:

1. `False`: Tokens may not be burned. This is the default mode.
2. `True`: Tokens may be burned by approved users on the `burner_list`.

| EnableBurn | Bool  |
| ---------- | ----- |
| Disabled   | False |
| Enabled    | True  |

##### Casper Event Standard

`CES` is an option within the `EventsMode` modality that determines how changes to tokens issued by the contract instance will be recorded. Any changes are recorded in the `__events` dictionary and can be observed via a node's Server Side Events stream. They may also be viewed by querying the dictionary at any time using the JSON-RPC interface.

The emitted events are encoded according to the [Casper Event Standard](https://github.com/make-software/casper-event-standard), and the schema is visible to an observer reading the `__events_schema` contract named key.

For this CEP-85 reference implementation, the events schema is as follows:

| Event name     | Included values and type                                                  |
| -------------- | ------------------------------------------------------------------------- |
| Mint           | id (U256), recipient (Key), amount (U256)                                 |
| Burn           | id (U256), owner (Key), amount (U256)                                     |
| ApprovalForAll | owner (Key), operator (Key), approved (bool)                              |
| TransferSingle | operator (Key), from (Key), to (Key), id (U256), value (U256)             |
| TransferBatch  | operator (Key), from (Key), to (Key), ids (Vec<U256>), values (Vec<U256>) |
| Uri            | value (String), id (Option<U256>)                                         |
| SetTotalSupply | id (U256), total_supply (U256)                                            |
| ChangeSecurity | admin (Key), sec_change_map (BTreeMap<Key, SecurityBadge>)                |
| SetModalities  |                                                                           |
| Migration      |                                                                           |

#### Transfer Filter Hook

If enabled, the transfer filter modality specifies a contract package hash pointing to a contract that will be called when the `safe_transfer_from` or `safe_batch_transfer_from` methods are invoked on the contract. CEP-85 will call the transfer filter method on the specified callback contract, which is expected to return a value of `TransferFilterContractResult`, represented as a u8.

- `TransferFilterContractResult::DenyTransfer` will block the transfer regardless of the outcome of other checks
- `TransferFilterContractResult::ProceedTransfer` will allow the transfer to proceed if other checks also pass

The transfer filter can be enabled by passing an `ARG_TRANSFER_FILTER_CONTRACT` argument to the install method, with a value of type `Key`. The transfer filter method can be defined with the `ARG_TRANSFER_FILTER_METHOD` argument.

This parameter is optional and cannot be changed after installation.

## Installing and Interacting with the Contract using the Rust Casper Client

You can find instructions on installing an instance of the CEP-85 contract using the [Rust CLI Casper client](/docs/using-casper-client.md).

The following is an example of installing the CEP-85 contract via a deploy using the Rust CLI Casper client.

```bash
casper-client put-deploy -n https://rpc.testnet.casperlabs.io/ \
--chain-name "casper-test" \
--payment-amount 500000000000 \
-k keys/secret_key.pem \
--session-path target/wasm32-unknown-unknown/release/cep85.wasm \
--session-arg "name:string='multi-token-1'" \
--session-arg "uri:string='https://docs.casper.network/'" \
--session-arg "events_mode:u8='0'" \
--session-arg "enable_burn:bool='true'"
```

## Installing and Interacting with the Contract using the JavaScript Casper Client

You can find instructions on installing an instance of the CEP-85 contract using the [JS Casper client](/client-js/README.md).

## Test Suite and Specification

The expected behavior of the multi-token contract implementation is asserted by its test suite found in the `tests` folder. The test suite and the corresponding unit tests comprise the specification around the contract and outline the expected behaviors of the multi-token contract across the entire range of possible configurations. The test suite ensures that as new modalities are added, and current modalities are extended, no regressions and conflicting behaviors are introduced. The test suite also asserts the correct working behavior of the utility session code provided in the client folder. The tests can be run by using the provided `Makefile` and running the `make test` command.

## Error Codes

| Code | Error                         |
| ---- | ----------------------------- |
| 1    | BurnDisabled                  |
| 2    | ContractAlreadyInitialized    |
| 3    | ExceededMaxTotalSupply        |
| 4    | FailedToBatchTransferBalance  |
| 5    | FailedToCreateArg             |
| 6    | FailedToCreateDictionary      |
| 7    | FailedToGetArgBytes           |
| 8    | FailToBatchTransferBalance    |
| 9    | FailToTransferBalance         |
| 10   | InsufficientBalance           |
| 11   | InsufficientRights            |
| 12   | InvalidAccount                |
| 13   | InvalidAccounts               |
| 14   | InvalidAdminList              |
| 15   | InvalidAmount                 |
| 16   | InvalidAmounts                |
| 17   | InvalidBurnTarget             |
| 18   | InvalidBurnerList             |
| 19   | InvalidCollectionName         |
| 20   | InvalidContractHash           |
| 21   | InvalidData                   |
| 22   | InvalidEnableBurnFlag         |
| 23   | InvalidEventsMode             |
| 24   | InvalidFrom                   |
| 25   | InvalidId                     |
| 26   | InvalidIds                    |
| 27   | InvalidKey                    |
| 28   | InvalidMetaList               |
| 29   | InvalidMinterList             |
| 30   | InvalidNoneList               |
| 31   | InvalidOperator               |
| 32   | InvalidOwner                  |
| 33   | InvalidPackageHash            |
| 34   | InvalidRecipient              |
| 35   | InvalidStorageUref            |
| 36   | InvalidTo                     |
| 37   | InvalidTotalSupply            |
| 38   | InvalidTotalSupplies          |
| 39   | InvalidTransferFilterContract |
| 40   | InvalidTransferFilterMethod   |
| 41   | InvalidUri                    |
| 42   | MissingAccount                |
| 43   | MissingAccounts               |
| 44   | MissingAmount                 |
| 45   | MissingAmounts                |
| 46   | MissingCollectionName         |
| 47   | MissingContractHash           |
| 48   | MissingEnableMBFlag           |
| 49   | MissingEventsMode             |
| 50   | MissingFrom                   |
| 51   | MissingId                     |
| 52   | MissingIds                    |
| 53   | MissingOperator               |
| 54   | MissingOwner                  |
| 55   | MissingPackageHash            |
| 56   | MissingRecipient              |
| 57   | MissingStorageUref            |
| 58   | MissingTo                     |
| 59   | MissingTotalSupply            |
| 60   | MissingTotalSupplies          |
| 61   | MissingTransferFilterContract |
| 62   | MissingTransferFilterMethod   |
| 63   | MissingUri                    |
| 64   | MismatchParamsLength          |
| 65   | NotApproved                   |
| 66   | Overflow                      |
| 67   | OverflowBatchBurn             |
| 68   | OverflowBatchMint             |
| 69   | OverflowBurn                  |
| 70   | OverflowMint                  |
| 71   | Phantom                       |
| 72   | SelfOperatorApproval          |
| 73   | SelfTransfer                  |
| 74   | TokenSupplyDepleted           |
| 75   | TransferFilterContractDenied  |
| 76   | UnexpectedKeyVariant          |
| 77   | InvalidUpgradeFlag            |
| 78   | MissingKey                    |
| 79   | InvalidKeyName                |
| 80   | InvalidValue                  |
| 81   | MissingValue                  |
