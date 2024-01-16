import {
  Keys,
  DeployUtil,
  CLPublicKey,
  CLKey,
  CLValueBuilder,
  CLValue,
  Contracts
} from "casper-js-sdk";
import { utf8ToBytes } from "@noble/hashes/utils";
import { BigNumber } from "@ethersproject/bignumber";
import {
  CEP85Client, EventsMode,
} from "../src/index";

import INSTALL_ARGS_JSON from "./jsons/install-args.json";
import INSTALL_ARGS_JSON_BURNER_LIST from "./jsons/install-args-burner_list.json";
import UPGRADE_ARGS_JSON from "./jsons/upgrade-args.json";

import MINT_DEPLOY_ARGS_JSON from "./jsons/mint-args.json";
import BATCH_MINT_DEPLOY_ARGS_JSON from "./jsons/batch_mint-args.json";
import BURN_DEPLOY_ARGS_JSON from "./jsons/burn-args.json";
import BATCH_DEPLOY_ARGS_JSON from "./jsons/batch_burn-args.json";
import TRANSFER_DEPLOY_ARGS_JSON from "./jsons/transfer-args.json";
import BATCH_TRANSFER_DEPLOY_ARGS_JSON from "./jsons/batch_transfer-args.json";
import SET_APPROVAL_FOR_ALL_JSON from "./jsons/set_approval_for_all-args.json";
import SET_MODALITIES_JSON from "./jsons/set_modalities-args.json";
import SET_TOTAL_SUPPLY_OF_JSON from "./jsons/set_total_supply_of-args.json";
import SET_TOTAL_SUPPLY_OF_BATCH_JSON from "./jsons/set_total_supply_of_batch-args.json";
import SET_URI_JSON from "./jsons/set_uri-args.json";
import CHANGE_SECURITY_JSON from "./jsons/change_security-args.json";

interface DeployJson {
  deploy: {
    session: {
      ModuleBytes: {
        args: string[];
      };
      StoredContractByHash: {
        entry_point: string;
      };
    };
  };
}

const name = "casper_test";
const uri = "https://test-cdn-domain/{id}.json";
const id = "1";
const ids = ['3', '4'];
const mintAmount = '20';
const transferAmount = '10';
const burnAmount = '1';
const totalSupply = '40';
const text = "hello Casper";
const data = utf8ToBytes(text);

const MOCKED_OWNER_PUBKEY = CLPublicKey.fromHex(
  "0145fb72c75e1b459839555d70356a5e6172e706efa204d86c86050e2f7878960f"
);
const MOCKED_RECIPIENT_PUBKEY = CLPublicKey.fromHex(
  "0112b28459a5c90b7c90f700788302d463b5c29acfef1dd3da5d1ef162f71061f7"
);
const keyPair = Keys.Ed25519.new();
const cc = new CEP85Client("http://localhost:11101/rpc", "casper-net-1");

