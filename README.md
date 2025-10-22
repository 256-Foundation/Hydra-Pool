# Hydrapool

[Hydrapool](https://hydrapool.org) is an open Source Bitcoin Mining
Pool with support for solo mining and PPLNS accounting.

## Features

1. Run a private solo pool or a private PPLNS pool for your community
   of miners.
2. Payouts are made directly from coinbase - pool operator doesn't
   custody any funds. No need to trust the pool operator.
3. Let users download and validate the accounting of shares. We
   provide an API for the same. See [API Server](#api)
3. Prometheus and Grafana based dashboard for pool, user and worker
   hashrates and uptimes.
4. Use any bitcoin node that supports bitcoin RPC.
5. Implemented in Rust, for ease of extending the pool with novel
   accounting and payout schemes.
6. Open source with GPlv3. Feel free to extend and/or make changes.

## Limitation

Currently we limit 20 users, but this limit will be removed in the
future. This limit is in place to allow all older hardware that has a
limit on the size of the coinbase.

# Running Your Own Hydrapool Instance

## Install Hydrapool

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/256-Foundation/Hydra-Pool/releases/latest/download/hydrapool-installer.sh | sh
```

Binaries are available on the
[releases](https://github.com/256-Foundation/Hydra-Pool/releases)
page. We provide Linux, Windows and MacOS binaries. Go to releases
page to access an older release.

## Start Hydrapool

Download the config file from
[config.toml](https://github.com/256-Foundation/Hydra-Pool/blob/main/config.toml)
and edit it for your addresses, donation and fee amounts as well as
your bitcoin RPC ports and credentials. Als consider securing your
server with new API Server credentials. See [Securing Your
Server](#secure).

Once your config.toml file is ready, start the pool as:

```
hydrapool --config config.toml
```

We recommend running a systemd service for a pool you want others to
access. A sample service is provided as
[hydrapool.service](hydrapool.service).

### Start Dashboard

Hydrapool ships with a prometheus/grafana dashboard to track the
hashrate of the pool and individual users and ASICs miners.

The easiest way to run the dashboard is using docker. There is a
docker compose file for prometheus.

```
git clone https://github.com/256-foundation/Hydra-Pool/
cd Hydra-Pool
docker compose up -d
```

To provide public facing dashboard, we recommend using nginx/apache as
a reverse proxy and running the dashboard as a system service.

Also see the section on securing the server for securing your API
server.

## Config

Before starting the pool, make sure you edit config.toml to point to
your bitcoin node.

You will also need to provide a bootstrap address. This address gets
the payout if in the extremely unlikely case the first few shares find
a bitcoin block. We build a new template every 10 seconds, so this
address can get lucky in the first 10 seconds.

We recommend pointing a single machine and warm up the pool for 10
seconds before pointing more hashrate to the pool.

<a id="secure"></a>
## Securing your Server

At the very least, change the API Server password in your
config.toml. We provide a command line tool to generate the salt and
hashed password to use in your config file.

```
/path/to/hydrapool_cli gen-auth <USERNAME> <PASSWORD>
```

The above will generate config lines for pasting into your
config.toml.

Once the password is changed, you need to share that with your
prometheus setup. The same credentials will also be used to access the
API Server.

Edit the file prometheus/prometheus.yaml and change the username and
password as you output by hydrapool_cli above.

```yaml
    basic_auth:
      username: '<USERNAME>'
      password: '<PASSWORD>'
```

<a id="api"></a>
## API Server

When you start the mining pool an API server is also started on the
port you specify in the config file.

The API Server is secured using the credentials you provide in the
config file. These credentials are used by prometheus to build the
dashboard and for your users to download shares for validating the
accounting and payouts.

Go to `http://<your_server_ip>:<your_api_server_port>/pplns_shares` to
download a json file of all the PPLNS Shares tracked by the pool for
distributing the block rewards.

The above URL accepts optional query parameters `start_time` and
`end_time` in RFC3339 format, e.g. `1996-12-19T16:39:57-08:00` to
limit the range of pplns shares to download.

We will add new config options soon to rate limit the API Server for
easier administration of the server.

To expose the API Server to public, we recommend using nginx as a
reverse proxy for the port, just like for the prometheus/grafana
dashboard.


# Other Options to Run Hydrapool

## Build from Source

To build from source, use cargo.

```
git clone https://github.com/256-foundation/Hydra-Pool/
cargo build --release
```

Then run from target directory.

```
./target/release/hydrapool
```

## Run with Docker

We provide Dockerfile and docker compose files to run hydrapool using
Docker as well.

```bash
docker compose build hydrapool
docker compose --profile hydrapool up
```

Using the profile option we can start hydrapool along with prometheus
and grafana.

If you don't provide the profile option, hydrapool won't be started
and only prometheus and grafana will start as normal.

When using docker, be careful that you need to build docker image
after changes to config. Also you need to make sure your bitcoin RPC
port is accessible from the docker container.
