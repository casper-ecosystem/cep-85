//  since `deployProcessed` is any type in L49 the eslint gives error for this line
/* eslint-disable eslint-comments/disable-enable-pair */
/* eslint-disable @typescript-eslint/no-unsafe-assignment */
/* eslint-disable @typescript-eslint/no-unsafe-member-access */
import { Parser } from '@make-software/ces-js-parser';
import {
  CasperClient,
  Contracts,
  encodeBase16,
  EventName,
  EventStream,
  ExecutionResult
} from 'casper-js-sdk';

import { CEP85Event, CEP85EventWithDeployInfo, WithDeployInfo } from './events';

const { Contract } = Contracts;

export default class EventEnabledContract {
  public contractClient: Contracts.Contract;

  private casperClient: CasperClient;

  private eventStream?: EventStream;

  private parser?: Parser;

  private readonly events: Record<
    string,
    ((event: WithDeployInfo<CEP85Event>) => void)[]
  > = {};

  constructor(public nodeAddress: string, public networkName: string) {
    this.casperClient = new CasperClient(nodeAddress);
    this.contractClient = new Contract(this.casperClient);
  }

  stopEventStream() {
    if (!this.eventStream) {
      return;
    }
    this.eventStream.unsubscribe(EventName.DeployProcessed);
    this.eventStream.stop();
  }

  async setupEventStream(eventStream: EventStream) {
    this.eventStream = eventStream;

    if (!this.parser) {
      this.parser = await Parser.create(this.casperClient.nodeClient, [
        (this.contractClient.contractHash || '').slice(5)
      ]);
    }

    this.eventStream.start();

    this.eventStream.subscribe(EventName.DeployProcessed, deployProcessed => {
      const {
        timestamp,
        deploy_hash: deployHash
      } = deployProcessed.body.DeployProcessed;
      const executionResult: ExecutionResult = deployProcessed.body.DeployProcessed.execution_result;

      if (!executionResult?.Success || !this.parser) {
        return;
      }

      const results = this.parseExecutionResult(
        executionResult
      );

      results
        .map(
          r =>
          ({
            ...r,
            deployInfo: { deployHash, timestamp }
          } as CEP85EventWithDeployInfo)
        )
        .forEach(event => this.emit(event));
    });
  }

  on(name: string, listener: (event: CEP85EventWithDeployInfo) => void) {
    this.addEventListener(name, listener);
  }

  addEventListener(
    name: string,
    listener: (event: CEP85EventWithDeployInfo) => void
  ) {
    if (!this.events[name]) this.events[name] = [];

    this.events[name].push(listener);
  }

  off(name: string, listener: (event: CEP85EventWithDeployInfo) => void) {
    this.removeEventListener(name, listener);
  }

  removeEventListener(
    name: string,
    listenerToRemove: (event: CEP85EventWithDeployInfo) => void
  ) {
    if (!this.events[name]) {
      throw new Error(
        `Can't remove a listener. Event "${name}" doesn't exits.`
      );
    }

    const filterListeners = (
      listener: (event: CEP85EventWithDeployInfo) => void
    ) => listener !== listenerToRemove;

    this.events[name] = this.events[name].filter(filterListeners);
  }

  emit(event: CEP85EventWithDeployInfo) {
    this.events[event.name]?.forEach(cb => cb(event));
  }

  parseExecutionResult(result: ExecutionResult): CEP85Event[] {
    const results = this.parser?.parseExecutionResult(result);
    if (!results) {
      return [];
    }
    return results
      .filter(r => r.error === null)
      .map(r => ({
        ...r.event,
        contractHash: `hash-${encodeBase16(r.event.contractHash || new Uint8Array())}`,
        contractPackageHash: `hash-${encodeBase16(r.event.contractPackageHash || new Uint8Array())}`
      })) as CEP85Event[];
  }
}