# NEX Example

<https://docs.nats.io/using-nats/nex/getting-started/deploying-services/>
<https://github.com/nats-io/nats.go/blob/main/micro/README.md/>

## Setup

Install UTM and download a Linux VM ([Ubuntu 22.04](https://mac.getutm.app/gallery/) is a good place to start). 
Create a Shared Directory pointing to `~/Downloads` (or similar).

```bash

Log in and create a directory for Nex-related assets:

```bash
mkdir /home/ubuntu/nex
```

Create a Nex configuration file named `config.json` in the `nex` directory:

```json
{
    "default_resource_dir": "/tmp/wd",
    "kernel_filepath": "/home/ubuntu/nex/vmlinux-5.10",
    "rootfs_filepath": "/home/ubuntu/nex/rootfs.ext4",
    "machine_pool_size": 1,
    "cni": {
        "network_name": "fcnet",
        "interface_name": "veth0"
    },
    "machine_template": {
        "vcpu_count": 1,
        "memsize_mib": 256
    },
    "tags": {
        "simple": "true"
    },
    "no_sandbox": true
}
```

## Install and Configure Nex

Install Nex:

```bash
sudo apt update && sudo apt install -y jq
curl -sSf https://nex.synadia.com/install.sh | sudo sh
curl -sf https://binaries.nats.dev/nats-io/nats-server/v2@v2.10.16 | sudo sh
curl -sf https://binaries.nats.dev/nats-io/natscli/nats@latest | sudo sh

# move installed binaries to `/usr/local/bin`
sudo mv nats-server /usr/local/bin
sudo mv nats /usr/local/bin
```

Configure Nex:

```bash
# install missing deps (rootfs.ext4, vmlinux-5.10)
sudo nex node preflight

# HACK: copy agent binary from rootfs  to /usr/local/bin
sudo mkdir /mnt/rootfs
sudo mount /home/ubuntu/nex/rootfs.ext4 /mnt/rootfs
sudo cp /mnt/rootfs/usr/local/bin/agent /usr/local/bin/nex-agent
```

## Run

Copy the service binary (see [README](README.md) for build instructions) to the Linux 
host:

```bash
# (from MacOS)
cp target/aarch64-unknown-linux-musl/release/wasmgrid ~/Downloads/wasmgrid

# (on Linux host)
sudo cp /mnt/macos/wasmgrid /home/ubuntu/nex/wasmgrid
```

Start NATS server Nex node:

```bash
nats-server -js
sudo nex node up
```

Run the service using Nex:

```bash
nex devrun /home/ubuntu/nex/wasmgrid --argv=http.wasm nats_url=nats://192.168.127.1:4222
```
