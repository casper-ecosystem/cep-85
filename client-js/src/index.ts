import {
  CLPublicKey,
  CLKey,
  RuntimeArgs,
  CasperClient,
  Contracts,
  Keys,
  CLValueBuilder,
  CLValue,
  CLByteArrayType,
  encodeBase16,
  CLAccountHash,
  CLU256,
  CLValueParsers,
  CLBool
} from "casper-js-sdk";
import { BigNumber } from "@ethersproject/bignumber";
import { blake2b } from '@noble/hashes/blake2b';
import { None } from "ts-results";
import {
  InstallArgs,
  MintArgs,
  BurnArgs,
  SetApprovallForAllArgs,
  TransferArgs,
  SetUriArgs,
  TotalSupplyOfArgs,
  EventsMode,
  ChangeSecurityArgs,
  BatchMintArgs,
  BatchBurnArgs,
  TotalSupplyOfArgsBatch,
  BatchTransferArgs,
} from "./types";
import ContractBinary from "../wasm/cep85.wasm";

export * from "./types";
export * from "./events";


const { Contract } = Contracts;

/**
 * Converts a hash string to a Buffer.
 * @param hashStr The input hash string to be converted.
 * @returns A Buffer containing the hexadecimal representation of the input hash string.
 */
const convertHashStringToBuffer = (hashStr: string): Buffer => {
  const hashHex = hashStr.startsWith("hash-") ? hashStr.slice(5) : hashStr;
  return Buffer.from(hashHex, "hex");
};

export class CEP85Client {
  private casperClient: CasperClient;

  public contractClient: Contracts.Contract;

  public contractHashKey: CLKey;

  /**
 * Constructs a new instance of the SmartContractService.
 * @param nodeAddress The address of the Casper node to connect to.
 * @param networkName The name of the network (e.g., 'mainnet', 'testnet').
 */
  constructor(public nodeAddress: string, public networkName: string) {
    this.casperClient = new CasperClient(nodeAddress);
    this.contractClient = new Contract(this.casperClient);
  }

  /**
 * Install a smart contract.
 * @param args Arguments for installing the smart contract. See {@link InstallArgs}.
 * @param paymentAmount Payment amount required for installing the contract.
 * @param deploySender Deploy sender's public key.
 * @param keys (Optional) Array of signing keys. Returns a signed deploy if keys are provided.
 * @param wasm (Optional) Wasm code for the smart contract.
 */
  public install(
    args: InstallArgs,
    paymentAmount: string,
    deploySender: CLPublicKey,
    keys?: Keys.AsymmetricKey[],
    wasm?: Uint8Array
  ) {
    const wasmToInstall = wasm || ContractBinary;

    const runtimeArgs = RuntimeArgs.fromMap({
      name: CLValueBuilder.string(args.name),
      uri: CLValueBuilder.string(args.uri),
      events_mode: CLValueBuilder.u8(args.eventsMode),
      enable_burn: CLValueBuilder.u8(1),
    });

    if (args.burner_list) {
      runtimeArgs.insert("burner_list", CLValueBuilder.list(args.burner_list.map(burner => CLValueBuilder.key(burner))));
    }

    return this.contractClient.install(
      wasmToInstall,
      runtimeArgs,
      paymentAmount,
      deploySender,
      this.networkName,
      keys || []
    );
  }

  /**
 * Sets the contract hash and contract package hash for the SmartContractService.
 * @param contractHash The hash of the smart contract.
 * @param contractPackageHash (Optional) The hash of the contract package if applicable.
 */
  public setContractHash(contractHash: string, contractPackageHash?: string) {
    this.contractClient.setContractHash(contractHash, contractPackageHash);
    this.contractHashKey = CLValueBuilder.key(
      CLValueBuilder.byteArray(convertHashStringToBuffer(contractHash))
    );
  }

  /**
 * Retrieves the name of the collection from the smart contract.
 * @returns A Promise that resolves to the collection name.
 */
  public async collectionName() {
    try {
      const result = await this.contractClient.queryContractData(["name"]) as CLValue;
      return result.toJSON() as string;
    } catch (error) {
      console.error(error);
      return '';
    }
  }

