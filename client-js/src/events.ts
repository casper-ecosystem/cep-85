import { CLKey, CLU256, CLValue } from 'casper-js-sdk';

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
>;

export type EventsMap = {
  Mint: Event<Mint>;
  Burn: Event<Burn>;
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