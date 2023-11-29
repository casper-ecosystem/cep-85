import { BigNumber } from "@ethersproject/bignumber";
import {
  CLMap,
  CLString,
  CLPublicKey,
  CLKey,
  RuntimeArgs,
  CasperClient,
  Contracts,
  Keys,
  CLValueBuilder,
  CLU8,
  OPTION_TYPE,
  CLValue,
  CLKeyType,
  CLByteArrayType,
} from "casper-js-sdk";

import { None } from "ts-results";

import {
  CallConfig,
  InstallArgs,
  ConfigurableVariables,
  MintArgs,
  BurnArgs,
  ApproveArgs,
  ApproveAllArgs,
  TransferArgs,
  TokenMetadataArgs,
  StoreBalanceOfArgs,
} from "./types";

import ContractBinary from "../wasm/cep85.wasm";

const { Contract } = Contracts;

export * from "./types";
export * from "./events";

enum ERRORS {
  CONFLICT_CONFIG = "Conflicting arguments provided",
}

const convertHashStrToHashBuff = (hashStr: string) => {
  let hashHex = hashStr;
  if (hashStr.startsWith("hash-")) {
    hashHex = hashStr.slice(5);
  }
  return Buffer.from(hashHex, "hex");
};

const buildHashList = (list: string[]) =>
  list.map((hashStr) =>
    CLValueBuilder.byteArray(convertHashStrToHashBuff(hashStr))
  );

export class CEP85Client {
  private casperClient: CasperClient;

  public contractClient: Contracts.Contract;

  public contractHashKey: CLKey;

  constructor(public nodeAddress: string, public networkName: string) {
    this.casperClient = new CasperClient(nodeAddress);
    this.contractClient = new Contract(this.casperClient);
  }

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

  public setContractHash(contractHash: string, contractPackageHash?: string) {
    this.contractClient.setContractHash(contractHash, contractPackageHash);
    this.contractHashKey = CLValueBuilder.key(
      CLValueBuilder.byteArray(convertHashStrToHashBuff(contractHash))
    );
  }

  public async collectionName() {
    return this.contractClient.queryContractData(["collection_name"]);
  }

  public async collectionSymbol() {
    return this.contractClient.queryContractData(["collection_symbol"]);
  }

  public async tokenTotalSupply() {
    return this.contractClient.queryContractData(["total_token_supply"]);
  }

  public setVariables(
    args: ConfigurableVariables,
    paymentAmount: string,
    deploySender: CLPublicKey,
    keys?: Keys.AsymmetricKey[]
  ) {
    const runtimeArgs = RuntimeArgs.fromMap({});

    if (args.allowMinting !== undefined) {
      runtimeArgs.insert(
        "allow_minting",
        CLValueBuilder.bool(args.allowMinting)
      );
    }

    if (args.contractWhitelist !== undefined) {
      const list = buildHashList(args.contractWhitelist);
      runtimeArgs.insert("contract_whitelist", CLValueBuilder.list(list));
    }

    const preparedDeploy = this.contractClient.callEntrypoint(
      "set_variables",
      runtimeArgs,
      deploySender,
      this.networkName,
      paymentAmount,
      keys
    );

    return preparedDeploy;
  }

  public revoke(
    paymentAmount: string,
    deploySender: CLPublicKey,
    keys?: Keys.AsymmetricKey[]
  ) {
    const preparedDeploy = this.contractClient.callEntrypoint(
      "revoke",
      RuntimeArgs.fromMap({}),
      deploySender,
      this.networkName,
      paymentAmount,
      keys
    );

    return preparedDeploy;
  }

  public mint(
    args: MintArgs,
    config: CallConfig,
    paymentAmount: string,
    deploySender: CLPublicKey,
    keys?: Keys.AsymmetricKey[],
    wasm?: Uint8Array
  ) {
    if (config.useSessionCode === false && !!wasm)
      throw new Error(ERRORS.CONFLICT_CONFIG);

    const runtimeArgs = RuntimeArgs.fromMap({
      recipient: CLValueBuilder.key(args.recipient),
      id: CLValueBuilder.u256(args.id),
      amount: CLValueBuilder.u256(args.amount),
    });

    const preparedDeploy = this.contractClient.callEntrypoint(
      "mint",
      runtimeArgs,
      deploySender,
      this.networkName,
      paymentAmount,
      keys
    );

    return preparedDeploy;
  }

  public burn(
    args: BurnArgs,
    paymentAmount: string,
    deploySender: CLPublicKey,
    keys?: Keys.AsymmetricKey[]
  ) {
    const runtimeArgs = RuntimeArgs.fromMap({
      owner: CLValueBuilder.key(args.owner),
      id: CLValueBuilder.u256(args.id),
      amount: CLValueBuilder.u256(args.amount),
    });

    const preparedDeploy = this.contractClient.callEntrypoint(
      "burn",
      runtimeArgs,
      deploySender,
      this.networkName,
      paymentAmount,
      keys
    );

    return preparedDeploy;
  }

