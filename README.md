<div align="center">
  <h1><code>foundry</code></h1>

  <p><b>Foundry is a CLI tool for automating and managing configurations across remote machines via SSH without requiring a dedicated agent on each target machine like <a href="https://github.com/johnnyfreeman/outpost">outpost</a>.
</b></p>
</div>

> [!WARNING]
> **This project is in its early stages and may undergo many breaking changes in the future.**

# Overview

Foundry focuses on simple, SSH-based automation across multiple operating systems, including RHEL 7, RHEL 9, Ubuntu 20.04, and Arch Linux. With an emphasis on reliability and modularity, Foundry provides a streamlined interface for handling configuration tasks, ensuring systems are configured precisely as specified.
Key Capabilities

- **Cross-Platform Compatibility**: Apply configurations consistently across multiple Linux distributions.
- **Agentless Execution**: Connects via SSH without requiring any agents on remote systems.
- **OS-Specific Task Management**: Supports common system tasks tailored to each OS.
- **Modular Design**: Designed to grow with additional tasks and features over time.

# How It Works

Foundry uses a TOML configuration file to define hosts, users, and tasks. It then reads this configuration, connects to each specified machine, and applies configurations based on the parameters provided.

# Setup Instructions

## Prerequisites

- **Rust**: Required to build Foundry.
- **SSH Access**: Ensure SSH connectivity to each remote machine.
- **Supported OSes**: Compatible with RHEL 7, RHEL 9, Ubuntu 20.04, and Arch Linux.

## Installation

Clone and Build:

```sh
git clone https://github.com/johnnyfreeman/foundry.git
cd foundry
cargo build --release
```

Run Foundry:

```sh
./target/release/foundry --config /path/to/config.toml
```

Example Configuration File

A sample configuration file in TOML format defines hosts and users. Foundry reads this file and applies configurations as instructed:

```toml
[[hosts]]
name = "server-01"
ip = "admin@192.168.1.101"

[[hosts]]
name = "server-02"
ip = "admin@192.168.1.102"

[[hosts]]
name = "server-03"
ip = "admin@192.168.1.103"

[[hosts]]
name = "server-04"
ip = "admin@192.168.1.104"

[[users]]
username = "admin_user"
password = "strongpassword456"
groups = ["admin"]
```

This configuration will connect to the specified servers, create user accounts, and assign them to the defined groups. Future support could extend this format to allow for additional task definitions, such as installing packages or configuring firewall rules.

# Usage

To execute configurations with Foundry:

```sh
foundry --config /path/to/config.toml
```

Foundry will connect to each host in the configuration and apply specified tasks, managing users and permissions as defined.
