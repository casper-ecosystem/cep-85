import {
  CEP85Client, EventsMode,
} from "../src/index";

import {
  Keys,
  DeployUtil,
  CLPublicKey,
  CLKey,
  Contracts,
} from "casper-js-sdk";

import INSTALL_ARGS_JSON from "./jsons/install-args.json";
import INSTALL_ARGS_JSON_BURNER_LIST from "./jsons/install-args-burner_list.json";
import SET_VARIABLES_ARGS_JSON from "./jsons/set-variables-args.json";
import MINT_DEPLOY_ARGS_JSON from "./jsons/mint-args.json";
import BURN_DEPLOY_ARGS_JSON from "./jsons/burn-args.json";
import TRANSFER_DEPLOY_ARGS_JSON from "./jsons/transfer-args.json";
import BALANCE_OF_DEPLOY_ARGS_JSON from "./jsons/balance-of-args.json";

const name = "casper_test";
const uri = "https://test-cdn-domain/{id}.json";
const id = "1";

const MOCKED_OWNER_PUBKEY = CLPublicKey.fromHex(
  "0145fb72c75e1b459839555d70356a5e6172e706efa204d86c86050e2f7878960f"
);
const MOCKED_RECIPIENT_PUBKEY = CLPublicKey.fromHex(
  "0112b28459a5c90b7c90f700788302d463b5c29acfef1dd3da5d1ef162f71061f7"
);
const keyPair = Keys.Ed25519.new();
const cc = new CEP85Client("http://localhost:11101/rpc", "casper-net-1");

describe("CEP85Client", () => {

  cc.setContractHash(
    "hash-0c0f9056626a55273bd8238f595908f2e4d78acc2546bf1f78f39f814bc60fe4"
  );

  it("Should correctly construct contract install deploy", async () => {
    const installDeploy = cc.install(
      {
        name,
        uri,
        eventsMode: EventsMode.CES,
      },
      "250000000000",
      keyPair.publicKey
    );

    const JSONDeploy = DeployUtil.deployToJson(installDeploy) as any;

    expect(installDeploy).toBeInstanceOf(DeployUtil.Deploy);
    expect(JSONDeploy.deploy.session.ModuleBytes.args.sort()).toEqual(
      INSTALL_ARGS_JSON.sort()
    );
  });

  it("Should correctly construct contract install deploy with burner list", async () => {
    const installDeploy = cc.install(
      {
        name,
        uri,
        eventsMode: EventsMode.CES,
        burner_list: [MOCKED_RECIPIENT_PUBKEY]
      },
      "250000000000",
      keyPair.publicKey
    );

    const JSONDeploy = DeployUtil.deployToJson(installDeploy) as any;

    expect(installDeploy).toBeInstanceOf(DeployUtil.Deploy);
    expect(JSONDeploy.deploy.session.ModuleBytes.args.sort()).toEqual(
      INSTALL_ARGS_JSON_BURNER_LIST.sort()
    );
  });

  it("Should correctly initialize itself when correct hash is provided", async () => {
    expect(cc.contractClient).toBeInstanceOf(Contracts.Contract);
    expect(cc.contractHashKey).toBeInstanceOf(CLKey);
  });

  xit("Should correctly construct deploy for 'set_variables'", async () => {
    const setVariablesDeploy = cc.setVariables(
      {
      },
      "250000000000",
      keyPair.publicKey
    );

    const JSONDeploy = DeployUtil.deployToJson(setVariablesDeploy) as any;

    expect(setVariablesDeploy).toBeInstanceOf(DeployUtil.Deploy);
    expect(JSONDeploy.deploy.session.StoredContractByHash.entry_point).toEqual(
      "set_variables"
    );
    expect(JSONDeploy.deploy.session.StoredContractByHash.args).toEqual(
      SET_VARIABLES_ARGS_JSON
    );
  });

  it("Should correctly construct deploy for 'mint'", async () => {
    const mintDeploy = cc.mint(
      {
        recipient: MOCKED_OWNER_PUBKEY,
        id,
        amount: "1",
      },
      { useSessionCode: false },
      "3000000000",
      keyPair.publicKey
    );

    const JSONDeploy = DeployUtil.deployToJson(mintDeploy) as any;

    expect(mintDeploy).toBeInstanceOf(DeployUtil.Deploy);
    expect(JSONDeploy.deploy.session).toEqual(
      MINT_DEPLOY_ARGS_JSON
    );
  });

  it("Should correctly construct deploy for 'transfer'", async () => {
    const transferDeploy = cc.transfer(
      {
        from: MOCKED_OWNER_PUBKEY,
        to: MOCKED_RECIPIENT_PUBKEY,
        id,
        amount: "1",
      },
      { useSessionCode: true },
      "13000000000",
      keyPair.publicKey
    );

    const JSONDeploy = DeployUtil.deployToJson(transferDeploy) as any;

    expect(transferDeploy).toBeInstanceOf(DeployUtil.Deploy);
    expect(JSONDeploy.deploy.session).toEqual(
      TRANSFER_DEPLOY_ARGS_JSON
    );
  });

  it("Should correctly construct deploy for 'burn'", async () => {
    const burnDeploy = cc.burn(
      {
        owner: MOCKED_RECIPIENT_PUBKEY,
        id,
        amount: "1"
      },
      "13000000000",
      keyPair.publicKey
    );

    const JSONDeploy = DeployUtil.deployToJson(burnDeploy) as any;

    expect(burnDeploy).toBeInstanceOf(DeployUtil.Deploy);
    expect(JSONDeploy.deploy.session.StoredContractByHash.entry_point).toEqual(
      "burn"
    );
    console.log(JSON.stringify(JSONDeploy.deploy.session));
    expect(JSONDeploy.deploy.session).toEqual(
      BURN_DEPLOY_ARGS_JSON
    );
  });

  xit("Should correctly construct deploy for 'balance_of'", async () => {
    const balanceOfDeploy = cc.storeBalanceOf(
      {
        tokenOwner: MOCKED_OWNER_PUBKEY,
        keyName: "abc",
      },
      "1000000000",
      keyPair.publicKey
    );

    const JSONDeploy = DeployUtil.deployToJson(balanceOfDeploy) as any;

    expect(balanceOfDeploy).toBeInstanceOf(DeployUtil.Deploy);
    expect(JSONDeploy.deploy.session.ModuleBytes.args).toEqual(
      BALANCE_OF_DEPLOY_ARGS_JSON
    );
  });
});
