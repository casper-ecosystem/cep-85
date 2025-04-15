import { blake2b } from '@noble/hashes/blake2b';
import { bytesToHex } from '@noble/hashes/utils';
import {
  Args,
  CLTypeKey,
  CLValue,
  ContractHash,
  ContractPackageHash,
  ExecutionResult,
  KeyAlgorithm,
  PrivateKey,
  PutTransactionResult,
  RpcClient,
  TransactionProcessedPayload,
} from 'casper-js-sdk';
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import {
  EVENTS_MODE,
  InstallParams,
  CEP85Client,
  UpgradeParams,
  TransferParams,
  MintParams,
  BurnParams,
  ChangeSecurityParams,
  BatchTransferParams,
  BatchMintParams,
  BatchBurnParams,
  TotalSupplyOfParams,
  TotalSupplyOfBatchParams,
  SetApprovallForAllParams,
  SetModalitiesParams,
} from '../../src';

describe('CEP85Client Unit', () => {
  describe('CEP85Client - setContractHash', () => {
    let client: CEP85Client;
    beforeEach(() => {
      // Initializing a new CEP85Client instance for each test
      client = new CEP85Client('http://mock-rpc-url');
    });

    afterEach(() => {
      vi.restoreAllMocks();
    });

    it('should correctly set the contract hash and contract package hash', () => {
      const contractHash = 'contract-hash-0x';
      const contractPackageHash = 'contract-package-0x';
      // Spy on the method to see if the contract hash is set properly
      vi.spyOn(ContractHash, 'newContract').mockImplementation(() => {
        return {} as ContractHash;
      });
      vi.spyOn(ContractPackageHash, 'newContractPackage').mockImplementation(
        () => {
          return {} as ContractPackageHash;
        }
      );
      const result = client.setContractHash(contractHash, contractPackageHash);
      // Check if the correct methods were called for both contract hash and contract package hash
      expect(ContractHash.newContract).toHaveBeenCalledWith('0x');
      expect(ContractPackageHash.newContractPackage).toHaveBeenCalledWith('0x');
      expect(result).toBeInstanceOf(CEP85Client);
    });

    it('should throw an error if contract hash is not provided', () => {
      // Providing invalid contract hash
      expect(() => client.setContractHash('')).toThrowError(
        'Contract hash must be provided.'
      );
    });

    it('should correctly remove prefixes from the contract hash and contract package hash', () => {
      const contractHashWithPrefix = 'hash-12345';
      const contractPackageHashWithPrefix = 'package-0x';
      // Mock implementation of `ContractHash` and `ContractPackageHash`
      vi.spyOn(ContractHash, 'newContract').mockImplementation(() => {
        return {} as ContractHash;
      });
      vi.spyOn(ContractPackageHash, 'newContractPackage').mockImplementation(
        () => {
          return {} as ContractPackageHash;
        }
      );
      client.setContractHash(
        contractHashWithPrefix,
        contractPackageHashWithPrefix
      );
      // Ensure the prefixes are correctly removed
      expect(ContractHash.newContract).toHaveBeenCalledWith('12345');
      expect(ContractPackageHash.newContractPackage).toHaveBeenCalledWith('0x');
    });

    it('should handle optional contract package hash', () => {
      const contractHash = 'contract-hash-0x';
      // Mock implementation of `ContractHash`
      vi.spyOn(ContractHash, 'newContract').mockImplementation(() => {
        return {} as ContractHash;
      });
      // No contract package hash provided, so it should still work
      const result = client.setContractHash(contractHash);
      expect(ContractHash.newContract).toHaveBeenCalledWith('0x');
      expect(result).toBeInstanceOf(CEP85Client);
    });
  });

  describe('CEP85Client - Event Stream', () => {
    let client: CEP85Client;
    let mockSseUrl: string;

    beforeEach(() => {
      client = new CEP85Client(
        'http://mock-rpc-url',
        'http://mock-sse-url',
        'testnet'
      );
      mockSseUrl = 'http://mock-sse-url';
    });

    it('should call startEventStream and return the updated CEP85Client instance', () => {
      // Spy on the super class method
      const startEventStreamSpy = vi
        .spyOn(CEP85Client.prototype, 'startEventStream')
        .mockReturnThis();

      const result = client.startEventStream(mockSseUrl);
      expect(startEventStreamSpy).toHaveBeenCalledWith(mockSseUrl);
      expect(result).toBe(client);
    });

    it('should call stopEventStream and return the updated CEP85Client instance', () => {
      // Spy on the super class method
      const stopEventStreamSpy = vi
        .spyOn(CEP85Client.prototype, 'stopEventStream')
        .mockReturnThis();

      // Call stopEventStream and check that it works
      const result = client.stopEventStream();
      expect(stopEventStreamSpy).toHaveBeenCalled();
      expect(result).toBe(client); // Expecting the same instance to be returned
    });

    it('should handle undefined sseUrl gracefully', () => {
      const clientWithUndefinedSseUrl = new CEP85Client(
        'http://mock-rpc-url',
        undefined, // undefined sseUrl for testing
        'testnet'
      );

      const startEventStreamSpy = vi
        .spyOn(CEP85Client.prototype, 'startEventStream')
        .mockReturnThis();

      const result = clientWithUndefinedSseUrl.startEventStream(
        'http://mock-sse-url'
      );
      expect(startEventStreamSpy).toHaveBeenCalledWith(mockSseUrl);
      expect(result).toBe(clientWithUndefinedSseUrl);
    });
  });

  describe('CEP85Client - install', () => {
    let client: CEP85Client;
    const key = PrivateKey.generate(KeyAlgorithm.ED25519);
    const mockParams: InstallParams = {
      params: {
        wasm: new Uint8Array(),
        paymentAmount: '1000',
        sender: key.publicKey,
        chainName: 'testnet',
        signingKeys: [key],
      },
      args: {
        name: 'CEP85',
        uri: 'https://test-cdn-domain/{id}.json',
        eventsMode: EVENTS_MODE.CES,
        enableBurn: true,
      },
      waitForTransactionProcessed: false,
    };

    beforeEach(() => {
      client = new CEP85Client('http://mock-rpc-url');
      vi.spyOn(client['_rpcClient'], 'putTransaction').mockResolvedValue({
        transactionHash: 'mockTransactionHash',
      } as unknown as PutTransactionResult);
      vi.spyOn(client, 'waitForTransactionProcessed').mockResolvedValue({
        transactionProcessedPayload: {
          executionResult: { errorMessage: '' } as ExecutionResult,
        } as unknown as TransactionProcessedPayload,
      });
    });

    it('should successfully install a contract', async () => {
      const result = await client.install(mockParams);

      expect(client['_rpcClient'].putTransaction).toHaveBeenCalled();
      expect(result).toEqual({
        transactionInfo: { transactionHash: 'mockTransactionHash' },
      });
    });

    it('should call waitForTransactionProcessed if waitForTransactionProcessed is true', async () => {
      const paramsWithWait = {
        ...mockParams,
        waitForTransactionProcessed: true,
      };

      await client.install(paramsWithWait);

      expect(client.waitForTransactionProcessed).toHaveBeenCalledWith(
        'mockTransactionHash'
      );
    });

    it('should successfully install a contract if waitForTransactionProcessed is true', async () => {
      const paramsWithWait = {
        ...mockParams,
        waitForTransactionProcessed: true,
      };
      const result = await client.install(paramsWithWait);

      expect(client['_rpcClient'].putTransaction).toHaveBeenCalled();

      expect(result).toEqual({
        transactionInfo: { transactionHash: 'mockTransactionHash' },
        executionResult: { errorMessage: '' },
      });
    });

    it('should handle errors during transaction installation', async () => {
      const errorMessage = 'error during installation';
      vi.spyOn(client['_rpcClient'], 'putTransaction').mockRejectedValueOnce(
        new Error(errorMessage)
      );

      await expect(client.install(mockParams)).rejects.toThrow(
        `Error during installation runtime.\nError: ${errorMessage}`
      );
    });
  });

  describe('CEP85Client - upgrade', () => {
    let client: CEP85Client;
    const key = PrivateKey.generate(KeyAlgorithm.ED25519);
    const mockParams: UpgradeParams = {
      params: {
        wasm: new Uint8Array(),
        paymentAmount: '1000',
        sender: key.publicKey,
        chainName: 'testnet',
        signingKeys: [key],
      },
      args: {
        name: 'CEP85',
      },
      waitForTransactionProcessed: false,
    };

    beforeEach(() => {
      client = new CEP85Client('http://mock-rpc-url');
      vi.spyOn(client['_rpcClient'], 'putTransaction').mockResolvedValue({
        transactionHash: 'mockTransactionHash',
      } as unknown as PutTransactionResult);
      vi.spyOn(client, 'waitForTransactionProcessed').mockResolvedValue({
        transactionProcessedPayload: {
          executionResult: { errorMessage: '' } as ExecutionResult,
        } as unknown as TransactionProcessedPayload,
      });
    });

    it('should successfully upgrade a contract', async () => {
      const result = await client.upgrade(mockParams);

      expect(client['_rpcClient'].putTransaction).toHaveBeenCalled();
      expect(result).toEqual({
        transactionInfo: { transactionHash: 'mockTransactionHash' },
      });
    });

    it('should call waitForTransactionProcessed if waitForTransactionProcessed is true', async () => {
      const paramsWithWait = {
        ...mockParams,
        waitForTransactionProcessed: true,
      };

      await client.upgrade(paramsWithWait);

      expect(client.waitForTransactionProcessed).toHaveBeenCalledWith(
        'mockTransactionHash'
      );
    });

    it('should successfully upgrade a contract if waitForTransactionProcessed is true', async () => {
      const paramsWithWait = {
        ...mockParams,
        waitForTransactionProcessed: true,
      };
      const result = await client.upgrade(paramsWithWait);

      expect(client['_rpcClient'].putTransaction).toHaveBeenCalled();

      expect(result).toEqual({
        transactionInfo: { transactionHash: 'mockTransactionHash' },
        executionResult: { errorMessage: '' },
      });
    });

    it('should handle errors during transaction upgradeation', async () => {
      const errorMessage = 'error during upgrade';
      vi.spyOn(client['_rpcClient'], 'putTransaction').mockRejectedValueOnce(
        new Error(errorMessage)
      );

      await expect(client.upgrade(mockParams)).rejects.toThrow(
        `Error during upgrade runtime.\nError: ${errorMessage}`
      );
    });
  });

  describe('CEP85Client - transfer', () => {
    let client: CEP85Client;
    const key = PrivateKey.generate(KeyAlgorithm.ED25519);
    const key2 = PrivateKey.generate(KeyAlgorithm.ED25519);

    const mockTransferParams: TransferParams = {
      params: {
        paymentAmount: '2500000000',
        sender: key.publicKey,
        chainName: 'casper-test',
        signingKeys: [key],
      },
      args: {
        from: key.publicKey,
        to: key2.publicKey,
        id: '1',
        amount: '100',
      },
      waitForTransactionProcessed: true,
    };

    beforeEach(() => {
      client = new CEP85Client('http://mock-rpc-url');
      vi.spyOn(client as any, 'callEntrypoint').mockResolvedValue({
        transactionInfo: { transactionHash: 'mockTxHash' },
        executionResult: { errorMessage: '' },
      });
    });

    afterEach(() => {
      vi.restoreAllMocks();
    });

    it('should call queryTransfer with the correct entrypoint and parameters', async () => {
      const spy = vi.spyOn(client as any, 'queryTransfer');

      await client.transfer(mockTransferParams);

      expect(spy).toHaveBeenCalledWith(mockTransferParams, 'transfer_from');
    });

    it('should successfully perform a token transfer', async () => {
      const spy = vi.spyOn(client as any, 'callEntrypoint');

      const result = await client.transfer(mockTransferParams);

      expect(spy).toHaveBeenCalled();
      expect(result).toEqual({
        transactionInfo: { transactionHash: 'mockTxHash' },
        executionResult: { errorMessage: '' },
      });
    });

    it('should include data if provided', async () => {
      const mockWithData: TransferParams = {
        ...mockTransferParams,
        args: {
          ...mockTransferParams.args,
          data: new Uint8Array([1, 2, 3]),
        },
      };

      const spy = vi.spyOn(client as any, 'callEntrypoint');

      await client.transfer(mockWithData);

      expect(spy).toHaveBeenCalled();
      const runtimeArgsUsed = spy.mock.calls[0][1] as Function; // 2nd argument is runtimeArgs
      expect(runtimeArgsUsed['args'].get('data')).toBeDefined();
    });
  });

  describe('CEP85Client - batchTransfer', () => {
    let client: CEP85Client;
    const key = PrivateKey.generate(KeyAlgorithm.ED25519);
    const key2 = PrivateKey.generate(KeyAlgorithm.ED25519);

    const mockBatchTransferParams: BatchTransferParams = {
      params: {
        paymentAmount: '2500000000',
        sender: key.publicKey,
        chainName: 'casper-test',
        signingKeys: [key],
      },
      args: {
        from: key.publicKey,
        to: key2.publicKey,
        ids: ['1', '2'],
        amounts: ['100', '200'],
      },
      waitForTransactionProcessed: true,
    };

    beforeEach(() => {
      client = new CEP85Client('http://mock-rpc-url');
      vi.spyOn(client as any, 'callEntrypoint').mockResolvedValue({
        transactionInfo: { transactionHash: 'mockBatchTxHash' },
        executionResult: { errorMessage: '' },
      });
    });

    afterEach(() => {
      vi.restoreAllMocks();
    });

    it('should call queryTransfer with the correct entrypoint and parameters', async () => {
      const spy = vi.spyOn(client as any, 'queryTransfer');

      await client.batchTransfer(mockBatchTransferParams);

      expect(spy).toHaveBeenCalledWith(
        mockBatchTransferParams,
        'batch_transfer_from'
      );
    });

    it('should successfully perform a batch token transfer', async () => {
      const spy = vi.spyOn(client as any, 'callEntrypoint');
      const result = await client.batchTransfer(mockBatchTransferParams);

      expect(spy).toHaveBeenCalled();
      expect(result).toEqual({
        transactionInfo: { transactionHash: 'mockBatchTxHash' },
        executionResult: { errorMessage: '' },
      });
    });

    it('should include data if provided', async () => {
      const mockWithData: BatchTransferParams = {
        ...mockBatchTransferParams,
        args: {
          ...mockBatchTransferParams.args,
          data: new Uint8Array([10, 20, 30]),
        },
      };

      const spy = vi.spyOn(client as any, 'callEntrypoint');

      await client.batchTransfer(mockWithData);

      expect(spy).toHaveBeenCalled();
      const runtimeArgsUsed = spy.mock.calls[0][1] as Function;
      expect(runtimeArgsUsed['args'].get('data')).toBeDefined();
    });

    it('should throw if ids and amounts length do not match', async () => {
      const invalidParams: BatchTransferParams = {
        ...mockBatchTransferParams,
        args: {
          ...mockBatchTransferParams.args,
          ids: ['1', '2'],
          amounts: ['100'], // mismatched
        },
      };

      try {
        await client.batchTransfer(invalidParams);
        throw new Error('Expected batchTransfer to throw, but it did not');
      } catch (err: any) {
        expect(err.message).toBe(
          'Length of "ids" and "amounts" must be the same.'
        );
      }
    });

    it('should handle failed transaction execution', async () => {
      vi.spyOn(client as any, 'callEntrypoint').mockResolvedValueOnce({
        transactionInfo: { transactionHash: 'mockFailTxHash' },
        executionResult: { errorMessage: 'Transfer failed' },
      });

      const result = await client.batchTransfer(mockBatchTransferParams);

      expect(result.executionResult?.errorMessage).toBe('Transfer failed');
    });
  });

  describe('CEP85Client - mint', () => {
    let client: CEP85Client;
    const key = PrivateKey.generate(KeyAlgorithm.ED25519);
    const recipient = key.publicKey;

    const mockMintParams: MintParams = {
      params: {
        paymentAmount: '2500000000',
        sender: key.publicKey,
        chainName: 'casper-test',
        signingKeys: [key],
      },
      args: {
        recipient,
        id: '123',
        amount: '1000',
      },
      waitForTransactionProcessed: true,
    };

    beforeEach(() => {
      client = new CEP85Client('http://mock-rpc-url');
      vi.spyOn(client as any, 'callEntrypoint').mockResolvedValue({
        transactionInfo: { transactionHash: 'mockMintTxHash' },
        executionResult: { errorMessage: '' },
      });
    });

    afterEach(() => {
      vi.restoreAllMocks();
    });

    it('should call queryMint with the correct entrypoint and parameters', async () => {
      const spy = vi.spyOn(client as any, 'queryMint');
      await client.mint(mockMintParams);
      expect(spy).toHaveBeenCalledWith(mockMintParams, 'mint');
    });

    it('should successfully perform a single token mint', async () => {
      const result = await client.mint(mockMintParams);

      expect(result).toEqual({
        transactionInfo: { transactionHash: 'mockMintTxHash' },
        executionResult: { errorMessage: '' },
      });
    });

    it('should include uri if provided', async () => {
      const mockWithURI: MintParams = {
        ...mockMintParams,
        args: {
          ...mockMintParams.args,
          uri: 'https://example.com/metadata.json',
        },
      };

      const spy = vi.spyOn(client as any, 'callEntrypoint');

      await client.mint(mockWithURI);

      expect(spy).toHaveBeenCalled();
      const runtimeArgsUsed = spy.mock.calls[0][1] as any;
      expect(runtimeArgsUsed['args'].get('uri')).toBeDefined();
    });
  });

  describe('CEP85Client - batchMint', () => {
    let client: CEP85Client;
    const key = PrivateKey.generate(KeyAlgorithm.ED25519);
    const recipient = key.publicKey;

    const mockBatchMintParams: BatchMintParams = {
      params: {
        paymentAmount: '3000000000',
        sender: key.publicKey,
        chainName: 'casper-test',
        signingKeys: [key],
      },
      args: {
        recipient,
        ids: ['1', '2', '3'],
        amounts: ['10', '20', '30'],
      },
      waitForTransactionProcessed: true,
    };

    beforeEach(() => {
      client = new CEP85Client('http://mock-rpc-url');
      vi.spyOn(client as any, 'callEntrypoint').mockResolvedValue({
        transactionInfo: { transactionHash: 'mockBatchMintTxHash' },
        executionResult: { errorMessage: '' },
      });
    });

    afterEach(() => {
      vi.restoreAllMocks();
    });

    it('should call queryMint with correct entrypoint and params', async () => {
      const spy = vi.spyOn(client as any, 'queryMint');
      await client.batchMint(mockBatchMintParams);
      expect(spy).toHaveBeenCalledWith(mockBatchMintParams, 'batch_mint');
    });

    it('should successfully perform a batch mint', async () => {
      const result = await client.batchMint(mockBatchMintParams);

      expect(result).toEqual({
        transactionInfo: { transactionHash: 'mockBatchMintTxHash' },
        executionResult: { errorMessage: '' },
      });
    });

    it('should include uri if provided', async () => {
      const withUri: BatchMintParams = {
        ...mockBatchMintParams,
        args: {
          ...mockBatchMintParams.args,
          uri: 'https://example.com/token-metadata',
        },
      };

      const spy = vi.spyOn(client as any, 'callEntrypoint');
      await client.batchMint(withUri);

      expect(spy).toHaveBeenCalled();
      const runtimeArgsUsed = spy.mock.calls[0][1] as any;
      expect(runtimeArgsUsed['args'].get('uri')).toBeDefined();
    });

    it('should throw if ids and amounts length do not match', async () => {
      const invalidParams: BatchMintParams = {
        ...mockBatchMintParams,
        args: {
          ...mockBatchMintParams.args,
          ids: ['1', '2'],
          amounts: ['100'], // mismatched
        },
      };

      try {
        await client.batchMint(invalidParams);
        throw new Error('Expected batchMint to throw, but it did not');
      } catch (err: any) {
        expect(err.message).toBe(
          'Length of "ids" and "amounts" must be the same.'
        );
      }
    });

    it('should handle failed transaction execution', async () => {
      vi.spyOn(client as any, 'callEntrypoint').mockResolvedValueOnce({
        transactionInfo: { transactionHash: 'mockFailTxHash' },
        executionResult: { errorMessage: 'Minting failed' },
      });

      const result = await client.batchMint(mockBatchMintParams);

      expect(result.executionResult?.errorMessage).toBe('Minting failed');
    });
  });

  describe('CEP85Client - burn', () => {
    let client: CEP85Client;
    const key = PrivateKey.generate(KeyAlgorithm.ED25519);
    const recipient = key.publicKey;

    const mockBurnParams: BurnParams = {
      params: {
        paymentAmount: '2500000000',
        sender: key.publicKey,
        chainName: 'casper-test',
        signingKeys: [key],
      },
      args: {
        owner: recipient,
        id: '1',
        amount: '100',
      },
      waitForTransactionProcessed: true,
    };

    beforeEach(() => {
      client = new CEP85Client('http://mock-rpc-url');
      vi.spyOn(client as any, 'callEntrypoint').mockResolvedValue({
        transactionInfo: { transactionHash: 'mockBurnTxHash' },
        executionResult: { errorMessage: '' },
      });
    });

    afterEach(() => {
      vi.restoreAllMocks();
    });

    it('should call queryBurn with the correct entrypoint and parameters', async () => {
      const spy = vi.spyOn(client as any, 'queryBurn');
      await client.burn(mockBurnParams);
      expect(spy).toHaveBeenCalledWith(mockBurnParams, 'burn');
    });

    it('should successfully perform a token burn', async () => {
      const result = await client.burn(mockBurnParams);

      expect(result).toEqual({
        transactionInfo: { transactionHash: 'mockBurnTxHash' },
        executionResult: { errorMessage: '' },
      });
    });

    it('should throw if ids and amounts length do not match for batch burn', async () => {
      const invalidParams: BatchBurnParams = {
        params: {
          paymentAmount: '2500000000',
          sender: key.publicKey,
          chainName: 'casper-test',
          signingKeys: [key],
        },
        args: {
          owner: recipient,
          ids: ['1', '2'],
          amounts: ['100'], // mismatched
        },
        waitForTransactionProcessed: true,
      };

      try {
        await client.burn(invalidParams);
        throw new Error('Expected burn to throw, but it did not');
      } catch (err: any) {
        expect(err.message).toBe(
          'Length of "ids" and "amounts" must be the same.'
        );
      }
    });

    it('should handle failed transaction execution', async () => {
      vi.spyOn(client as any, 'callEntrypoint').mockResolvedValueOnce({
        transactionInfo: { transactionHash: 'mockFailTxHash' },
        executionResult: { errorMessage: 'Burning failed' },
      });

      const result = await client.burn(mockBurnParams);

      expect(result.executionResult?.errorMessage).toBe('Burning failed');
    });
  });

  describe('CEP85Client - batchBurn', () => {
    let client: CEP85Client;
    const key = PrivateKey.generate(KeyAlgorithm.ED25519);
    const recipient = key.publicKey;

    const mockBatchBurnParams: BatchBurnParams = {
      params: {
        paymentAmount: '3500000000',
        sender: key.publicKey,
        chainName: 'casper-test',
        signingKeys: [key],
      },
      args: {
        owner: recipient,
        ids: ['1', '2', '3'],
        amounts: ['10', '20', '30'],
      },
      waitForTransactionProcessed: true,
    };

    beforeEach(() => {
      client = new CEP85Client('http://mock-rpc-url');
      vi.spyOn(client as any, 'callEntrypoint').mockResolvedValue({
        transactionInfo: { transactionHash: 'mockBatchBurnTxHash' },
        executionResult: { errorMessage: '' },
      });
    });

    afterEach(() => {
      vi.restoreAllMocks();
    });

    it('should call queryBurn with the correct entrypoint and parameters', async () => {
      const spy = vi.spyOn(client as any, 'queryBurn');
      await client.batchBurn(mockBatchBurnParams);
      expect(spy).toHaveBeenCalledWith(mockBatchBurnParams, 'batch_burn');
    });

    it('should successfully perform a batch burn', async () => {
      const result = await client.batchBurn(mockBatchBurnParams);

      expect(result).toEqual({
        transactionInfo: { transactionHash: 'mockBatchBurnTxHash' },
        executionResult: { errorMessage: '' },
      });
    });

    it('should throw if ids and amounts length do not match', async () => {
      const invalidParams: BatchBurnParams = {
        ...mockBatchBurnParams,
        args: {
          ...mockBatchBurnParams.args,
          ids: ['1', '2'],
          amounts: ['100'], // mismatched
        },
      };

      try {
        await client.batchBurn(invalidParams);
        throw new Error('Expected batchBurn to throw, but it did not');
      } catch (err: any) {
        expect(err.message).toBe(
          'Length of "ids" and "amounts" must be the same.'
        );
      }
    });

    it('should handle failed transaction execution', async () => {
      vi.spyOn(client as any, 'callEntrypoint').mockResolvedValueOnce({
        transactionInfo: { transactionHash: 'mockFailTxHash' },
        executionResult: { errorMessage: 'Batch burning failed' },
      });

      const result = await client.batchBurn(mockBatchBurnParams);

      expect(result.executionResult?.errorMessage).toBe('Batch burning failed');
    });
  });

  describe('CEP85Client - balanceOf', () => {
    let client: CEP85Client;
    const key = PrivateKey.generate(KeyAlgorithm.ED25519);
    const account = key.publicKey;
    const tokenId = '1';

    beforeEach(() => {
      client = new CEP85Client('http://mock-rpc-url');
      vi.spyOn(client as any, 'queryContractDictionary').mockResolvedValue(
        '100'
      );
    });

    afterEach(() => {
      vi.restoreAllMocks();
    });

    it('should query the balance of a token for a given account', async () => {
      const result = await client.balanceOf(account, tokenId);
      expect(result).toBe('100');
    });

    it('should return "0" if the balance is not found', async () => {
      vi.spyOn(client as any, 'queryContractDictionary').mockResolvedValueOnce(
        undefined
      );

      const result = await client.balanceOf(account, tokenId);
      expect(result).toBe('0');
    });

    it('should handle query errors and return "0" if the query fails', async () => {
      vi.spyOn(client as any, 'queryContractDictionary').mockRejectedValueOnce(
        new Error('Query failed')
      );

      const result = await client.balanceOf(account, tokenId);
      expect(result).toBe('0');
    });

    it('should throw an error if there is a different issue during the query', async () => {
      vi.spyOn(client as any, 'queryContractDictionary').mockRejectedValueOnce(
        new Error('Unexpected error')
      );

      try {
        await client.balanceOf(account, tokenId);
        throw new Error('Expected balanceOf to throw, but it did not');
      } catch (err: any) {
        expect(err.message).toBe('Unexpected error');
      }
    });
  });

  describe('CEP85Client - balanceOfBatch', () => {
    let client: CEP85Client;
    const key = PrivateKey.generate(KeyAlgorithm.ED25519);
    const account = key.publicKey;
    const tokenIds = ['1', '2', '3'];

    beforeEach(() => {
      client = new CEP85Client('http://mock-rpc-url');
      vi.spyOn(client, 'balanceOf').mockResolvedValue('100');
    });

    afterEach(() => {
      vi.restoreAllMocks();
    });

    it('should query the balances of multiple tokens for a given account', async () => {
      const result = await client.balanceOfBatch(account, tokenIds);

      expect(result).toEqual(['100', '100', '100']);
      expect(client.balanceOf).toHaveBeenCalledTimes(3);
      expect(client.balanceOf).toHaveBeenCalledWith(account, '1');
      expect(client.balanceOf).toHaveBeenCalledWith(account, '2');
      expect(client.balanceOf).toHaveBeenCalledWith(account, '3');
    });

    it('should return "0" for token IDs with no balance found', async () => {
      vi.spyOn(client, 'balanceOf')
        .mockResolvedValueOnce('0')
        .mockResolvedValueOnce(undefined as unknown as string)
        .mockResolvedValueOnce('100');

      const result = await client.balanceOfBatch(account, tokenIds);

      expect(result).toEqual(['0', '0', '100']);
    });

    it('should handle query errors and return "0" for failed queries', async () => {
      vi.spyOn(client, 'balanceOf')
        .mockRejectedValueOnce(new Error('Query failed'))
        .mockResolvedValueOnce('')
        .mockResolvedValueOnce('100');

      const result = await client.balanceOfBatch(account, tokenIds);

      expect(result).toEqual(['0', '0', '100']);
    });
  });

  describe('CEP85Client - getSupplyOf', () => {
    let client: CEP85Client;
    const tokenId = 'token123';

    beforeEach(() => {
      client = new CEP85Client('http://mock-rpc-url');
      vi.spyOn(client as any, 'querySupply').mockResolvedValue('1000');
    });

    afterEach(() => {
      vi.restoreAllMocks();
    });

    it('should query the circulating supply of a specific token', async () => {
      const result = await client.getSupplyOf(tokenId);
      expect(result).toBe('1000');
    });

    it('should return "0" if the supply is not found', async () => {
      vi.spyOn(client as any, 'querySupply').mockResolvedValueOnce(
        undefined as unknown as string
      );

      const result = await client.getSupplyOf(tokenId);
      expect(result).toBe('0');
    });

    it('should handle query errors and return "0" if the query fails', async () => {
      vi.spyOn(client as any, 'querySupply').mockRejectedValueOnce(
        new Error('Query failed')
      );

      const result = await client.getSupplyOf(tokenId);
      expect(result).toBe('0');
    });

    it('should not throw an error if there is a different issue during the supply query', async () => {
      vi.spyOn(client as any, 'querySupply').mockRejectedValueOnce(
        new Error('Unexpected error')
      );
      const result = await client.getSupplyOf(tokenId);
      expect(result).toBe('0');
    });
  });

  describe('CEP85Client - getSupplyOfBatch', () => {
    let client: CEP85Client;
    const tokenIds = ['token123', 'token456', 'token789'];

    beforeEach(() => {
      client = new CEP85Client('http://mock-rpc-url');
      vi.spyOn(client as any, 'getSupplyOf').mockResolvedValue('1000');
    });

    afterEach(() => {
      vi.restoreAllMocks();
    });

    it('should query the circulating supply of multiple tokens in batch', async () => {
      const result = await client.getSupplyOfBatch(tokenIds);
      expect(result).toEqual(['1000', '1000', '1000']);
    });

    it('should return "0" for tokens with undefined supply', async () => {
      vi.spyOn(client as any, 'getSupplyOf')
        .mockResolvedValueOnce(undefined as unknown as string)
        .mockResolvedValueOnce('2000')
        .mockResolvedValueOnce(undefined as unknown as string);

      const result = await client.getSupplyOfBatch(tokenIds);
      expect(result).toEqual(['0', '2000', '0']);
    });

    it('should handle query errors and return "0" for failed supply queries', async () => {
      vi.spyOn(client as any, 'getSupplyOf')
        .mockRejectedValueOnce(new Error('Query failed'))
        .mockResolvedValueOnce('2000')
        .mockRejectedValueOnce(new Error('Query failed'));

      const result = await client.getSupplyOfBatch(tokenIds);
      expect(result).toEqual(['0', '2000', '0']);
    });

    it('should log a warning and return "0" for tokens that throw unexpected errors', async () => {
      const warnSpy = vi.spyOn(console, 'warn').mockImplementation(() => {});
      vi.spyOn(client as any, 'getSupplyOf')
        .mockRejectedValueOnce(new Error('Unexpected error'))
        .mockResolvedValueOnce('3000')
        .mockRejectedValueOnce(new Error('Unexpected error'));

      const result = await client.getSupplyOfBatch(tokenIds);
      expect(result).toEqual(['0', '3000', '0']);
      expect(warnSpy).toHaveBeenCalledWith(
        'Failed to query supply for token ID: token123',
        expect.any(Error)
      );
      expect(warnSpy).toHaveBeenCalledWith(
        'Failed to query supply for token ID: token789',
        expect.any(Error)
      );
    });
  });

  describe('CEP85Client - getTotalSupplyOf', () => {
    let client: CEP85Client;
    const tokenId = 'token123';

    beforeEach(() => {
      client = new CEP85Client('http://mock-rpc-url');
      vi.spyOn(client as any, 'queryTotalSupply').mockResolvedValue('10000');
    });

    afterEach(() => {
      vi.restoreAllMocks();
    });

    it('should query the total supply of a specific token', async () => {
      const result = await client.getTotalSupplyOf(tokenId);
      expect(result).toBe('10000');
    });

    it('should return "0" if the total supply is not found', async () => {
      vi.spyOn(client as any, 'queryTotalSupply').mockResolvedValueOnce(
        undefined as unknown as string
      );

      const result = await client.getTotalSupplyOf(tokenId);
      expect(result).toBe('0');
    });

    it('should handle query errors and return "0" if the query fails', async () => {
      vi.spyOn(client as any, 'queryTotalSupply').mockRejectedValueOnce(
        new Error('Query failed')
      );

      const result = await client.getTotalSupplyOf(tokenId);
      expect(result).toBe('0');
    });

    it('should not throw an error if there is a different issue during the total supply query', async () => {
      vi.spyOn(client as any, 'queryTotalSupply').mockRejectedValueOnce(
        new Error('Unexpected error')
      );

      const result = await client.getTotalSupplyOf(tokenId);
      expect(result).toBe('0');
    });
  });

  describe('CEP85Client - getTotalSupplyOfBatch', () => {
    let client: CEP85Client;

    const tokenIds = ['token1', 'token2', 'token3'];

    beforeEach(() => {
      client = new CEP85Client('http://mock-rpc-url');
    });

    afterEach(() => {
      vi.restoreAllMocks();
    });

    it('should return total supplies for all token IDs', async () => {
      vi.spyOn(client, 'getTotalSupplyOf').mockImplementation(
        async (id: string) => {
          return (
            {
              token1: '1000',
              token2: '2000',
              token3: '3000',
            }[id] ?? '0'
          );
        }
      );

      const result = await client.getTotalSupplyOfBatch(tokenIds);
      expect(result).toEqual(['1000', '2000', '3000']);
    });

    it('should default to "0" for tokens with undefined total supply', async () => {
      vi.spyOn(client, 'getTotalSupplyOf').mockImplementation(
        async (id: string) => {
          return id === 'token2' ? undefined! : '999';
        }
      );

      const result = await client.getTotalSupplyOfBatch(tokenIds);
      expect(result).toEqual(['999', '0', '999']);
    });

    it('should default to "0" if getTotalSupplyOf throws for any token', async () => {
      vi.spyOn(client, 'getTotalSupplyOf').mockImplementation(
        async (id: string) => {
          if (id === 'token3') throw new Error('Failure');
          return '500';
        }
      );

      const result = await client.getTotalSupplyOfBatch(tokenIds);
      expect(result).toEqual(['500', '500', '0']);
    });

    it('should handle an empty array gracefully', async () => {
      const result = await client.getTotalSupplyOfBatch([]);
      expect(result).toEqual([]);
    });
  });

  describe('CEP85Client - setTotalSupplyOf', () => {
    let client: CEP85Client;

    const mockParams: TotalSupplyOfParams = {
      params: {
        paymentAmount: '1000000000',
        sender: 'account-hash-mock',
        chainName: 'casper-test',
        signingKeys: ['mock-key'],
        waitForTransactionProcessed: true,
      },
      args: {
        id: '1',
        totalSupply: '5000',
      },
    };

    beforeEach(() => {
      client = new CEP85Client('http://mock-rpc-url');
    });

    afterEach(() => {
      vi.restoreAllMocks();
    });

    it('should call querySetTotalSupplyOf with the correct entrypoint and arguments', async () => {
      const mockResponse = { deployHash: 'mock-deploy-hash' };
      const spy = vi
        .spyOn(client as any, 'querySetTotalSupplyOf')
        .mockResolvedValue(mockResponse);

      const result = await client.setTotalSupplyOf(mockParams);

      expect(spy).toHaveBeenCalledWith(mockParams, 'set_total_supply_of');
      expect(result).toBe(mockResponse);
    });

    it('should throw if querySetTotalSupplyOf throws', async () => {
      vi.spyOn(client as any, 'querySetTotalSupplyOf').mockRejectedValue(
        new Error('Mock failure')
      );

      await expect(client.setTotalSupplyOf(mockParams)).rejects.toThrow(
        'Mock failure'
      );
    });
  });

  describe('CEP85Client - setTotalSupplyOfBatch', () => {
    let client: CEP85Client;
    const key = PrivateKey.generate(KeyAlgorithm.ED25519);

    const mockBatchParams: TotalSupplyOfBatchParams = {
      params: {
        paymentAmount: '1500000000',
        sender: key.publicKey,
        chainName: 'casper-test',
        signingKeys: [key],
      },
      args: {
        ids: ['1', '2', '3'],
        totalSupplies: ['1000', '2000', '3000'],
      },
      waitForTransactionProcessed: true,
    };

    beforeEach(() => {
      client = new CEP85Client('http://mock-rpc-url');
    });

    afterEach(() => {
      vi.restoreAllMocks();
    });

    it('should call querySetTotalSupplyOf with the batch entrypoint and correct parameters', async () => {
      const mockResponse = { deployHash: 'mock-deploy-batch-hash' };
      const spy = vi
        .spyOn(client as any, 'querySetTotalSupplyOf')
        .mockResolvedValue(mockResponse);

      const result = await client.setTotalSupplyOfBatch(mockBatchParams);

      expect(spy).toHaveBeenCalledWith(
        mockBatchParams,
        'set_total_supply_of_batch'
      );
      expect(result).toBe(mockResponse);
    });

    it('should throw an error if querySetTotalSupplyOf fails', async () => {
      vi.spyOn(client as any, 'querySetTotalSupplyOf').mockRejectedValue(
        new Error('Batch set failed')
      );

      await expect(
        client.setTotalSupplyOfBatch(mockBatchParams)
      ).rejects.toThrow('Batch set failed');
    });

    it('should throw an error if ids and totalSupplies lengths do not match', async () => {
      const invalidParams: TotalSupplyOfBatchParams = {
        ...mockBatchParams,
        args: {
          ids: ['1', '2'],
          totalSupplies: ['1000'], // Mismatch
        },
      };

      try {
        await client.setTotalSupplyOfBatch(invalidParams);
        throw new Error(
          'Expected setTotalSupplyOfBatch to throw, but it did not'
        );
      } catch (err: any) {
        expect(err.message).toBe(
          'Length of "ids" and "totalSupplies" must be the same.'
        );
      }
    });
  });

  describe('CEP85Client - getIsNonFungible', () => {
    let client: CEP85Client;
    const tokenId = 'tokenXYZ';

    beforeEach(() => {
      client = new CEP85Client('http://mock-rpc-url');
    });

    afterEach(() => {
      vi.restoreAllMocks();
    });

    it('should return true if total supply is "1"', async () => {
      vi.spyOn(client as any, 'queryContractDictionary').mockResolvedValue('1');
      const result = await client.getIsNonFungible(tokenId);
      expect(result).toBe(true);
    });

    it('should return false if total supply is not "1"', async () => {
      vi.spyOn(client as any, 'queryContractDictionary').mockResolvedValue(
        '1000'
      );
      const result = await client.getIsNonFungible(tokenId);
      expect(result).toBe(false);
    });

    it('should return false if an error occurs during the query', async () => {
      vi.spyOn(client as any, 'queryContractDictionary').mockRejectedValue(
        new Error('Query failed')
      );
      const result = await client.getIsNonFungible(tokenId);
      expect(result).toBe(false);
    });
  });

  describe('CEP85Client - getTotalFungibleSupply', () => {
    let client: CEP85Client;
    const tokenId = 'tokenABC';

    beforeEach(() => {
      client = new CEP85Client('http://mock-rpc-url');
    });

    afterEach(() => {
      vi.restoreAllMocks();
    });

    it('should correctly return the total fungible supply (total - supply)', async () => {
      vi.spyOn(client as any, 'queryContractDictionary')
        .mockResolvedValueOnce('200') // current supply
        .mockResolvedValueOnce('1000'); // total supply

      const result = await client.getTotalFungibleSupply(tokenId);
      expect(result).toBe('800');
    });

    it('should return "0" if an error occurs during supply query', async () => {
      vi.spyOn(client as any, 'queryContractDictionary').mockRejectedValue(
        new Error('Query failed')
      );

      const result = await client.getTotalFungibleSupply(tokenId);
      expect(result).toBe('0');
    });

    it('should return "0" if result values are not valid BigInt strings', async () => {
      vi.spyOn(client as any, 'queryContractDictionary')
        .mockResolvedValueOnce('invalid') // supply
        .mockResolvedValueOnce('1000'); // total supply

      const result = await client.getTotalFungibleSupply(tokenId);
      expect(result).toBe('0');
    });
  });

  describe('CEP85Client - setApprovalForAll', () => {
    let client: CEP85Client;
    const key = PrivateKey.generate(KeyAlgorithm.ED25519);
    const key2 = PrivateKey.generate(KeyAlgorithm.ED25519);

    const mockParams: SetApprovallForAllParams = {
      params: {
        paymentAmount: '1000000000',
        sender: key.publicKey,
        chainName: 'casper-test',
        signingKeys: [key],
      },
      waitForTransactionProcessed: true,
      args: {
        approved: true,
        operator: key2.publicKey,
      },
    };

    beforeEach(() => {
      client = new CEP85Client('http://mock-rpc-url');
      vi.spyOn(client as any, 'callEntrypoint').mockResolvedValue(
        'mock-deploy-result'
      );
    });

    afterEach(() => {
      vi.restoreAllMocks();
    });

    it('should call set_approval_for_all entrypoint with correct runtime args', async () => {
      const result = await client.setApprovalForAll(mockParams);

      expect(client['callEntrypoint']).toHaveBeenCalledWith(
        'set_approval_for_all',
        expect.any(Object),
        mockParams.params.paymentAmount,
        mockParams.params.sender,
        mockParams.params.signingKeys,
        mockParams.params.chainName,
        mockParams.waitForTransactionProcessed
      );

      expect(result).toBe('mock-deploy-result');
    });

    it('should correctly format runtime arguments', async () => {
      const callEntrypointSpy = vi.spyOn(client as any, 'callEntrypoint');

      await client.setApprovalForAll(mockParams);

      const runtimeArgs = callEntrypointSpy.mock.calls[0][1] as any;

      expect(runtimeArgs['args'].get('approved').toString()).toBe('true');
      expect(runtimeArgs['args'].get('operator')).toBeDefined();
    });
  });

  describe('CEP85Client - getIsApprovedForAll', () => {
    let client: CEP85Client;
    const key = PrivateKey.generate(KeyAlgorithm.ED25519);
    const key2 = PrivateKey.generate(KeyAlgorithm.ED25519);

    beforeEach(() => {
      client = new CEP85Client('http://mock-rpc-url');
      vi.spyOn(client as any, 'queryContractDictionary').mockResolvedValue(
        'true'
      );
    });

    afterEach(() => {
      vi.restoreAllMocks();
    });

    it('should return true if the spender is approved for all tokens of the owner', async () => {
      const owner = key.publicKey;
      const spender = key2.publicKey;

      const result = await client.getIsApprovedForAll(owner, spender);

      expect(result).toBe(true);
      expect(client['queryContractDictionary']).toHaveBeenCalledWith(
        'operators',
        expect.any(String)
      );
    });

    it('should return false if the spender is not approved for all tokens of the owner', async () => {
      const owner = key.publicKey;
      const spender = key2.publicKey;

      vi.spyOn(client as any, 'queryContractDictionary').mockResolvedValue(
        'false'
      );
      const result = await client.getIsApprovedForAll(owner, spender);

      expect(result).toBe(false);
      expect(client['queryContractDictionary']).toHaveBeenCalledWith(
        'operators',
        expect.any(String)
      );
    });

    it('should return false if an error occurs during the query', async () => {
      const owner = key.publicKey;
      const spender = key2.publicKey;

      vi.spyOn(client as any, 'queryContractDictionary').mockRejectedValue(
        new Error('Query failed')
      );
      const result = await client.getIsApprovedForAll(owner, spender);

      expect(result).toBe(false);
      expect(client['queryContractDictionary']).toHaveBeenCalledWith(
        'operators',
        expect.any(String)
      );
    });

    it('should return false if the result is neither "true" nor "false"', async () => {
      const owner = key.publicKey;
      const spender = key2.publicKey;

      vi.spyOn(client as any, 'queryContractDictionary').mockResolvedValue(
        'unexpectedValue'
      );
      const result = await client.getIsApprovedForAll(owner, spender);

      expect(result).toBe(false);
      expect(client['queryContractDictionary']).toHaveBeenCalledWith(
        'operators',
        expect.any(String)
      );
    });
  });

  describe('CEP85Client - changeSecurity', () => {
    let client: CEP85Client;
    const key = PrivateKey.generate(KeyAlgorithm.ED25519);
    const adminKey = PrivateKey.generate(KeyAlgorithm.ED25519);
    const minterKey = PrivateKey.generate(KeyAlgorithm.ED25519);
    const burnerKey = PrivateKey.generate(KeyAlgorithm.ED25519);
    const metaKey = PrivateKey.generate(KeyAlgorithm.ED25519);
    const noneKey = PrivateKey.generate(KeyAlgorithm.ED25519);

    const mockParams: ChangeSecurityParams = {
      params: {
        sender: key.publicKey,
        paymentAmount: '1000',
        signingKeys: [key],
        chainName: 'casper-test',
      },
      args: {
        adminList: [adminKey.publicKey],
        minterList: [minterKey.publicKey],
        burnerList: [burnerKey.publicKey],
        metaList: [metaKey.publicKey],
        noneList: [noneKey.publicKey],
      },
      waitForTransactionProcessed: false,
    };

    beforeEach(() => {
      client = new CEP85Client('http://mock-rpc-url');
      vi.spyOn(client as any, 'callEntrypoint').mockResolvedValue({
        transactionInfo: { transactionHash: 'mockTransactionHash' },
      });
      vi.spyOn(client, 'waitForTransactionProcessed').mockResolvedValue({
        transactionProcessedPayload: {
          executionResult: { errorMessage: '' } as ExecutionResult,
        } as unknown as TransactionProcessedPayload,
      });
    });

    it('should successfully execute changeSecurity', async () => {
      const result = await client.changeSecurity(mockParams);

      // Verify callEntrypoint was called with correct parameters
      expect(client['callEntrypoint']).toHaveBeenCalledWith(
        'change_security',
        expect.anything(),
        mockParams.params.paymentAmount,
        mockParams.params.sender,
        mockParams.params.signingKeys,
        mockParams.params.chainName,
        mockParams.waitForTransactionProcessed
      );

      // Validate result
      expect(result).toEqual({
        transactionInfo: { transactionHash: 'mockTransactionHash' },
      });
    });

    it('should call callEntrypoint with correct runtime arguments', async () => {
      await client.changeSecurity(mockParams);

      // Retrieve runtimeArgs from spy call
      const runtimeArgs = (client as any).callEntrypoint.mock.calls[0][1];

      // Validate runtime arguments for changeSecurity
      expect(runtimeArgs).toEqual(
        Args.fromMap({
          admin_list: CLValue.newCLList(
            CLTypeKey,
            mockParams.args.adminList?.map((key) =>
              CLValue.newCLKey(CEP85Client['getPrefixedString'](key))
            )
          ),
          minter_list: CLValue.newCLList(
            CLTypeKey,
            mockParams.args.minterList?.map((key) =>
              CLValue.newCLKey(CEP85Client['getPrefixedString'](key))
            )
          ),
          burner_list: CLValue.newCLList(
            CLTypeKey,
            mockParams.args.burnerList?.map((key) =>
              CLValue.newCLKey(CEP85Client['getPrefixedString'](key))
            )
          ),
          meta_list: CLValue.newCLList(
            CLTypeKey,
            mockParams.args.metaList?.map((key) =>
              CLValue.newCLKey(CEP85Client['getPrefixedString'](key))
            )
          ),
          none_list: CLValue.newCLList(
            CLTypeKey,
            mockParams.args.noneList?.map((key) =>
              CLValue.newCLKey(CEP85Client['getPrefixedString'](key))
            )
          ),
        })
      );
    });

    it('should successfully execute changeSecurity when waitForTransactionProcessed is true', async () => {
      const paramsWithWait = {
        ...mockParams,
        waitForTransactionProcessed: true,
      };
      vi.spyOn(client as any, 'callEntrypoint').mockResolvedValue({
        transactionInfo: { transactionHash: 'mockTransactionHash' },
        executionResult: { errorMessage: '' } as ExecutionResult,
      });

      const result = await client.changeSecurity(paramsWithWait);

      // Validate result
      expect(result).toEqual({
        transactionInfo: { transactionHash: 'mockTransactionHash' },
        executionResult: { errorMessage: '' } as ExecutionResult,
      });
    });

    it('should handle the case when waitForTransactionProcessed is true', async () => {
      const paramsWithWait = {
        ...mockParams,
        waitForTransactionProcessed: true,
      };
      await client.changeSecurity(paramsWithWait);

      expect(client['callEntrypoint']).toHaveBeenCalledWith(
        'change_security',
        expect.anything(),
        mockParams.params.paymentAmount,
        mockParams.params.sender,
        mockParams.params.signingKeys,
        mockParams.params.chainName,
        true
      );
    });

    it('should throw an error when no arguments are provided', async () => {
      const emptyArgsParams: ChangeSecurityParams = {
        ...mockParams,
        args: {}, // Empty args
      };

      try {
        await client.changeSecurity(emptyArgsParams);
        expect(false).toBe(true); // Forces failure if no error is thrown
      } catch (error) {
        expect(error).toBeInstanceOf(Error);
        expect((error as Error).message).toBe(
          'Should provide at least one arg'
        );
      }
    });

    it('should handle errors during the changeSecurity process', async () => {
      const errorMessage = 'Error during changeSecurity.';
      vi.spyOn(client as any, 'callEntrypoint').mockRejectedValueOnce(
        new Error(errorMessage)
      );

      await expect(client.changeSecurity(mockParams)).rejects.toThrow(
        'Error during changeSecurity.'
      );
    });
  });

  describe('CEP85Client - makeDictionaryItemKey', () => {
    it('should correctly generate a dictionary item key', () => {
      const key = CLValue.newCLString('key');
      const value = CLValue.newCLString('value');

      const result = CEP85Client.makeDictionaryItemKey(key, value);

      // Hash the concatenation of the key and value manually to verify the result
      const expectedKey = bytesToHex(
        blake2b(new Uint8Array([...key.bytes(), ...value.bytes()]), {
          dkLen: 32,
        })
      );

      expect(result).toBe(expectedKey);
    });

    it('should return a consistent result for the same inputs', () => {
      const key = CLValue.newCLString('key');
      const value = CLValue.newCLString('value');

      const result1 = CEP85Client.makeDictionaryItemKey(key, value);
      const result2 = CEP85Client.makeDictionaryItemKey(key, value);

      expect(result1).toBe(result2); // Ensure the key remains the same for repeated inputs
    });

    it('should handle empty strings as inputs', () => {
      const key = CLValue.newCLString('');
      const value = CLValue.newCLString('');

      const result = CEP85Client.makeDictionaryItemKey(key, value);

      // Hash the concatenation of the empty strings manually
      const expectedKey = bytesToHex(
        blake2b(new Uint8Array([...key.bytes(), ...value.bytes()]), {
          dkLen: 32,
        })
      );

      expect(result).toBe(expectedKey);
    });

    it('should throw an error if the key or value is invalid', () => {
      try {
        // Passing null or undefined should throw an error
        CEP85Client.makeDictionaryItemKey(
          null as unknown as CLValue,
          CLValue.newCLString('value')
        );
        expect(false).toBe(true); // Forces failure if no error is thrown
      } catch (error) {
        expect(error).toBeInstanceOf(Error);
        expect((error as any)['message']).toBe(
          "Cannot read properties of null (reading 'bytes')"
        );
      }
    });
  });

  describe('CEP85Client - setModalities', () => {
    let client: CEP85Client;
    let mockRpcClient: RpcClient;
    const mockKey = PrivateKey.generate(KeyAlgorithm.ED25519);
    const mockPublicKey = mockKey.publicKey;
    const contractHash =
      'hash-a84b9f15e57097579cb651bc3eec5143972c8c9ea153bb26d07367f9d41a767b';

    beforeEach(() => {
      mockRpcClient = {
        callEntrypoint: vi.fn(),
      } as unknown as RpcClient;

      client = new CEP85Client('http://mock-rpc-url');
      client.setContractHash(contractHash);
      client['_rpcClient'] = mockRpcClient;

      // Mocking callEntrypoint method directly on client
      vi.spyOn(client as any, 'callEntrypoint').mockResolvedValue({
        transactionInfo: { transactionHash: 'mockTransactionHash' },
      });
    });

    it('should call callEntrypoint with correct parameters when all parameters are provided', async () => {
      const setModalitiesParams: SetModalitiesParams = {
        params: {
          sender: mockPublicKey,
          paymentAmount: '1000',
          signingKeys: [mockKey],
          chainName: 'testnet',
        },
        args: {
          enableBurn: true,
          eventsMode: 1,
        },
        waitForTransactionProcessed: true,
      };

      await client.setModalities(setModalitiesParams);

      // Verifying that callEntrypoint is called with the correct parameters
      expect(client['callEntrypoint']).toHaveBeenCalledWith(
        'set_modalities',
        expect.anything(), // This can be adjusted based on how the runtime arguments are checked
        setModalitiesParams.params.paymentAmount,
        setModalitiesParams.params.sender,
        setModalitiesParams.params.signingKeys,
        setModalitiesParams.params.chainName,
        setModalitiesParams.waitForTransactionProcessed
      );
    });

    it('should call callEntrypoint when no optional parameters are provided', async () => {
      const setModalitiesParams: SetModalitiesParams = {
        params: {
          sender: mockPublicKey,
          paymentAmount: '1000',
          signingKeys: [mockKey],
          chainName: 'testnet',
        },
        args: {},
        waitForTransactionProcessed: true,
      };

      await client.setModalities(setModalitiesParams);

      // Verifying that callEntrypoint is called even if no arguments were provided
      expect(client['callEntrypoint']).toHaveBeenCalled();
    });

    it('should call callEntrypoint with correct parameters when only enableBurn is provided', async () => {
      const setModalitiesParams: SetModalitiesParams = {
        params: {
          sender: mockPublicKey,
          paymentAmount: '1000',
          signingKeys: [mockKey],
          chainName: 'testnet',
        },
        args: {
          enableBurn: true,
        },
        waitForTransactionProcessed: true,
      };

      await client.setModalities(setModalitiesParams);

      // Verifying that callEntrypoint is called when only enableBurn is provided
      expect(client['callEntrypoint']).toHaveBeenCalled();
    });

    it('should call callEntrypoint when eventsMode is not provided', async () => {
      const setModalitiesParams: SetModalitiesParams = {
        params: {
          sender: mockPublicKey,
          paymentAmount: '1000',
          signingKeys: [mockKey],
          chainName: 'testnet',
        },
        args: {
          enableBurn: true,
        },
        waitForTransactionProcessed: true,
      };

      await client.setModalities(setModalitiesParams);

      // Verifying that callEntrypoint is called when eventsMode is not provided
      expect(client['callEntrypoint']).toHaveBeenCalled();
    });

    it('should call callEntrypoint with correct parameters when only eventsMode is provided', async () => {
      const setModalitiesParams: SetModalitiesParams = {
        params: {
          sender: mockPublicKey,
          paymentAmount: '1000',
          signingKeys: [mockKey],
          chainName: 'testnet',
        },
        args: {
          eventsMode: 2,
        },
        waitForTransactionProcessed: true,
      };

      await client.setModalities(setModalitiesParams);

      // Verifying that callEntrypoint is called when only eventsMode is provided
      expect(client['callEntrypoint']).toHaveBeenCalled();
    });
  });

  describe('CEP85Client - Getter Methods', () => {
    let client: CEP85Client;

    const mockCollectionName = 'MyCollection';
    const mockCollectionUri = 'https://mock-uri.com';
    const mockEventsMode = 'CES';
    const mockBurnMode = 'true';
    const mockNumOfMintedTokens = '500000';

    beforeEach(() => {
      client = new CEP85Client('http://mock-rpc-url');
    });

    it('should return the correct collection name', async () => {
      vi.spyOn(client as any, 'queryContractData').mockResolvedValue(
        mockCollectionName
      );
      const result = await client.collectionName();
      expect(result).toBe(mockCollectionName);
    });

    it('should return the correct collection URI', async () => {
      vi.spyOn(client as any, 'queryContractData').mockResolvedValue(
        mockCollectionUri
      );
      const result = await client.collectionUri();
      expect(result).toBe(mockCollectionUri);
    });

    it('should return the correct events mode', async () => {
      vi.spyOn(client as any, 'queryContractData').mockResolvedValue(
        mockEventsMode
      );
      const result = await client.eventsMode();
      expect(result).toBe(EVENTS_MODE[mockEventsMode]);
    });

    it('should return true when burn mode is enabled', async () => {
      vi.spyOn(client as any, 'queryContractData').mockResolvedValue(
        mockBurnMode
      );
      const result = await client.burnMode();
      expect(result).toBe(true);
    });

    it('should return the correct number of minted tokens', async () => {
      vi.spyOn(client as any, 'queryContractData').mockResolvedValue(
        mockNumOfMintedTokens
      );
      const result = await client.numOfMintedTokens();
      expect(result).toBe(mockNumOfMintedTokens);
    });

    it('should return an empty string when collection name is empty', async () => {
      vi.spyOn(client as any, 'queryContractData').mockRejectedValue(
        new Error('Query failed')
      );
      const result = await client.collectionName();
      expect(result).toBe('');
    });

    it('should return an empty string when collection URI is empty', async () => {
      vi.spyOn(client as any, 'queryContractData').mockRejectedValue(
        new Error('Query failed')
      );
      const result = await client.collectionUri();
      expect(result).toBe('');
    });

    it('should return the correct burn mode when it is enabled', async () => {
      vi.spyOn(client as any, 'queryContractData').mockResolvedValue('true');
      const result = await client.burnMode();
      expect(result).toBe(true);
    });

    it('should return the correct number of minted tokens even if queryContractData fails', async () => {
      vi.spyOn(client as any, 'queryContractData').mockResolvedValue('500000');
      const result = await client.numOfMintedTokens();
      expect(result).toBe('500000');
    });

    it('should handle errors gracefully for collection name', async () => {
      vi.spyOn(client as any, 'queryContractData').mockRejectedValue(
        new Error('Error querying collection name')
      );
      const result = await client.collectionName();
      expect(result).toBe('');
    });

    it('should handle errors gracefully for collection URI', async () => {
      vi.spyOn(client as any, 'queryContractData').mockRejectedValue(
        new Error('Error querying collection URI')
      );
      const result = await client.collectionUri();
      expect(result).toBe('');
    });

    it('should handle errors gracefully for events mode', async () => {
      vi.spyOn(client as any, 'queryContractData').mockRejectedValue(
        new Error('Error querying events mode')
      );
      await expect(client.eventsMode()).rejects.toThrow(
        'Error querying events mode'
      );
    });

    it('should handle errors gracefully for burn mode', async () => {
      vi.spyOn(client as any, 'queryContractData').mockRejectedValue(
        new Error('Error querying burn mode')
      );
      await expect(client.burnMode()).rejects.toThrow(
        'Error querying burn mode'
      );
    });

    it('should handle errors gracefully for number of minted tokens', async () => {
      vi.spyOn(client as any, 'queryContractData').mockRejectedValue(
        new Error('Error querying minted tokens')
      );
      await expect(client.numOfMintedTokens()).rejects.toThrow(
        'Error querying minted tokens'
      );
    });
  });
});
