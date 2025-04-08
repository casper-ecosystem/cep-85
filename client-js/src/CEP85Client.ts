import { blake2b } from '@noble/hashes/blake2b';
import { bytesToHex } from '@noble/hashes/utils';
import {
  Args as RuntimeArgs,
  CLTypeKey,
  CLValue,
  ContractHash,
  ContractPackageHash,
  Key,
  PublicKey,
  SessionBuilder,
  AddressableEntityHash,
  CLTypeUInt256,
  CLTypeUInt8,
} from 'casper-js-sdk';
import Client from './client';
import {
  EVENTS_MODE,
  type InstallParams,
  type TransactionResult,
  type ChangeSecurityParams,
  type UpgradeParams,
  type BatchMintParams,
  type MintParams,
  type SetUriParams,
  Entity,
  type BatchBurnParams,
  type BurnParams,
  type BatchTransferParams,
  type TransferParams,
  type SetModalitiesParams,
  type SetApprovallForAllParams,
  type TotalSupplyOfBatchParams,
  type TotalSupplyOfParams,
} from './types';
import ContractWASM from './wasm/cep85';

/**
 * CEP85Client extends the base `Client` class to provide specific functionality
 * for interacting with CEP-85 token contracts on the Casper blockchain.
 */
export default class CEP85Client extends Client {
  contractClient: any;
  /**
   * Initializes a new CEP85Client instance.
   *
   * @param rpcUrl - The RPC URL of the Casper network.
   * @param ssUrl - (Optional) The SSE URL for event streaming.
   * @param chainName - (Optional) The name of the blockchain network.
   */
  constructor(rpcUrl: string, ssUrl?: string, chainName?: string) {
    super(rpcUrl, ssUrl, chainName);
  }

  /**
   * Sets the contract hash and optionally the contract package hash.
   *
   * This method removes prefixes from the provided contract hash and package hash
   * before converting them into the appropriate `ContractHash` and `ContractPackageHash` objects.
   *
   * @param contractHash - The contract hash as a string or `ContractHash` instance.
   * @param contractPackageHash - (Optional) The contract package hash as a string or `ContractPackageHash` instance.
   * @returns The updated `CEP85Client` instance.
   * @throws `Error` if the contract hash is not provided or invalid.
   */
  public setContractHash(
    contractHash: string | ContractHash,
    contractPackageHash?: string | ContractPackageHash
  ): CEP85Client {
    const removePrefix = (str: string | undefined) =>
      str ? str.replace(/^.*-/, '') : '';

    const hexContractHash =
        typeof contractHash === 'string' ? removePrefix(contractHash) : '',
      hexContractPackageHash =
        typeof contractPackageHash === 'string'
          ? removePrefix(contractPackageHash)
          : '',
      newContractHash = hexContractHash
        ? ContractHash.newContract(hexContractHash)
        : undefined,
      newContractPackageHash = hexContractPackageHash
        ? ContractPackageHash.newContractPackage(hexContractPackageHash)
        : undefined;

    if (!newContractHash) {
      throw new Error('Contract hash must be provided.');
    }
    return super.setContractHash(
      newContractHash,
      newContractPackageHash
    ) as unknown as CEP85Client;
  }

  /**
   * Starts the SSE event stream to listen for contract-related events.
   *
   * This method enables real-time event listening for the contract by calling
   * the parent `startEventStream` method.
   *
   * @param sseUrl - (Optional) The SSE endpoint URL. If not provided, the previously set URL is used.
   * @returns The updated `CEP85Client` instance.
   */
  public startEventStream(sseUrl?: string): CEP85Client {
    return super.startEventStream(sseUrl) as unknown as CEP85Client;
  }

  /**
   * Stops the SSE event stream, preventing further event processing.
   *
   * This method ensures that the event stream is properly stopped and unsubscribed.
   *
   * @returns The updated `CEP85Client` instance.
   */
  public stopEventStream(): CEP85Client {
    return super.stopEventStream() as unknown as CEP85Client;
  }

  /**
   * Installs the CEP-85 contract on the Casper network.
   *
   * @param params - The installation parameters, including:
   *   - `wasm`: (Optional) The compiled contract in `Uint8Array` format. If not provided, a default wasm will be used.
   *   - `paymentAmount`: The amount of payment required for contract installation.
   *   - `sender`: The public key of the account deploying the contract.
   *   - `chainName`: (Optional) The name of the network where the contract will be deployed.
   *   - `signingKeys`: (Optional) An array of private keys used for signing the transaction.
   *   - `args`: Contract-specific arguments, including:
   *     - `name`: The name of the token or contract.
   *     - `uri`: A base URI for metadata association.
   *     - `eventsMode`: (Optional) The mode in which contract events are emitted (`NoEvents`, `CES`, `Native`, or `NativeBytes`).
   *     - `enableBurn`: (Optional) A boolean flag to enable or disable burning functionality.
   *     - `adminList`: (Optional) A list of entities with administrative privileges.
   *     - `minterList`: (Optional) A list of entities allowed to mint tokens.
   *     - `burnerList`: (Optional) A list of entities allowed to burn tokens.
   *     - `metaList`: (Optional) A list of entities with metadata modification rights.
   *     - `noneList`: (Optional) A list of entities with no assigned roles.
   *     - `transferFilterContract`: (Optional) A contract hash for filtering token transfers.
   *     - `transferFilterMethod`: (Optional) The method name on the filter contract used during transfer operations.
   *   - `waitForTransactionProcessed`: (Optional) If `true`, waits for the transaction to be processed and returns the execution result.
   *
   * @returns A `Promise` resolving to `TransactionResult`, containing the transaction details and optional execution result.
   *
   * @throws Will throw an error if the Wasm file is missing or if an error occurs during contract installation.
   *
   * @remarks
   * This method installs a new CEP-85 contract on the Casper network. It supports advanced role configurations including admin, minter, burner, metadata, and none roles.
   * Additional customization can be achieved through event modes and transfer filtering logic.
   * Ensure that all required arguments are provided and that the contract wasm file is valid.
   */

