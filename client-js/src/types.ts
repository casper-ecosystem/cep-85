import {
  AccountHash,
  AddressableEntityHash,
  ContractHash,
  ContractPackageHash,
  ExecutionResult,
  PrivateKey,
  PublicKey,
  PutTransactionResult,
} from 'casper-js-sdk';

export enum EVENTS_MODE {
  NoEvents = 0,
  CES = 1,
  Native = 2,
  NativeBytes = 3,
}

export type Entity =
  | PublicKey
  | AccountHash
  | ContractHash
  | ContractPackageHash
  | AddressableEntityHash;

export type ConfigurableVariables = {
  eventsMode?: EVENTS_MODE;
  enableBurn?: boolean;
  adminList?: Entity[];
  minterList?: Entity[];
  burnerList?: Entity[];
  metaList?: Entity[];
  noneList?: Entity[];
  transferFilterContract?: ContractHash;
  transferFilterMethod?: string;
};

export type InstallArgs = {
  name: string;
  uri: string;
} & ConfigurableVariables;

export type UpgradeArgs = {
  name: string;
};

export interface MintArgs {
  recipient: Entity;
  id: string;
  amount: string;
  uri?: string;
}

export interface BatchMintArgs {
  recipient: Entity;
  ids: string[];
  amounts: string[];
  uri?: string;
}

export type TransferArgs = {
  from: Entity;
  to: Entity;
  id: string;
  amount: string;
  data?: Uint8Array;
};

export type BatchTransferArgs = {
  from: Entity;
  to: Entity;
  ids: string[];
  amounts: string[];
  data?: Uint8Array;
};

export type BurnArgs = {
  owner: Entity;
  id: string;
  amount: string;
};

export type BatchBurnArgs = {
  owner: Entity;
  ids: string[];
  amounts: string[];
};

export type BalanceOfArgs = {
  account: Entity;
  id: string;
};

export type SetUriArgs = {
  id?: string;
  uri: string;
};

export type ChangeSecurityArgs = {
  adminList?: Entity[];
  minterList?: Entity[];
  burnerList?: Entity[];
  metaList?: Entity[];
  noneList?: Entity[];
};

export type TotalSupplyOfArgs = {
  id: string;
  totalSupply: string;
};

export type TotalSupplyOfBatchArgs = {
  ids: string[];
  totalSupplies: string[];
};

export type SetApprovallForAllArgs = {
  operator: Entity;
  approved: boolean;
};

export type SetModalitiesArgs = {
  eventsMode?: EVENTS_MODE;
  enableBurn?: boolean;
};

export type TransactionParams = {
  sender: PublicKey;
  paymentAmount: string;
  wasm?: Uint8Array;
  signingKeys?: PrivateKey[];
  chainName?: string;
};

export type TransactionResult = {
  transactionInfo: PutTransactionResult;
  executionResult?: ExecutionResult;
};

export interface JSONSchemaEntry {
  name: string;
  description: string;
  required: boolean;
}

export interface JSONSchemaObject {
  properties: Record<string, JSONSchemaEntry>;
}

interface BaseParams {
  params: TransactionParams;
  waitForTransactionProcessed?: boolean;
}

export interface InstallParams extends BaseParams {
  args: InstallArgs;
}

export interface UpgradeParams extends BaseParams {
  args: UpgradeArgs;
}

export interface MintParams extends BaseParams {
  args: MintArgs;
}

export interface BatchMintParams extends BaseParams {
  args: BatchMintArgs;
}

export interface TransferParams extends BaseParams {
  args: TransferArgs;
}

export interface BatchTransferParams extends BaseParams {
  args: BatchTransferArgs;
}

export interface BurnParams extends BaseParams {
  args: BurnArgs;
}

export interface BatchBurnParams extends BaseParams {
  args: BatchBurnArgs;
}

export interface BalanceOfParams extends BaseParams {
  args: BalanceOfArgs;
}

export interface SetUriParams extends BaseParams {
  args: SetUriArgs;
}

export interface ChangeSecurityParams extends BaseParams {
  args: ChangeSecurityArgs;
}

export interface SetModalitiesParams extends BaseParams {
  args: SetModalitiesArgs;
}

export interface SetApprovallForAllParams extends BaseParams {
  args: SetApprovallForAllArgs;
}

export interface TotalSupplyOfParams extends BaseParams {
  args: TotalSupplyOfArgs;
}

export interface TotalSupplyOfBatchParams extends BaseParams {
  args: TotalSupplyOfBatchArgs;
}
