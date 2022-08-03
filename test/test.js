const {ApiPromise, WsProvider,keyring} = require('@polkadot/api')

const main() = async() => {
  const provider = new WsProvider('ws://127.0.0.1:9944');
  const api = await ApiPromise.create({provider});
  const keyring new Keyring({type:'sr25519'});
  const alice = keyring.addFromUri("//Alice");
  const bob = keyring.addFromUri("//Bob");

  const txs = [

  ]
}