  /**
 * Retrieves the URI of the collection from the smart contract.
 * @returns A Promise that resolves to the collection URI.
 */
  public async collectionUri() {
    try {
      const result = await this.contractClient.queryContractData(["uri"]) as CLValue;
      return result.toJSON() as string;
    } catch (error) {
      console.error(error);
      return '';
    }
  }

  /**
 * Constructs a dictionary item key by concatenating and hashing the bytes of the provided CLKey and CLValue.
 * @param key The CLKey for the dictionary item.
 * @param value The CLValue for the dictionary item.
 * @returns The resulting dictionary item key as a hexadecimal string.
 */
  public static makeDictionaryItemKey(key: CLKey, value: CLValue): string {
    const bytesA = new Uint8Array(CLValueParsers.toBytes(key));
    const bytesB = new Uint8Array(CLValueParsers.toBytes(value));

    const concatenatedBytes: Uint8Array = new Uint8Array(bytesA.length + bytesB.length);
    concatenatedBytes.set(bytesA);
    concatenatedBytes.set(bytesB, bytesA.length);

    const hashedBytes: Uint8Array = blake2b(concatenatedBytes, {
      dkLen: 32
    });
    const result: string = encodeBase16(hashedBytes);
    return result;
  }

  /**
 * Setting the URI for tokens.
 * @param args Arguments for setting URI. See {@link SetUriArgs}.
 * @param paymentAmount Payment amount required for setting the URI.
 * @param deploySender Deploy sender's public key.
 * @param keys (Optional) Array of signing keys. Returns a signed deploy if keys are provided.
 */
  public setUri(
    args: SetUriArgs,
    paymentAmount: string,
    deploySender: CLPublicKey,
    keys?: Keys.AsymmetricKey[],
  ) {

    const runtimeArgs = RuntimeArgs.fromMap({
      uri: CLValueBuilder.string(args.uri),
    });

    if ('id' in args) {
      runtimeArgs.insert('id', CLValueBuilder.u256(args.id));
    }

    const preparedDeploy = this.contractClient.callEntrypoint(
      "set_uri",
      runtimeArgs,
      deploySender,
      this.networkName,
      paymentAmount,
      keys
    );

    return preparedDeploy;
  }

  /**
 * Retrieves the URI associated with a specific token ID or the default URI if no ID is provided.
 * @param id The optional token ID for which to retrieve the URI.
 * @returns The URI as a string
 */
  public async getURI(id?: string) {
    try {
      let result: string;
      if (id) {
        const res = await this.contractClient.queryContractDictionary(
          'token_uri',
          id
        );
        result = res.toJSON() as string;
      }
      else {
        result = await this.collectionUri();
      }
      return result && result.replace('{id}', id);
    } catch (error) {
      // console.error(error);
      return '';
    }
  }

  /**
 * Querying minting of tokens.
 * @param args Arguments for minting. See {@link MintArgs} or {@link BatchMintArgs}.
 * @param entrypoint Entrypoint for querying minting.
 * @param paymentAmount Payment amount required for the query.
 * @param deploySender Deploy sender's public key.
 * @param keys (Optional) Array of signing keys. Returns a signed deploy if keys are provided.
 */
  private queryMint(
    args: MintArgs | BatchMintArgs,
    entrypoint: string,
    paymentAmount: string,
    deploySender: CLPublicKey,
    keys?: Keys.AsymmetricKey[],
  ) {
    const commonArgs = {
      recipient: CLValueBuilder.key(args.recipient),
    };

    const runtimeArgs = RuntimeArgs.fromMap(commonArgs);

    if ('id' in args) {
      runtimeArgs.insert('id', CLValueBuilder.u256(args.id));
    }
    if ('amount' in args) {
      runtimeArgs.insert('amount', CLValueBuilder.u256(args.amount));
    }
    if ('ids' in args) {
      runtimeArgs.insert('ids', CLValueBuilder.list(args.ids.map(CLValueBuilder.u256)));
    }
    if ('amounts' in args) {
      runtimeArgs.insert('amounts', CLValueBuilder.list(args.amounts.map(CLValueBuilder.u256)));
    }

    const preparedDeploy = this.contractClient.callEntrypoint(
      entrypoint,
      runtimeArgs,
      deploySender,
      this.networkName,
      paymentAmount,
      keys
    );

    return preparedDeploy;
  }

