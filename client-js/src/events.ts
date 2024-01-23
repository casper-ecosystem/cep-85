import { CLBool, CLKey, CLList, CLMap, CLOption, CLString, CLU256, CLValue } from 'casper-js-sdk';

export type Event<E extends Record<string, CLValue>> = {
  name: string;
  contractHash: `hash-${string}`;
  contractPackageHash: `hash-${string}`;
  data: E;
};

export interface DeployInfo {
  deployHash: string;
  timestamp: string;
}

export type WithDeployInfo<E> = E & { deployInfo: DeployInfo; };

export type CEP85EventWithDeployInfo = WithDeployInfo<CEP85Event>;

export type CEP85Event = Event<
  | Mint
  | Burn
  | ApprovalForAll
  | TransferSingle
  | TransferBatch
  | Uri
  | SetTotalSupply
  | ChangeSecurity
  | SetModalities
  | Upgrade
>;

export type EventsMap = {
  Mint: Event<Mint>;
  Burn: Event<Burn>;
  ApprovalForAll: Event<ApprovalForAll>;
  TransferSingle: Event<TransferSingle>;
  TransferBatch: Event<TransferBatch>;
  Uri: Event<Uri>;
  SetTotalSupply: Event<SetTotalSupply>;
  ChangeSecurity: Event<ChangeSecurity>;
  SetModalities: Event<SetModalities>;
  Upgrade: Event<Upgrade>;
};

export type Mint = {
  id: CLU256;
  recipient: CLKey;
  amount: CLU256;
};

export type Burn = {
  id: CLU256;
  owner: CLKey;
  amount: CLU256;
};

export type ApprovalForAll = {
  owner: CLKey,
  operator: CLKey,
  approved: CLBool,
};

export type TransferSingle = {
  operator: CLKey,
  from: CLKey,
  to: CLKey,
  id: CLU256,
  value: CLU256,
};

export type TransferBatch = {
  operator: CLKey,
  from: CLKey,
  to: CLKey,
  ids: CLList<CLU256>,
  values: CLList<CLU256>,
};

export type Uri = {
  value: CLString,
  id: CLOption<CLU256>,
};

export type SetTotalSupply = {
  id: CLU256,
  total_supply: CLU256,
};

export type ChangeSecurity = {
  admin: CLKey,
  sec_change_map: CLMap<CLKey, CLValue>,
};

export type SetModalities = Record<string, never>;

export type Upgrade = Record<string, never>;