  public async install(params: InstallParams): Promise<TransactionResult> {
    const {
      params: { wasm, paymentAmount, sender, chainName, signingKeys },
      args: {
        name,
        uri,
        eventsMode,
        enableBurn,
        adminList,
        minterList,
        burnerList,
        metaList,
        noneList,
        transferFilterContract,
        transferFilterMethod,
      },
    } = params;

    const runtimeArgs = RuntimeArgs.fromMap({
      name: CLValue.newCLString(name),
      uri: CLValue.newCLString(uri),
    });

    if (eventsMode !== undefined) {
      runtimeArgs.insert('events_mode', CLValue.newCLUint8(eventsMode));
    }

    if (enableBurn !== undefined) {
      runtimeArgs.insert('enable_burn', CLValue.newCLValueBool(enableBurn));
    }

    if (adminList) {
      runtimeArgs.insert(
        'admin_list',
        CLValue.newCLList(
          CLTypeKey,
          adminList.map((key) =>
            CLValue.newCLKey(CEP85Client.getPrefixedString(key))
          )
        )
      );
    }
    if (minterList) {
      runtimeArgs.insert(
        'minter_list',
        CLValue.newCLList(
          CLTypeKey,
          minterList.map((key) =>
            CLValue.newCLKey(CEP85Client.getPrefixedString(key))
          )
        )
      );
    }
    if (burnerList) {
      runtimeArgs.insert(
        'burner_list',
        CLValue.newCLList(
          CLTypeKey,
          burnerList.map((key) =>
            CLValue.newCLKey(CEP85Client.getPrefixedString(key))
          )
        )
      );
    }
    if (metaList) {
      runtimeArgs.insert(
        'meta_list',
        CLValue.newCLList(
          CLTypeKey,
          metaList.map((key) =>
            CLValue.newCLKey(CEP85Client.getPrefixedString(key))
          )
        )
      );
    }
    if (noneList) {
      runtimeArgs.insert(
        'none_list',
        CLValue.newCLList(
          CLTypeKey,
          noneList.map((key) =>
            CLValue.newCLKey(CEP85Client.getPrefixedString(key))
          )
        )
      );
    }

    if (transferFilterContract && transferFilterMethod) {
      runtimeArgs.insert(
        'transfer_filter_contract',
        CLValue.newCLKey(CEP85Client.getPrefixedString(transferFilterContract))
      );
      runtimeArgs.insert(
        'transfer_filter_method',
        CLValue.newCLString(transferFilterMethod)
      );
    }

    const wasmBytes = wasm || ContractWASM;

    if (!wasmBytes) {
      throw new Error('Wasm file is missing.');
    }

    const transaction = new SessionBuilder()
      .installOrUpgrade()
      .wasm(wasmBytes)
      .runtimeArgs(runtimeArgs)
      .payment(Number(paymentAmount))
      .from(sender)
      .chainName(chainName ? chainName : this.chainName || '')
      .build();

    if (signingKeys) {
      signingKeys.forEach((key) => transaction.sign(key));
    }
    try {
      const transactionInfo = await this.rpcClient.putTransaction(transaction);
      if (
        params.waitForTransactionProcessed &&
        transactionInfo.transactionHash
      ) {
        const transactionProcessedEvent =
          await this.waitForTransactionProcessed(
            transactionInfo.transactionHash.toString()
          );
        const executionResult =
          transactionProcessedEvent.transactionProcessedPayload.executionResult;
        if (executionResult?.errorMessage) {
          this.handleExecutionError(executionResult.errorMessage);
        }
        return { transactionInfo, executionResult };
      }
      return { transactionInfo };
    } catch (error) {
      throw new Error(`Error during installation runtime.\n${error}`);
    }
  }

  /**
   * Upgrades an existing CEP-85 contract to a new version on the Casper network.
   *
   * @param params - Parameters for upgrading the contract, including:
   *   - `wasm`: (Optional) The compiled contract in `Uint8Array` format representing the new version. If not provided, a default wasm will be used.
   *   - `paymentAmount`: The amount of payment required for the upgrade.
   *   - `sender`: The public key of the account performing the upgrade.
   *   - `chainName`: (Optional) The name of the Casper network where the contract resides.
   *   - `signingKeys`: (Optional) An array of private keys used to sign the transaction.
   *   - `args`: Upgrade-specific arguments, including:
   *     - `name`: The name for the upgraded contract.
   *   - `waitForTransactionProcessed`: (Optional) If `true`, waits for the transaction to be processed and returns the execution result.
   *
   * @returns A `Promise` resolving to `TransactionResult`, which contains the transaction information and optionally the execution result.
   *
   * @throws Will throw an error if the Wasm file is missing or if an error occurs during the upgrade process.
   *
   * @remarks
   * This method facilitates upgrading a CEP-85 contract to a newer version. It injects the provided `name` and a `true` `upgrade` flag into the runtime arguments.
   * Ensure the provided wasm file is valid and represents a compatible upgrade for the existing contract.
   * If `waitForTransactionProcessed` is `true`, the function waits for the contract execution result before resolving.
   */

  public async upgrade(params: UpgradeParams): Promise<TransactionResult> {
    const {
      params: { wasm, paymentAmount, sender, chainName, signingKeys },
      args: { name },
    } = params;

    const runtimeArgs = RuntimeArgs.fromMap({
      name: CLValue.newCLString(name),
      upgrade: CLValue.newCLValueBool(true),
    });

    const wasmBytes = wasm || ContractWASM;

    if (!wasmBytes) {
      throw new Error('Wasm file is missing.');
    }

    const transaction = new SessionBuilder()
      .installOrUpgrade()
      .wasm(wasmBytes)
      .runtimeArgs(runtimeArgs)
      .payment(Number(paymentAmount))
      .from(sender)
      .chainName(chainName ? chainName : this.chainName || '')
      .build();

    if (signingKeys) {
      signingKeys.forEach((key) => transaction.sign(key));
    }
    try {
      const transactionInfo = await this.rpcClient.putTransaction(transaction);
      if (
        params.waitForTransactionProcessed &&
        transactionInfo.transactionHash
      ) {
        const transactionProcessedEvent =
          await this.waitForTransactionProcessed(
            transactionInfo.transactionHash.toString()
          );
        const executionResult =
          transactionProcessedEvent.transactionProcessedPayload.executionResult;
        if (executionResult?.errorMessage) {
          this.handleExecutionError(executionResult.errorMessage);
        }
        return { transactionInfo, executionResult };
      }
      return { transactionInfo };
    } catch (error) {
      throw new Error(`Error during upgrade runtime.\n${error}`);
    }
  }

