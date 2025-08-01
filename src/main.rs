// Copyright (C) 2024, 2025 Hydra-Pool Developers (see AUTHORS)
//
// This file is part of Hydra-Pool.
//
// Hydra-Pool is free software: you can redistribute it and/or modify it under
// the terms of the GNU General Public License as published by the Free
// Software Foundation, either version 3 of the License, or (at your option)
// any later version.
//
// Hydra-Pool is distributed in the hope that it will be useful, but WITHOUT ANY
// WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with
// Hydra-Pool. If not, see <https://www.gnu.org/licenses/>.

use bitcoin::Address;
use clap::Parser;
use p2poolv2_lib::config::Config;
use p2poolv2_lib::logging::setup_logging;
use std::process::exit;
use std::str::FromStr;
use stratum::server::StratumServer;
use stratum::work::gbt::start_gbt;
use stratum::work::tracker::start_tracker_actor;
use stratum::zmq_listener::{ZmqListener, ZmqListenerTrait};
use tracing::error;
use tracing::info;

/// Interval in seconds to poll for new block templates since the last blocknotify signal
const GBT_POLL_INTERVAL: u64 = 60; // seconds

/// Path to the Unix socket for receiving blocknotify signals from bitcoind
pub const SOCKET_PATH: &str = "/tmp/p2pool_blocknotify.sock";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    config: String,
}

#[tokio::main]
async fn main() -> Result<(), String> {
    info!("Starting hydrapool...");
    // Parse command line arguments
    let args = Args::parse();

    // Load configuration
    let config = Config::load(&args.config);
    if config.is_err() {
        let err = config.unwrap_err();
        error!("Failed to load config: {err}");
        return Err(format!("Failed to load config: {err}"));
    }
    let config = config.unwrap();
    // Configure logging based on config
    let logging_result = setup_logging(&config.logging);
    // hold guard to ensure logging is set up correctly
    let _guard = match logging_result {
        Ok(guard) => {
            info!("Logging set up successfully");
            guard
        }
        Err(e) => {
            error!("Failed to set up logging: {e}");
            return Err(format!("Failed to set up logging: {e}"));
        }
    };

    let stratum_config = config.stratum.clone();
    let bitcoinrpc_config = config.bitcoinrpc.clone();
    let (_stratum_shutdown_tx, stratum_shutdown_rx) = tokio::sync::oneshot::channel();
    let (notify_tx, notify_rx) = tokio::sync::mpsc::channel(1);
    let tracker_handle = start_tracker_actor();

    let notify_tx_for_gbt = notify_tx.clone();
    let bitcoinrpc_config_cloned = bitcoinrpc_config.clone();
    // Setup ZMQ publisher for block notifications
    let zmq_trigger_rx = match ZmqListener.start(&stratum_config.zmqpubhashblock) {
        Ok(rx) => rx,
        Err(e) => {
            error!("Failed to set up ZMQ publisher: {}", e);
            return Err("Failed to set up ZMQ publisher".into());
        }
    };

    tokio::spawn(async move {
        if let Err(e) = start_gbt(
            &bitcoinrpc_config_cloned,
            notify_tx_for_gbt,
            SOCKET_PATH,
            GBT_POLL_INTERVAL,
            stratum_config.network,
            zmq_trigger_rx,
        )
        .await
        {
            tracing::error!("Failed to fetch block template. Shutting down. \n {}", e);
            exit(1);
        }
    });

    let connections_handle = stratum::client_connections::spawn().await;
    let connections_cloned = connections_handle.clone();

    let output_address = Address::from_str(stratum_config.solo_address.clone().unwrap().as_str())
        .expect("Invalid output address in Stratum config")
        .require_network(stratum_config.network)
        .expect("Output address must match the bitcoin network in config");

    let tracker_handle_cloned = tracker_handle.clone();
    tokio::spawn(async move {
        info!("Starting Stratum notifier...");
        // This will run indefinitely, sending new block templates to the Stratum server as they arrive
        stratum::work::notify::start_notify(
            notify_rx,
            connections_cloned,
            Some(output_address),
            tracker_handle_cloned,
        )
        .await;
    });

    let mut stratum_server = StratumServer::new(
        stratum_config,
        stratum_shutdown_rx,
        connections_handle.clone(),
    )
    .await;
    info!("Starting Stratum server...");
    let _result = stratum_server
        .start(None, notify_tx, tracker_handle, bitcoinrpc_config)
        .await;
    info!("Stratum server stopped");

    Ok(())
}
