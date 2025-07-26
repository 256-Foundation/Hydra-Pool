# Hydrapool

Open Source Bitcoin Mining Pool with support for solo mining and PPLNS accounting.

# Help Us Test

We have built our custom stratum implementation from scratch and we need to test that our implementation supports the various stratum client implementations. For this we need your help.

## How Can You Help?

We are running an test instance of hydrapool using a private signet network. All you need to do is point your miner to our stratum server for two to five minutes and then switch them off.

Stratum server details: `stratum+tcp://test.hydrapool.org:3333`

We will capture the stratum interaction, if it passes we will mark the self identified hardware as passing and we will credit you with helping us test for that hardware.

We publish a list of users who helped us test and which devices they helped test with on https://test.hydrapool.org. 

If the stratum session fails, we will have the interaction available in our logs and we can try and make the changes to our stratum implementation. Once the changes are released we will announce it on the [256 Foundation Telegram group](https://t.me/the256foundation).

## Bitaxe Users

Please upgrade to Bitaxe firmware v2.9.0 or higher. There are issues with older versions that result in disconnections from hydrapool and reconnect.

# Development Status

✅ The solo mining pool is functional.

🔜 PPLNS accounting is coming soon.

⏳ Stats pages are in the development queue.

If you want to help out with any of the development please ping us on the [256 Foundation Telegram group](https://t.me/the256foundation) 💬

# Running Your Own Hydrapool Instance

Currently we only support manual deployment. We will release rust binary and Linux packages once we are have PPLNS accounting working.

1. Make sure you are on Rust MSRV 1.88
2. Download latest source from https://github.com/256-Foundation/Hydra-Pool/archive/refs/heads/main.zip
3. Unzip the zip file and run `cargo build --release`
4. Make sure you have a bitcoind node running for your choice of network.
5. Edit config.toml file to edit the bitcoindrpc configuration and bitcoin network type.
5. Run `cargo run --release -- --config=config.toml`

We have docker and ansible scripts, which are not being maintained any longer.