describe("CEP85Client", () => {

  beforeAll(() => {
    cc.setContractHash(
      "hash-0c0f9056626a55273bd8238f595908f2e4d78acc2546bf1f78f39f814bc60fe4"
    );
  });

  afterEach(() => {
    jest.restoreAllMocks(); // Reset all mocks after each test
  });

  it("Should correctly initialize itself when correct hash is provided", () => {
    expect(cc.contractClient).toBeInstanceOf(Contracts.Contract);
    expect(cc.contractHashKey).toBeInstanceOf(CLKey);
  });

  it("Should correctly construct contract install deploy", () => {
    const installDeploy = cc.install(
      {
        name,
        uri,
        events_mode: EventsMode.CES,
      },
      "250000000000",
      keyPair.publicKey
    );

    expect(installDeploy).toBeInstanceOf(DeployUtil.Deploy);
    const deployJson = (DeployUtil.deployToJson(installDeploy) as DeployJson).deploy;
    expect(deployJson.session.ModuleBytes.args.sort()).toEqual(
      INSTALL_ARGS_JSON.sort()
    );
  });

  it("Should correctly construct contract install deploy with burner list", () => {
    const installDeploy = cc.install(
      {
        name,
        uri,
        events_mode: EventsMode.CES,
        enable_burn: true,
        burner_list: [MOCKED_RECIPIENT_PUBKEY]
      },
      "250000000000",
      keyPair.publicKey
    );

    expect(installDeploy).toBeInstanceOf(DeployUtil.Deploy);
    const deployJson = (DeployUtil.deployToJson(installDeploy) as DeployJson).deploy;
    expect(deployJson.session.ModuleBytes.args.sort()).toEqual(
      INSTALL_ARGS_JSON_BURNER_LIST.sort()
    );
  });

  it("Should correctly construct deploy for 'setUri'", () => {
    const burnDeploy = cc.setUri(
      {
        id,
        uri: uri.replace('test', 'usage'),
      },
      "13000000000",
      keyPair.publicKey
    );

    expect(burnDeploy).toBeInstanceOf(DeployUtil.Deploy);
    const deployJson = (DeployUtil.deployToJson(burnDeploy) as DeployJson).deploy;
    expect(deployJson.session.StoredContractByHash.entry_point).toEqual(
      "set_uri"
    );
    expect(deployJson.session).toEqual(
      SET_URI_JSON
    );
  });

  it("Should correctly construct deploy for 'mint'", () => {
    const mintDeploy = cc.mint(
      {
        recipient: MOCKED_OWNER_PUBKEY,
        id,
        amount: "1",
      },
      "3000000000",
      keyPair.publicKey
    );

    expect(mintDeploy).toBeInstanceOf(DeployUtil.Deploy);
    const deployJson = (DeployUtil.deployToJson(mintDeploy) as DeployJson).deploy;
    expect(deployJson.session.StoredContractByHash.entry_point).toEqual(
      "mint"
    );
    expect(deployJson.session).toEqual(
      MINT_DEPLOY_ARGS_JSON
    );
  });

  it("Should correctly construct deploy for 'batchMint'", () => {
    const mintDeploy = cc.batchMint(
      {
        recipient: MOCKED_OWNER_PUBKEY,
        ids,
        amounts: [mintAmount, mintAmount],
      },
      "3000000000",
      keyPair.publicKey
    );

    expect(mintDeploy).toBeInstanceOf(DeployUtil.Deploy);
    const deployJson = (DeployUtil.deployToJson(mintDeploy) as DeployJson).deploy;
    expect(deployJson.session.StoredContractByHash.entry_point).toEqual(
      "batch_mint"
    );
    expect(deployJson.session).toEqual(
      BATCH_MINT_DEPLOY_ARGS_JSON
    );
  });

  it("Should correctly construct deploy for 'transfer'", () => {
    const transferDeploy = cc.transfer(
      {
        from: MOCKED_OWNER_PUBKEY,
        to: MOCKED_RECIPIENT_PUBKEY,
        id,
        amount: "1",
        data
      },
      "13000000000",
      keyPair.publicKey
    );

    expect(transferDeploy).toBeInstanceOf(DeployUtil.Deploy);
    const deployJson = (DeployUtil.deployToJson(transferDeploy) as DeployJson).deploy;
    expect(deployJson.session.StoredContractByHash.entry_point).toEqual(
      "safe_transfer_from"
    );
    expect(deployJson.session).toEqual(
      TRANSFER_DEPLOY_ARGS_JSON
    );
  });

  it("Should correctly construct deploy for 'batchTransfer'", () => {
    const transferDeploy = cc.batchTransfer(
      {
        from: MOCKED_OWNER_PUBKEY,
        to: MOCKED_RECIPIENT_PUBKEY,
        ids,
        amounts: [transferAmount, transferAmount],

      },
      "13000000000",
      keyPair.publicKey
    );

    expect(transferDeploy).toBeInstanceOf(DeployUtil.Deploy);
    const deployJson = (DeployUtil.deployToJson(transferDeploy) as DeployJson).deploy;
    expect(deployJson.session.StoredContractByHash.entry_point).toEqual(
      "safe_batch_transfer_from"
    );
    expect(deployJson.session).toEqual(
      BATCH_TRANSFER_DEPLOY_ARGS_JSON
    );
  });

  it("Should correctly construct deploy for 'burn'", () => {
    const burnDeploy = cc.burn(
      {
        owner: MOCKED_RECIPIENT_PUBKEY,
        id,
        amount: "1"
      },
      "13000000000",
      keyPair.publicKey
    );

    expect(burnDeploy).toBeInstanceOf(DeployUtil.Deploy);
    const deployJson = (DeployUtil.deployToJson(burnDeploy) as DeployJson).deploy;
    expect(deployJson.session.StoredContractByHash.entry_point).toEqual(
      "burn"
    );
    expect(deployJson.session).toEqual(
      BURN_DEPLOY_ARGS_JSON
    );
  });

  it("Should correctly construct deploy for 'batchBurn'", () => {
    const burnDeploy = cc.batchBurn(
      {
        owner: MOCKED_RECIPIENT_PUBKEY,
        ids,
        amounts: [burnAmount, burnAmount],
      },
      "13000000000",
      keyPair.publicKey
    );

    expect(burnDeploy).toBeInstanceOf(DeployUtil.Deploy);
    const deployJson = (DeployUtil.deployToJson(burnDeploy) as DeployJson).deploy;
    expect(deployJson.session.StoredContractByHash.entry_point).toEqual(
      "batch_burn"
    );
    expect(deployJson.session).toEqual(
      BATCH_DEPLOY_ARGS_JSON
    );
  });

  it("Should correctly construct deploy for 'setApprovalForAll'", () => {
    const deploy = cc.setApprovalForAll(
      {
        operator: MOCKED_RECIPIENT_PUBKEY,
        approved: true,
      },
      "13000000000",
      keyPair.publicKey
    );

    expect(deploy).toBeInstanceOf(DeployUtil.Deploy);
    const deployJson = (DeployUtil.deployToJson(deploy) as DeployJson).deploy;
    expect(deployJson.session.StoredContractByHash.entry_point).toEqual(
      "set_approval_for_all"
    );
    expect(deployJson.session).toEqual(
      SET_APPROVAL_FOR_ALL_JSON
    );
  });

  it("Should correctly construct deploy for 'setTotalSupplyOf'", () => {
    const deploy = cc.setTotalSupplyOf(
      {
        id,
        total_supply: totalSupply,
      },
      "13000000000",
      keyPair.publicKey
    );

    expect(deploy).toBeInstanceOf(DeployUtil.Deploy);
    const deployJson = (DeployUtil.deployToJson(deploy) as DeployJson).deploy;
    expect(deployJson.session.StoredContractByHash.entry_point).toEqual(
      "set_total_supply_of"
    );
    expect(deployJson.session).toEqual(
      SET_TOTAL_SUPPLY_OF_JSON
    );
  });

  it("Should correctly construct deploy for 'setTotalSupplyOfBatch'", () => {
    const deploy = cc.setTotalSupplyOfBatch(
      {
        ids,
        total_supplies: [totalSupply, totalSupply],
      },
      "13000000000",
      keyPair.publicKey
    );

    expect(deploy).toBeInstanceOf(DeployUtil.Deploy);
    const deployJson = (DeployUtil.deployToJson(deploy) as DeployJson).deploy;
    expect(deployJson.session.StoredContractByHash.entry_point).toEqual(
      "set_total_supply_of_batch"
    );
    expect(deployJson.session).toEqual(
      SET_TOTAL_SUPPLY_OF_BATCH_JSON
    );
  });

  it("Should correctly construct deploy for 'changeSecurity'", () => {
    const deploy = cc.changeSecurity(
      {
        admin_list: [MOCKED_RECIPIENT_PUBKEY],
        minter_list: [MOCKED_RECIPIENT_PUBKEY],
        burner_list: [MOCKED_RECIPIENT_PUBKEY],
        meta_list: [MOCKED_RECIPIENT_PUBKEY],
        none_list: [MOCKED_RECIPIENT_PUBKEY],
      },
      "13000000000",
      keyPair.publicKey
    );

    expect(deploy).toBeInstanceOf(DeployUtil.Deploy);
    const deployJson = (DeployUtil.deployToJson(deploy) as DeployJson).deploy;
    expect(deployJson.session.StoredContractByHash.entry_point).toEqual(
      "change_security"
    );
    expect(deployJson.session).toEqual(
      CHANGE_SECURITY_JSON
    );
  });

  it("Should correctly construct deploy for 'setModalities'", () => {
    const setModalitiesDeploy = cc.setModalities(
      {
        events_mode: EventsMode.NoEvents,
        enable_burn: true,
      },
      "13000000000",
      keyPair.publicKey
    );

    expect(setModalitiesDeploy).toBeInstanceOf(DeployUtil.Deploy);
    const deployJson = (DeployUtil.deployToJson(setModalitiesDeploy) as DeployJson).deploy;
    expect(deployJson.session.StoredContractByHash.entry_point).toEqual(
      "set_modalities"
    );
    expect(deployJson.session).toEqual(
      SET_MODALITIES_JSON
    );
  });

  it("Should correctly construct contract upgrade deploy", () => {
    const upgradeDeploy = cc.upgrade(
      {
        name
      },
      "250000000000",
      keyPair.publicKey
    );

    expect(upgradeDeploy).toBeInstanceOf(DeployUtil.Deploy);
    const deployJson = (DeployUtil.deployToJson(upgradeDeploy) as DeployJson).deploy;
    expect(deployJson.session.ModuleBytes.args.sort()).toEqual(
      UPGRADE_ARGS_JSON.sort()
    );
  });

  it("Should correctly return for 'getBalanceOf'", async () => {
    let balance = await cc.getBalanceOf(
      MOCKED_RECIPIENT_PUBKEY,
      id
    );
    expect(balance).toBe(
      '0'
    );

    const mockValue = '10';
    const mockResult = CLValueBuilder.u256(mockValue) as CLValue;
    jest.spyOn(cc.contractClient, 'queryContractDictionary').mockResolvedValue(mockResult);
    balance = await cc.getBalanceOf(
      MOCKED_RECIPIENT_PUBKEY,
      id
    );
    expect(balance).toBe(
      mockValue
    );
  });

  it("Should correctly return for 'getBalanceOfBatch'", async () => {
    let balance = await cc.getBalanceOfBatch(
      MOCKED_RECIPIENT_PUBKEY,
      ids
    );
    expect(balance).toEqual(
      ['0', '0']
    );

    const mockValue = '10';
    const mockResult = CLValueBuilder.u256(mockValue) as CLValue;
    jest.spyOn(cc.contractClient, 'queryContractDictionary').mockResolvedValue(mockResult);
    balance = await cc.getBalanceOfBatch(
      MOCKED_RECIPIENT_PUBKEY,
      ids
    );
    expect(balance).toEqual(
      ['10', '10']
    );
  });

  it("Should correctly return for 'getIsApprovedForAll'", async () => {
    let approved = await cc.getIsApprovedForAll(
      MOCKED_OWNER_PUBKEY,
      MOCKED_RECIPIENT_PUBKEY,
    );
    expect(approved).toBeFalsy();

    const mockValue = true;
    const mockResult = CLValueBuilder.bool(mockValue) as CLValue;
    jest.spyOn(cc.contractClient, 'queryContractDictionary').mockResolvedValue(mockResult);
    approved = await cc.getIsApprovedForAll(
      MOCKED_OWNER_PUBKEY,
      MOCKED_RECIPIENT_PUBKEY,
    );
    expect(approved).toBeTruthy();
  });

  it("Should correctly return for 'getSupplyOf'", async () => {
    let supply = await cc.getSupplyOf(
      id
    );
    expect(supply).toBe(
      '0'
    );

    const mockValue = '10';
    const mockResult = CLValueBuilder.u256(mockValue) as CLValue;
    jest.spyOn(cc.contractClient, 'queryContractDictionary').mockResolvedValue(mockResult);
    supply = await cc.getSupplyOf(
      id
    );
    expect(supply).toBe(
      mockValue
    );
  });

  it("Should correctly return for 'getSupplyOfBatch'", async () => {
    let supply = await cc.getSupplyOfBatch(
      ids
    );
    expect(supply).toEqual(
      ['0', '0']
    );

    const mockValue = '10';
    const mockResult = CLValueBuilder.u256(mockValue) as CLValue;
    jest.spyOn(cc.contractClient, 'queryContractDictionary').mockResolvedValue(mockResult);
    supply = await cc.getSupplyOfBatch(
      ids
    );
    expect(supply).toEqual(
      ['10', '10']
    );
  });

  it("Should correctly return for 'getTotalSupplyOf'", async () => {
    let supply = await cc.getTotalSupplyOf(
      id
    );
    expect(supply).toBe(
      '0'
    );

    const mockValue = '10';
    const mockResult = CLValueBuilder.u256(mockValue) as CLValue;
    jest.spyOn(cc.contractClient, 'queryContractDictionary').mockResolvedValue(mockResult);
    supply = await cc.getTotalSupplyOf(
      id
    );
    expect(supply).toBe(
      mockValue
    );
  });

  it("Should correctly return for 'getTotalSupplyOfBatch'", async () => {
    let supply = await cc.getTotalSupplyOfBatch(
      ids
    );
    expect(supply).toEqual(
      ['0', '0']
    );

    const mockValue = '10';
    const mockResult = CLValueBuilder.u256(mockValue) as CLValue;
    jest.spyOn(cc.contractClient, 'queryContractDictionary').mockResolvedValue(mockResult);
    supply = await cc.getTotalSupplyOfBatch(
      ids
    );
    expect(supply).toEqual(
      ['10', '10']
    );
  });

  it("Should correctly return for 'getURI'", async () => {
    let actualUri = await cc.getURI(
      id
    );
    expect(actualUri).toBe("");
    const mockValue = uri.replace('test', 'usage');
    const mockResult = CLValueBuilder.string(mockValue) as CLValue;
    jest.spyOn(cc.contractClient, 'queryContractDictionary').mockResolvedValue(mockResult);
    actualUri = await cc.getURI(
      id
    );
    expect(actualUri).toBe(mockValue.replace('{id}', id));
  });

  it("Should correctly return for 'getIsNonFungible'", async () => {
    let isNFT = await cc.getIsNonFungible(
      id
    );
    expect(isNFT).toBeFalsy();

    const mockValue = 1;
    const mockResult = CLValueBuilder.u256(mockValue) as CLValue;
    jest.spyOn(cc.contractClient, 'queryContractDictionary').mockResolvedValue(mockResult);
    isNFT = await cc.getIsNonFungible(
      id
    );
    expect(isNFT).toBeTruthy();
  });

  it("Should correctly return for 'getTotalFungibleSupply'", async () => {
    let fungibleSupply = await cc.getTotalFungibleSupply(
      id
    );
    expect(fungibleSupply).toBe('0');

    const mockValue = '10';
    const mockResult = CLValueBuilder.u256(mockValue) as CLValue;
    jest.spyOn(cc.contractClient, 'queryContractDictionary').mockResolvedValue(mockResult);
    fungibleSupply = await cc.getTotalFungibleSupply(
      id
    );
    expect(fungibleSupply).toBe('0');
  });

  it("Should correctly return for 'collectionName'", async () => {
    let collectionName = await cc.collectionName();
    expect(collectionName).toBe('');

    const mockValue = 'test';
    jest.spyOn(cc.contractClient, 'queryContractData').mockResolvedValue(mockValue);
    collectionName = await cc.collectionName();
    expect(collectionName).toBe(mockValue);
  });

  it("Should correctly return for 'collectionUri'", async () => {
    let collectionUri = await cc.collectionUri();
    expect(collectionUri).toBe('');

    const mockValue = 'test';
    jest.spyOn(cc.contractClient, 'queryContractData').mockResolvedValue(mockValue);
    collectionUri = await cc.collectionUri();
    expect(collectionUri).toBe(mockValue);
  });

  it("Should correctly return for 'getEventsMode'", async () => {
    let actualEventsMode = await cc.getEventsMode();
    expect(actualEventsMode).toBe(EventsMode[EventsMode.NoEvents] as keyof typeof EventsMode);

    const mockValue = BigNumber.from(EventsMode.CES);
    jest.spyOn(cc.contractClient, 'queryContractData').mockResolvedValue(mockValue);
    actualEventsMode = await cc.getEventsMode();
    expect(actualEventsMode).toBe(EventsMode[EventsMode.CES] as keyof typeof EventsMode);
  });

  it('should generate a valid dictionary item key for balances dict', () => {
    // Call the function with actual input
    const owner = CLValueBuilder.key(MOCKED_OWNER_PUBKEY);
    const tokenID = CLValueBuilder.u256(1);
    const dictionaryItemKeyForBalancesDict = CEP85Client.makeDictionaryItemKey(owner, tokenID);
    expect(dictionaryItemKeyForBalancesDict).toBe("bb0a3d6e53ffeaa21385ff2c9a5ed057b5c12bcdb3df7c30e89c924f17eccf9b");
  });

  it('should generate a valid dictionary item key for operators dict', () => {
    // Call the function with actual input
    const owner = CLValueBuilder.key(MOCKED_OWNER_PUBKEY);
    const operator = CLValueBuilder.key(MOCKED_RECIPIENT_PUBKEY);
    const dictionaryItemKeyForOperatorsDict = CEP85Client.makeDictionaryItemKey(owner, operator);
    expect(dictionaryItemKeyForOperatorsDict).toBe("d6a8988bc9827114c64fcc0d7950c4fab1727207a3609cfc5ae635667f7b2e49");
  });

});
