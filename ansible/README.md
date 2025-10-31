
NOT RECOMMENDED. THE ANSIBLE TEMPLATES ARE DEPRECATED.

# Hydra Pool Ansible Deployment

This directory contains Ansible playbooks and templates to deploy the
Hydra Pool services on a bare metal Linux server using Docker
containers from the GitHub Container Registry
(ghcr.io/256-Foundation).

## Prerequisites

- Target server running a supported Linux distribution:
  - Debian/Ubuntu
  - RHEL/CentOS/Fedora
  - Arch Linux
  - SUSE/openSUSE
- SSH access to the target server
- [Ansible installed](https://docs.ansible.com/ansible/latest/installation_guide/intro_installation.html) on your local machine

## Configuration

1. Update the `inventory.ini` file with your server details:

```ini
[hydrapool_servers]
hydrapool-node ansible_host=YOUR_SERVER_IP ansible_user=YOUR_USERNAME

[all:vars]
network=signet  # Default network, can be 'signet' or 'testnet4'
```

2. Configure sudo authentication using one of these methods:

   - **Method 1:** Use the `--ask-become-pass` flag when running the playbook:
   ```bash
   ansible-playbook -i inventory.ini hydrapool.yml --ask-become-pass
   ```

   - **Method 2:** Edit `inventory.ini` to include your sudo password:
   ```ini
   [all:vars]
   ansible_become_password=YOUR_SUDO_PASSWORD
   ```
   
   - **Method 3:** Configure passwordless sudo on your target server (recommended for production):
   ```bash
   # On the target server, add to /etc/sudoers.d/USERNAME
   USERNAME ALL=(ALL) NOPASSWD: ALL
   ```

I personally prefer method 1.

## Deployment

To deploy the Hydra Pool services:

```bash
# Deploy with default network (signet)
ansible-playbook -i inventory.ini hydrapool.yml --ask-become-pass

# Deploy with specific network
ansible-playbook -i inventory.ini hydrapool.yml -e "network=testnet4" --ask-become-pass
```

## Supported Linux Distributions

The Ansible playbook automatically detects and configures the services
for different Linux distributions:

- **Debian/Ubuntu:** Uses apt package manager
- **RHEL/CentOS/Fedora:** Uses dnf package manager
- **Arch Linux:** Uses pacman package manager
- **SUSE/openSUSE:** Uses zypper package manager

When you run the ansible playbook, each Docker will be installed
through its native package manager and repositories.

## Service Management

The deployment creates systemd services that will automatically start
on boot and restart if they crash.

To check their status, ssh into your server and use the following
systemd commands to check status.

```bash
# Check status of services
sudo systemctl status hydrapool

# Restart services
sudo systemctl restart hydrapool
```

## Data Directories

On your server the following directories will have the logs for the various services.

- Hydrapool data: `/var/lib/hydrapool/hydrapool/`
- Logs: `/var/log/hydrapool/`

## Ports

We use the following ports on the server.

- Bitcoin P2P: 38333 (signet) / 48333 (testnet4) / 8333 (mainnet)
- Bitcoin RPC: 38332 (signet) / 48332 (testnet4) / 8332 (mainnet)

## Security Considerations

- The current configuration opens RPC access to all IPs (0.0.0.0/0)
- For production use, restrict RPC access in the bitcoin configuration
  templates. See templates/bitcoin.*.conf.j2.
