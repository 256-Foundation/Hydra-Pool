# hydra-pool
Open Source Bitcoin Mining Pool

We use ansible to deploy an instance of hydrapool on your linux server.

See the [README.md](./ansible/README.md) to setup your instance of hydrapool.

tl;dr of running an instance is:

1. Get SSH access to a server
2. Install ansible on your laptop/desktop (called the control machine)
3. Copy `inventory.ini` to your own inventory, say `my-hydrapool-inventory.ini`
3. Run `ansible-playbook -i my-hydrapool-inventory.ini hydrapool.yml --ask-become-pass`
4. It'll ask for you user password, set up hydrapool and start running it. 

For the moment we only support the solo mining with hydrapool.
