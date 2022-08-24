We are going to deploy on devnet.

Here is your deployment checklist 🚀

1. Run anchor build. Your program keypair is now in target/deploy. Keep this keypair secret. You can reuse it on all clusters.
2. Run anchor keys list to display the keypair's public key and copy it into your declare_id! macro at the top of lib.rs.
3. Run anchor build again. This step is necessary to include the new program id in the binary.
4. Change the provider.cluster variable in Anchor.toml to devnet.
5. Run anchor deploy
6. Run anchor test



```bash
solana airdrop 1 3gs6QDEDCu5MbfkbwMDq6RfUAh9KN5XUF9tLp4qvf3AG --url devnet
```

```bash
solana config set --url https://api.devnet.solana.com
solana config set --url https://api.testnet.solana.com  
solana config set --url localhost
```