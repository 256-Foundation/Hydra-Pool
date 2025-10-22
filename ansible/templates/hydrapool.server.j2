[Unit]
Description=P2Pool service for Hydrapool on {{ network }}
After=hydrapool-ckpool.service
Requires=hydrapool-ckpool.service

[Service]
Type=simple
User=root
Group=root
ExecStartPre=-/usr/bin/docker pull ghcr.io/256-foundation/hydrapool-p2pool:{{p2pool_image_tag}}
ExecStart=/usr/bin/docker run --rm --name hydrapool-p2pool \
  --network hydrapool_network \
  -e P2POOL_BITCOIN_NETWORK={{ network }} \
  -e P2POOL_BITCOIN_URL=hydrapool-bitcoind:{{ btc_rpc_port }} \
  -e P2POOL_STORE_PATH=/data/{{ network }} \
  -e P2POOL_CKPOOL_HOST=hydrapool-ckpool \
  -e RUST_LOG=info \
  -v {{ data_dir }}/hydrapool:/data \
  -v {{ data_dir }}/p2pool-config.toml:/p2pool/config.toml:ro \
  ghcr.io/256-foundation/hydrapool-p2pool:{{p2pool_image_tag}} \
  /usr/local/bin/p2poolv2 --config=/p2pool/config.toml
ExecStop=/usr/bin/docker stop hydrapool-p2pool
Restart=always
RestartSec=30
StandardOutput=journal
StandardError=journal
SyslogIdentifier=hydrapool-p2pool

[Install]
WantedBy=multi-user.target