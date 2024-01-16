import { Contracts, EventStream, ExecutionResult } from 'casper-js-sdk';

import EventEnabledContract from './EventEnabledContract';
import { CEP85Event, EventsMap } from './events';

interface ITypedContract {
  contractClient: Contracts.Contract;

  setupEventStream(eventStream: EventStream): Promise<void>;
  stopEventStream(): void;
  parseExecutionResult(result: ExecutionResult): CEP85Event[];

  on<K extends keyof EventsMap>(
    type: K,
    listener: (ev: EventsMap[K]) => void
  ): void;

  addEventListener<K extends keyof EventsMap>(
    type: K,
    listener: (ev: EventsMap[K]) => void
  ): void;

  off<K extends keyof EventsMap>(
    type: K,
    listener: (ev: EventsMap[K]) => void
  ): void;

  removeEventListener<K extends keyof EventsMap>(
    type: K,
    listener: (ev: EventsMap[K]) => void
  ): void;
}

interface TypedContractConstructor {
  new(nodeAddress: string, networkName: string): ITypedContract;
  prototype: ITypedContract;
}

const TypedContract: TypedContractConstructor = EventEnabledContract as TypedContractConstructor;

export default TypedContract;