# Interact with RPC node in 3 ways
In this section, we will learn how to interact with Near RPC node by sending 1 NEAR testnet token.  
>__Mission: Sending 1 NEAR token to `boomlabs.testnet` in 3 ways__

## By Using near-cli (HIGH LEVEL) 
* Docs Reference: [NEAR CLI](https://docs.near.org/tools/near-cli#near-send)
* Github Reference: [`near/near-cli`](https://github.com/near/near-cli)
```bash
near login
near send sender.testnet boomlabs.testnet 1
```
## By Using near-api-js (MID LEVEL)
* Docs Reference: [Create a Transaction](https://docs.near.org/integrator/create-transactions#high-level----create-a-transaction)
```bash
near repl
```
We will use repl environment in this tutorial.
input `near repl` in CLI to connect a near-api-js repl environment .
```javascript
const nearAPI  = require('near-api-js'); // skip this line in repl environment
const { connect, KeyPair, keyStores, utils } = nearAPI;
```
Import `connect`, `KeyPair`, `keyStores`, `utils` from nearAPI
```javascript
// set keystore from local storage
const homedir = require("os").homedir();
const CREDENTIALS_DIR = ".near-credentials";
const credentialsPath = require("path").join(homedir, CREDENTIALS_DIR);
const myKeyStore = new keyStores.UnencryptedFileSystemKeyStore(credentialsPath);
```
create KeyStores object from keys stored in local storage (~/.near-credentials in Mac OS)
```javascript
const connectionConfig = {
  networkId: "testnet",
  keyStore: myKeyStore,
  nodeUrl: "https://rpc.testnet.near.org",
  walletUrl: "https://wallet.testnet.near.org",
  helperUrl: "https://helper.testnet.near.org",
  explorerUrl: "https://explorer.testnet.near.org",
};
const nearConnection = await connect(connectionConfig);
```
create configuration to connect with NEAR blockchain, and connect with NEAR blockchain by using this config (via RPC call).
```javascript
const senderAccount = await nearConnection.account("sender.testnet");
```
get sender's account object. Modify `"sender.testnet"` to your account address. If you type senderAccount in repl environment, you can get account object below.
```bash
Account {
  accessKeyByPublicKeyCache: {},
  connection: Connection {
    networkId: 'testnet',
    provider: JsonRpcProvider { connection: [Object] },
    signer: InMemorySigner { keyStore: [UnencryptedFileSystemKeyStore] }
  },
  accountId: 'c0wjay.boomlabs.testnet'
}
```
```javascript
// converts NEAR amount into yoctoNEAR (10^-24) using a near-api-js utility
const amount = utils.format.parseNearAmount('1');
const result = await senderAccount.sendMoney('boomlabs.testnet', amount);
```
finally send 1 NEAR from your account to `boomlabs.testnet`.
and you can get transaction result in result object, so type `result` in repl environment!
You can get result object below.
```bash
{
  receipts_outcome: [
    {
      block_hash: 'Htd8w4wvFJUmW46wuZjm3ePijFrxvWTHVdg7A5opkoEJ',
      id: 'DYDjn4DvCR2EsCFYKoypiPmQTS27RpmHDDDAS46ZDeF8',
      outcome: [Object],
      proof: [Array]
    },
    {
      block_hash: '6oJC2fX7jcGGWc813qx5Exaxk8YtsYmtYjCxYQTPge3R',
      id: '49fmFe1ctnaTeCuwi3yb4NqXiRP2NsNCsmRV58nGtqnR',
      outcome: [Object],
      proof: []
    }
  ],
  status: { SuccessValue: '' },
  transaction: {
    actions: [ [Object] ],
    hash: 'J18eUC9EV2ev6cA5TBy1SyrMGwMKnVv4Auxxx1QFsZvv',
    nonce: 95390718000009,
    public_key: 'ed25519:7Tusp9kQTm6JC1vsH8475McHLofHBDM2fK2qFc1QciKr',
    receiver_id: 'boomlabs.testnet',
    signature: 'ed25519:5sGHon1wYUwvX2KaNwz44XPEYk3VHbbvtxy9tjCHZZdMn2LLioEcK46tTTazEM3xqaHwFGF7AKmhR8nAC2y57nDA',
    signer_id: 'c0wjay.boomlabs.testnet'
  },
  transaction_outcome: {
    block_hash: 'zwzegqpdQuGCZ2CsUpJYyPa4k7L7FKqdfJXBMgiZ9iz',
    id: 'J18eUC9EV2ev6cA5TBy1SyrMGwMKnVv4Auxxx1QFsZvv',
    outcome: {
      executor_id: 'c0wjay.boomlabs.testnet',
      gas_burnt: 223182562500,
      logs: [],
      metadata: [Object],
      receipt_ids: [Array],
      status: [Object],
      tokens_burnt: '22318256250000000000'
    },
    proof: [ [Object], [Object] ]
  }
}
```
## By Using near-api-js & postman (LOW LEVEL)
* Docs Reference: [Create a Transaction](https://docs.near.org/integrator/create-transactions#low-level----create-a-transaction)

In this tutorial, we will use `near-meetup-example.ts` file stored in this branch.
```javascript
const nearAPI  = require('near-api-js');
const sha256 = require('js-sha256');
```
Import nearAPI & sha256 from corresponding libralies.
```javascript
const provider = new nearAPI.providers.JsonRpcProvider(`https://rpc.testnet.near.org`);
```
Set NEAR API/RPC provider.
```javascript
// Refer sender.testnet.json file stored in .near-credentials folder.
const privateKey = "";
const keyPair = nearAPI.utils.key_pair.KeyPairEd25519.fromString(privateKey);
```
In this line, please refer `'your account address'.testnet.json` file stored in local key storage. If your computer is Mac OS, it will stored in `~/.near-credentials/testnet/`
Copy & paste the value of private key in first line.
Then, second line will create keyPair from privateKey.
```javascript
const sender = 'sender.testnet';
const publicKey = keyPair.getPublicKey();
const accessKey = await provider.query(`access_key/${sender}/${publicKey.toString()}`, '');
```
Modify `'sender.testnet'` to your account address.
This lines will get publicKey from keyPair, and query access key of your account via public key.
```javascript
const nonce = ++accessKey.nonce;
```
Get nonce value from access key, and increase 1 of that value. Increment should be applied, because nonce value is always larger than previous one in NEAR.
```javascript
const amount = nearAPI.utils.format.parseNearAmount('1');
const actions = [nearAPI.transactions.transfer(amount)];
```
Create Action that means transferring 1 NEAR. This will be put in transaction
```javascript
const recentBlockHash = nearAPI.utils.serialize.base_decode(accessKey.block_hash);
```
Serialize recent block hash to byte array. Block hash can get from accesskey, and this will proove that transaction was created within 24 hours.
```javascript
const transaction = nearAPI.transactions.createTransaction(
    sender, 
    publicKey, 
    'boomlabs.testnet', 
    nonce, 
    actions, 
    recentBlockHash
  );
```
Create transaction
```javascript
const serializedTx = nearAPI.utils.serialize.serialize(
    nearAPI.transactions.SCHEMA, 
    transaction
  );
```
Borsh serialize this transaction.
```javascript
const serializedTxHash = new Uint8Array(sha256.array(serializedTx));
```
Get Hash value of this transaction via sha256.
```javascript
const signature = keyPair.sign(serializedTxHash);
```
Create signature via your key pair & serialized transaction hash.
```javascript
const signedTransaction = new nearAPI.transactions.SignedTransaction({
    transaction,
    signature: new nearAPI.transactions.Signature({ 
      keyType: transaction.publicKey.keyType, 
      data: signature.signature 
    })
  });
```
Create signed transaction object via signing transaction by your signature.
```javascript
const signedSerializedTx = signedTransaction.encode();
const result = await provider.sendJsonRpc(
      'broadcast_tx_commit', 
      [Buffer.from(signedSerializedTx).toString('base64')]
    );
```
Borsh serialize of signed transaction, and encode by base64, then RPC call it & get result. In this example, we will not try RPC call directly in typescript file, but get SignedSerializedTx, and RPC call it via Postman.
```bash
ts-node near-meetup-example.ts
```
finally, you can type command above to run this scrypt. Then you will get SignedSerializedTx value in CLI log like below
```bash
Transaction Results: DgAAAHNlbmRlci50ZXN0bmV0AOrmAai64SZOv9e/naX4W15pJx0GAap35wTT1T/DwcbbDwAAAAAAAAAQAAAAcmVjZWl2ZXIudGVzdG5ldNMnL7URB1cxPOu3G8jTqlEwlcasagIbKlAJlF5ywVFLAQAAAAMAAACh7czOG8LTAAAAAAAAAGQcOG03xVSFQFjoagOb4NBBqWhERnnz45LY4+52JgZhm1iQKz7qAdPByrGFDQhQ2Mfga8RlbysuQ8D8LlA6bQE=
```
Copy it, and RPC call via postman by sending json under here. You can refer [NEAR Docs](https://docs.near.org/api/rpc/setup#postman-setup) to learn how to use postman.
```json
{
  "jsonrpc": "2.0",
  "id": "dontcare",
  "method": "broadcast_tx_async",
  "params": [
    "DgAAAHNlbmRlci50ZXN0bmV0AOrmAai64SZOv9e/naX4W15pJx0GAap35wTT1T/DwcbbDwAAAAAAAAAQAAAAcmVjZWl2ZXIudGVzdG5ldNMnL7URB1cxPOu3G8jTqlEwlcasagIbKlAJlF5ywVFLAQAAAAMAAACh7czOG8LTAAAAAAAAAGQcOG03xVSFQFjoagOb4NBBqWhERnnz45LY4+52JgZhm1iQKz7qAdPByrGFDQhQ2Mfga8RlbysuQ8D8LlA6bQE="
  ]
}
```
## NEXT STEP: [Learn Basic Structure of NEAR Contract](https://github.com/boomlabs-web3/near-meetup/tree/2.contract/template)
