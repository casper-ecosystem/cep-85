import { CLType, CLValue, CLKeyParameters, CLByteArray, CLPublicKey } from "casper-js-sdk";

export interface CallConfig {
  useSessionCode: boolean;
}

export enum EventsMode {
  NoEvents,
  CES,
}

export interface JSONSchemaEntry {
  name: string;
  description: string;
  required: boolean;
}

export interface JSONSchemaObject {
  properties: Record<string, JSONSchemaEntry>;
}

export type ConfigurableVariables = {
  burner_list?: CLKeyParameters[];
  eventsMode?: EventsMode;
};

export type InstallArgs = {
  name: string;
  uri: string;
} & ConfigurableVariables;

export interface MintArgs {
  recipient: CLKeyParameters;
  id: string;
  amount: string;
}

export type TransferArgs = {
  from: CLKeyParameters;
  to: CLKeyParameters;
  id: string;
  amount: string;
};

export type BurnArgs = {
  id: string;
  owner: CLKeyParameters;
  amount: string;
};

export interface RegisterArgs {
  tokenOwner: CLKeyParameters;
}

export interface TokenArgs {
  id: string;
}

export type TokenMetadataArgs = {
  tokenMetaData: Record<string, string>;
};

export type StoreBalanceOfArgs = {
  tokenOwner: CLKeyParameters;
  keyName: string;
};

export type ApproveArgs = {
  operator: CLKeyParameters;
} & TokenArgs;

export type ApproveAllArgs = {
  operator: CLKeyParameters;
  approveAll: boolean;
  tokenOwner: CLKeyParameters;
};

type WriteCLValue = {
  cl_type: string;
  bytes: string;
  parsed: string;
};

// TODO: Most of this types can be moved to casper-js-sdk in feature release
// https://github.com/casper-ecosystem/casper-js-sdk/issues/268

type TransformValue = {
  WriteCLValue?: WriteCLValue;
};

export interface Transform {
  key: string;
  transform: TransformValue;
}

interface Effect {
  transforms: Transform[];
}

interface ExecutionResultBody {
  cost: number;
  error_message?: string | null;
  transfers: string[];
  effect: Effect;
}

export interface ExecutionResult {
  Success?: ExecutionResultBody;
  Failure?: ExecutionResultBody;
}

export interface WithRemainder<T> {
  data: T;
  remainder: Uint8Array;
}

export interface RawCLValue {
  clType: CLType;
  bytes: Uint8Array;
}

export interface EventItem {
  id: number;
  body: {
    DeployProcessed: {
      execution_result: ExecutionResult;
    };
  };
}

export interface EventParsed {
  name: string;
  clValue: CLValue;
}
