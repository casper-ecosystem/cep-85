import {
  CasperServiceByJsonRPC,
} from "casper-js-sdk";

import { Parser } from "@make-software/ces-js-parser";

import { EventItem } from "./types";

export const CESEventParserFactory =
  ({
    contractHashes,
    // TODO: IDEALLY in future I would love to have here a schema as an argument instead of casperClient. That way the whole thing can be initialized offline as the whole client.
    casperClient,
  }: {
    contractHashes: string[];
    casperClient: CasperServiceByJsonRPC;
  }) =>
    async (event: EventItem) => {
      const validatedHashes = contractHashes.map((hash) =>
        hash.startsWith("hash-") ? hash.slice(5) : hash
      );
      const parser = await Parser.create(casperClient, validatedHashes);

      try {
        const toParse = event.body.DeployProcessed.execution_result;
        const events = parser.parseExecutionResult(toParse);
        return { error: null, success: !!events.length, data: events };
      } catch (error: unknown) {
        return { error, success: false, data: null };
      }
    };