  /**
   * Retrieves the name of the collection from the smart contract.
   * @returns A Promise that resolves to the collection name.
   */
  public async collectionName() {
    try {
      return (await this.queryContractData(['name'])) as string;
    } catch (error) {
      // console.error(error);
      console.warn('Contract collection name is empty');
      return '';
    }
  }

  /**
   * Retrieves the URI of the collection from the smart contract.
   * @returns A Promise that resolves to the collection URI.
   */
  public async collectionUri() {
    try {
      return (await this.queryContractData(['uri'])) as string;
    } catch (error) {
      // console.error(error);
      console.warn('Contract collection uri is empty');
      return '';
    }
  }

  /**
   * Constructs a dictionary item key by concatenating and hashing the bytes of the provided CLKey and CLValue.
   * @param key The CLValue for the dictionary item.
   * @param value The CLValue for the dictionary item.
   * @returns The resulting dictionary item key as a hexadecimal string.
   */
  public static makeDictionaryItemKey(key: CLValue, value: CLValue): string {
    const keyBytes = key.bytes();
    const valueBytes = value.bytes();

    const concatenatedBytes = new Uint8Array(
      keyBytes.length + valueBytes.length
    );
    concatenatedBytes.set(keyBytes);
    concatenatedBytes.set(valueBytes, keyBytes.length);

    const hashedBytes = blake2b(concatenatedBytes, { dkLen: 32 });

    return bytesToHex(hashedBytes);
  }

  /**
   * Setting the URI for tokens.
   * @param args Arguments for setting URI. See {@link SetUriParams}.
   */
  public setUri(params: SetUriParams) {
    const {
      params: { paymentAmount, sender, chainName, signingKeys },
      waitForTransactionProcessed,
      args: { id, uri },
    } = params;

    const runtimeArgs = RuntimeArgs.fromMap({
      uri: CLValue.newCLString(uri),
    });

    if (id) {
      runtimeArgs.insert('id', CLValue.newCLUInt256(id));
    }

    return this.callEntrypoint(
      'set_uri',
      runtimeArgs,
      paymentAmount,
      sender,
      signingKeys,
      chainName,
      waitForTransactionProcessed
    );
  }

  /**
   * Retrieves the URI associated with a specific token ID or the default URI if no ID is provided.
   * @param id The optional token ID for which to retrieve the URI.
   * @returns The URI as a string
   */
  public async getURI(id?: string) {
    let result: string;
    if (id) {
      result =
        (await this.queryContractDictionary('token_uri', id)) ||
        (await this.collectionUri());
      result = result.replace('{id}', id!);
    } else {
      result = await this.collectionUri();
    }
    return result;
  }

  /**
   * Internal helper method to invoke the minting entrypoint on the CEP-85 contract.
   *
   * @param params - The minting parameters, supporting both single and batch minting operations:
   *   - `paymentAmount`: The amount of payment required for the minting operation.
   *   - `sender`: The public key of the account initiating the mint.
   *   - `chainName`: (Optional) The name of the Casper network.
   *   - `signingKeys`: (Optional) An array of private keys used to sign the transaction.
   *   - `waitForTransactionProcessed`: (Optional) If `true`, waits for the transaction to be processed and returns the execution result.
   *   - `args`: Mint-specific arguments, either:
   *     - For single mint:
   *       - `recipient`: The target entity receiving the minted token.
   *       - `id`: The identifier of the token to be minted.
   *       - `amount`: The amount of tokens to mint.
   *       - `uri`: (Optional) Metadata URI associated with the token.
   *     - For batch mint:
   *       - `recipient`: The target entity receiving the tokens.
   *       - `ids`: An array of token IDs to mint.
   *       - `amounts`: An array of corresponding amounts for each token ID.
   *       - `uri`: (Optional) Metadata URI applied to all minted tokens.
   *
   * @param entrypoint - The name of the entrypoint to invoke (e.g., `'mint'` or `'mint_batch'`).
   *
   * @returns A `Promise` resolving to `TransactionResult`, containing the transaction and optionally the execution result.
   *
   * @remarks
   * This method dynamically constructs the runtime arguments based on whether the minting is single or batch.
   * It then calls the provided contract entrypoint using these arguments.
   * Designed for internal use by public `mint` or `batchMint` methods.
   */
  private queryMint(params: MintParams | BatchMintParams, entrypoint: string) {
    const {
      params: { paymentAmount, sender, chainName, signingKeys },
      waitForTransactionProcessed,
      args,
    } = params;

    const runtimeArgs = RuntimeArgs.fromMap({
      recipient: CLValue.newCLKey(
        CEP85Client.getPrefixedString(args.recipient)
      ),
    });

    if ('id' in args) {
      runtimeArgs.insert('id', CLValue.newCLUInt256(args.id));
    }
    if ('amount' in args) {
      runtimeArgs.insert('amount', CLValue.newCLUInt256(args.amount));
    }
    if ('ids' in args) {
      runtimeArgs.insert(
        'ids',
        CLValue.newCLList(CLTypeUInt256, args.ids.map(CLValue.newCLUInt256))
      );
    }
    if ('amounts' in args) {
      runtimeArgs.insert(
        'amounts',
        CLValue.newCLList(CLTypeUInt256, args.amounts.map(CLValue.newCLUInt256))
      );
    }
    if ('uri' in args && args.uri) {
      runtimeArgs.insert('uri', CLValue.newCLString(args.uri));
    }

    return this.callEntrypoint(
      entrypoint,
      runtimeArgs,
      paymentAmount,
      sender,
      signingKeys,
      chainName,
      waitForTransactionProcessed
    );
  }