  /**
 * Minting of tokens.
 * @param args Arguments for minting. See {@link MintArgs}.
 * @param paymentAmount Payment amount required for minting the tokens.
 * @param deploySender Deploy sender's public key.
 * @param keys (Optional) Array of signing keys. Returns a signed deploy if keys are provided.
 */
  public mint(
    args: MintArgs,
    paymentAmount: string,
    deploySender: CLPublicKey,
    keys?: Keys.AsymmetricKey[],
  ) {
    return this.queryMint(args, "mint", paymentAmount, deploySender, keys);
  }

  /**
 * Batch minting of tokens.
 * @param args Arguments for the batch minting. See {@link BatchMintArgs}.
 * @param paymentAmount Payment amount required for minting the tokens.
 * @param deploySender Deploy sender's public key.
 * @param keys (Optional) Array of signing keys. Returns a signed deploy if keys are provided.
 */
  public batchMint(
    args: BatchMintArgs,
    paymentAmount: string,
    deploySender: CLPublicKey,
    keys?: Keys.AsymmetricKey[],
  ) {
    return this.queryMint(args, "batch_mint", paymentAmount, deploySender, keys);
  }

  /**
 * Query transfer of tokens.
 * @param args Arguments for the transfer. See {@link TransferArgs} or {@link BatchTransferArgs}.
 * @param entrypoint Entrypoint for the transfer query.
 * @param paymentAmount Payment amount required for querying the transfer.
 * @param deploySender Deploy sender's public key.
 * @param keys (Optional) Array of signing keys. Returns a signed deploy if keys are provided.
 */
  private queryTransfer(
    args: TransferArgs | BatchTransferArgs,
    entrypoint: string,
    paymentAmount: string,
    deploySender: CLPublicKey,
    keys?: Keys.AsymmetricKey[],
  ) {
    const commonArgs = {
      from: CLValueBuilder.key(args.from),
      to: CLValueBuilder.key(args.to),
    };

    const runtimeArgs = RuntimeArgs.fromMap(commonArgs);

    if ('id' in args) {
      runtimeArgs.insert('id', CLValueBuilder.u256(args.id));
    }
    if ('amount' in args) {
      runtimeArgs.insert('amount', CLValueBuilder.u256(args.amount));
    }
    if ('ids' in args) {
      runtimeArgs.insert('ids', CLValueBuilder.list(args.ids.map(CLValueBuilder.u256)));
    }
    if ('amounts' in args) {
      runtimeArgs.insert('amounts', CLValueBuilder.list(args.amounts.map(CLValueBuilder.u256)));
    }

    if ('data' in args && args.data !== undefined) {
      runtimeArgs.insert('data', CLValueBuilder.byteArray(args.data));
    }

    const preparedDeploy = this.contractClient.callEntrypoint(
      entrypoint,
      runtimeArgs,
      deploySender,
      this.networkName,
      paymentAmount,
      keys
    );

    return preparedDeploy;
  }

  /**
 * Perform a transfer of tokens.
 * @param args Arguments for the transfer. See {@link TransferArgs}.
 * @param paymentAmount Payment amount required for performing the transfer.
 * @param deploySender Deploy sender's public key.
 * @param keys (Optional) Array of signing keys. Returns a signed deploy if keys are provided.
 */
  public transfer(
    args: TransferArgs,
    paymentAmount: string,
    deploySender: CLPublicKey,
    keys?: Keys.AsymmetricKey[],
  ) {
    return this.queryTransfer(args, "safe_transfer_from", paymentAmount, deploySender, keys);
  }

  /**
 * Perform a batch transfer of tokens.
 * @param args Arguments for the batch transfer. See {@link BatchTransferArgs}.
 * @param paymentAmount Payment amount required for performing the batch transfer.
 * @param deploySender Deploy sender's public key.
 * @param keys (Optional) Array of signing keys. Returns a signed deploy if keys are provided.
 */
  public batchTransfer(
    args: BatchTransferArgs,
    paymentAmount: string,
    deploySender: CLPublicKey,
    keys?: Keys.AsymmetricKey[],
  ) {
    return this.queryTransfer(args, "safe_batch_transfer_from", paymentAmount, deploySender, keys);
  }

