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
   * @param key The CLKey for the dictionary item.
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

  public mint(params: MintParams) {
    return this.queryMint(params, 'mint');
  }

  public batchMint(params: BatchMintParams) {
    return this.queryMint(params, 'batch_mint');
  }

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

  public transfer(params: TransferParams) {
    return this.queryTransfer(params, 'transfer_from');
  }

  public batchTransfer(params: BatchTransferParams) {
    return this.queryTransfer(params, 'batch_transfer_from');
  }

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

  public burn(params: BurnParams) {
    return this.queryBurn(params, 'burn');
  }

  public batchBurn(params: BatchBurnParams) {
    return this.queryBurn(params, 'batch_burn');
  }

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

  public async balanceOf(account: Entity, id: string): Promise<string> {
    return this.queryBalance(account, id);
  }

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
   * Queries the data for setting the total supply, either for a single token or a batch of tokens.
   * @param args Arguments for setting the total supply. It can be of type {@link TotalSupplyOfArgs} or {@link TotalSupplyOfArgsBatch}.
   * @param entrypoint The entry point for setting the total supply.
   * @param paymentAmount Payment amount required for installing the contract.
   * @param deploySender Deploy sender's public key.
   * @param keys (Optional) Array of signing keys. Returns a signed deploy if keys are provided.
   * @returns Deploy object which can be sent to the node.
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

  public setTotalSupplyOf(params: TotalSupplyOfParams) {
    return this.querySetTotalSupplyOf(params, 'set_total_supply_of');
  }

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

  public async eventsMode(): Promise<keyof typeof EVENTS_MODE> {
    const internalValue = (await this.queryContractData([
      'events_mode',
    ])) as string;

    return EVENTS_MODE[internalValue] as keyof typeof EVENTS_MODE;
  }

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