  /**
   * Mints a new token to a specified recipient.
   *
   * @param params - Parameters for minting a token, including:
   *   - `paymentAmount`: The amount of payment required for minting.
   *   - `sender`: The public key of the account initiating the mint.
   *   - `chainName`: (Optional) The name of the Casper network.
   *   - `signingKeys`: (Optional) An array of private keys used to sign the transaction.
   *   - `args`: Arguments specific to minting:
   *     - `recipient`: The target entity receiving the minted token.
   *     - `id`: The token ID to be minted.
   *     - `amount`: The amount of the token to mint.
   *     - `uri`: (Optional) Metadata URI associated with the token.
   *   - `waitForTransactionProcessed`: (Optional) Whether to wait for the transaction to be processed.
   *
   * @returns A `Promise` resolving to `TransactionResult`, including transaction and optional execution result.
   *
   * @remarks
   * This is a public method that wraps the internal `queryMint` helper to mint a single token.
   */
  public mint(params: MintParams) {
    return this.queryMint(params, 'mint');
  }

  /**
   * Mints multiple tokens to a specified recipient in a single transaction.
   *
   * @param params - Parameters for batch minting, including:
   *   - `paymentAmount`: The amount of payment required for minting.
   *   - `sender`: The public key of the account initiating the mint.
   *   - `chainName`: (Optional) The name of the Casper network.
   *   - `signingKeys`: (Optional) An array of private keys used to sign the transaction.
   *   - `args`: Arguments specific to batch minting:
   *     - `recipient`: The target entity receiving the minted tokens.
   *     - `ids`: Array of token IDs to be minted.
   *     - `amounts`: Array of amounts for each token ID.
   *     - `uri`: (Optional) Metadata URI applied to all minted tokens.
   *   - `waitForTransactionProcessed`: (Optional) Whether to wait for the transaction to be processed.
   *
   * @returns A `Promise` resolving to `TransactionResult`, including transaction and optional execution result.
   *
   * @remarks
   * This is a public method that wraps the internal `queryMint` helper to perform batch minting of multiple tokens.
   */
  public batchMint(params: BatchMintParams) {
    return this.queryMint(params, 'batch_mint');
  }

  /**
   * Handles internal logic for both single and batch token transfers.
   *
   * @param params - Transfer parameters, supporting both single and batch modes:
   *   - `paymentAmount`: The payment amount for the transaction.
   *   - `sender`: The public key of the account initiating the transfer.
   *   - `chainName`: (Optional) The name of the Casper network.
   *   - `signingKeys`: (Optional) An array of private keys used to sign the transaction.
   *   - `args`: Transfer-specific arguments:
   *     - `from`: The address of the token sender.
   *     - `to`: The address of the token recipient.
   *     - `id`: (For single transfer) The ID of the token to transfer.
   *     - `amount`: (For single transfer) The amount of the token to transfer.
   *     - `ids`: (For batch transfer) Array of token IDs to transfer.
   *     - `amounts`: (For batch transfer) Array of amounts corresponding to each token ID.
   *     - `data`: (Optional) Additional binary data as a `Uint8Array`.
   *   - `waitForTransactionProcessed`: (Optional) Whether to wait for transaction finalization.
   * @param entrypoint - The name of the contract entrypoint to invoke (`"transfer"` or `"batch_transfer"`).
   *
   * @returns A `Promise` resolving to a `TransactionResult` containing transaction info and optional execution result.
   *
   * @remarks
   * This internal method builds the appropriate runtime arguments and calls the contract's entrypoint.
   * It supports both single and batch token transfer operations and can handle optional data payloads.
   */
  private queryTransfer(
    params: TransferParams | BatchTransferParams,
    entrypoint: string
  ) {
    const {
      params: { paymentAmount, sender, chainName, signingKeys },
      waitForTransactionProcessed,
      args,
    } = params;

    const runtimeArgs = RuntimeArgs.fromMap({
      from: CLValue.newCLKey(CEP85Client.getPrefixedString(args.from)),
      to: CLValue.newCLKey(CEP85Client.getPrefixedString(args.to)),
    });

    if ('id' in args) {
      runtimeArgs.insert('id', CLValue.newCLUInt256(args.id));
    }
    if ('amount' in args) {
      runtimeArgs.insert('amount', CLValue.newCLUInt256(args.amount));
    }
    if ('ids' in args) {
      runtimeArgs.insert(
        'ids',
        CLValue.newCLList(CLTypeUInt256, args.ids.map(CLValue.newCLUInt256))
      );
    }
    if ('amounts' in args) {
      runtimeArgs.insert(
        'amounts',
        CLValue.newCLList(CLTypeUInt256, args.amounts.map(CLValue.newCLUInt256))
      );
    }

    if ('data' in args && args.data !== undefined) {
      const clValues = Array.from(args.data).map(CLValue.newCLUint8);
      runtimeArgs.insert('data', CLValue.newCLList(CLTypeUInt8, clValues));
    }

    return this.callEntrypoint(
      entrypoint,
      runtimeArgs,
      paymentAmount,
      sender,
      signingKeys,
      chainName,
      waitForTransactionProcessed
    );
  }

  /**
   * Transfers a specific token amount from one account to another.
   *
   * @param params - Parameters for the token transfer, including:
   *   - `paymentAmount`: The payment amount for the transaction.
   *   - `sender`: The public key of the account initiating the transfer.
   *   - `chainName`: (Optional) The name of the Casper network.
   *   - `signingKeys`: (Optional) An array of private keys used to sign the transaction.
   *   - `args`: Transfer-specific arguments:
   *     - `from`: The address sending the token.
   *     - `to`: The address receiving the token.
   *     - `id`: The ID of the token to transfer.
   *     - `amount`: The amount of the token to transfer.
   *     - `data`: (Optional) Additional binary data as a `Uint8Array`.
   *   - `waitForTransactionProcessed`: (Optional) Whether to wait for the transaction to be finalized.
   *
   * @returns A `Promise` resolving to `TransactionResult`, containing transaction info and execution result (if available).
   *
   * @remarks
   * This method wraps the internal `queryTransfer` logic and calls the `"transfer_from"` entrypoint
   * to move a specific token amount from one address to another.
   */
  public transfer(params: TransferParams) {
    return this.queryTransfer(params, 'transfer_from');
  }

