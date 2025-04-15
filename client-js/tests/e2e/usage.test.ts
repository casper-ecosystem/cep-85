import { expect, describe, it, beforeEach } from 'vitest';
import { RPC_URL, SSE_URL, CHAIN_NAME } from '../../config';
import {
  CEP85Client,
  ChangeSecurityArgs,
  EVENTS_MODE,
  SetApprovallForAllParams,
  TransactionParams,
  TransferArgs,
} from '../../src';
import { getAccountInfo, findKeyFromAccountNamedKeys } from '../utils';
import { install, owner, ali, bob, mint, uri, eventsMode } from './helpers';

let client: CEP85Client;
const name = `TEST_CEP85_E2E_${Math.floor(Math.random() * 1000000)}`;

let id: string;

describe('CEP85Client - E2E Usage', () => {
  beforeEach(async () => {
    client = new CEP85Client(RPC_URL, SSE_URL, CHAIN_NAME);
    await install(client, name);
    const account = await getAccountInfo(RPC_URL, owner.publicKey),
      contractHash = findKeyFromAccountNamedKeys(
        account,
        `cep85_contract_hash_${name}`
      );
    expect(contractHash).toBeDefined();
    client.setContractHash(contractHash);

    id = `${+(await client.numOfMintedTokens())}`;
  }, 60000);

  it('should return correct values for collectionName, collectionUri, eventsMode, and burnMode', async () => {
    const account = await getAccountInfo(RPC_URL, owner.publicKey);
    const contractHash = findKeyFromAccountNamedKeys(
      account,
      `cep85_contract_hash_${name}`
    );
    expect(contractHash).toBeDefined();
    client.setContractHash(contractHash);

    // Query each value
    const collectionName = await client.collectionName();
    const collectionUri = await client.collectionUri();
    const burn = await client.burnMode();
    const mode = await client.eventsMode();

    // Assert expected results
    expect(collectionName).toBe(name);
    expect(collectionUri).toBe(uri);
    expect(burn).toBe(true);
    expect(mode).toBe(EVENTS_MODE[eventsMode]);
  }, 60000);

  it('should mint tokens successfully', async () => {
    const initialBalance = await client.balanceOf(ali.publicKey, id);
    const mintAmount = BigInt(10_000_000_000);

    const mintResult = await mint(client, id, mintAmount);

    expect(mintResult.transactionInfo.transactionHash).toBeDefined();
    expect(mintResult.executionResult?.errorMessage).toBeFalsy();

    const newBalance = await client.balanceOf(ali.publicKey, id);
    expect(BigInt(newBalance)).toBe(BigInt(initialBalance) + mintAmount);
  }, 60000);

  it('should batch mint tokens successfully', async () => {
    const initialBalanceBob = await client.balanceOf(bob.publicKey, id);

    const mintAmounts = [BigInt(5_000_000_000), BigInt(3_000_000_000)];

    const id2 = `${+(await client.numOfMintedTokens()) + 1}`;

    const batchMintArgs = {
      recipient: bob.publicKey,
      ids: [id, id2],
      amounts: mintAmounts.map(String),
    };

    const params: TransactionParams = {
      sender: owner.publicKey,
      paymentAmount: String(10_000_000_000),
      signingKeys: [owner],
    };

    const batchMintResult = await client.batchMint({
      params,
      args: batchMintArgs,
      waitForTransactionProcessed: true,
    });

    expect(batchMintResult.transactionInfo.transactionHash).toBeDefined();
    expect(batchMintResult.executionResult?.errorMessage).toBeFalsy();

    let newBalanceBob = await client.balanceOf(bob.publicKey, id);
    expect(BigInt(newBalanceBob)).toBe(
      BigInt(initialBalanceBob) + mintAmounts[0]
    );

    newBalanceBob = await client.balanceOf(bob.publicKey, id2);
    expect(BigInt(newBalanceBob)).toBe(
      BigInt(initialBalanceBob) + mintAmounts[1]
    );
  }, 60000);

  it('should burn tokens successfully', async () => {
    const mintAmount = BigInt(10_000_000_000);

    await mint(client, id, mintAmount);

    const initialBalance = await client.balanceOf(ali.publicKey, id);

    const burnAmount = BigInt(10_000_000_000);

    const burnResult = await client.burn({
      params: {
        sender: ali.publicKey,
        paymentAmount: String(5_000_000_000),
        signingKeys: [ali],
      },
      args: {
        owner: ali.publicKey,
        id,
        amount: String(burnAmount),
      },
      waitForTransactionProcessed: true,
    });

    expect(burnResult.transactionInfo.transactionHash).toBeDefined();
    expect(burnResult.executionResult?.errorMessage).toBeFalsy();

    const newBalance = await client.balanceOf(ali.publicKey, id);
    expect(BigInt(newBalance)).toBe(BigInt(initialBalance) - burnAmount);
  }, 60000);

  it('should batch burn tokens successfully', async () => {
    const mintAmounts = [BigInt(5_000_000_000), BigInt(3_000_000_000)];

    const id2 = `${+(await client.numOfMintedTokens()) + 1}`;

    const batchMintArgs = {
      recipient: ali.publicKey,
      ids: [id, id2],
      amounts: mintAmounts.map(String),
    };

    let params: TransactionParams = {
      sender: owner.publicKey,
      paymentAmount: String(10_000_000_000),
      signingKeys: [owner],
    };

    await client.batchMint({
      params,
      args: batchMintArgs,
      waitForTransactionProcessed: true,
    });

    const batchBurnArgs = {
      owner: ali.publicKey,
      ids: [id, id2],
      amounts: [
        BigInt(1_000_000_000).toString(),
        BigInt(2_000_000_000).toString(),
      ],
    };

    params = {
      sender: ali.publicKey,
      paymentAmount: String(5_000_000_000),
      signingKeys: [ali],
    };

    // Perform the batch burn
    const batchBurnResult = await client.batchBurn({
      params,
      args: batchBurnArgs,
      waitForTransactionProcessed: true,
    });

    expect(batchBurnResult.transactionInfo.transactionHash).toBeDefined();
    expect(batchBurnResult.executionResult?.errorMessage).toBeFalsy();

    const newBalanceAli = await client.balanceOf(ali.publicKey, id);
    const newBalanceAli2 = await client.balanceOf(ali.publicKey, id2);

    expect(BigInt(newBalanceAli)).toBe(
      BigInt(mintAmounts[0]) - BigInt(1_000_000_000)
    );

    expect(BigInt(newBalanceAli2)).toBe(
      BigInt(mintAmounts[1]) - BigInt(2_000_000_000)
    );
  }, 60000);

  it('should transfer tokens successfully', async () => {
    const mintAmount = BigInt(10_000_000_000);
    await mint(client, id, mintAmount);

    const initialBalanceBob = await client.balanceOf(bob.publicKey, id),
      initialBalanceAli = await client.balanceOf(ali.publicKey, id),
      params: TransactionParams = {
        sender: ali.publicKey,
        paymentAmount: String(5_000_000_000),
        signingKeys: [ali],
      },
      transferArgs: TransferArgs = {
        from: ali.publicKey,
        to: bob.publicKey,
        id,
        amount: String(1_000_000_000),
      },
      transferResult = await client.transfer({
        params,
        args: transferArgs,
        waitForTransactionProcessed: true,
      });

    expect(transferResult.transactionInfo.transactionHash).toBeDefined();
    expect(transferResult.executionResult?.errorMessage).toBeFalsy();

    const newBalance = await client.balanceOf(ali.publicKey, id);
    expect(BigInt(newBalance)).toBe(
      BigInt(initialBalanceAli) - BigInt(1_000_000_000)
    );

    const newBalanceAli = await client.balanceOf(bob.publicKey, id);
    expect(BigInt(newBalanceAli)).toBe(
      BigInt(initialBalanceBob) + BigInt(1_000_000_000)
    );
  }, 60000);

  it('should batch transfer tokens successfully', async () => {
    const mintAmount = BigInt(10_000_000_000);
    await mint(client, id, mintAmount);

    const initialBalanceBob = await client.balanceOf(bob.publicKey, id);
    const initialBalanceAli = await client.balanceOf(ali.publicKey, id);

    const id2 = `${+(await client.numOfMintedTokens()) + 1}`;
    const mintAmount2 = BigInt(5_000_000_000);
    await mint(client, id2, mintAmount2);

    const batchTransferArgs = {
      from: ali.publicKey,
      to: bob.publicKey,
      ids: [id, id2],
      amounts: [
        BigInt(1_000_000_000).toString(),
        BigInt(2_000_000_000).toString(),
      ],
    };

    const params: TransactionParams = {
      sender: ali.publicKey,
      paymentAmount: String(5_000_000_000),
      signingKeys: [ali],
    };

    // Perform the batch transfer
    const batchTransferResult = await client.batchTransfer({
      params,
      args: batchTransferArgs,
      waitForTransactionProcessed: true,
    });

    expect(batchTransferResult.transactionInfo.transactionHash).toBeDefined();
    expect(batchTransferResult.executionResult?.errorMessage).toBeFalsy();

    const newBalanceAli = await client.balanceOf(ali.publicKey, id);
    const newBalanceBob = await client.balanceOf(bob.publicKey, id);
    const newBalanceAli2 = await client.balanceOf(ali.publicKey, id2);
    const newBalanceBob2 = await client.balanceOf(bob.publicKey, id2);

    // Ensure the balances are updated correctly
    expect(BigInt(newBalanceAli)).toBe(
      BigInt(initialBalanceAli) - BigInt(1_000_000_000)
    );
    expect(BigInt(newBalanceBob)).toBe(
      BigInt(initialBalanceBob) + BigInt(1_000_000_000)
    );

    expect(BigInt(newBalanceAli2)).toBe(
      BigInt(mintAmount2) - BigInt(2_000_000_000)
    );
    expect(BigInt(newBalanceBob2)).toBe(BigInt(0) + BigInt(2_000_000_000));
  }, 60000);

  it('should return correct balances using balanceOfBatch', async () => {
    const mintAmounts = [BigInt(5_000_000_000), BigInt(3_000_000_000)];

    const id2 = `${+(await client.numOfMintedTokens()) + 1}`;

    const batchMintArgs = {
      recipient: ali.publicKey,
      ids: [id, id2],
      amounts: mintAmounts.map(String),
    };

    const mintParams: TransactionParams = {
      sender: owner.publicKey,
      paymentAmount: String(10_000_000_000),
      signingKeys: [owner],
    };

    await client.batchMint({
      params: mintParams,
      args: batchMintArgs,
      waitForTransactionProcessed: true,
    });

    const balances = await client.balanceOfBatch(ali.publicKey, [id, id2]);

    expect(balances).toHaveLength(2);

    expect(BigInt(balances[0])).toBe(BigInt(mintAmounts[0]));
    expect(BigInt(balances[1])).toBe(BigInt(mintAmounts[1]));
  }, 60000);

  it('should return correct supply using getSupplyOf', async () => {
    const mintAmount = BigInt(10_000_000_000);
    const mintResult = await mint(client, id, mintAmount);

    expect(mintResult.transactionInfo.transactionHash).toBeDefined();
    expect(mintResult.executionResult?.errorMessage).toBeFalsy();

    const supply = await client.getSupplyOf(id);

    expect(BigInt(supply)).toBe(mintAmount);
  }, 60000);

  it('should return correct supplies using getSupplyOfBatch', async () => {
    const mintAmounts = [BigInt(7_000_000_000), BigInt(4_000_000_000)];

    const id2 = `${+(await client.numOfMintedTokens()) + 1}`;

    const batchMintArgs = {
      recipient: ali.publicKey,
      ids: [id, id2],
      amounts: mintAmounts.map(String),
    };

    const params: TransactionParams = {
      sender: owner.publicKey,
      paymentAmount: String(10_000_000_000),
      signingKeys: [owner],
    };

    const mintResult = await client.batchMint({
      params,
      args: batchMintArgs,
      waitForTransactionProcessed: true,
    });

    expect(mintResult.transactionInfo.transactionHash).toBeDefined();
    expect(mintResult.executionResult?.errorMessage).toBeFalsy();

    const supplies = await client.getSupplyOfBatch([id, id2]);

    expect(supplies.length).toBe(2);
    expect(BigInt(supplies[0])).toBe(mintAmounts[0]);
    expect(BigInt(supplies[1])).toBe(mintAmounts[1]);
  }, 60000);

  it('should return correct supply using getSupplyOf', async () => {
    const mintAmount = BigInt(6_000_000_000);

    // Mint tokens to create circulating supply
    const mintResult = await mint(client, id, mintAmount);

    expect(mintResult.transactionInfo.transactionHash).toBeDefined();
    expect(mintResult.executionResult?.errorMessage).toBeFalsy();

    // Query supply using getSupplyOf
    const supply = await client.getSupplyOf(id);

    // Validate the supply matches the minted amount
    expect(BigInt(supply)).toBe(mintAmount);
  }, 60000);

  it('should return correct total supplies using getTotalSupplyOfBatch', async () => {
    const mintAmount1 = BigInt(7_000_000_000);
    const mintAmount2 = BigInt(5_000_000_000);

    const id2 = `${+(await client.numOfMintedTokens()) + 1}`;

    const mintArgs = {
      recipient: ali.publicKey,
      ids: [id, id2],
      amounts: [mintAmount1.toString(), mintAmount2.toString()],
    };

    const params: TransactionParams = {
      sender: owner.publicKey,
      paymentAmount: String(15_000_000_000),
      signingKeys: [owner],
    };

    const mintResult = await client.batchMint({
      params,
      args: mintArgs,
      waitForTransactionProcessed: true,
    });

    expect(mintResult.transactionInfo.transactionHash).toBeDefined();
    expect(mintResult.executionResult?.errorMessage).toBeFalsy();

    // Burn part of id1
    const burnAmount = BigInt(3_000_000_000);

    const burnResult = await client.burn({
      params: {
        sender: ali.publicKey,
        paymentAmount: String(5_000_000_000),
        signingKeys: [ali],
      },
      args: {
        owner: ali.publicKey,
        id,
        amount: burnAmount.toString(),
      },
      waitForTransactionProcessed: true,
    });

    expect(burnResult.transactionInfo.transactionHash).toBeDefined();
    expect(burnResult.executionResult?.errorMessage).toBeFalsy();

    // Test total supply remains unchanged after burn
    const totalSupplies = await client.getTotalSupplyOfBatch([id, id2]);

    expect(BigInt(totalSupplies[0])).toBe(mintAmount1);
    expect(BigInt(totalSupplies[1])).toBe(mintAmount2);
  }, 60000);

  it('should set total supply of a token using setTotalSupplyOf', async () => {
    const newTotalSupply = BigInt(9_000_000_000);
    const params: TransactionParams = {
      sender: owner.publicKey,
      paymentAmount: String(10_000_000_000),
      signingKeys: [owner],
    };

    const setSupplyResult = await client.setTotalSupplyOf({
      params,
      args: {
        id,
        totalSupply: newTotalSupply.toString(),
      },
      waitForTransactionProcessed: true,
    });

    expect(setSupplyResult.transactionInfo.transactionHash).toBeDefined();
    expect(setSupplyResult.executionResult?.errorMessage).toBeFalsy();

    const updatedTotalSupply = await client.getTotalSupplyOf(id);
    expect(BigInt(updatedTotalSupply)).toBe(newTotalSupply);
  }, 60000);

  it('should return correct supply values using getSupplyOfBatch', async () => {
    const mintAmounts = [BigInt(4_000_000_000), BigInt(6_000_000_000)];

    const id2 = `${+(await client.numOfMintedTokens()) + 1}`;

    const batchMintArgs = {
      recipient: ali.publicKey,
      ids: [id, id2],
      amounts: mintAmounts.map(String),
    };

    const params: TransactionParams = {
      sender: owner.publicKey,
      paymentAmount: String(10_000_000_000),
      signingKeys: [owner],
    };

    const mintResult = await client.batchMint({
      params,
      args: batchMintArgs,
      waitForTransactionProcessed: true,
    });

    expect(mintResult.transactionInfo.transactionHash).toBeDefined();
    expect(mintResult.executionResult?.errorMessage).toBeFalsy();

    const supplies = await client.getSupplyOfBatch([id, id2]);

    expect(supplies.length).toBe(2);
    expect(BigInt(supplies[0])).toBe(mintAmounts[0]);
    expect(BigInt(supplies[1])).toBe(mintAmounts[1]);
  }, 60000);

  it('should return correct total supply using getTotalSupplyOf', async () => {
    const mintAmount = BigInt(7_000_000_000);
    const mintArgs = {
      recipient: ali.publicKey,
      ids: [id],
      amounts: [mintAmount.toString()],
    };

    const params: TransactionParams = {
      sender: owner.publicKey,
      paymentAmount: String(10_000_000_000),
      signingKeys: [owner],
    };

    const mintResult = await client.batchMint({
      params,
      args: mintArgs,
      waitForTransactionProcessed: true,
    });

    expect(mintResult.transactionInfo.transactionHash).toBeDefined();
    expect(mintResult.executionResult?.errorMessage).toBeFalsy();

    const burnAmount = BigInt(3_000_000_000);

    const burnResult = await client.burn({
      params: {
        sender: ali.publicKey,
        paymentAmount: String(5_000_000_000),
        signingKeys: [ali],
      },
      args: {
        owner: ali.publicKey,
        id,
        amount: String(burnAmount),
      },
      waitForTransactionProcessed: true,
    });

    expect(burnResult.transactionInfo.transactionHash).toBeDefined();
    expect(burnResult.executionResult?.errorMessage).toBeFalsy();

    const totalSupply = await client.getTotalSupplyOf(id);

    expect(BigInt(totalSupply)).toBe(mintAmount);
  }, 60000);

  it('should set total supply of multiple tokens using setTotalSupplyOfBatch', async () => {
    const id2 = `${+(await client.numOfMintedTokens()) + 1}`;
    const totalSupplies = [BigInt(9_000_000_000), BigInt(6_000_000_000)];

    const params: TransactionParams = {
      sender: owner.publicKey,
      paymentAmount: String(10_000_000_000),
      signingKeys: [owner],
    };

    const batchSetSupplyResult = await client.setTotalSupplyOfBatch({
      params,
      args: {
        ids: [id, id2],
        totalSupplies: totalSupplies.map((amount) => amount.toString()),
      },
      waitForTransactionProcessed: true,
    });

    expect(batchSetSupplyResult.transactionInfo.transactionHash).toBeDefined();
    expect(batchSetSupplyResult.executionResult?.errorMessage).toBeFalsy();

    const updatedSupplies = await client.getTotalSupplyOfBatch([id, id2]);

    expect(BigInt(updatedSupplies[0])).toBe(totalSupplies[0]);
    expect(BigInt(updatedSupplies[1])).toBe(totalSupplies[1]);
  }, 60000);

  it('should correctly identify non-fungible tokens using getIsNonFungible', async () => {
    const id2 = `${+(await client.numOfMintedTokens()) + 1}`;
    const mintArgs1 = {
      recipient: ali.publicKey,
      ids: [id, id2],
      amounts: ['1', '2'],
    };

    const params: TransactionParams = {
      sender: owner.publicKey,
      paymentAmount: String(10_000_000_000),
      signingKeys: [owner],
    };

    const mintResult1 = await client.batchMint({
      params,
      args: mintArgs1,
      waitForTransactionProcessed: true,
    });

    expect(mintResult1.transactionInfo.transactionHash).toBeDefined();
    expect(mintResult1.executionResult?.errorMessage).toBeFalsy();

    const isNonFungible1 = await client.getIsNonFungible(id);
    expect(isNonFungible1).toBe(true);

    // Check if token is non-fungible (should be false since minted more than 1)
    const isNonFungible2 = await client.getIsNonFungible(id2);
    expect(isNonFungible2).toBe(false);
  }, 60000);

  it('should correctly calculate total fungible supply using getTotalFungibleSupply', async () => {
    const id2 = `${+(await client.numOfMintedTokens()) + 1}`;

    const mintArgs = {
      recipient: ali.publicKey,
      ids: [id, id2],
      amounts: ['1', '1000'],
    };

    const params: TransactionParams = {
      sender: owner.publicKey,
      paymentAmount: String(10_000_000_000),
      signingKeys: [owner],
    };

    const mintResult = await client.batchMint({
      params,
      args: mintArgs,
      waitForTransactionProcessed: true,
    });

    expect(mintResult.transactionInfo.transactionHash).toBeDefined();
    expect(mintResult.executionResult?.errorMessage).toBeFalsy();

    // Verify that `id` is non-fungible
    const isNonFungible1 = await client.getIsNonFungible(id);
    expect(isNonFungible1).toBe(true);

    // Verify that `id2` is fungible
    const isNonFungible2 = await client.getIsNonFungible(id2);
    expect(isNonFungible2).toBe(false);

    // Check total fungible supply for the fungible token id2 is zero, all token were minted
    const totalFungibleSupply2 = await client.getTotalFungibleSupply(id2);
    expect(BigInt(totalFungibleSupply2)).toBe(BigInt(0));

    // For the non-fungible token (id), the fungible supply should be zero
    const totalFungibleSupply1 = await client.getTotalFungibleSupply(id);
    expect(BigInt(totalFungibleSupply1)).toBe(BigInt(0));
  }, 60000);

  it('should correctly calculate total fungible supply using getTotalFungibleSupply after setting total supply for id2', async () => {
    const id2 = `${+(await client.numOfMintedTokens()) + 1}`;

    // First mint the tokens
    const mintArgs = {
      recipient: ali.publicKey,
      ids: [id, id2],
      amounts: ['1', '1000'],
    };

    let params: TransactionParams = {
      sender: owner.publicKey,
      paymentAmount: String(10_000_000_000),
      signingKeys: [owner],
    };

    const mintResult = await client.batchMint({
      params,
      args: mintArgs,
      waitForTransactionProcessed: true,
    });

    expect(mintResult.transactionInfo.transactionHash).toBeDefined();
    expect(mintResult.executionResult?.errorMessage).toBeFalsy();

    // Check total fungible supply for the fungible token id2 (should initially be 0)
    const totalFungibleSupply2 = await client.getTotalFungibleSupply(id2);
    expect(BigInt(totalFungibleSupply2)).toBe(BigInt(0));

    // Check total fungible supply for the non-fungible token id (should be 0)
    const totalFungibleSupply1 = await client.getTotalFungibleSupply(id);
    expect(BigInt(totalFungibleSupply1)).toBe(BigInt(0));

    // Set the total supply of id2 to a new value
    params = {
      sender: owner.publicKey,
      paymentAmount: String(3_000_000_000),
      signingKeys: [owner],
    };

    const setTotalSupplyResult = await client.setTotalSupplyOf({
      params,
      args: {
        id: id2,
        totalSupply: '5000',
      },
      waitForTransactionProcessed: true,
    });

    expect(setTotalSupplyResult.transactionInfo.transactionHash).toBeDefined();
    expect(setTotalSupplyResult.executionResult?.errorMessage).toBeFalsy();

    // After setting the total supply for id2, check the fungible supply again
    const updatedTotalFungibleSupply2 =
      await client.getTotalFungibleSupply(id2);
    expect(BigInt(updatedTotalFungibleSupply2)).toBe(BigInt(4000)); // 5000 total - 1000 already minted
  }, 60000);

  it('should set URI successfully', async () => {
    const mintAmount = BigInt(10_000_000_000);
    await mint(client, id, mintAmount);

    const newUri = 'https://example.com/{id}.json';

    const params: TransactionParams = {
      sender: owner.publicKey,
      paymentAmount: String(5_000_000_000),
      signingKeys: [owner],
    };

    const setUriArgs = {
      id,
      uri: newUri,
    };

    const setUriResult = await client.setUri({
      params,
      args: setUriArgs,
      waitForTransactionProcessed: true,
    });

    expect(setUriResult.transactionInfo.transactionHash).toBeDefined();
    expect(setUriResult.executionResult?.errorMessage).toBeFalsy();

    const updatedUri = await client.getURI(id);
    expect(updatedUri).toBe(`https://example.com/${id}.json`);
  }, 60000);

  it('should get default URI for a token by ID', async () => {
    const mintAmount = BigInt(10_000_000_000);
    await mint(client, id, mintAmount);

    const uriForToken = await client.getURI(id);

    expect(uriForToken).toBe(uri.replace('{id}', id));
  }, 60000);

  it('should get URI for the collection when no ID is provided', async () => {
    const collectionUri = 'https://example.com/collection/{id}.json';

    // Set the collection URI
    await client.setUri({
      params: {
        sender: owner.publicKey,
        paymentAmount: String(5_000_000_000),
        signingKeys: [owner],
      },
      args: {
        uri: collectionUri,
      },
      waitForTransactionProcessed: true,
    });

    // Get the URI for the collection (no ID passed)
    const uriForCollection = await client.getURI();

    // Ensure the URI is correctly set for the collection
    expect(uriForCollection).toBe('https://example.com/collection/{id}.json');
  }, 60000);

  it('should set and verify approval for all using setApprovalForAll and getIsApprovedForAll', async () => {
    // Ensure ali is not approved initially
    let isApprovedBefore = await client.getIsApprovedForAll(
      owner.publicKey,
      ali.publicKey
    );
    expect(isApprovedBefore).toBe(false);

    // Set approval for ali to manage owner's tokens
    const approvalParams: SetApprovallForAllParams = {
      params: {
        sender: owner.publicKey,
        paymentAmount: String(5_000_000_000),
        signingKeys: [owner],
      },
      args: {
        operator: ali.publicKey,
        approved: true,
      },
      waitForTransactionProcessed: true,
    };

    const approvalResult = await client.setApprovalForAll(approvalParams);

    expect(approvalResult.transactionInfo.transactionHash).toBeDefined();
    expect(approvalResult.executionResult?.errorMessage).toBeFalsy();

    // Verify that approval is now set
    const isApprovedAfter = await client.getIsApprovedForAll(
      owner.publicKey,
      ali.publicKey
    );
    expect(isApprovedAfter).toBe(true);

    // Optionally, revoke the approval
    const revokeParams: SetApprovallForAllParams = {
      ...approvalParams,
      args: {
        operator: ali.publicKey,
        approved: false,
      },
    };

    const revokeResult = await client.setApprovalForAll(revokeParams);
    expect(revokeResult.transactionInfo.transactionHash).toBeDefined();
    expect(revokeResult.executionResult?.errorMessage).toBeFalsy();

    const isApprovedRevoked = await client.getIsApprovedForAll(
      owner.publicKey,
      ali.publicKey
    );
    expect(isApprovedRevoked).toBe(false);
  }, 60000);

  it('should correctly update modalities using setModalities', async () => {
    const params: TransactionParams = {
      sender: owner.publicKey,
      paymentAmount: String(10_000_000_000),
      signingKeys: [owner],
    };

    // Update modalities: disable burn and change events mode to Native
    const setModalitiesResult = await client.setModalities({
      params,
      args: {
        enableBurn: false,
        eventsMode: EVENTS_MODE.Native,
      },
      waitForTransactionProcessed: true,
    });

    expect(setModalitiesResult.transactionInfo.transactionHash).toBeDefined();
    expect(setModalitiesResult.executionResult?.errorMessage).toBeFalsy();

    // Verify the values have been updated in the contract
    const burnMode = await client.burnMode();
    const eventsMode = await client.eventsMode();

    expect(burnMode).toBe(false);
    expect(eventsMode).toBe('Native');
  }, 60000);

  it('should change security settings successfully', async () => {
    const newAdminList = [owner.publicKey];
    const newMinterList = [ali.publicKey];
    const newBurnerList = [ali.publicKey];
    const newMetaList = [owner.publicKey];
    const newNoneList = [bob.publicKey];

    const params: TransactionParams = {
      sender: owner.publicKey,
      paymentAmount: String(5_000_000_000),
      signingKeys: [owner],
    };

    const changeSecurityArgs: ChangeSecurityArgs = {
      adminList: newAdminList,
      minterList: newMinterList,
      burnerList: newBurnerList,
      metaList: newMetaList,
      noneList: newNoneList,
    };

    const changeSecurityResult = await client.changeSecurity({
      params,
      args: changeSecurityArgs,
      waitForTransactionProcessed: true,
    });

    expect(changeSecurityResult.transactionInfo.transactionHash).toBeDefined();
    expect(changeSecurityResult.executionResult?.errorMessage).toBeFalsy();
  }, 60000);
});
