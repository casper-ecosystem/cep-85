import { CLValue, Hash, Message, TransactionHash } from 'casper-js-sdk';

export enum CEP85_EVENTS {
  Mint = 'Mint',
  MintBatch = 'MintBatch',
  Burn = 'Burn',
  BurnBatch = 'BurnBatch',
  ApprovalForAll = 'ApprovalForAll',
  Transfer = 'Transfer',
  TransferBatch = 'TransferBatch',
  Uri = 'Uri',
  UriBatch = 'UriBatch',
  SetTotalSupply = 'SetTotalSupply',
  ChangeSecurity = 'ChangeSecurity',
  SetModalities = 'SetModalities',
  Upgrade = 'Upgrade',
}

type EventName = keyof typeof CEP85_EVENTS;

export type Event<E extends Record<string, CLValue>> = {
  name: EventName;
  contractHash: Hash;
  contractPackageHash: Hash;
  eventId: number;
  data: E;
};

export interface TransactionInfo {
  transactionHash: TransactionHash;
  timestamp: string;
  messages: Message[];
}

export type WithTransactionInfo<E> = E & { transactionInfo: TransactionInfo };

export type CEP85EventResult = WithTransactionInfo<CEP85Event>;

export type CEP85Event = Event<
  | Mint
  | MintBatch
  | Burn
  | BurnBatch
  | ApprovalForAll
  | Transfer
  | TransferBatch
  | Uri
  | UriBatch
  | SetTotalSupply
  | ChangeSecurity
  | SetModalities
  | Upgrade
>;

export type EventsMap = {
  Mint: Event<Mint>;
  MintBatch: Event<MintBatch>;
  Burn: Event<Burn>;
  BurnBatch: Event<BurnBatch>;
  ApprovalForAll: Event<ApprovalForAll>;
  Transfer: Event<Transfer>;
  TransferBatch: Event<TransferBatch>;
  Uri: Event<Uri>;
  UriBatch: Event<UriBatch>;
  SetTotalSupply: Event<SetTotalSupply>;
  ChangeSecurity: Event<ChangeSecurity>;
  SetModalities: Event<SetModalities>;
  Upgrade: Event<Upgrade>;
};

export type Mint = {
  id: CLValue;
  recipient: CLValue;
  amount: CLValue;
};

export type MintBatch = {
  ids: CLValue;
  recipient: CLValue;
  amounts: CLValue;
};

export type Burn = {
  id: CLValue;
  owner: CLValue;
  amount: CLValue;
};

export type BurnBatch = {
  ids: CLValue;
  owner: CLValue;
  amounts: CLValue;
};

export type ApprovalForAll = {
  owner: CLValue;
  operator: CLValue;
  approved: CLValue;
};

export type Transfer = {
  operator: CLValue;
  from: CLValue;
  to: CLValue;
  id: CLValue;
  value: CLValue;
};

export type TransferBatch = {
  operator: CLValue;
  from: CLValue;
  to: CLValue;
  ids: CLValue;
  values: CLValue;
};

export type Uri = {
  value: CLValue;
  id: CLValue;
};

export type UriBatch = {
  value: CLValue;
  ids: CLValue;
};

export type SetTotalSupply = {
  id: CLValue;
  total_supply: CLValue;
};

export type ChangeSecurity = {
  admin: CLValue;
  sec_change_map: CLValue;
};

export type SetModalities = Record<string, never>;

export type Upgrade = Record<string, never>;
