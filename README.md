# Ainulindalë

*A Rust-based automation tool with a name inspired by Tolkien's legendarium.*

## Overview

**Ainulindalë** is an automation tool designed to orchestrate and manage system configurations across multiple operating systems. With a focus on flexibility, security, and performance, Ainulindalë leverages the power and safety of Rust to streamline infrastructure automation across diverse environments.

At the heart of Ainulindalë is **Eä**, the core engine that brings configurations to life, ensuring that each system is shaped according to your specifications.

## Features

- **Multi-OS Support**: Ensure consistent system management across diverse environments like RHEL 7, RHEL 9, Ubuntu 20.04, and Arch Linux.
- **SSH-Based Connectivity**: Securely connect to remote machines and execute commands without the need for agents.
- **User Management**: Handle user creation, password setting, and group assignments tailored to the specific OS.
- **Modular Design**: Easily extend functionality with custom modules and scripts.
- **Compiled Binary Deployment**: Distribute pre-built Rust binaries to remote machines, eliminating dependency on installed runtimes.

## Getting Started

### Prerequisites

To use Ainulindalë, you'll need:

- Rust installed on your local machine for building the tool.
- SSH access to the remote machines you want to manage.
- Remote machines should be running one of the supported operating systems: RHEL 7, RHEL 9, Ubuntu 20.04, or Arch Linux.

### Installation

1. **Clone the repository:**

    ```bash
    git clone https://github.com/yourusername/ainulindale.git
    cd ainulindale
    ```

2. **Build the tool:**

    ```bash
    cargo build --release
    ```

3. **Run the tool:**

    ```bash
    ./target/release/ainulindale
    ```

### Configuration

Ainulindalë uses a simple configuration file to define the tasks to be executed on remote machines. Here’s a basic example:

```toml
[hosts]
rhel7 = ["192.168.1.10", "192.168.1.11"]
ubuntu = ["192.168.1.12"]

[users]
create = [
    { username = "john_doe", password = "password123", groups = ["sudo", "users"] }
]
```

### Usage

To run Ainulindalë with your configuration file, use the following command:

```bash
./ainulindale --config /path/to/config.toml
```

Ainulindalë will connect to each machine listed in the configuration, ensure users are present, and perform any other defined tasks.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request with your ideas and improvements.

## License

Ainulindalë is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.

## Acknowledgments

While the name *Ainulindalë* is inspired by Tolkien’s *Silmarillion*, the functionality of this tool is independent and purely focused on robust and efficient infrastructure automation. The core engine, **Eä**, brings these configurations to life, ensuring your systems are precisely as you intend.

