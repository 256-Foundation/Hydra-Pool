# Hydrapool

Open Source Bitcoin Mining Pool with support for solo mining and PPLNS
accounting.

# Running Your Own Hydrapool Instance

Binaries are available on the
[releases](https://github.com/256-Foundation/Hydra-Pool/releases)
page. We provide Linux and MacOS binaries for aarch64 and x86_64.

### Start Hydrapool

```
/path/to/hydrapool --config config.toml
```

### Start Dashboard

Hydrapool ships with a prometheus/grafana dashboard to track the
hashrate of the pool and individual users and ASICs miners.

The easiest way to run the dashboard is using docker. There is a
docker compose file for prometheus with configuration provided in the
.env file.

```
git clone https://github.com/256-foundation/Hydra-Pool/
cd Hydra-Pool
cp .env.example .env   # edit passwords,ports,etc
docker compose up -d
```

To provide public facing dashboard, we recommend using nginx/apache as
a reverse proxy and running the dashboard as a system service.

## Config

Before starting the pool, make sure you edit config.toml to point to
your bitcoin node.

You will also need to provide a bootstrap address. This address gets
the payout if in the extremely unlikely case the first few shares find
a bitcoin block. We build a new template every 10 seconds, so this
address can get lucky in the first 10 seconds.

We recommend pointing a single machine and warm up the pool for 10
seconds before pointing more hashrate to the pool.


### Securing your Server

At the very least, change the API Server password in your
config.toml. We provide a command line tool to generate the salt and
hashed password to use in your config file.

```
/path/to/hydrapool_cli gen-auth <USERNAME> <PASSWORD>
```

The above will generate config lines for pasting into your
config.toml.

Once the password is changed, you need to share that with your
prometheus setup.

Edit the file prometheus/prometheus.yaml and change the username and
password as you output by hydrapool_cli above.

```yaml
    basic_auth:
      username: '<USERNAME>'
      password: '<PASSWORD>'

```

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