  public transfer(
    args: TransferArgs,
    config: CallConfig,
    paymentAmount: string,
    deploySender: CLPublicKey,
    keys?: Keys.AsymmetricKey[],
    wasm?: Uint8Array
  ) {
    if (config.useSessionCode === false && !!wasm)
      throw new Error(ERRORS.CONFLICT_CONFIG);

    const runtimeArgs = RuntimeArgs.fromMap({
      from: CLValueBuilder.key(args.from),
      to: CLValueBuilder.key(args.to),
      amount: CLValueBuilder.u256(args.amount),
      data: CLValueBuilder.option(None, new CLByteArrayType(0)),
    });

    runtimeArgs.insert("id", CLValueBuilder.u256(args.id));

    const preparedDeploy = this.contractClient.callEntrypoint(
      "safe_transfer_from",
      runtimeArgs,
      deploySender,
      this.networkName,
      paymentAmount,
      keys
    );

    return preparedDeploy;
  }

  public setTokenMetadata(
    args: TokenMetadataArgs,
    paymentAmount: string,
    deploySender: CLPublicKey,
    keys?: Keys.AsymmetricKey[]
  ) {
    const runtimeArgs = RuntimeArgs.fromMap({
      token_meta_data: CLValueBuilder.string(
        JSON.stringify(args.tokenMetaData)
      ),
    });

    const preparedDeploy = this.contractClient.callEntrypoint(
      "set_token_metadata",
      runtimeArgs,
      deploySender,
      this.networkName,
      paymentAmount,
      keys
    );

    return preparedDeploy;
  }

  public async getOwnerOf(tokenId: string) {
    const result = await this.contractClient.queryContractDictionary(
      "token_owners",
      tokenId
    );

    return `account-hash-${(result as CLKey).toJSON()}`;
  }

  // public async getMetadataOf(tokenId: string) {
  // const metadataToCheck: NFTMetadataKind =
  //   metadataType || NFTMetadataKind[await this.getMetadataKindConfig()];

  // const mapMetadata = {
  //   [NFTMetadataKind.CEP85]: "metadata_cep85",
  //   [NFTMetadataKind.NFT721]: "metadata_nft721",
  //   [NFTMetadataKind.Raw]: "metadata_raw",
  //   [NFTMetadataKind.CustomValidated]: "metadata_custom_validated",
  // };

  // const result = await this.contractClient.queryContractDictionary(
  //   mapMetadata[metadataToCheck],
  //   tokenId
  // );

  // const clMap = result as CLMap<CLString, CLString>;

  // return clMap.toJSON() as { [key: string]: string; };
  // }

  public async getBalanceOf(account: CLPublicKey) {
    console.log(account.toAccountHashStr().slice(13));
    const result = await this.contractClient.queryContractDictionary(
      "balances",
      account.toAccountHashStr().slice(13)
    );

    return (result as CLU8).toJSON();
  }

  public approve(
    args: ApproveArgs,
    paymentAmount: string,
    deploySender: CLPublicKey,
    keys?: Keys.AsymmetricKey[]
  ) {
    const runtimeArgs = RuntimeArgs.fromMap({
      operator: CLValueBuilder.key(args.operator),
    });

    if (args.id !== undefined) {
      runtimeArgs.insert("token_id", CLValueBuilder.u64(args.id));
    }

    const preparedDeploy = this.contractClient.callEntrypoint(
      "approve",
      runtimeArgs,
      deploySender,
      this.networkName,
      paymentAmount,
      keys
    );

    return preparedDeploy;
  }

  public approveAll(
    args: ApproveAllArgs,
    paymentAmount: string,
    deploySender: CLPublicKey,
    keys?: Keys.AsymmetricKey[]
  ) {
    const runtimeArgs = RuntimeArgs.fromMap({
      token_owner: CLValueBuilder.key(args.tokenOwner),
      approve_all: CLValueBuilder.bool(args.approveAll),
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

  public storeBalanceOf(
    args: StoreBalanceOfArgs,
    paymentAmount: string,
    deploySender: CLPublicKey,
    keys?: Keys.AsymmetricKey[],
    wasm?: Uint8Array
  ) {
    const wasmToCall = wasm;

    const runtimeArgs = RuntimeArgs.fromMap({
      nft_contract_hash: this.contractHashKey,
      token_owner: args.tokenOwner,
      key_name: CLValueBuilder.string(args.keyName),
    });

    const preparedDeploy = this.contractClient.install(
      wasmToCall,
      runtimeArgs,
      paymentAmount,
      deploySender,
      this.networkName,
      keys
    );

    return preparedDeploy;
  }

}
