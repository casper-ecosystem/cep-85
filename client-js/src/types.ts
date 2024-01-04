import { CLType, CLValue, CLKeyParameters } from "casper-js-sdk";

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

export interface BatchMintArgs {
  recipient: CLKeyParameters;
  ids: string[];
  amounts: string[];
}

export type TransferArgs = {
  from: CLKeyParameters;
  to: CLKeyParameters;
  id: string;
  amount: string;
  data?: Uint8Array;
};

export type BatchTransferArgs = {
  from: CLKeyParameters;
  to: CLKeyParameters;
  ids: string[];
  amounts: string[];
  data?: Uint8Array;
};

export type BurnArgs = {
  owner: CLKeyParameters;
  id: string;
  amount: string;
};

export type BatchBurnArgs = {
  owner: CLKeyParameters;
  ids: string[];
  amounts: string[];
};

export type BalanceOfArgs = {
  account: CLKeyParameters;
  id: string;
};

export type SetUriArgs = {
  id?: string;
  uri: string;
};

export type ChangeSecurityArgs = {
  admin_list?: CLKeyParameters[];
  minter_list?: CLKeyParameters[];
  burner_list?: CLKeyParameters[];
  meta_list?: CLKeyParameters[];
  none_list?: CLKeyParameters[];
};

export type TotalSupplyOfArgs = {
  id: string;
  total_supply: string;
};

export type TotalSupplyOfArgsBatch = {
  ids: string[];
  total_supplies: string[];
};

export type SetApprovallForAllArgs = {
  operator: CLKeyParameters;
  approved: boolean;
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
