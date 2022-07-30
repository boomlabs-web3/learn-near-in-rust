const nearAPI = require("near-api-js"); // repl에선 건너뜀
const sha256 = require("js-sha256");

// const provider = new nearAPI.providers.JsonRpcProvider(
//   `https://rpc.testnet.near.org`
// );

let connInfo = { url: "https://rpc.testnet.near.org" };
// sets up a NEAR API/RPC provider to interact with the blockchain
const provider = new nearAPI.providers.JsonRpcProvider(connInfo);

// ~/.near-credentials 폴더에 저장된 sender.testnet.json 파일 참고하여 pk 수정
const privateKey = ""; // eg. const privateKey = "ed25519:2xiGAH5YDniiYocF53HY5bBQHNdHrMVsoakC32Rcxi4cAUeq66ch9AXfQhqTp9zKyaaYHrskizYeJmaLwdPdYRye";
const keyPair = nearAPI.utils.key_pair.KeyPairEd25519.fromString(privateKey);

// 본인의 testnet 계정 입력
const sender = ""; // eg. const sender = "sender.testnet"

async function main() {
  const publicKey = keyPair.getPublicKey();
  const accessKey = await provider.query(
    `access_key/${sender}/${publicKey.toString()}`,
    ""
  );

  const nonce = ++accessKey.nonce;

  const amount = nearAPI.utils.format.parseNearAmount("1");
  const actions = [nearAPI.transactions.transfer(amount)];

  const recentBlockHash = nearAPI.utils.serialize.base_decode(
    accessKey.block_hash
  );

  const transaction = nearAPI.transactions.createTransaction(
    sender,
    publicKey,
    "boomlabs.testnet",
    nonce,
    actions,
    recentBlockHash
  );

  const serializedTx = nearAPI.utils.serialize.serialize(
    nearAPI.transactions.SCHEMA,
    transaction
  );

  const serializedTxHash = new Uint8Array(sha256.array(serializedTx));

  const signature = keyPair.sign(serializedTxHash);

  const signedTransaction = new nearAPI.transactions.SignedTransaction({
    transaction,
    signature: new nearAPI.transactions.Signature({
      keyType: transaction.publicKey.keyType,
      data: signature.signature,
    }),
  });

  try {
    const signedSerializedTx = signedTransaction.encode();
    // const result = await provider.sendJsonRpc(
    //     'broadcast_tx_commit',
    //     [Buffer.from(signedSerializedTx).toString('base64')]
    // );

    console.log(
      "Transaction Results: ",
      Buffer.from(signedSerializedTx).toString("base64")
    );
  } catch (error) {
    console.log(error);
  }
}

main();
