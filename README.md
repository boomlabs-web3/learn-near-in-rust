# Upgrade Pre-deployed Smart Contract
In this section, we will upgrade FT contract which was deployed in previous section.

* Docs Reference: [Schema Migration](https://welcome.near.university/developers/contract-patterns/schema-migration)

```bash
git checkout 4.contract/upgraded-ft
export USER="sender.testnet"
yarn build && near deploy --accountId $USER --wasmFile export/main.wasm
```
Modify second line, `"sender.testnet"` to your account address.
![image](https://user-images.githubusercontent.com/96561121/187223648-cce22b15-0adb-4962-a6ab-2b4e9d503b3f.png)
