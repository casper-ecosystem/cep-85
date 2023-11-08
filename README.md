# CEP-85: Multi-Token Standard

## Design Goals

- DApp developers attempting to create a multi-token contract should be able to install the contract as is, with modalities as required for their needs.
- Reference implementation should showcase modalities and options for install and entrypoints for use after installation.
- CEP-85 reference implementation should be self-contained within a single repo, including all tests and documentation for Casper Labs provided SDKs.
- Should adhere to publicly perceived expected behavior of a multi-token standard.
- Standard session code should be provided to interact with the installed contract, allowing DApp developers to access normal functions without writing new Wasm producing logic.

## Table of Contents

1. [Building the Contract](#building-the-contract)

2. [Required Runtime Arguments](#required-runtime-arguments)
   
   - [Modalities](#modalities)

   - [Example Deploy](#example-deploy) 

3. [Installing and Interacting with CEP-85 Contracts using the Rust Casper Client](#installing-and-interacting-with-the-contract-using-the-rust-casper-client)

4. [Test Suite and Specification](#test-suite-and-specification)

5. [Error Codes](#error-codes)


## Building the Contract

The `main.rs` file within the contract provides the installer for the multi-token contract. Users can compile the contract to Wasm alongside support tests using the `make prepare` and `make test` commands from the Makefile provided.

The pre-built Wasm for the contract and all other utility session code can be found as part of the most current release. Users wishing to build the Wasm themselves can pull the code and use the `make build-contract` command provided in the Makefile. Please note, however, that you must install `wasm-strip` to build the contract.

The `call` method will install the contract with the necessary entrypoints and call the `init()` entrypoint, which allows the contract to self-initialize and set up the necessary state variables for operation.d

## Required Runtime Arguments

The following are the required runtime arguments that must be passed to the installer session code to correctly install the multi-token contract.

- `"name"`: The name of the multi-token collection, passed in as a `String`. This parameter is required and cannot be changed post installation.
- `"uri"`: A string URI for any off-chain resource associated with the token.

The following are the optional parameters that can be passed in at the time of installation.

- `"events_mode"`: The [`EventsMode`](#eventsmode) modality that selects the event schema used to record any changes that occur to tokens issued by the contract instance. This argument is passed in as a `u8` value.
- `"enable_burn"`: The [`EnableBurn`](#enableburn) modality dictates whether the contract instance will allow approved entities to permanently burn tokens. This argument is passed in as a `u8` value.
- `"transfer_filter_contract"`: This argument dictates a secondary contract instance that will serve as a transfer filter for the installing instance of CEP-85. Passing an argument with a value of type `Option<Key>` will enable this feature.
- `"transfer_filter_method"`: This argument outlines the name of the entry point on the transfer filter contract that is used to access the filter. It is passed as an `Option<String>`.

In addition, the following arguments may be passed to establish their associated user lists.

- `"admin_list"` : A list of users that will have `admin` access to this contract instance. Passed in as a string consisting of a list of `PublicKeys`.
- `"minter_list"` : A list of users that will have the ability to mint tokens using this contract instance. Passed in as a string consisting of a list of `PublicKeys`.
- `"burner_list"` : A list of users that will have the ability to burn tokens using this contract instance. Passed in as a string consisting of a list of `PublicKeys`.
- `"meta_list"` : A list of users that have access to the `set_uri` entrypoint. Passed in as a string consisting of a list of `PublicKeys`.
- `"none_list"` :  A list of users that have no special access to the contract instance. Passed in as a string consisting of a list of `PublicKeys`.

### Modalities

#### EventsMode

The `EventsMode` modality determines how the installed instance of CEP-85 will handle the recording of events that occur from interacting with the contract.

The modality provides two options:

1. `NoEvents`: This modality will signal the contract to not record events at all. This is the default mode.
2. `CES`: This modality will signal the contract to record events using the [Casper Event Standard](#casper-event-standard).

| EventsMode | u8  |
| ---------- | --- |
| NoEvents   | 0   |
| CES        | 1   |

##### Casper Event Standard

`CES` is an option within the `EventsMode` modality that determines how changes to tokens issued by the contract instance will be recorded. Any changes are recorded in the `__events` dictionary and can be observed via a node's Server Side Events stream. They may also be viewed by querying the dictionary at any time using the JSON-RPC interface.

The emitted events are encoded according to the [Casper Event Standard](https://github.com/make-software/casper-event-standard), and the schema is visible to an observer reading the `__events_schema` contract named key.

For this CEP-85 reference implementation, the events schema is as follows:

| Event name      | Included values and type                                                 |
| --------------- | ------------------------------------------------------------------------ |
| Mint            | id (U256), recipient (Key), amount (U256)                                | 
| Burn            | id (U256), owner (Key), amount (U256)                                    |
| ApprovalForAll  | owner (Key), operator (Key), approved (bool)                             |
| TransferSingle  | operator (Key), from (Key), to (Key), id (U256), value (U256)            |
| TransferBatch   | operator (Key), from (Key), to (Key), ids (Vec<U256>), values (Vec<U256>)|
| Uri             | value (String), id (Option<U256>)                                        |
| SetTotalSupply  | id (U256), total_supply (U256)                                           |
| ChangeSecurity  | admin (Key), sec_change_map (BTreeMap<Key, SecurityBadge>)               |

#### EnableBurn

The `EnableBurn` modality determines if the installed instance of CEP-85 will allow the burning of tokens.

The modality provides two options:

1. `False`: Tokens may not be burned. This is the default mode.
2. `True`: Tokens may be burned by approved users on the `burner_list`.

| EnableBurn | u8  |
| ---------- | --- |
| False      | 0   |
| True       | 1   |

#### Transfer Filter Hook

The transfer filter modality, if enabled, specifies a contract package hash pointing to a contract that will be called when the `safe_transfer_from` or `safe_batch_transfer_from` methods are invoked on the contract. CEP-85 will call the transfer filter method on the specified callback contract, which is expected to return a value of `TransferFilterContractResult`, represented as a u8.

- `TransferFilterContractResult::DenyTransfer` will block the transfer regardless of the outcome of other checks
- `TransferFilterContractResult::ProceedTransfer` will allow the transfer to proceed if other checks also pass

The transfer filter can be enabled by passing a `ARG_TRANSFER_FILTER_CONTRACT` argument to the install method, with a value of type `Option<Key>` The transfer filter method can be defined with the `ARG_TRANSFER_FILTER_METHOD` argument.

### Example deploy

The following is an example of installing the CEP-85 contract via a deploy using the Rust CLI Casper client. You can find more examples [here](/docs/using-casper-client.md).

```bash
casper-client put-deploy -n http://65.108.0.148:7777/rpc --chain-name "casper-test" --payment-amount 500000000000 -k keys/secret_key.pem --session-path target/wasm32-unknown-unknown/release/cep85.wasm \
--session-arg "name:string='multi-token-1'" \
--session-arg "uri:string='https://docs.casper.network/'" \
--session-arg "events_mode:u8='0'" \
--session-arg "enable_burn:u8='1'" \
```

## Installing and Interacting with the Contract using the Rust Casper Client

You can find instructions on installing an instance of the CEP-78 contract using the Rust CLI Casper client [here](/docs/using-casper-client.md).

## Test Suite and Specification

The expected behavior of the multi-token contract implementation is asserted by its test suite found in the `tests` folder. The test suite and the corresponding unit tests comprise the specification around the contract and outline the expected behaviors of the multi-token contract across the entire range of possible configurations. The test suite ensures that as new modalities are added, and current modalities are extended, no regressions and conflicting behaviors are introduced. The test suite also asserts the correct working behavior of the utility session code provided in the client folder. The tests can be run by using the provided `Makefile` and running the `make test` command.

## Error Codes

| Code | Error                                       |
| ---- | ------------------------------------------- |
| 1    | BurnDisabled                                |
| 2    | ContractAlreadyInitialized                  |
| 3    | ExceededMaxTotalSupply                      |
| 4    | FailedToBatchTransferBalance                |
| 5    | FailedToCreateArg                           |
| 6    | FailedToCreateDictionary                    |
| 7    | FailedToGetArgBytes                         |
| 8    | FailToBatchTransferBalance                  |
| 9    | FailToTransferBalance                       |
| 10   | InsufficientBalance                         |
| 11   | InsufficientRights                          |
| 12   | InvalidAccount                              |
| 13   | InvalidAccounts                             |
| 14   | InvalidAdminList                            |
| 15   | InvalidAmount                               |
| 16   | InvalidAmounts                              |
| 17   | InvalidBurnTarget                           |
| 18   | InvalidBurnerList                           |
| 19   | InvalidCollectionName                       |
| 20   | InvalidContractHash                         |
| 21   | InvalidData                                 |
| 22   | InvalidEnableMBFlag                         |
| 23   | InvalidEventsMode                           |
| 24   | InvalidFrom                                 |
| 25   | InvalidId                                   |
| 26   | InvalidIds                                  |
| 27   | InvalidKey                                  |
| 28   | InvalidMetaList                             |
| 29   | InvalidMinterList                           |
| 30   | InvalidNoneList                             |
| 31   | InvalidOperator                             |
| 32   | InvalidOwner                                |
| 33   | InvalidPackageHash                          |
| 34   | InvalidRecipient                            |
| 35   | InvalidStorageUref                          |
| 36   | InvalidTo                                   |
| 37   | InvalidTotalSupply                          |
| 38   | InvalidTotalSupplies                        |
| 39   | InvalidTransferFilterContract               |
| 40   | InvalidTransferFilterMethod                 |
| 41   | InvalidUri                                  |
| 42   | MissingAccount                              |
| 43   | MissingAccounts                             |
| 44   | MissingAmount                               |
| 45   | MissingAmounts                              |
| 46   | MissingCollectionName                       |
| 47   | MissingContractHash                         |
| 48   | MissingData                                 |
| 49   | MissingEnableMBFlag                         |
| 50   | MissingEventsMode                           |
| 51   | MissingFrom                                 |
| 52   | MissingId                                   |
| 53   | MissingIds                                  |
| 54   | MissingOperator                             |
| 55   | MissingOwner                                |
| 56   | MissingPackageHash                          |
| 57   | MissingRecipient                            |
| 58   | MissingStorageUref                          |
| 59   | MissingTo                                   |
| 60   | MissingTotalSupply                          |
| 61   | MissingTotalSupplies                        |
| 62   | MissingTransferFilterContract               |
| 63   | MissingTransferFilterMethod                 |
| 64   | MissingUri                                  |
| 65   | MismatchParamsLength                        |
| 66   | NotApproved                                 |
| 67   | Overflow                                    |
| 68   | OverflowBatchBurn                           |
| 69   | OverflowBatchMint                           |
| 70   | OverflowBurn                                |
| 71   | OverflowMint                                |
| 72   | Phantom                                     |
| 73   | SelfOperatorApproval                        |
| 74   | SelfTransfer                                |
| 75   | TokenSupplyDepleted                         |
| 76   | TransferFilterContractDenied                |
| 77   | UnexpectedKeyVariant                        |
