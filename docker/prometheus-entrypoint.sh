#!/bin/sh
set -e

# Check if custom prometheus config exists, otherwise use default
if [ -f /etc/prometheus/prometheus.custom.yml ]; then
  echo "Using custom prometheus configuration from prometheus.custom.yml"
  CONFIG_FILE=/etc/prometheus/prometheus.custom.yml
else
  echo "Using default prometheus configuration"
  CONFIG_FILE=/etc/prometheus/prometheus.yml
fi

# Start prometheus with the selected config
exec /bin/prometheus --config.file="$CONFIG_FILE" "$@"