  /**
   * Transfers multiple token types and amounts in a single batch transaction.
   *
   * @param params - Parameters for batch token transfer, including:
   *   - `paymentAmount`: The payment amount for the transaction.
   *   - `sender`: The public key of the account initiating the transfer.
   *   - `chainName`: (Optional) The name of the Casper network.
   *   - `signingKeys`: (Optional) An array of private keys used to sign the transaction.
   *   - `args`: Batch transfer-specific arguments:
   *     - `from`: The address sending the tokens.
   *     - `to`: The address receiving the tokens.
   *     - `ids`: Array of token IDs to transfer.
   *     - `amounts`: Array of token amounts corresponding to each ID.
   *     - `data`: (Optional) Additional binary data as a `Uint8Array`.
   *   - `waitForTransactionProcessed`: (Optional) Whether to wait for the transaction to be finalized.
   *
   * @returns A `Promise` resolving to `TransactionResult`, including transaction info and execution result (if available).
   *
   * @remarks
   * This method wraps the internal `queryTransfer` logic and calls the `"batch_transfer_from"` entrypoint
   * to perform a batch transfer of multiple token types and quantities between two addresses.
   */
  public batchTransfer(params: BatchTransferParams) {
    return this.queryTransfer(params, 'batch_transfer_from');
  }

  /**
   * Constructs and executes a burn or batch burn transaction on the Casper network.
   *
   * @param params - Parameters for the burn operation, including:
   *   - `paymentAmount`: The payment amount for the transaction.
   *   - `sender`: The public key of the account initiating the burn.
   *   - `chainName`: (Optional) The name of the Casper network.
   *   - `signingKeys`: (Optional) An array of private keys used to sign the transaction.
   *   - `args`: Arguments for the burn operation:
   *     - `owner`: The address that owns the tokens being burned.
   *     - `id`: (For single burn) The ID of the token to burn.
   *     - `amount`: (For single burn) The amount of the token to burn.
   *     - `ids`: (For batch burn) An array of token IDs to burn.
   *     - `amounts`: (For batch burn) An array of token amounts corresponding to each ID.
   *   - `waitForTransactionProcessed`: (Optional) Whether to wait for the transaction to be finalized.
   * @param entrypoint - The name of the contract entrypoint to call (`"burn"` or `"batch_burn"`).
   *
   * @returns A `Promise` resolving to `TransactionResult`, which includes the transaction info and execution result (if available).
   *
   * @remarks
   * This method dynamically builds and executes a burn or batch burn operation using the specified contract entrypoint.
   * It supports both single and batch token burns by inspecting the provided `args`.
   */
  private queryBurn(params: BurnParams | BatchBurnParams, entrypoint: string) {
    const {
      params: { paymentAmount, sender, chainName, signingKeys },
      waitForTransactionProcessed,
      args,
    } = params;

    const runtimeArgs = RuntimeArgs.fromMap({
      owner: CLValue.newCLKey(CEP85Client.getPrefixedString(args.owner)),
    });

    if ('id' in args) {
      runtimeArgs.insert('id', CLValue.newCLUInt256(args.id));
    }
    if ('amount' in args) {
      runtimeArgs.insert('amount', CLValue.newCLUInt256(args.amount));
    }
    if ('ids' in args) {
      runtimeArgs.insert(
        'ids',
        CLValue.newCLList(CLTypeUInt256, args.ids.map(CLValue.newCLUInt256))
      );
    }
    if ('amounts' in args) {
      runtimeArgs.insert(
        'amounts',
        CLValue.newCLList(CLTypeUInt256, args.amounts.map(CLValue.newCLUInt256))
      );
    }

    return this.callEntrypoint(
      entrypoint,
      runtimeArgs,
      paymentAmount,
      sender,
      signingKeys,
      chainName,
      waitForTransactionProcessed
    );
  }

  /**
   * Burns a specific amount of a single token from an owner's account.
   *
   * @param params - Parameters for the burn operation, including:
   *   - `paymentAmount`: The payment amount required for the transaction.
   *   - `sender`: The public key of the account initiating the burn.
   *   - `chainName`: (Optional) The name of the Casper network.
   *   - `signingKeys`: (Optional) An array of private keys used to sign the transaction.
   *   - `args.owner`: The address that owns the token to be burned.
   *   - `args.id`: The ID of the token to be burned.
   *   - `args.amount`: The amount of the token to be burned.
   *   - `waitForTransactionProcessed`: (Optional) Whether to wait for transaction processing.
   *
   * @returns A `Promise` resolving to `TransactionResult`, containing transaction details and execution result (if available).
   *
   * @throws Will throw an error if the transaction fails or the WASM is not found.
   *
   * @remarks
   * This method initiates a burn operation for a single token by calling the contract's `burn` entrypoint.
   */
  public burn(params: BurnParams) {
    return this.queryBurn(params, 'burn');
  }

  /**
   * Burns multiple token types or multiple amounts from an owner's account in a single transaction.
   *
   * @param params - Parameters for the batch burn operation, including:
   *   - `paymentAmount`: The payment amount required for the transaction.
   *   - `sender`: The public key of the account initiating the burn.
   *   - `chainName`: (Optional) The name of the Casper network.
   *   - `signingKeys`: (Optional) An array of private keys used to sign the transaction.
   *   - `args.owner`: The address that owns the tokens to be burned.
   *   - `args.ids`: An array of token IDs to be burned.
   *   - `args.amounts`: An array of amounts corresponding to each token ID.
   *   - `waitForTransactionProcessed`: (Optional) Whether to wait for transaction processing.
   *
   * @returns A `Promise` resolving to `TransactionResult`, containing transaction details and execution result (if available).
   *
   * @throws Will throw an error if the transaction fails or the WASM is not found.
   *
   * @remarks
   * This method initiates a batch burn operation by calling the contract's `batch_burn` entrypoint.
   */
  public batchBurn(params: BatchBurnParams) {
    return this.queryBurn(params, 'batch_burn');
  }

