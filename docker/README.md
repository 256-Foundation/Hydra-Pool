# Running Hydrapool with Docker

This guide shows how to run Hydrapool using Docker and Docker Compose.

## Quick Start

### 1. Copy and edit the configuration file

```bash
cd docker
cp config-example.toml config.toml
```

Edit `config.toml` and configure:
- **`bootstrap_address`** - Your Bitcoin address for bootstrap mining. Default to 256 Foundation's donation address.
- **`zmqpubhashblock`** - Your Bitcoin node's ZMQ endpoint (e.g., `tcp://host.docker.internal:28334`)
- **`bitcoinrpc.url`** - Your Bitcoin RPC URL (e.g., `http://host.docker.internal:38332`)
- **`bitcoinrpc.username`** and **`bitcoinrpc.password`** - Your Bitcoin RPC credentials
- **`network`** - Set to `main`, `testnet4`, or `signet`

### 2. Grafana credentials

By default, Grafana uses `admin/admin` for username and password.

You can and should login and change this password before exposing your
grafana instance to the public.

### 3. Start Hydrapool

Run Hydrapool only:
```bash
docker compose up -d hydrapool
```

Or run with monitoring dashboards (Prometheus + Grafana):
```bash
docker compose up -d
```

### 4. Check status

```bash
docker compose logs -f hydrapool
docker compose ps
```

### 5. Access services

- **Stratum mining:** `stratum+tcp://localhost:3333`
- **API server:** `http://localhost:46884`
- **Grafana dashboard:** `http://localhost:3000` (if running with dashboards, login with credentials from step 2)
- **Prometheus:** `http://localhost:9090` (if running with dashboards)

## Updating Configuration

After editing `config.toml`, restart Hydrapool:

```bash
docker compose restart hydrapool
```

## Updating to Latest Release

To update to the latest released version:

```bash
docker compose pull
docker compose up -d
```

This pulls the latest images from GitHub Container Registry and restarts services with the new versions.

## Stopping Hydrapool

```bash
# Stop all services
docker compose down

# Stop and remove volumes (WARNING: deletes all data)
docker compose down -v
```

## Using CLI Tools

Easy to run shell script from the docker directory, i.e. this directory

```bash
./cli.sh --help
```

Or, if you want to directly run using docker compose:

```bash
docker compose run --rm hydrapool-cli --help
docker compose run --rm hydrapool-cli gen-auth myuser mypassword
```

## Data Persistence

Data is stored in Docker volumes:
- **hydrapool_data:** Database and stats
- **hydrapool_logs:** Log files
- **prometheus_data:** Prometheus metrics
- **grafana_data:** Grafana dashboards

To backup data:
```bash
docker run --rm -v hydrapool_data:/data -v $(pwd):/backup alpine tar czf /backup/hydrapool-backup.tar.gz /data
```

## Building from Source

By default, `docker-compose.yml` uses pre-built images from GitHub Container Registry.

For local development and testing changes:

```bash
# Build and run with local changes
docker compose -f docker-compose.yml -f docker-compose.dev.yml up --build

# Or rebuild specific service
docker compose -f docker-compose.yml -f docker-compose.dev.yml build hydrapool
docker compose -f docker-compose.yml -f docker-compose.dev.yml up -d hydrapool
```

The `docker-compose.dev.yml` file overrides the image references to build locally.

## Troubleshooting

### Cannot connect to Bitcoin node

If Hydrapool can't connect to your Bitcoin node running on the host:

1. Make sure your Bitcoin node is configured to accept RPC connections
2. Use `host.docker.internal` in URLs instead of `localhost` or `127.0.0.1`
3. Check that ZMQ is enabled in your `bitcoin.conf`:
   ```
   zmqpubhashblock=tcp://0.0.0.0:28334
   ```

### View logs

```bash
# Follow logs
docker compose logs -f hydrapool

# Last 100 lines
docker compose logs --tail=100 hydrapool
```

### Container won't start

Check the configuration file is valid:
```bash
docker compose config
```

## Advanced Usage

### Using Pre-built Images

Instead of building locally, use pre-built images from Docker Hub (when available):

```yaml
services:
  hydrapool:
    image: hydrapool/hydrapool:latest
    # ... rest of config
```

### Custom Network

To use a custom Docker network:

```bash
docker network create my-network
```

Then update `docker-compose.yml` to use the external network.

## Support

For more information, see the main [Hydrapool README](../README.md) or visit [hydrapool.org](https://hydrapool.org).
