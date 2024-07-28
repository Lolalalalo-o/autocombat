//import initHostBind, * as hostbind from "./wasmbind/hostbind.js";
import { query, LeHexBN, ZKWasmAppRpc } from "zkwasm-ts-server";

const CMD_PLACE_TOWER = 1n;
const CMD_WITHDRAW_TOWER = 2n;
const CMD_MINT_TOWER = 3n;
const CMD_DROP_TOWER = 4n;
const CMD_UPGRADE_TOWER = 4n;

function createCommand(nonce: bigint, command: bigint, feature: bigint) {
  return (nonce << 48n) + (feature << 8n) + command;
}

let account = "1234";

//const rpc = new ZKWasmAppRpc("http://localhost:3000");
const rpc = new ZKWasmAppRpc("http://114.119.187.224:8085");

async function main() {
  //sending_transaction([0n,0n,0n,0n], "1234");
  let towerId = 0n;
  let x = 0n;
  let y = 0n;
  let state:any = await rpc.queryState(account);
  let data = JSON.parse(state.data);

  console.log("player info:");
  console.log(data.player);

  console.log("monsters info:");
  console.log(data.global.monsters);


  console.log("towers info:");
  console.log(data.global.towers);

  let config = await rpc.query_config();
  console.log("config", config);
}

main();
// sending_transaction([2n<<32n,2n + (1n<<8n) + (3n<<16n),0n,0n], "1234");

