# Fluxmeter

[![crates.io](https://img.shields.io/crates/v/fluxmeter.svg)](https://crates.io/crates/fluxmeter) [![MSRV](https://img.shields.io/crates/msrv/fluxmeter)](https://crates.io/crates/fluxmeter) [![license](https://img.shields.io/crates/l/fluxmeter.svg)](./LICENSE) [![downloads](https://img.shields.io/crates/d/fluxmeter)](https://crates.io/crates/fluxmeter)

Fluxmeter is a lightweight, self-hosted speedtest server written in Rust and Svelte. It embeds a static modern web interface directly into a single binary.

![Fluxmeter page](./assets/page.png)

## Getting Started

### Installation

| Method | Command / Source |
| --- | --- |
| **Pre-built Binary** | Download from [GitHub Releases](https://github.com/DarkCeptor44/fluxmeter/releases) |
| **Cargo** | `cargo install fluxmeter` |
| **Docker** | `docker run -p 7890:7890 ghcr.io/darkceptor44/fluxmeter:latest` |

### Docker Compose

A [`compose.yml`](./compose.yml) file is provided for convenience:

```yaml
services:
  flux:
    container_name: flux
    image: ghcr.io/darkceptor44/fluxmeter:latest
    restart: unless-stopped
    ports:
      - 7890:7890
    volumes:
      - /etc/localtime:/etc/localtime:ro # for correct timestamp in logs
```

### Building From Source

1. **Prerequisites:** Ensure you have the [Rust toolchain](https://rustup.rs/) and [Bun](https://bun.com/) installed.
2. Clone the repo:

    ```bash
    git clone https://github.com/DarkCeptor44/fluxmeter.git
    cd fluxmeter
    ```

3. Run or build the binary:

    ```bash
    # development
    cargo run

    # local installation
    cargo install --path .
    ```

## Usage

```console
$ fluxmeter
2026-09-22T10:49:07.4480111-03:00 [INFO]
===================================================
---------------- Fluxmeter v0.1.0 -----------------
===================================================

2026-09-22T10:49:07.4524887-03:00 [INFO]
    listening on http://0.0.0.0:7890
    listening on http://localhost:7890
```

```console
$ fluxmeter -h
A lightweight speedtest server

Usage: fluxmeter [OPTIONS]

Options:
  -H, --host <HOST>  Host to listen on [env: FM_HOST=] [default: 0.0.0.0]
  -p, --port <PORT>  Port to listen on [env: FM_PORT=] [default: 7890]
      --debug        Enable debug logging [env: FM_DEBUG=]
  -h, --help         Print help
  -V, --version      Print version
```

## MSRV

The minimum supported Rust version is:

| Version | Edition | MSRV |
| --- | --- | --- |
| `<= 0.1.0` | 2024 | 1.88.0 |

## Reverse Proxy

I wouldn't recommend putting this behind a reverse proxy unless your proxy is running on a decent machine. If you want higher throughput in the tests then you might need to use a L4 proxy instead. If you use Caddy you can try this for extra performance:

```Caddyfile
fluxmeter.yourdomain.com {
    reverse_proxy localhost:7890 {
        flush_interval -1
    }
}
```

But L4 instead of L7 is a better bet.

## Environment Variables

The following environment variables are currently supported, they are used if the CLI flags are not set:

| Variable | Default | Description |
| --- | --- | --- |
| `FM_HOST` | `0.0.0.0` | Host to listen on |
| `FM_PORT` | `7890` | Port to listen on |
| `FM_DEBUG` | `false` | Enable debug logging |

## Audits

| Auditor | Audit Date | Version | Vulnerabilities |
| --- | --- | --- | --- |
| [cargo-audit](https://crates.io/crates/cargo-audit) | 2026-09-22 | 0.1.0 | 0 |

## License

This project is licensed under the [Mozilla Public License, version 2.0](https://www.mozilla.org/MPL/2.0/). See the [LICENSE](LICENSE) file for details.