  /**
 * Query the burning of tokens.
 * @param args Arguments for burning. See {@link BurnArgs} or {@link BatchBurnArgs}.
 * @param entrypoint Entrypoint for the query.
 * @param paymentAmount Payment amount required for the query.
 * @param deploySender Deploy sender's public key.
 * @param keys (Optional) Array of signing keys. Returns a signed deploy if keys are provided.
 * @returns Result of the query.
 */
  private queryBurn(
    args: BurnArgs | BatchBurnArgs,
    entrypoint: string,
    paymentAmount: string,
    deploySender: CLPublicKey,
    keys?: Keys.AsymmetricKey[]
  ) {
    const commonArgs = {
      owner: CLValueBuilder.key(args.owner),
    };

    const runtimeArgs = RuntimeArgs.fromMap(commonArgs);

    if ('id' in args) {
      runtimeArgs.insert('id', CLValueBuilder.u256(args.id));
    }
    if ('amount' in args) {
      runtimeArgs.insert('amount', CLValueBuilder.u256(args.amount));
    }
    if ('ids' in args) {
      runtimeArgs.insert('ids', CLValueBuilder.list(args.ids.map(CLValueBuilder.u256)));
    }
    if ('amounts' in args) {
      runtimeArgs.insert('amounts', CLValueBuilder.list(args.amounts.map(CLValueBuilder.u256)));
    }

    const preparedDeploy = this.contractClient.callEntrypoint(
      entrypoint,
      runtimeArgs,
      deploySender,
      this.networkName,
      paymentAmount,
      keys
    );

    return preparedDeploy;
  }

  /**
 * Perform burning of tokens.
 * @param args Arguments for burning. See {@link BurnArgs}.
 * @param paymentAmount Payment amount required for installing the contract.
 * @param deploySender Deploy sender's public key.
 * @param keys (Optional) Array of signing keys. Returns a signed deploy if keys are provided.
 * @returns Deploy object which can be sent to the node.
 */
  public burn(
    args: BurnArgs,
    paymentAmount: string,
    deploySender: CLPublicKey,
    keys?: Keys.AsymmetricKey[]
  ) {
    return this.queryBurn(args, "burn", paymentAmount, deploySender, keys);
  }

  /**
 * Perform batch burning of tokens.
 * @param args Arguments for batch burning. See {@link BatchBurnArgs}.
 * @param paymentAmount Payment amount required for installing the contract.
 * @param deploySender Deploy sender's public key.
 * @param keys (Optional) Array of signing keys. Returns a signed deploy if keys are provided.
 * @returns Deploy object which can be sent to the node.
 */
  public batchBurn(
    args: BatchBurnArgs,
    paymentAmount: string,
    deploySender: CLPublicKey,
    keys?: Keys.AsymmetricKey[]
  ) {
    return this.queryBurn(args, "batch_burn", paymentAmount, deploySender, keys);
  }

  /**
 * Queries the contract dictionary to retrieve the balance associated with a specific account and token ID.
 * @param account The public key of the account for which to query the balance.
 * @param id The token ID for which to retrieve the balance.
 * @returns The balance as a string, or "0" if an error occurs or the balance is not found.
 */
  private async queryBalance(account: CLPublicKey, id: string): Promise<string> {
    const accountHash = new CLAccountHash(account.toAccountHash());
    const key = new CLKey(accountHash);
    const dictionaryItemKey = CEP85Client.makeDictionaryItemKey(key, new CLU256(id));
    try {
      const result = await this.contractClient.queryContractDictionary(
        "balances",
        dictionaryItemKey
      );
      return (result as CLU256).toJSON();
    } catch (error) {
      // console.error(error);
      return "0";
    }
  }

  /**
 * Retrieves the balance of a specific token for a given account.
 * @param account The public key of the account for which to retrieve the balance.
 * @param id The token ID for which to retrieve the balance.
 * @returns A promise that resolves to the balance as a string. If an error occurs, "0" is returned.
 */
  public async getBalanceOf(account: CLPublicKey, id: string): Promise<string> {
    return this.queryBalance(account, id);
  }