  /**
   * Queries the on-chain balance of a specific token for a given account.
   *
   * @private
   * @param account - The account to query the balance for. Can be a public key or account hash.
   * @param id - The ID of the token to check balance for.
   * @returns A `Promise` resolving to the balance as a string. Returns `'0'` if the query fails.
   *
   * @remarks
   * This method uses the contract's `balances` dictionary to fetch the balance.
   */
  private async queryBalance(account: Entity, id: string): Promise<string> {
    const dictionaryItemKey = CEP85Client.makeDictionaryItemKey(
      CLValue.newCLKey(CEP85Client.getPrefixedString(account)),
      CLValue.newCLUInt256(id)
    );
    try {
      const result = await this.queryContractDictionary(
        'balances',
        dictionaryItemKey
      );
      return result as string;
    } catch (error) {
      // console.error(error);
      return '0';
    }
  }

  /**
   * Retrieves the balance of a specific token for a given account.
   *
   * @param account - The account to query the balance for.
   * @param id - The token ID to check.
   * @returns A `Promise` resolving to the token balance as a string.
   *
   * @remarks
   * This is a public wrapper around the internal `queryBalance` method.
   */
  public async balanceOf(account: Entity, id: string): Promise<string> {
    return this.queryBalance(account, id);
  }

  /**
   * Retrieves the balances of multiple token IDs for a single account.
   *
   * @param account - The account to query the balances for.
   * @param ids - An array of token IDs to check.
   * @returns A `Promise` resolving to an array of token balances as strings, in the same order as the IDs provided.
   *
   * @remarks
   * If a balance lookup fails for a given ID, it will default to `'0'`.
   */
  public async balanceOfBatch(
    account: Entity,
    ids: string[]
  ): Promise<string[]> {
    const result: string[] = [];
    try {
      const supplyPromises = ids.map(async (id) => {
        const resultSupply = await this.balanceOf(account, id);
        return resultSupply;
      });
      const supplyResults = await Promise.all(supplyPromises);
      result.push(...supplyResults);
    } catch (error) {
      console.error(error);
    }
    return result;
  }

  /**
   * Queries the circulating supply of a specific token.
   * @param id The token ID for which to retrieve the circulating supply.
   * @returns A promise that resolves to the circulating supply as a string. If an error occurs, "0" is returned.
   */
  private async querySupply(id: string): Promise<string> {
    try {
      const result = await this.queryContractDictionary('supply', id);
      return result as string;
    } catch (error) {
      // console.error(error);
      return '0';
    }
  }

  /**
   * Retrieves the circulating supply of a specific token.
   * @param id The token ID for which to retrieve the circulating supply.
   * @returns A promise that resolves to the circulating supply as a string. If an error occurs, "0" is returned.
   */
  public async getSupplyOf(id: string): Promise<string> {
    return this.querySupply(id);
  }

  /**
   * Retrieves the circulating supply of multiple tokens in batch.
   * @param ids An array of token IDs for which to retrieve the circulating supply.
   * @returns A promise that resolves to an array of circulating supplies as strings. If an error occurs, an empty array is returned.
   */
  public async getSupplyOfBatch(ids: string[]): Promise<string[]> {
    const supplyPromises = ids.map((id) => this.getSupplyOf(id));
    try {
      return await Promise.all(supplyPromises);
    } catch (error) {
      // console.error(error);
      return [];
    }
  }

  /**
   * Queries the total supply of a specific token from the contract's dictionary.
   * @param id The token ID for which to retrieve the total supply.
   * @returns A promise that resolves to the total supply as a string. If an error occurs, "0" is returned.
   */
  private async queryTotalSupply(id: string): Promise<string> {
    try {
      const result = await this.queryContractDictionary('total_supply', id);
      return result as string;
    } catch (error) {
      // console.error(error);
      return '0';
    }
  }

  /**
   * Retrieves the total supply of a specific token by querying the contract's dictionary.
   * @param id The token ID for which to retrieve the total supply.
   * @returns A promise that resolves to the total supply as a string. If an error occurs, "0" is returned.
   */
  public async getTotalSupplyOf(id: string): Promise<string> {
    return this.queryTotalSupply(id);
  }

  /**
   * Retrieves the total supply of multiple tokens in batch by querying the contract's dictionary.
   * @param ids An array of token IDs for which to retrieve the total supplies.
   * @returns A promise that resolves to an array of total supplies as strings. If an error occurs, an empty array is returned.
   */
  public async getTotalSupplyOfBatch(ids: string[]): Promise<string[]> {
    const supplyPromises = ids.map((id) => this.getTotalSupplyOf(id));
    try {
      return await Promise.all(supplyPromises);
    } catch (error) {
      // console.error(error);
      return [];
    }
  }

  /**
   * Queries the contract to set the total supply of a token or multiple tokens.
   *
   * @private
   * @param params - The parameters for setting the total supply, which includes:
   *   - `id`: The ID of the token for which the total supply is being set.
   *   - `totalSupply`: The total supply to set for the token.
   *   - `ids`: (Optional) An array of token IDs for batch updates.
   *   - `totalSupplies`: (Optional) An array of total supplies for batch updates.
   * @param entrypoint - The entrypoint to call in the contract (either `set_total_supply_of` or `set_total_supply_of_batch`).
   * @returns A `Promise` that resolves to the transaction result.
   *
   * @remarks
   * This method is used to set the total supply of one or more tokens. If both `id` and `totalSupply` are provided, the total supply of the specified token is updated.
   * For batch updates, arrays of `ids` and `totalSupplies` are required.
   */
  private querySetTotalSupplyOf(
    params: TotalSupplyOfParams | TotalSupplyOfBatchParams,
    entrypoint: string
  ) {
    const {
      params: { paymentAmount, sender, chainName, signingKeys },
      waitForTransactionProcessed,
      args,
    } = params;
    const runtimeArgs = RuntimeArgs.fromMap({});
    if ('id' in args) {
      runtimeArgs.insert('id', CLValue.newCLUInt256(args.id));
    }
    if ('totalSupply' in args) {
      runtimeArgs.insert(
        'total_supply',
        CLValue.newCLUInt256(args.totalSupply)
      );
    }
    if ('ids' in args) {
      runtimeArgs.insert(
        'ids',
        CLValue.newCLList(CLTypeUInt256, args.ids.map(CLValue.newCLUInt256))
      );
    }
    if ('totalSupplies' in args) {
      runtimeArgs.insert(
        'total_supplies',
        CLValue.newCLList(
          CLTypeUInt256,
          args.totalSupplies.map(CLValue.newCLUInt256)
        )
      );
    }

    return this.callEntrypoint(
      entrypoint,
      runtimeArgs,
      paymentAmount,
      sender,
      signingKeys,
      chainName,
      waitForTransactionProcessed
    );
  }

  /**
   * Sets the total supply of a single token.
   *
   * @param params - Parameters containing the token ID and the new total supply.
   * @returns A `Promise` resolving to the transaction result.
   *
   * @remarks
   * This is a public wrapper for the `querySetTotalSupplyOf` method, which sets the total supply for a single token.
   */
  public setTotalSupplyOf(params: TotalSupplyOfParams) {
    return this.querySetTotalSupplyOf(params, 'set_total_supply_of');
  }

  /**
   * Sets the total supply of multiple tokens in a single batch.
   *
   * @param params - Parameters containing an array of token IDs and their respective total supplies.
   * @returns A `Promise` resolving to the transaction result.
   *
   * @remarks
   * This is a public wrapper for the `querySetTotalSupplyOf` method, which allows updating the total supply of multiple tokens in one transaction.
   */
  public setTotalSupplyOfBatch(params: TotalSupplyOfBatchParams) {
    return this.querySetTotalSupplyOf(params, 'set_total_supply_of_batch');
  }

  /**
   * Checks whether a token with the specified ID is non-fungible by querying the contract's dictionary.
   * @param id The ID of the token to check for non-fungibility.
   * @returns A promise that resolves to a boolean indicating whether the token is non-fungible. If an error occurs, false is returned.
   */
  public async getIsNonFungible(id: string): Promise<boolean> {
    try {
      const result = await this.queryContractDictionary('total_supply', id);
      return (result as string) === '1';
    } catch (error) {
      // console.error(error);
      return false;
    }
  }

  /**
   * Retrieves the total fungible supply of a token by querying the contract's dictionaries.
   * @param id The ID of the token to get the total fungible supply for.
   * @returns A promise that resolves to the total fungible supply of the token. If an error occurs, 0 is returned.
   */
  public async getTotalFungibleSupply(id: string) {
    try {
      const resultSupply = await this.queryContractDictionary('supply', id);
      const currentSupply = BigInt(resultSupply as string);
      const resultTotalSupply = await this.queryContractDictionary(
        'total_supply',
        id
      );
      const totalSupply = BigInt(resultTotalSupply as string);

      return String(totalSupply - currentSupply);
    } catch (error) {
      // console.error(error);
      return '0';
    }
  }

  /**
   * Set approval for all tokens of the given owner.
   * @param args @see {@link SetApprovallForAllArgs}
   * @param paymentAmount Payment amount required for installing the contract.
   * @param deploySender Deploy sender's public key.
   * @param keys (Optional) Array of signing keys. Returns a signed deploy if keys are provided.
   * @returns Deploy object which can be sent to the node.
   */
  public setApprovalForAll(params: SetApprovallForAllParams) {
    const {
      params: { paymentAmount, sender, chainName, signingKeys },
      waitForTransactionProcessed,
      args: { approved, operator },
    } = params;

    const runtimeArgs = RuntimeArgs.fromMap({
      approved: CLValue.newCLValueBool(approved),
      operator: CLValue.newCLKey(CEP85Client.getPrefixedString(operator)),
    });

    return this.callEntrypoint(
      'set_approval_for_all',
      runtimeArgs,
      paymentAmount,
      sender,
      signingKeys,
      chainName,
      waitForTransactionProcessed
    );
  }

  /**
   * Checks whether the specified spender is approved for all tokens of the owner by querying the contract's dictionary.
   * @param owner The Entity representing the owner of the tokens.
   * @param spender The Entity representing the spender for whom approval is checked.
   * @returns A promise that resolves to a boolean indicating whether the spender is approved for all tokens of the owner.
   * If an error occurs during the query, false is returned.
   */
  public async getIsApprovedForAll(
    owner: Entity,
    spender: Entity
  ): Promise<boolean> {
    try {
      const result = await this.queryContractDictionary(
        'operators',
        CEP85Client.makeDictionaryItemKey(
          CLValue.newCLKey(CEP85Client.getPrefixedString(owner)),
          CLValue.newCLKey(CEP85Client.getPrefixedString(spender))
        )
      );
      return result as unknown as boolean;
    } catch {
      return false;
    }
  }