  /**
 * Retrieves the balances of multiple tokens for a given account.
 * @param account The public key of the account for which to retrieve the balances.
 * @param ids An array of token IDs for which to retrieve the balances.
 * @returns A promise that resolves to an array of balances as strings. If an error occurs, an empty array is returned.
 */
  public async getBalanceOfBatch(account: CLPublicKey, ids: string[]): Promise<string[]> {
    const result: string[] = [];
    try {
      const supplyPromises = ids.map(async (id) => {
        const resultSupply = await this.getBalanceOf(account, id);
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
 * Queries the total supply of a specific token.
 * @param id The token ID for which to retrieve the total supply.
 * @returns A promise that resolves to the total supply as a string. If an error occurs, "0" is returned.
 */
  private async querySupply(id: string): Promise<string> {
    try {
      const result = await this.contractClient.queryContractDictionary(
        "supply",
        id
      );
      return (result as CLU256).toJSON();
    } catch (error) {
      // console.error(error);
      return "0";
    }
  }

  /**
 * Retrieves the total supply of a specific token.
 * @param id The token ID for which to retrieve the total supply.
 * @returns A promise that resolves to the total supply as a string. If an error occurs, "0" is returned.
 */
  public async getSupplyOf(id: string): Promise<string> {
    return this.querySupply(id);
  }

  /**
 * Retrieves the total supply of multiple tokens in batch.
 * @param ids An array of token IDs for which to retrieve the total supply.
 * @returns A promise that resolves to an array of total supplies as strings. If an error occurs, an empty array is returned.
 */
  public async getSupplyOfBatch(ids: string[]): Promise<string[]> {
    const supplyPromises = ids.map((id) => this.getSupplyOf(id));
    try {
      return await Promise.all(supplyPromises);
    } catch (error) {
      console.error(error);
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
      const result = await this.contractClient.queryContractDictionary(
        "total_supply",
        id
      );
      return (result as CLU256).toJSON();
    } catch (error) {
      // console.error(error);
      return "0";
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
      console.error(error);
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
    args: TotalSupplyOfArgs | TotalSupplyOfArgsBatch,
    entrypoint: string,
    paymentAmount: string,
    deploySender: CLPublicKey,
    keys?: Keys.AsymmetricKey[],
  ) {
    const runtimeArgs = RuntimeArgs.fromMap({});
    if ('id' in args) {
      runtimeArgs.insert('id', CLValueBuilder.u256(args.id));
    }
    if ('total_supply' in args) {
      runtimeArgs.insert('total_supply', CLValueBuilder.u256(args.total_supply));
    }
    if ('ids' in args) {
      runtimeArgs.insert('ids', CLValueBuilder.list(args.ids.map(CLValueBuilder.u256)));
    }
    if ('total_supplies' in args) {
      runtimeArgs.insert('total_supplies', CLValueBuilder.list(args.total_supplies.map(CLValueBuilder.u256)));
    }

    const preparedDeploy = this.contractClient.callEntrypoint(
      entrypoint,
      runtimeArgs,
      deploySender,
      this.networkName,
      paymentAmount,
      keys
    );

    return preparedDeploy;
  }

  /**
 * Set the total supply for a specific token.
 * @param args @see {@link TotalSupplyOfArgs}
 * @param paymentAmount Payment amount required for installing the contract.
 * @param deploySender Deploy sender's public key.
 * @param keys (Optional) Array of signing keys. Returns a signed deploy if keys are provided.
 * @returns Deploy object which can be sent to the node.
 */
  public setTotalSupplyOf(
    args: TotalSupplyOfArgs,
    paymentAmount: string,
    deploySender: CLPublicKey,
    keys?: Keys.AsymmetricKey[],
  ) {
    return this.querySetTotalSupplyOf(args, "set_total_supply_of", paymentAmount, deploySender, keys);
  }

  /**
 * Set the total supply for a batch of tokens.
 * @param args @see {@link TotalSupplyOfArgsBatch}
 * @param paymentAmount Payment amount required for installing the contract.
 * @param deploySender Deploy sender's public key.
 * @param keys (Optional) Array of signing keys. Returns a signed deploy if keys are provided.
 * @returns Deploy object which can be sent to the node.
 */
  public setTotalSupplyOfBatch(
    args: TotalSupplyOfArgsBatch,
    paymentAmount: string,
    deploySender: CLPublicKey,
    keys?: Keys.AsymmetricKey[],
  ) {
    return this.querySetTotalSupplyOf(args, "set_total_supply_of_batch", paymentAmount, deploySender, keys);
  }

  /**
 * Checks whether a token with the specified ID is non-fungible by querying the contract's dictionary.
 * @param id The ID of the token to check for non-fungibility.
 * @returns A promise that resolves to a boolean indicating whether the token is non-fungible. If an error occurs, false is returned.
 */
  public async getIsNonFungible(id: string): Promise<boolean> {
    try {
      const result = await this.contractClient.queryContractDictionary(
        "total_supply",
        id
      );
      return (result as CLU256).toJSON() === '1';
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
      const resultSupply = await this.contractClient.queryContractDictionary(
        "supply",
        id
      );
      const currentSupply = BigNumber.from((resultSupply as CLU256).toJSON());
      const resultTotalSupply = await this.contractClient.queryContractDictionary(
        "total_supply",
        id
      );
      const totalSupply = BigNumber.from((resultTotalSupply as CLU256).toJSON());

      return totalSupply.sub(currentSupply).toString();
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
  public setApprovalForAll(
    args: SetApprovallForAllArgs,
    paymentAmount: string,
    deploySender: CLPublicKey,
    keys?: Keys.AsymmetricKey[]
  ) {
    const runtimeArgs = RuntimeArgs.fromMap({
      approved: CLValueBuilder.bool(args.approved),
      operator: CLValueBuilder.key(args.operator),
    });

    const preparedDeploy = this.contractClient.callEntrypoint(
      "set_approval_for_all",
      runtimeArgs,
      deploySender,
      this.networkName,
      paymentAmount,
      keys
    );

    return preparedDeploy;
  }

  /**
 * Checks whether the specified spender is approved for all tokens of the owner by querying the contract's dictionary.
 * @param owner The CLPublicKey representing the owner of the tokens.
 * @param spender The CLPublicKey representing the spender for whom approval is checked.
 * @returns A promise that resolves to a boolean indicating whether the spender is approved for all tokens of the owner.
 * If an error occurs during the query, false is returned.
 */
  public async getIsApprovedForAll(owner: CLPublicKey, spender: CLPublicKey): Promise<boolean> {
    try {
      const result = await this.contractClient.queryContractDictionary(
        "operators",
        CEP85Client.makeDictionaryItemKey(CLValueBuilder.key(owner), CLValueBuilder.key(spender))
      );
      return (result as CLBool).toJSON() as boolean;
    } catch {
      return false;
    }
  }

  /**
   * Change token security
   * @param args @see {@link ChangeSecurityArgs}
   * @param paymentAmount payment amount required for installing the contract
   * @param sender deploy sender
   * @param keys array of signing keys optional, returns signed deploy if keys are provided
   * @returns Deploy object which can be send to the node.
   */
  public changeSecurity(
    args: ChangeSecurityArgs,
    paymentAmount: string,
    sender: CLPublicKey,
    keys?: Keys.AsymmetricKey[]
  ) {
    const runtimeArgs = RuntimeArgs.fromMap({});

    if (args.admin_list) {
      runtimeArgs.insert(
        'admin_list',
        CLValueBuilder.list(args.admin_list.map(CLValueBuilder.key))
      );
    }
    if (args.minter_list) {
      runtimeArgs.insert(
        'minter_list',
        CLValueBuilder.list(args.minter_list.map(CLValueBuilder.key))
      );
    }
    if (args.burner_list) {
      runtimeArgs.insert(
        'burner_list',
        CLValueBuilder.list(args.burner_list.map(CLValueBuilder.key))
      );
    }
    if (args.none_list) {
      runtimeArgs.insert(
        'none_list',
        CLValueBuilder.list(args.none_list.map(CLValueBuilder.key))
      );
    }

    // Check if at least one arg is provided and revert if none was provided
    if (runtimeArgs.args.size === 0) {
      throw new Error('Should provide at least one arg');
    }

    return this.contractClient.callEntrypoint(
      'change_security',
      runtimeArgs,
      sender,
      this.networkName,
      paymentAmount,
      keys
    );
  }

  /**
 * Retrieves the events mode from the contract by querying the contract's data.
 * @returns A promise that resolves to a string representing the events mode.
 * The events mode is converted from its internal numerical representation to its corresponding string value.
 */
  public async getEventsMode(): Promise<keyof typeof EventsMode> {
    const internalValue = (await this.contractClient.queryContractData([
      'events_mode'
    ])) as BigNumber;
    const u8res = internalValue.toNumber();
    return EventsMode[u8res] as keyof typeof EventsMode;
  }

}