  /**
   * Changes the security settings of the contract by modifying role-based access control (RBAC) lists.
   *
   * @param params - The parameters for changing the security, which include:
   *   - `args.adminList` - (Optional) A list of addresses to be added to the admin role.
   *   - `args.minterList` - (Optional) A list of addresses to be added to the minter role.
   *   - `args.burnerList` - (Optional) A list of addresses to be added to the burner role.
   *   - `args.metaList` - (Optional) A list of addresses to be added to the meta role.
   *   - `args.noneList` - (Optional) A list of addresses to be added to a generic "none" role.
   *   - `params.sender` - The account sending the transaction.
   *   - `params.paymentAmount` - The payment amount for the transaction.
   *   - `params.signingKeys` - An array of signing keys for signing the transaction.
   *   - `params.chainName` - The name of the blockchain network.
   *   - `params.waitForTransactionProcessed` - Whether to wait for the transaction to be processed before returning.
   *
   * @returns A `Promise` that resolves to a `TransactionResult` object, which contains details about the transaction.
   *
   * @throws Error if no arguments (such as lists) are provided, or if the transaction fails.
   *
   * @remarks
   * This method allows the modification of various role-based access control lists for the contract, including:
   *   - `adminList`: Addresses that can manage the contract.
   *   - `minterList`: Addresses that can mint tokens.
   *   - `burnerList`: Addresses that can burn tokens.
   *   - `metaList`: Addresses with special meta permissions.
   *   - `noneList`: Addresses that are not assigned to any specific role.
   *
   * At least one list must be provided in the `args` for the transaction to be valid. The transaction will fail if no lists are provided.
   */
  public changeSecurity(
    params: ChangeSecurityParams
  ): Promise<TransactionResult> {
    const {
      args: { adminList, minterList, burnerList, metaList, noneList },
      params: { sender, paymentAmount, signingKeys, chainName },
      waitForTransactionProcessed,
    } = params;
    const runtimeArgs = RuntimeArgs.fromMap({});
    // Add optional args
    if (adminList) {
      runtimeArgs.insert(
        'admin_list',
        CLValue.newCLList(
          CLTypeKey,
          adminList.map((key) =>
            CLValue.newCLKey(CEP85Client.getPrefixedString(key))
          )
        )
      );
    }
    if (minterList) {
      runtimeArgs.insert(
        'minter_list',
        CLValue.newCLList(
          CLTypeKey,
          minterList.map((key) =>
            CLValue.newCLKey(CEP85Client.getPrefixedString(key))
          )
        )
      );
    }
    if (burnerList) {
      runtimeArgs.insert(
        'burner_list',
        CLValue.newCLList(
          CLTypeKey,
          burnerList.map((key) =>
            CLValue.newCLKey(CEP85Client.getPrefixedString(key))
          )
        )
      );
    }
    if (metaList) {
      runtimeArgs.insert(
        'meta_list',
        CLValue.newCLList(
          CLTypeKey,
          metaList.map((key) =>
            CLValue.newCLKey(CEP85Client.getPrefixedString(key))
          )
        )
      );
    }
    if (noneList) {
      runtimeArgs.insert(
        'none_list',
        CLValue.newCLList(
          CLTypeKey,
          noneList.map((key) =>
            CLValue.newCLKey(CEP85Client.getPrefixedString(key))
          )
        )
      );
    }

    // Check if at least one arg is provided and revert if none was provided
    if (runtimeArgs.args.size === 0) {
      throw new Error('Should provide at least one arg');
    }

    return this.callEntrypoint(
      'change_security',
      runtimeArgs,
      paymentAmount,
      sender,
      signingKeys,
      chainName,
      waitForTransactionProcessed
    );
  }

  /**
   * Retrieves the current events mode for the contract.
   *
   * @returns A `Promise` that resolves to a string value representing the current events mode.
   *   The value will be one of the keys from the `EVENTS_MODE` enum.
   *
   * @throws Error if there is an issue retrieving the events mode from the contract.
   *
   * @remarks
   * This method queries the contract for the current events mode, which determines the type of events
   * that the contract is configured to generate. The returned value corresponds to a key from the `EVENTS_MODE` enum.
   */
  public async eventsMode(): Promise<keyof typeof EVENTS_MODE> {
    const internalValue = (await this.queryContractData([
      'events_mode',
    ])) as string;

    return EVENTS_MODE[internalValue] as keyof typeof EVENTS_MODE;
  }

  /**
   * Checks whether the burn feature is enabled on the contract.
   *
   * @returns A `Promise` that resolves to a boolean indicating whether burning is enabled (`true`) or disabled (`false`).
   *
   * @throws Error if there is an issue retrieving the burn mode from the contract.
   *
   * @remarks
   * This method queries the contract to determine whether the burn functionality is enabled. If the
   * contract allows burning of tokens, it will return `true`; otherwise, it returns `false`.
   */
  public async burnMode(): Promise<boolean> {
    const internalValue = await this.queryContractData(['enable_burn']);
    return internalValue === 'true';
  }

  /**
   * Returns the number of minted tokens.
   *
   * @returns A `Promise` that resolves to the number of minted tokens.
   *
   * @remarks This method queries the `number_of_minted_tokens` field from the contract.
   */
  public async numOfMintedTokens() {
    return this.queryContractData(['number_of_minted_tokens']);
  }

  /**
   * Sets modalities by calling the "set_modalities" entrypoint on the contract.
   * @param args - The arguments for setting modalities. @see {@link SetModalitiesArgs}
   * @param paymentAmount - The payment amount in string format.
   * @param deploySender - The deploy sender's public key.
   * @param keys - Optional asymmetric keys for the deployment.
   * @returns The prepared deploy for setting modalities.
   */
  public setModalities(params: SetModalitiesParams) {
    const {
      params: { paymentAmount, sender, chainName, signingKeys },
      waitForTransactionProcessed,
      args: { enableBurn, eventsMode },
    } = params;

    const runtimeArgs = RuntimeArgs.fromMap({});
    if (enableBurn !== undefined) {
      runtimeArgs.insert('enable_burn', CLValue.newCLValueBool(enableBurn));
    }
    if (eventsMode !== undefined) {
      runtimeArgs.insert('events_mode', CLValue.newCLUint8(eventsMode));
    }
    return this.callEntrypoint(
      'set_modalities',
      runtimeArgs,
      paymentAmount,
      sender,
      signingKeys,
      chainName,
      waitForTransactionProcessed
    );
  }

  // ! TODO toPrefixedString() ?
  // Error: prefix is not found, source: contract-0x, see Key.newKey()
  private static getPrefixedString(entity: Entity): Key {
    if (entity instanceof PublicKey) {
      return Key.newKey(
        entity
          .accountHash()
          .toPrefixedString()
          .replace('account-hash-', 'entity-account-')
      );
    }
    if (
      entity instanceof ContractHash ||
      entity instanceof ContractPackageHash
    ) {
      return Key.newKey(`entity-contract-${entity.hash.toHex()}`);
    }
    return Key.newKey((entity as AddressableEntityHash).toPrefixedString());
  }
}
